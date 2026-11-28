use std::collections::HashMap;

use super::axis::Axis;
use super::spec::{
    ArcSide, BossChamfer, BossSpec, CutoutSpec, HoleHead, HoleSpec, OutlinePocketSpec, PadSpec, PlateSpec, PocketSpec, ProfileSpec,
    SideBossSpec, SlotSpec, Wall,
};

use crate::design::{
    Design, DesignBuilder, Extent, FeatureKind, HoleKind, Operation, Param, PatternKind, Plane, PolygonAngle, Profiles, SlotDimensions,
    dimensioned_angle, dimensioned_sides, dimensioned_sides_for_angle, line_arc_tangent, polygon_sketch_arcs, polygon_sketch_dimensioned,
    polygons_sketch_dimensioned, rectangle_sketch, slot_sketch,
};
use crate::mesh::{Vec3, snap_dim, snap_mm};
use crate::patterns::{HoleGroup, group_holes_with_members, row_is_axis_aligned};

/// The expressions for a point's world coordinates: along the extrusion axis the face's
/// expression, along the profile axes the parameters named after their world axis letters.
fn world_exprs(prefix: &str, axis: Axis, face: &str) -> [String; 3] {
    let mut out = [const { String::new() }; 3];
    for (i, slot) in out.iter_mut().enumerate() {
        *slot = match axis.profile_axes().iter().position(|&a| a == i) {
            Some(k) => format!("{prefix}_{}", axis.profile_letter(k)),
            None => String::from(face),
        };
    }
    out
}

/// The two position parameters of a point `(u, v)` in the profile plane, named after their world
/// axis letters (mesh2cad's `hole_<n>_y`, `hole_<n>_z` on an X extrusion), in world coordinates
/// for a part whose bounding box's lowest corner is at `at`.
fn pos_params(b: &mut DesignBuilder, name: &str, axis: Axis, at: Vec3, p: [f64; 2]) {
    for (i, value) in p.iter().enumerate() {
        b.param(Param::user(format!("{name}_{}", axis.profile_letter(i)), snap_mm(axis.profile_world(at, i, *value))));
    }
}

/// The corner radii `radii` grouped by equal value, each group's radius with its corners' indices,
/// in the order the groups first occur. A fillet feature takes one radius, so the design writes
/// one feature per group; a single group keeps the plain parameter name (`fillet_r` /
/// `outline_fillet_r`) and feature name `Fillet1`, so designs with one radius stay as they were.
#[allow(clippy::float_cmp)] // exact equality is intended: the radii are snapped output values
fn radius_groups(radii: &[f64]) -> Vec<(f64, Vec<usize>)> {
    let mut groups: Vec<(f64, Vec<usize>)> = Vec::new();
    for (i, &r) in radii.iter().enumerate() {
        if let Some(g) = groups.iter_mut().find(|(gr, _)| *gr == r) {
            g.1.push(i);
        } else {
            groups.push((r, vec![i]));
        }
    }
    groups
}

impl PlateSpec {
    /// The plate as a parametric history, its corner at the origin; see [`PlateSpec::to_design_at`].
    #[must_use]
    pub fn to_design(&self, name: &str) -> Design {
        self.to_design_at(name, [0.0; 3])
    }

    /// The plate as a parametric history with its lowest corner at `at`, as mesh2cad writes it:
    /// parameters `base_x`, `base_y`, `base_t`, the centre `base_cx`, `base_cy` and the bottom
    /// `base_z0`, `hole_<n>_x/_y/_d` (in world coordinates) and `fillet_r` (or `fillet_r_<g>` per
    /// radius group when the corners are rounded with different radii); a sketch and an
    /// extrusion at the origin, a move to where the plate is, a hole per hole and a fillet of the
    /// four vertical corners (one `Fillet<g>` per radius group). A plate is always a Z extrusion.
    #[must_use]
    pub fn to_design_at(&self, name: &str, at: Vec3) -> Design {
        let axis = Axis::Z;
        let mut b = DesignBuilder::new(name);
        b.param(Param::user("base_x", self.x)).param(Param::user("base_y", self.y)).param(Param::user("base_t", self.z));
        b.param(Param::user("base_cx", snap_mm(at[0] + self.x / 2.0)))
            .param(Param::user("base_cy", snap_mm(at[1] + self.y / 2.0)))
            .param(Param::user("base_z0", snap_mm(at[2])));
        hole_params(&mut b, &self.holes, at, axis);
        slot_params(&mut b, &self.slots, at, axis);
        boss_params(&mut b, &self.bosses, at, axis);
        side_boss_params(&mut b, &self.side_bosses, at, self.bosses.len());
        block_params(&mut b, &self.pads, &self.pockets, at, axis);
        let fillet_groups = radius_groups(&self.outline_fillet_rs);
        if fillet_groups.len() == 1 {
            b.param(Param::user("fillet_r", fillet_groups[0].0));
        } else {
            for (g, (r, _)) in fillet_groups.iter().enumerate() {
                b.param(Param::user(format!("fillet_r_{}", g + 1), *r));
            }
        }
        if let Some(r) = self.edge_fillet_r {
            b.param(Param::user("edge_fillet_r", r));
        }
        b.param(Param::model("d1", "base_x")).param(Param::model("d2", "base_y"));

        let sketch = b.feature(
            "Sketch1",
            vec![],
            FeatureKind::Sketch { plane: Plane::Origin { name: "XY" }, sketch: rectangle_sketch(self.x, self.y) },
        );
        let extrude = b.feature(
            "Extrude1",
            vec!["Body1".into()],
            FeatureKind::Extrude { sketch, profiles: Profiles::All, extent: Extent::positive("base_t"), operation: Operation::NewBody },
        );
        b.feature(
            "Move1",
            vec![],
            FeatureKind::Move {
                bodies: vec!["Body1".into()],
                translate: ["base_cx - base_x / 2".into(), "base_cy - base_y / 2".into(), "base_z0".into()],
            },
        );
        if self.edge_fillet_r.is_some() {
            // All twelve edges of the box, as mesh2cad's `.edges().fillet(edge_fillet_r)` does;
            // the holes are cut after the fillet. The extrude's caps are `top` (at the far end)
            // and `start` (on the sketch plane); the sides are l1 bottom, l2 right, l3 top, l4
            // left (see `rectangle_sketch`).
            let mid = at[2] + self.z / 2.0;
            let (x0, y0, x1, y1) = (at[0], at[1], at[0] + self.x, at[1] + self.y);
            let side = |a: usize, b: usize| format!("F{extrude}:side:l{a}|F{extrude}:side:l{b}");
            let mut edge_names = vec![side(1, 4), side(1, 2), side(2, 3), side(3, 4)];
            let mut edges = vec![[x0, y0, mid], [x1, y0, mid], [x1, y1, mid], [x0, y1, mid]];
            let mids = [[f64::midpoint(x0, x1), y0], [x1, f64::midpoint(y0, y1)], [f64::midpoint(x0, x1), y1], [x0, f64::midpoint(y0, y1)]];
            for (k, m) in mids.iter().enumerate() {
                let i = k + 1;
                edge_names.push(format!("F{extrude}:side:l{i}|F{extrude}:top"));
                edges.push([m[0], m[1], at[2] + self.z]);
                edge_names.push(format!("F{extrude}:side:l{i}|F{extrude}:start"));
                edges.push([m[0], m[1], at[2]]);
            }
            b.feature("Fillet1", vec![], FeatureKind::Fillet { edge_names, edges, radius: "edge_fillet_r".into() });
        }
        cutout_features(&mut b, &self.cutouts, at, axis);
        let mut pending_rims = Vec::new();
        hole_features(
            &mut b,
            &self.holes,
            at,
            axis,
            "base_z0 + base_t",
            "base_z0",
            self.z,
            extrude,
            &[],
            0,
            crate::plate::tolerance(self.x.max(self.y).max(self.z)),
            &mut pending_rims,
        );
        slot_features(&mut b, &self.slots, at, axis);
        boss_features(&mut b, &self.bosses, at, self.z, axis, "base_z0 + base_t", extrude);
        side_boss_features(&mut b, &self.side_bosses, self.bosses.len());
        block_features(&mut b, &self.pads, &self.pockets, &[], at, axis, "base_z0 + base_t", "base_z0", &mut pending_rims);
        if !fillet_groups.is_empty() {
            let mid = at[2] + self.z / 2.0;
            let plate_corners = [[at[0], at[1]], [at[0] + self.x, at[1]], [at[0] + self.x, at[1] + self.y], [at[0], at[1] + self.y]];
            // Corner i joins line l<i> (ending there) and line l<i+1> (starting there), l1..l4
            // 1-based (see `rectangle_sketch`).
            let corner_side = |i: usize| {
                let (a, b) = (if i == 0 { 4 } else { i }, i + 1);
                format!("F{extrude}:side:l{}|F{extrude}:side:l{}", a.min(b), a.max(b))
            };
            for (g, (_, idxs)) in fillet_groups.iter().enumerate() {
                let (name, radius) = if fillet_groups.len() == 1 {
                    ("Fillet1".to_owned(), "fillet_r".to_owned())
                } else {
                    (format!("Fillet{}", g + 1), format!("fillet_r_{}", g + 1))
                };
                let edge_names = idxs.iter().map(|&i| corner_side(i)).collect();
                let edges = idxs.iter().map(|&i| [plate_corners[i][0], plate_corners[i][1], mid]).collect();
                b.feature(name, vec![], FeatureKind::Fillet { edge_names, edges, radius });
            }
        }
        b.build()
    }
}

impl ProfileSpec {
    /// The profile as a parametric history, its bounding box's corner at the origin; see
    /// [`ProfileSpec::to_design_at`].
    #[must_use]
    pub fn to_design(&self, name: &str) -> Design {
        self.to_design_at(name, [0.0; 3])
    }

    /// The profile as a parametric history with its bounding box's lowest corner at `at`,
    /// extruded along Z; see [`ProfileSpec::to_design_on`].
    #[must_use]
    pub fn to_design_at(&self, name: &str, at: Vec3) -> Design {
        self.to_design_on(name, at, Axis::Z)
    }

    /// The profile as a parametric history with its bounding box's lowest corner at `at`,
    /// extruded along `axis`, as mesh2cad writes an extruded outline: parameters `extrude_len`
    /// (the thickness), `extrude_0` (where it starts along the axis), `outline_<i>_len` per
    /// straight side, `outline_<i>_angle` on the longest slanted side, `arc_<i>_d` per arc side
    /// (its centre a free sketch point the solver places equidistant from the side's two
    /// corners), `hole_<n>_<a>/_<b>/_d` for the profile plane's world axis letters and
    /// `outline_fillet_r` (or `outline_fillet_r_<g>` per radius group when the corners are rounded
    /// with different radii);
    /// a sketch of the outline on the axis's plane (see [`Axis::plane`]), every side dimensioned
    /// by its own parameter (a line side's `Length`, an arc side's `Diameter`), an extrusion along
    /// the plane's normal, a move to where the part is,
    /// a fillet of every corner edge parallel to the axis (one `Fillet<g>` per radius group) and a
    /// hole per hole. The fillet comes
    /// before the holes: an inside corner's rounding adds material, and a hole may lie in it.
    #[must_use]
    pub fn to_design_on(&self, name: &str, at: Vec3, axis: Axis) -> Design {
        let mut b = DesignBuilder::new(name);
        let top = "extrude_0 + extrude_len";
        b.param(Param::user("extrude_len", self.z)).param(Param::user("extrude_0", snap_mm(at[axis.index()])));
        hole_params(&mut b, &self.holes, at, axis);
        slot_params(&mut b, &self.slots, at, axis);
        boss_params(&mut b, &self.bosses, at, axis);
        block_params(&mut b, &self.pads, &self.pockets, at, axis);
        let fillet_groups = radius_groups(&self.outline_fillet_rs);
        if fillet_groups.len() == 1 {
            b.param(Param::user("outline_fillet_r", fillet_groups[0].0));
        } else {
            for (g, (r, _)) in fillet_groups.iter().enumerate() {
                b.param(Param::user(format!("outline_fillet_r_{}", g + 1), *r));
            }
        }
        let arcs = self.arc_sides();
        let has_arcs = arcs.iter().any(Option::is_some);
        let (dim, arc_equals) = if has_arcs { arc_polygon_dim(&self.corners, arcs) } else { (polygon_dim(&self.corners), Vec::new()) };
        let outline_lens = side_len_params(&mut b, "outline", &self.corners, arcs, &dim);
        let outline_angle = if has_arcs { None } else { side_angle_param(&mut b, "outline", &self.corners) };
        let sketch_geom = if has_arcs {
            let arc_params = arc_side_params(&mut b, "arc", arcs, &dim);
            let geom: Vec<Option<([f64; 2], f64)>> = arcs.iter().map(|a| a.map(|a| (a.centre, a.r))).collect();
            polygon_sketch_arcs(&self.corners, &geom, &outline_lens, &arc_params, &arc_equals)
        } else {
            polygon_sketch_dimensioned(&self.corners, &outline_lens, outline_angle.as_ref())
        };
        let sketch = b.feature("Sketch1", vec![], FeatureKind::Sketch { plane: Plane::Origin { name: axis.plane() }, sketch: sketch_geom });
        let extrude = b.feature(
            "Extrude1",
            vec!["Body1".into()],
            FeatureKind::Extrude {
                sketch,
                profiles: Profiles::All,
                extent: Extent::positive("extrude_len"),
                operation: Operation::NewBody,
            },
        );
        b.feature("Move1", vec![], FeatureKind::Move { bodies: vec!["Body1".into()], translate: axis.translate(at) });
        if !fillet_groups.is_empty() {
            let n = self.corners.len();
            for (g, (_, idxs)) in fillet_groups.iter().enumerate() {
                let (name, radius) = if fillet_groups.len() == 1 {
                    ("Fillet1".to_owned(), "outline_fillet_r".to_owned())
                } else {
                    (format!("Fillet{}", g + 1), format!("outline_fillet_r_{}", g + 1))
                };
                // Corner i joins line l<i> (ending there) and line l<i+1> (starting there).
                let edge_names = idxs
                    .iter()
                    .map(|&i| {
                        let (a, b) = (if i == 0 { n } else { i }, i + 1);
                        let (a, b) = (a.min(b), a.max(b));
                        format!("F{extrude}:side:l{a}|F{extrude}:side:l{b}")
                    })
                    .collect();
                let edges = idxs.iter().map(|&i| axis.world(at, self.corners[i][0], self.corners[i][1], self.z / 2.0)).collect();
                b.feature(name, vec![], FeatureKind::Fillet { edge_names, edges, radius });
            }
        }
        cutout_features(&mut b, &self.cutouts, at, axis);
        let mut pending_rims = Vec::new();
        let scale = {
            let (mut lo, mut hi) = (self.corners[0], self.corners[0]);
            for c in &self.corners {
                lo = [lo[0].min(c[0]), lo[1].min(c[1])];
                hi = [hi[0].max(c[0]), hi[1].max(c[1])];
            }
            (hi[0] - lo[0]).max(hi[1] - lo[1]).max(self.z)
        };
        hole_features(
            &mut b,
            &self.holes,
            at,
            axis,
            top,
            "extrude_0",
            self.z,
            extrude,
            &self.outline_pockets,
            self.pockets.len(),
            crate::plate::tolerance(scale),
            &mut pending_rims,
        );
        slot_features(&mut b, &self.slots, at, axis);
        boss_features(&mut b, &self.bosses, at, self.z, axis, top, extrude);
        block_features(&mut b, &self.pads, &self.pockets, &self.outline_pockets, at, axis, top, "extrude_0", &mut pending_rims);
        b.build()
    }
}

/// One user parameter `<prefix>_<i>_angle` for the polygon's longest slanted side (see
/// [`dimensioned_angle`]) and a model parameter driving its `Angle` constraint in the sketch, so
/// a slanted polygon re-angles in SolidCraft too. `None` when no side is slanted.
pub(crate) fn side_angle_param(b: &mut DesignBuilder, prefix: &str, corners: &[[f64; 2]]) -> Option<PolygonAngle> {
    let side = dimensioned_angle(corners)?;
    let (a, c) = (corners[side], corners[(side + 1) % corners.len()]);
    // The side's own direction, as drawn: the angle from the X axis in [0, 360) degrees (the
    // `Angle` constraint turns line `b` as drawn, so collapsing to [0, 180) would mirror a
    // side pointing below the X axis).
    let degrees = (c[1] - a[1]).atan2(c[0] - a[0]).to_degrees().rem_euclid(360.0);
    let user = format!("{prefix}_{}_angle", side + 1);
    b.param(Param::angle(&user, degrees));
    let model = b.next_model_param_name();
    b.param(Param::model_angle(&model, &user));
    Some(PolygonAngle { side, degrees, param: model })
}

/// Which sides of `corners` drive their `Length`: [`dimensioned_sides`], or
/// [`dimensioned_sides_for_angle`] when the polygon has an angle side (see
/// [`dimensioned_angle`]).
pub(crate) fn polygon_dim(corners: &[[f64; 2]]) -> Vec<bool> {
    match dimensioned_angle(corners) {
        Some(angle) => dimensioned_sides_for_angle(corners, angle),
        None => dimensioned_sides(corners),
    }
}

/// A straight-sided polygon's sketch parameters, as `<prefix>_<i>_len` per driving side and
/// `<prefix>_<i>_angle` on its longest slanted side: the `Length` and `Angle` model parameters
/// for [`polygon_sketch_dimensioned`]/[`polygons_sketch_dimensioned`], so the polygon
/// re-dimensions in SolidCraft side by side like a plate's outline.
pub(crate) fn dimensioned_polygon_params(
    b: &mut DesignBuilder,
    prefix: &str,
    corners: &[[f64; 2]],
) -> (Vec<Option<String>>, Option<PolygonAngle>) {
    let dim = polygon_dim(corners);
    let lens = side_len_params(b, prefix, corners, &vec![None; corners.len()], &dim);
    let angle = side_angle_param(b, prefix, corners);
    (lens, angle)
}

/// Which sides of a polygon with arc sides `arcs` drive, and which arc side pairs share one
/// radius: an arc side's curve closes the loop differently from its chord, so it always drives
/// (its `Diameter`), even where its chord would be the derived side of a parallel group. A
/// straight side between two arc sides whose length equals the centres' separation ties the two
/// radii together (their endpoints' offsets from the centres are equal), so alongside both
/// driving diameters it must not drive: changing one driving diameter alone would leave no
/// solution — let the side adjust instead. When the side is also tangent to both arcs at its
/// ends (a stadium's straight sides are), the tangents force both radii to half the sides'
/// distance, so the two arcs share one radius: an `Equal` constraint ties them and only the
/// first drives its `Diameter`, exactly as a slot's two ends do. And an arc side tangent to both
/// its neighbouring straight sides, which run parallel, spans them at its diameter: in a
/// four-sided loop the opposite side connects their far ends square to them and so has that same
/// length, so alongside the arc's driving `Diameter` it must not drive (a semicircular top on a
/// rectangle — the two tangents force the neighbouring sides' separation to the diameter, and
/// changing one parameter alone would leave no solution).
#[allow(clippy::many_single_char_names)] // points a..f, directions u, v, w, as in the geometry
fn arc_polygon_dim(corners: &[[f64; 2]], arcs: &[Option<ArcSide>]) -> (Vec<bool>, Vec<(usize, usize)>) {
    let mut dim = dimensioned_sides(corners);
    for (i, arc) in arcs.iter().enumerate() {
        if arc.is_some() {
            dim[i] = true;
        }
    }
    let n = corners.len();
    let mut arc_equals: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
        if !dim[i] || arcs.get(i).is_some_and(Option::is_some) {
            continue;
        }
        let (Some(prev), Some(next)) = (arcs.get((i + n - 1) % n).copied().flatten(), arcs.get((i + 1) % n).copied().flatten()) else {
            continue;
        };
        let (a, b) = (corners[i], corners[(i + 1) % n]);
        let len = (a[0] - b[0]).hypot(a[1] - b[1]);
        let dc = (prev.centre[0] - next.centre[0]).hypot(prev.centre[1] - next.centre[1]);
        let cross = (b[0] - a[0]) * (next.centre[1] - prev.centre[1]) - (b[1] - a[1]) * (next.centre[0] - prev.centre[0]);
        if cross.abs() <= 1e-9 * len * dc && (len - dc).abs() <= 1e-6 * len.max(1.0) {
            dim[i] = false;
            let (prev_side, next_side) = ((i + n - 1) % n, (i + 1) % n);
            let known = arc_equals.iter().any(|&(x, y)| (x, y) == (prev_side, next_side) || (x, y) == (next_side, prev_side));
            if !known && line_arc_tangent(b, a, prev.centre, prev.r) && line_arc_tangent(a, b, next.centre, next.r) {
                arc_equals.push((prev_side, next_side));
                dim[next_side] = false;
            }
        }
    }
    // The tangent-tangent arc of the doc comment: release the opposite side of a four-sided loop.
    for j in 0..n {
        let Some(arc) = arcs.get(j).copied().flatten() else { continue };
        if !dim[j] {
            continue;
        }
        let (prev, next) = ((j + n - 1) % n, (j + 1) % n);
        if arcs.get(prev).is_some_and(Option::is_some) || arcs.get(next).is_some_and(Option::is_some) {
            continue;
        }
        let (a, b) = (corners[prev], corners[j]);
        let (c, d) = (corners[next], corners[(j + 2) % n]);
        if !line_arc_tangent(a, b, arc.centre, arc.r) || !line_arc_tangent(c, d, arc.centre, arc.r) {
            continue;
        }
        let (u, w) = ([b[0] - a[0], b[1] - a[1]], [d[0] - c[0], d[1] - c[1]]);
        let (lu, lw) = (u[0].hypot(u[1]), w[0].hypot(w[1]));
        if (u[0] * w[1] - u[1] * w[0]).abs() > 1e-9 * lu * lw {
            continue; // the neighbouring sides are not parallel
        }
        let opp = (j + 2) % n;
        if arcs.get(opp).is_some_and(Option::is_some) || (opp + 1) % n != prev {
            continue; // no single opposite side connects their far ends (the loop is not four-sided)
        }
        let (e, f) = (corners[opp], corners[(opp + 1) % n]);
        let v = [f[0] - e[0], f[1] - e[1]];
        if (v[0] * u[0] + v[1] * u[1]).abs() > 1e-9 * v[0].hypot(v[1]) * lu {
            continue; // the opposite side is not square to them, so its length is not the diameter
        }
        dim[opp] = false;
    }
    // Arc sides about one centre — a full circle's two semicircles — share their radius through
    // the sketch's shared centre point, so only the first of them drives its `Diameter`.
    for (i, arc_i) in arcs.iter().enumerate() {
        let Some(a) = *arc_i else { continue };
        if !dim[i] {
            continue;
        }
        for (j, arc_j) in arcs.iter().enumerate().skip(i + 1) {
            let Some(b) = *arc_j else { continue };
            if (a.centre[0] - b.centre[0]).hypot(a.centre[1] - b.centre[1]) <= 1e-9 * a.r.max(b.r).max(1.0) {
                dim[j] = false;
                let known = arc_equals.iter().any(|&(x, y)| (x, y) == (i, j) || (x, y) == (j, i));
                if !known {
                    arc_equals.push((i, j));
                }
            }
        }
    }
    (dim, arc_equals)
}

/// One user parameter `<prefix>_<i>_len` per driving straight side of `corners` and a model
/// parameter driving its `Length` constraint in the sketch, so the polygon re-dimensions in
/// SolidCraft side by side, like the rectangle's `base_x`/`base_y`. `arcs` marks the arc sides
/// (they drive their `Diameter` instead, see [`arc_side_params`]); `dim` says which straight
/// sides drive — [`polygon_dim`], adjusted for the arc ties in [`ProfileSpec::to_design_on`].
/// Returns the model parameters' names, `None` for a side that does not drive.
pub(crate) fn side_len_params(
    b: &mut DesignBuilder,
    prefix: &str,
    corners: &[[f64; 2]],
    arcs: &[Option<ArcSide>],
    dim: &[bool],
) -> Vec<Option<String>> {
    let n = corners.len();
    (0..n)
        .map(|i| {
            if !dim[i] || arcs.get(i).is_some_and(Option::is_some) {
                return None;
            }
            let (a, c) = (corners[i], corners[(i + 1) % n]);
            let len = snap_dim((a[0] - c[0]).hypot(a[1] - c[1]));
            let user = format!("{prefix}_{}_len", i + 1);
            b.param(Param::user(&user, len));
            let model = b.next_model_param_name();
            b.param(Param::model(&model, &user));
            Some(model)
        })
        .collect()
}

/// The parameters of an arc side: when the side drives, a user parameter `<prefix>_<i>_d` (the
/// outline's prefix is `arc`, a cut-out's `cutout_<n>_arc`) plus the model parameter driving the
/// arc's `Diameter` constraint. The arc's centre is not placed by a parameter: it is a free
/// sketch point the solver keeps equidistant from the side's two corners at the driving
/// diameter (see [`polygon_sketch_arcs`]), so changing one `<prefix>_<i>_d` leaves the sketch
/// solvable.
fn arc_side_params(b: &mut DesignBuilder, prefix: &str, arcs: &[Option<ArcSide>], dim: &[bool]) -> Vec<Option<String>> {
    let n = arcs.len();
    (0..n)
        .map(|i| {
            let arc = arcs.get(i).copied().flatten()?;
            if dim[i] {
                let user = format!("{prefix}_{}_d", i + 1);
                b.param(Param::user(&user, snap_dim(2.0 * arc.r)));
                let model = b.next_model_param_name();
                b.param(Param::model(&model, &user));
                Some(model)
            } else {
                None
            }
        })
        .collect()
}

/// The slots' parameters, as mesh2cad names them: `slot_<n>_len` (between the end centres),
/// `_w`, the middle's two profile-plane coordinates (in world coordinates, named after their
/// world axis letters) and `_angle` (of the long axis from the profile's first axis).
fn slot_params(b: &mut DesignBuilder, slots: &[SlotSpec], at: Vec3, axis: Axis) {
    for (i, s) in slots.iter().enumerate() {
        let p = format!("slot_{}", i + 1);
        b.param(Param::user(format!("{p}_len"), s.len)).param(Param::user(format!("{p}_w"), s.w));
        pos_params(b, &p, axis, at, [s.x, s.y]);
        b.param(Param::angle(format!("{p}_angle"), s.angle));
    }
}

/// The expression placing a slot's middle in sketch coordinates on the axis's plane: the
/// parameter named after the profile coordinate's world axis, negated where the plane's sketch
/// axis runs against the world axis (the XZ plane's sketch y is -Z).
fn sketch_pos_expr(p: &str, axis: Axis, i: usize) -> String {
    let name = format!("{p}_{}", axis.profile_letter(i));
    if axis.profile_sign(i) < 0.0 { format!("-{name}") } else { name }
}

/// A sketch of each slot on the profile plane, at its world profile coordinates for a part whose
/// corner is at `at`, and a cut through everything both ways from it. The sketch's dimensions are
/// model parameters driven by the slot's user parameters.
fn slot_features(b: &mut DesignBuilder, slots: &[SlotSpec], at: Vec3, axis: Axis) {
    for (i, s) in slots.iter().enumerate() {
        let p = format!("slot_{}", i + 1);
        let mut model = |expr: String, angle: bool| {
            let name = b.next_model_param_name();
            b.param(if angle { Param::model_angle(&name, expr) } else { Param::model(&name, expr) });
            name
        };
        let centre = axis.sketch_point(at, [s.x, s.y]).map(snap_mm);
        let dims = SlotDimensions {
            len: model(format!("{p}_len"), false),
            w: model(format!("{p}_w"), false),
            x: sketch_pos_expr(&p, axis, 0),
            y: sketch_pos_expr(&p, axis, 1),
            angle: Some(model(format!("{p}_angle"), true)),
        };
        let n = i + 1;
        let sketch = b.feature(
            format!("Slot{n}Sketch"),
            vec![],
            FeatureKind::Sketch { plane: Plane::Origin { name: axis.plane() }, sketch: slot_sketch(centre, s.len, s.w, s.angle, &dims) },
        );
        b.feature(
            format!("Slot{n}"),
            vec![],
            FeatureKind::Extrude { sketch, profiles: Profiles::All, extent: Extent::through_all(), operation: Operation::Cut },
        );
    }
}

/// The cut-outs as one sketch on the profile plane (at their world profile coordinates for a part
/// whose corner is at `at`) and a cut through everything both ways from it. It comes after the
/// outline's fillet: a cut-out may lie in the material an inside corner's rounding adds. When
/// every cut-out is straight-sided they share that one sketch; as soon as one cut-out has an
/// arc side, each cut-out gets its own `Cutout<n>Sketch` and `Cutout<n>` cut, the arc-sided ones
/// drawn as [`polygon_sketch_arcs`] with an arc side's `Diameter` driven by
/// `cutout_<n>_arc_<i>_d`.
fn cutout_features(b: &mut DesignBuilder, cutouts: &[CutoutSpec], at: Vec3, axis: Axis) {
    if cutouts.is_empty() {
        return;
    }
    if !cutouts.iter().any(CutoutSpec::has_arcs) {
        let sketched: Vec<Vec<[f64; 2]>> =
            cutouts.iter().map(|c| c.corners.iter().map(|p| axis.sketch_point(at, *p).map(snap_mm)).collect()).collect();
        let lens: Vec<Vec<Option<String>>> = sketched
            .iter()
            .enumerate()
            .map(|(k, poly)| side_len_params(b, &format!("cutout_{}", k + 1), poly, &vec![None; poly.len()], &polygon_dim(poly)))
            .collect();
        let angles: Vec<Option<PolygonAngle>> =
            sketched.iter().enumerate().map(|(k, poly)| side_angle_param(b, &format!("cutout_{}", k + 1), poly)).collect();
        let sketch = b.feature(
            "CutoutsSketch",
            vec![],
            FeatureKind::Sketch {
                plane: Plane::Origin { name: axis.plane() },
                sketch: polygons_sketch_dimensioned(&sketched, &lens, &angles),
            },
        );
        b.feature(
            "Cutouts",
            vec![],
            FeatureKind::Extrude { sketch, profiles: Profiles::All, extent: Extent::through_all(), operation: Operation::Cut },
        );
        return;
    }
    for (k, c) in cutouts.iter().enumerate() {
        let p = format!("cutout_{}", k + 1);
        let poly: Vec<[f64; 2]> = c.corners.iter().map(|q| axis.sketch_point(at, *q).map(snap_mm)).collect();
        let sketch_geom = if c.has_arcs() {
            let arcs: Vec<Option<ArcSide>> =
                c.arc_sides().iter().map(|a| a.map(|a| ArcSide { centre: axis.sketch_point(at, a.centre).map(snap_mm), r: a.r })).collect();
            let (dim, arc_equals) = arc_polygon_dim(&poly, &arcs);
            let lens = side_len_params(b, &p, &poly, &arcs, &dim);
            let arc_params = arc_side_params(b, &format!("{p}_arc"), &arcs, &dim);
            let geom: Vec<Option<([f64; 2], f64)>> = arcs.iter().map(|a| a.map(|a| (a.centre, a.r))).collect();
            polygon_sketch_arcs(&poly, &geom, &lens, &arc_params, &arc_equals)
        } else {
            let lens = side_len_params(b, &p, &poly, &vec![None; poly.len()], &polygon_dim(&poly));
            let angle = side_angle_param(b, &p, &poly);
            polygon_sketch_dimensioned(&poly, &lens, angle.as_ref())
        };
        let sketch = b.feature(
            format!("Cutout{}Sketch", k + 1),
            vec![],
            FeatureKind::Sketch { plane: Plane::Origin { name: axis.plane() }, sketch: sketch_geom },
        );
        b.feature(
            format!("Cutout{}", k + 1),
            vec![],
            FeatureKind::Extrude { sketch, profiles: Profiles::All, extent: Extent::through_all(), operation: Operation::Cut },
        );
    }
}

/// The bosses' parameters, as mesh2cad names them: `boss_<n>_d`, `_h` (above the face the boss
/// stands on), `_fillet` (the round at its foot, if it has one) and the centre's two
/// profile-plane coordinates (in world coordinates).
fn boss_params(b: &mut DesignBuilder, bosses: &[BossSpec], at: Vec3, axis: Axis) {
    for (i, s) in bosses.iter().enumerate() {
        let p = format!("boss_{}", i + 1);
        b.param(Param::user(format!("{p}_d"), s.d)).param(Param::user(format!("{p}_h"), s.h));
        if let Some(f) = s.fillet {
            b.param(Param::user(format!("{p}_fillet"), f));
        }
        pos_params(b, &p, axis, at, [s.x, s.y]);
        if let Some(draft) = s.draft_deg {
            b.param(Param::angle(format!("{p}_draft_deg"), draft));
        } else if let Some(BossChamfer { w, h }) = s.chamfer {
            // A 45° chamfer is one parameter, as mesh2cad writes it.
            if (w - h).abs() < 1e-9 {
                b.param(Param::user(format!("{p}_chamfer"), w));
            } else {
                b.param(Param::user(format!("{p}_chamfer_h"), h)).param(Param::user(format!("{p}_chamfer_w"), w));
            }
        }
    }
}

/// A cylinder joined to the part per boss, standing on the face at height `top` (its
/// expression) along the axis, and a fillet of its foot where it has one: the edge between the
/// extrusion `extrude`'s top face and the cylinder's side, picked at the boss's +X side on a
/// part `z` thick whose corner is at `at`.
fn boss_features(b: &mut DesignBuilder, bosses: &[BossSpec], at: Vec3, z: f64, axis: Axis, top: &str, extrude: u32) {
    for (i, s) in bosses.iter().enumerate() {
        let p = format!("boss_{}", i + 1);
        let kind = if s.draft_deg.is_some() {
            FeatureKind::Cone {
                base: world_exprs(&p, axis, top),
                axis: axis.dir(),
                radius: format!("{p}_d / 2"),
                top_radius: format!("{p}_d / 2 - {p}_h * tan({p}_draft_deg)"),
                height: format!("{p}_h"),
                operation: Operation::Join,
            }
        } else {
            FeatureKind::Cylinder {
                base: world_exprs(&p, axis, top),
                axis: axis.dir(),
                radius: format!("{p}_d / 2"),
                height: format!("{p}_h"),
                operation: Operation::Join,
            }
        };
        let cylinder = b.feature(format!("Boss{}", i + 1), vec![], kind);
        if let Some(BossChamfer { w, h }) = s.chamfer {
            // The top face faces up most, so the first distance is along it (in from the side).
            let (distance, distance2) = if (w - h).abs() < 1e-9 {
                (format!("{p}_chamfer"), None)
            } else {
                (format!("{p}_chamfer_w"), Some(format!("{p}_chamfer_h")))
            };
            b.feature(
                format!("Boss{}Chamfer", i + 1),
                vec![],
                FeatureKind::Chamfer {
                    edge_names: vec![format!("F{cylinder}:side|F{cylinder}:top")],
                    edges: vec![axis.world(at, s.x + s.d / 2.0, s.y, z + s.h)],
                    distance,
                    distance2,
                },
            );
        }
        if s.fillet.is_some() {
            b.feature(
                format!("Boss{}Fillet", i + 1),
                vec![],
                FeatureKind::Fillet {
                    edge_names: vec![format!("F{extrude}:top|F{cylinder}:side")],
                    edges: vec![axis.world(at, s.x + s.d / 2.0, s.y, z)],
                    radius: format!("{p}_fillet"),
                },
            );
        }
    }
}

/// The side bosses' parameters, numbered after the top bosses: `boss_<n>_d`, `boss_<n>_h` and the
/// axis's two world coordinates across the wall (`boss_<n>_y`/`_z` on an X wall, `boss_<n>_x`/`_z`
/// on a Y wall), in world coordinates for a plate whose corner is at `at`.
fn side_boss_params(b: &mut DesignBuilder, bosses: &[SideBossSpec], at: Vec3, base: usize) {
    for (i, s) in bosses.iter().enumerate() {
        let p = format!("boss_{}", base + i + 1);
        b.param(Param::user(format!("{p}_d"), s.d)).param(Param::user(format!("{p}_h"), s.h));
        let (horiz, world) = match s.wall {
            Wall::PosX | Wall::NegX => ("y", at[1] + s.at[0]),
            Wall::PosY | Wall::NegY => ("x", at[0] + s.at[0]),
        };
        b.param(Param::user(format!("{p}_{horiz}"), snap_mm(world)));
        b.param(Param::user(format!("{p}_z"), snap_mm(at[2] + s.at[1])));
    }
}

/// One `cylinder` joined per side boss (roadmap 3.10): its base on the wall's expression, its
/// axis the wall's outward normal, so it protrudes from the wall. A plate is always a Z
/// extrusion, so the walls' expressions are the `base_*` parameters'.
fn side_boss_features(b: &mut DesignBuilder, bosses: &[SideBossSpec], base: usize) {
    for (i, s) in bosses.iter().enumerate() {
        let n = base + i + 1;
        let p = format!("boss_{n}");
        let (axis, wall) = match s.wall {
            Wall::PosX => ([1.0, 0.0, 0.0], "base_cx + base_x / 2"),
            Wall::NegX => ([-1.0, 0.0, 0.0], "base_cx - base_x / 2"),
            Wall::PosY => ([0.0, 1.0, 0.0], "base_cy + base_y / 2"),
            Wall::NegY => ([0.0, -1.0, 0.0], "base_cy - base_y / 2"),
        };
        let base3 = match s.wall {
            Wall::PosX | Wall::NegX => [wall.to_owned(), format!("{p}_y"), format!("{p}_z")],
            Wall::PosY | Wall::NegY => [format!("{p}_x"), wall.to_owned(), format!("{p}_z")],
        };
        b.feature(
            format!("Boss{n}"),
            vec![],
            FeatureKind::Cylinder { base: base3, axis, radius: format!("{p}_d / 2"), height: format!("{p}_h"), operation: Operation::Join },
        );
    }
}

/// The pads' and the pockets' parameters: `pad_<n>_w` (along the profile's first axis), `_l`
/// (along its second), `_h` and the middle's two profile-plane coordinates, as mesh2cad names
/// a pad, and `pocket_<n>_w`, `_l`, `_depth`, and the middle's alike (in world coordinates for a
/// part whose corner is at `at`).
fn block_params(b: &mut DesignBuilder, pads: &[PadSpec], pockets: &[PocketSpec], at: Vec3, axis: Axis) {
    for (i, s) in pads.iter().enumerate() {
        let p = format!("pad_{}", i + 1);
        b.param(Param::user(format!("{p}_w"), s.w)).param(Param::user(format!("{p}_l"), s.l)).param(Param::user(format!("{p}_h"), s.h));
        pos_params(b, &p, axis, at, [s.x, s.y]);
    }
    for (i, s) in pockets.iter().enumerate() {
        let p = format!("pocket_{}", i + 1);
        b.param(Param::user(format!("{p}_w"), s.w))
            .param(Param::user(format!("{p}_l"), s.l))
            .param(Param::user(format!("{p}_depth"), s.depth));
        pos_params(b, &p, axis, at, [s.x, s.y]);
    }
}

/// A box joined to the part per pad, standing on the face at `top`, and a box cut out of it per
/// pocket, from `top` or from `bottom`; a pocket of any other outline of straight sides gets
/// its own sketch and cut instead. The box's corner and extents are in world axes: along
/// the extrusion axis the pad's height or the pocket's depth, along the profile axes its width
/// and length.
#[allow(clippy::too_many_arguments)] // the two block kinds, their frame and faces
fn block_features(
    b: &mut DesignBuilder,
    pads: &[PadSpec],
    pockets: &[PocketSpec],
    outline_pockets: &[OutlinePocketSpec],
    at: Vec3,
    axis: Axis,
    top: &str,
    bottom: &str,
    pending_rims: &mut Vec<PendingRim>,
) {
    let box_at = |p: &str, along: &str, extent: &str| -> ([String; 3], [String; 3]) {
        let mut corner = [const { String::new() }; 3];
        let mut size = [const { String::new() }; 3];
        for i in 0..3 {
            match axis.profile_axes().iter().position(|&a| a == i) {
                Some(0) => {
                    corner[i] = format!("{p}_{} - {p}_w / 2", axis.profile_letter(0));
                    size[i] = format!("{p}_w");
                }
                Some(1) => {
                    corner[i] = format!("{p}_{} - {p}_l / 2", axis.profile_letter(1));
                    size[i] = format!("{p}_l");
                }
                _ => {
                    corner[i] = String::from(along);
                    size[i] = String::from(extent);
                }
            }
        }
        (corner, size)
    };
    for i in 1..=pads.len() {
        let p = format!("pad_{i}");
        let (corner, size) = box_at(&p, top, &format!("{p}_h"));
        b.feature(
            format!("Pad{i}"),
            vec![],
            FeatureKind::Box {
                corner,
                length: size[0].clone(),
                width: size[1].clone(),
                height: size[2].clone(),
                operation: Operation::Join,
            },
        );
    }
    for (i, s) in pockets.iter().enumerate() {
        let p = format!("pocket_{}", i + 1);
        let along = if s.bottom { bottom.to_owned() } else { format!("{top} - {p}_depth") };
        let (corner, size) = box_at(&p, &along, &format!("{p}_depth"));
        b.feature(
            format!("Pocket{}", i + 1),
            vec![],
            FeatureKind::Box {
                corner,
                length: size[0].clone(),
                width: size[1].clone(),
                height: size[2].clone(),
                operation: Operation::Cut,
            },
        );
    }
    // A pocket of any other outline of straight sides gets its own sketch, every side
    // dimensioned by `pocket_<n>_<i>_len` (numbered after the rectangular pockets), the longest
    // slanted side angled by `pocket_<n>_<i>_angle`, and a cut from its face down by
    // `pocket_<n>_depth`.
    for (i, s) in outline_pockets.iter().enumerate() {
        let n = pockets.len() + i + 1;
        let p = format!("pocket_{n}");
        b.param(Param::user(format!("{p}_depth"), s.depth));
        let poly: Vec<[f64; 2]> = s.corners.iter().map(|c| axis.sketch_point(at, *c).map(snap_mm)).collect();
        let lens = side_len_params(b, &p, &poly, &vec![None; poly.len()], &polygon_dim(&poly));
        let angle = side_angle_param(b, &p, &poly);
        let sketch = b.feature(
            format!("Pocket{n}Sketch"),
            vec![],
            FeatureKind::Sketch {
                plane: Plane::Origin { name: axis.plane() },
                sketch: polygon_sketch_dimensioned(&poly, &lens, angle.as_ref()),
            },
        );
        let start = if s.bottom { bottom.to_owned() } else { format!("{top} - {p}_depth") };
        let pocket = b.feature(
            format!("Pocket{n}"),
            vec![],
            FeatureKind::Extrude {
                sketch,
                profiles: Profiles::All,
                extent: Extent::positive(format!("{p}_depth")).starting_at(start),
                operation: Operation::Cut,
            },
        );
        // The rounded rims of holes opening on this pocket's level: their edge lies on the
        // pocket's start face, `F<Pocket n>:start`, which exists only now.
        for rim in pending_rims.iter().filter(|r| r.pocket == i) {
            b.feature(
                rim.name.clone(),
                vec![],
                FeatureKind::Fillet {
                    edge_names: vec![format!("F{pocket}:start|F{}:bore:side:0", rim.hole)],
                    edges: vec![axis.world(at, rim.x + rim.d / 2.0, rim.y, rim.w)],
                    radius: rim.radius.clone(),
                },
            );
        }
        pending_rims.retain(|r| r.pocket != i);
    }
}

/// The holes, grouped into patterns (see [`crate::patterns`]), each with the parameter prefix
/// mesh2cad gives it: `hole_<n>`, `hole_row_<n>`, `hole_grid_<n>` or `hole_circle_<n>`, numbered
/// per kind. A row that is not axis-aligned comes out as its single holes instead: a pattern's
/// direction is not an expression in SolidCraft (roadmap 6.5).
fn hole_groups(holes: &[HoleSpec]) -> Vec<(String, HoleGroup)> {
    let mut counts: HashMap<&'static str, usize> = HashMap::new();
    let mut named = Vec::new();
    for (group, members) in group_holes_with_members(holes) {
        if let HoleGroup::Row { angle, .. } = &group
            && !row_is_axis_aligned(*angle)
        {
            for &i in &members {
                let n = counts.entry("hole").or_insert(0);
                *n += 1;
                named.push((format!("hole_{n}"), HoleGroup::Single(holes[i].clone())));
            }
            continue;
        }
        let prefix = match &group {
            HoleGroup::Single(_) => "hole",
            HoleGroup::Row { .. } => "hole_row",
            HoleGroup::Grid { .. } => "hole_grid",
            HoleGroup::Circle { .. } => "hole_circle",
        };
        let n = counts.entry(prefix).or_insert(0);
        *n += 1;
        named.push((format!("{prefix}_{n}"), group));
    }
    named
}

/// The holes' parameters, positions in world coordinates for a part whose corner is at `at`,
/// named after the profile plane's world axis letters.
#[allow(clippy::many_single_char_names)] // x, y, d, r as in the parameter names
fn hole_params(b: &mut DesignBuilder, holes: &[HoleSpec], at: Vec3, axis: Axis) {
    for (p, group) in hole_groups(holes) {
        match group {
            HoleGroup::Single(h) => {
                pos_params(b, &p, axis, at, [h.x, h.y]);
                b.param(Param::user(format!("{p}_d"), h.d));
                if let Some(blind) = h.blind {
                    b.param(Param::user(format!("{p}_depth"), blind.depth));
                }
                match h.head {
                    None => {}
                    Some(HoleHead::Counterbore { d, depth, .. }) => {
                        b.param(Param::user(format!("{p}_cbore_d"), d)).param(Param::user(format!("{p}_cbore_depth"), depth));
                    }
                    Some(HoleHead::Countersink { d, angle, .. }) => {
                        b.param(Param::user(format!("{p}_csk_d"), d)).param(Param::angle(format!("{p}_csk_angle"), angle));
                    }
                    // mesh2cad's names: the top rim's `_rim_r`, the bottom's `_bot_rim_r`.
                    Some(HoleHead::Round { r, bottom }) => {
                        b.param(Param::user(format!("{p}_{}rim_r", if bottom { "bot_" } else { "" }), r));
                    }
                }
            }
            HoleGroup::Row { n, pitch, d, x, y, .. } => {
                b.param(Param::count(format!("{p}_n"), n))
                    .param(Param::user(format!("{p}_pitch"), pitch))
                    .param(Param::user(format!("{p}_d"), d));
                pos_params(b, &p, axis, at, [x, y]);
            }
            HoleGroup::Grid { nx, ny, pitch_x, pitch_y, d, x, y } => {
                b.param(Param::count(format!("{p}_nx"), nx))
                    .param(Param::count(format!("{p}_ny"), ny))
                    .param(Param::user(format!("{p}_pitch_x"), pitch_x))
                    .param(Param::user(format!("{p}_pitch_y"), pitch_y))
                    .param(Param::user(format!("{p}_d"), d));
                pos_params(b, &p, axis, at, [x, y]);
            }
            HoleGroup::Circle { n, d, r, cx, cy, angle } => {
                b.param(Param::count(format!("{p}_n"), n)).param(Param::user(format!("{p}_d"), d)).param(Param::user(format!("{p}_r"), r));
                for (i, c) in [cx, cy].iter().enumerate() {
                    b.param(Param::user(format!("{p}_c{}", axis.profile_letter(i)), snap_mm(axis.profile_world(at, i, *c))));
                }
                b.param(Param::angle(format!("{p}_angle"), angle));
            }
        }
    }
}

/// The part the holes are drilled into: where its corner is in the file, the extrusion axis, the
/// two faces' expressions, the thickness and the drilling direction from the +axis face.
struct HoleCtx<'a> {
    at: Vec3,
    axis: Axis,
    top: &'a str,
    bottom: &'a str,
    z: f64,
    down: Vec3,
    /// The extrusion whose faces (`top`, `start`) the holes open on.
    extrude: u32,
    /// The pockets of any outline, for holes opening on a step's level; `rect_pockets` is the
    /// number of rectangular pockets before them, so a level's parameter is
    /// `pocket_<rect_pockets + i + 1>_depth` (as `block_features` numbers them).
    outline_pockets: &'a [OutlinePocketSpec],
    rect_pockets: usize,
    /// The recognition tolerance (see `plate::tolerance`), so a hole is matched to a step's
    /// region exactly as the recogniser matched it.
    tol: f64,
}

/// A rounded rim on a step's level, to be written once the pocket that makes the level exists:
/// the rim's fillet edge is named after that pocket's start face (`F<Pocket n>:start`).
struct PendingRim {
    /// The rim feature's name, `Hole<n>Rim`.
    name: String,
    /// The hole feature's id.
    hole: u32,
    /// The hole's centre and diameter, and the level's depth along the axis (relative to `at`).
    x: f64,
    y: f64,
    d: f64,
    w: f64,
    /// The rim radius's expression, `hole_<n>_rim_r`.
    radius: String,
    /// The index in the outline pockets the level belongs to.
    pocket: usize,
}

/// A through hole per hole, or per pattern a hole and a pattern feature that repeats it, drilled
/// from the face at depth `z` along the axis (the `top` expression); a hole with a counterbore or
/// countersink on the other face (at depth 0, the `bottom` expression) is drilled from there
/// along the axis. A headed hole opening on a step's level is drilled from that level instead.
#[allow(clippy::too_many_arguments)] // the part's frame, its faces, the extrusion and the pockets
fn hole_features(
    b: &mut DesignBuilder,
    holes: &[HoleSpec],
    at: Vec3,
    axis: Axis,
    top: &str,
    bottom: &str,
    z: f64,
    extrude: u32,
    outline_pockets: &[OutlinePocketSpec],
    rect_pockets: usize,
    tol: f64,
    pending_rims: &mut Vec<PendingRim>,
) {
    // Drilled from the +axis face: against the axis. (Built component-wise, so no -0.0.)
    let down = {
        let mut d = [0.0; 3];
        d[axis.index()] = -1.0;
        d
    };
    let ctx = HoleCtx { at, axis, top, bottom, z, down, extrude, outline_pockets, rect_pockets, tol };
    let mut counts = [0, 0, 0, 0]; // singles, rows, grids, circles
    for (p, group) in hole_groups(holes) {
        match group {
            HoleGroup::Single(h) => {
                counts[0] += 1;
                single_hole(b, &ctx, &p, &h, counts[0], pending_rims);
            }
            group => pattern_hole(b, &ctx, &p, &group, &mut counts),
        }
    }
}

/// One hole in no pattern: drilled from the face its counterbore, countersink or blind floor is
/// on, or from the +axis face into the part. A headed hole whose mouth lies in a step's region
/// opens on the step's level, so it is drilled from there: the level's face expression, one
/// `pocket_<n>_depth` below the top.
#[allow(clippy::many_single_char_names)] // x, y, d, r as in the parameter names
fn single_hole(b: &mut DesignBuilder, ctx: &HoleCtx, p: &str, h: &HoleSpec, n: usize, pending: &mut Vec<PendingRim>) {
    let (kind, from_bottom) = match h.head {
        None => (HoleKind::Simple, h.blind.is_some_and(|b| b.bottom)),
        Some(HoleHead::Counterbore { bottom, .. }) => {
            (HoleKind::Counterbore { cb_diameter: format!("{p}_cbore_d"), cb_depth: format!("{p}_cbore_depth") }, bottom)
        }
        Some(HoleHead::Countersink { bottom, .. }) => {
            (HoleKind::Countersink { cs_diameter: format!("{p}_csk_d"), cs_angle: format!("{p}_csk_angle") }, bottom)
        }
        // A rounded rim: a plain hole drilled from the face the rim is on, and a fillet of its
        // edge on that face below — so a bottom-rimmed hole beneath a step drills from the
        // bottom, as deep as the level lies above it, not from the top.
        Some(HoleHead::Round { bottom, .. }) => (HoleKind::Simple, bottom),
    };
    // A head opening on a step's level (the pocket's floor faces up like a top face): drill from
    // the level, not from the plate's top face above the pocket.
    let level =
        ctx.outline_pockets.iter().enumerate().find(|(_, pk)| !pk.bottom && pk.contains([h.x, h.y], ctx.tol)).map(|(i, pk)| (i, pk.depth));
    let on_level = h.head.is_some_and(|head| !head.bottom()) && level.is_some();
    let level_face = level.map(|(i, _)| format!("{} - pocket_{}_depth", ctx.top, ctx.rect_pockets + i + 1));
    let (face_w, face, direction) = if on_level {
        let (_, depth) = level.expect("a level for a headed hole on it");
        (ctx.z - depth, level_face.clone().expect("a face for the level"), ctx.down)
    } else if from_bottom {
        (0.0, ctx.bottom.to_owned(), ctx.axis.dir())
    } else {
        (ctx.z, ctx.top.to_owned(), ctx.down)
    };
    let hole = b.feature(
        format!("Hole{n}"),
        vec![],
        FeatureKind::Hole {
            position: ctx.axis.world(ctx.at, h.x, h.y, face_w),
            at: world_exprs(p, ctx.axis, &face),
            direction,
            diameter: format!("{p}_d"),
            depth: h.blind.map(|_| format!("{p}_depth")),
            hole: kind,
        },
    );
    if let Some(HoleHead::Round { bottom, .. }) = h.head {
        if on_level {
            // The rim's edge lies on the pocket's start face, which `block_features` creates
            // after the holes: leave the fillet to it.
            let (i, depth) = level.expect("a level for a headed hole on it");
            pending.push(PendingRim {
                name: format!("Hole{n}Rim"),
                hole,
                x: h.x,
                y: h.y,
                d: h.d,
                w: ctx.z - depth,
                radius: format!("{p}_rim_r"),
                pocket: i,
            });
            return;
        }
        // The edge between the face and the hole's bore, as SolidCraft names it.
        let (face, w, radius) = if bottom { ("start", 0.0, format!("{p}_bot_rim_r")) } else { ("top", ctx.z, format!("{p}_rim_r")) };
        b.feature(
            format!("Hole{n}Rim"),
            vec![],
            FeatureKind::Fillet {
                edge_names: vec![format!("F{}:{face}|F{hole}:bore:side:0", ctx.extrude)],
                edges: vec![ctx.axis.world(ctx.at, h.x + h.d / 2.0, h.y, w)],
                radius,
            },
        );
    }
}

/// A pattern's seed hole: drilled from the +axis face at depth `z`.
fn seed_hole(b: &mut DesignBuilder, ctx: &HoleCtx, name: String, profile: [f64; 2], p: &str) -> u32 {
    b.feature(
        name,
        vec![],
        FeatureKind::Hole {
            position: ctx.axis.world(ctx.at, profile[0], profile[1], ctx.z),
            at: world_exprs(p, ctx.axis, ctx.top),
            direction: ctx.down,
            diameter: format!("{p}_d"),
            depth: None,
            hole: HoleKind::Simple,
        },
    )
}

/// One hole and a pattern feature per pattern: a row, a grid or a bolt circle, all in the profile
/// plane, so the pattern's directions and axis are the frame's vectors rotated back to world.
#[allow(clippy::many_single_char_names)] // x, y, d, r as in the parameter names
fn pattern_hole(b: &mut DesignBuilder, ctx: &HoleCtx, p: &str, group: &HoleGroup, counts: &mut [usize; 4]) {
    let axis = ctx.axis;
    match group {
        HoleGroup::Single(_) => unreachable!("a single hole has no pattern"),
        HoleGroup::Row { x: hx, y: hy, angle, .. } => {
            counts[1] += 1;
            let n = counts[1];
            let seed = seed_hole(b, ctx, format!("HoleRow{n}"), [*hx, *hy], p);
            let (sin, cos) = angle.to_radians().sin_cos();
            let pattern = PatternKind::Rectangular {
                dir1: axis.unrotate([cos, sin, 0.0]),
                count1: format!("{p}_n"),
                spacing1: format!("{p}_pitch"),
                dir2: None,
                count2: None,
                spacing2: None,
            };
            b.feature(format!("HoleRow{n}Pattern"), vec![], FeatureKind::Pattern { features: vec![seed], pattern });
        }
        HoleGroup::Grid { x: hx, y: hy, .. } => {
            counts[2] += 1;
            let n = counts[2];
            let seed = seed_hole(b, ctx, format!("HoleGrid{n}"), [*hx, *hy], p);
            let pattern = PatternKind::Rectangular {
                dir1: axis.unrotate([1.0, 0.0, 0.0]),
                count1: format!("{p}_nx"),
                spacing1: format!("{p}_pitch_x"),
                dir2: Some(axis.unrotate([0.0, 1.0, 0.0])),
                count2: Some(format!("{p}_ny")),
                spacing2: Some(format!("{p}_pitch_y")),
            };
            b.feature(format!("HoleGrid{n}Pattern"), vec![], FeatureKind::Pattern { features: vec![seed], pattern });
        }
        HoleGroup::Circle { r, cx, cy, angle, .. } => {
            counts[3] += 1;
            let n = counts[3];
            let (sin, cos) = angle.to_radians().sin_cos();
            let profile = [cx + r * cos, cy + r * sin]; // Copy fields of &HoleGroup
            let mut at_exprs = [const { String::new() }; 3];
            for (i, slot) in at_exprs.iter_mut().enumerate() {
                *slot = match axis.profile_axes().iter().position(|&a| a == i) {
                    Some(k) => {
                        // The centre's parameter is a world coordinate; the radius's offset is
                        // in profile coordinates, so it carries the profile sign (the second
                        // profile coordinate maps to -Z on a Y extrusion).
                        let trig = if k == 0 { ("cos", "+") } else { ("sin", if axis.profile_sign(k) < 0.0 { "-" } else { "+" }) };
                        format!("{p}_c{} {} {p}_r * {}({p}_angle)", axis.profile_letter(k), trig.1, trig.0)
                    }
                    _ => String::from(ctx.top),
                };
            }
            let seed = b.feature(
                format!("HoleCircle{n}"),
                vec![],
                FeatureKind::Hole {
                    position: axis.world(ctx.at, profile[0], profile[1], ctx.z),
                    at: at_exprs,
                    direction: ctx.down,
                    diameter: format!("{p}_d"),
                    depth: None,
                    hole: HoleKind::Simple,
                },
            );
            let mut origin = [const { String::new() }; 3];
            for (i, slot) in origin.iter_mut().enumerate() {
                *slot = match axis.profile_axes().iter().position(|&a| a == i) {
                    Some(k) => format!("{p}_c{}", axis.profile_letter(k)),
                    _ => "0".into(),
                };
            }
            let pattern = PatternKind::Circular { origin, axis: axis.dir(), count: format!("{p}_n"), angle: "360 deg".into() };
            b.feature(format!("HoleCircle{n}Pattern"), vec![], FeatureKind::Pattern { features: vec![seed], pattern });
        }
    }
}
