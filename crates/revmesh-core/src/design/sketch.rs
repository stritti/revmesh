use serde::Serialize;

use crate::mesh::snap_dim;

/// A sketch: points, curves between them, and constraints on both.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Sketch {
    /// Points; index 0 is the fixed origin.
    pub points: Vec<SketchPoint>,
    /// Curves, referring to points by index.
    pub curves: Vec<SketchCurve>,
    /// Constraints, referring to points and curves by index.
    pub constraints: Vec<Constraint>,
    /// Geometry linked from outside the sketch, such as its X axis.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    counters: Counters,
}

/// Geometry a sketch takes from elsewhere: `j<n>`, a projection of one of the sketch's own axes.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Link {
    /// `j<n>`.
    pub id: String,
    kind: &'static str,
    source: LinkSource,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct LinkSource {
    #[serde(rename = "type")]
    kind: &'static str,
    name: &'static str,
}

/// A sketch point.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SketchPoint {
    /// `origin`, or `p<n>`.
    pub id: String,
    /// Position in sketch coordinates.
    pub pos: [f64; 2],
    /// Whether the solver may move it.
    pub fixed: bool,
    /// The link (`j<n>`) that places it, for a point of linked geometry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// The expressions that place a fixed point, which follow the parameters they name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exprs: Option<PointExprs>,
}

/// Where a point placed by parameters lies: an expression for each coordinate.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PointExprs {
    /// X.
    pub x: String,
    /// Y.
    pub y: String,
}

impl SketchPoint {
    fn new(id: impl Into<String>, pos: [f64; 2], fixed: bool) -> Self {
        Self { id: id.into(), pos, fixed, link: None, exprs: None }
    }
}

/// A sketch curve.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SketchCurve {
    /// A line between two points.
    Line {
        /// `l<n>`.
        id: String,
        /// Start point index.
        a: usize,
        /// End point index.
        b: usize,
        /// Construction geometry makes no profile.
        construction: bool,
        /// The link (`j<n>`) it comes from, for linked geometry.
        #[serde(skip_serializing_if = "Option::is_none")]
        link: Option<String>,
    },
    /// An arc about centre `c`, counter-clockwise from point `a` to point `b`.
    Arc {
        /// `a<n>`.
        id: String,
        /// Centre point index.
        c: usize,
        /// Start point index.
        a: usize,
        /// End point index.
        b: usize,
        /// Construction geometry makes no profile.
        construction: bool,
    },
}

/// A sketch constraint.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Constraint {
    /// Line `l` is horizontal.
    Horizontal {
        /// `k<n>`.
        id: String,
        /// Curve index.
        l: usize,
    },
    /// Line `l` is vertical.
    Vertical {
        /// `k<n>`.
        id: String,
        /// Curve index.
        l: usize,
    },
    /// Points `p` and `q` coincide.
    Coincident {
        /// `k<n>`.
        id: String,
        /// Point index.
        p: usize,
        /// Point index.
        q: usize,
    },
    /// Line `l` has a length driven by the model parameter `param`.
    Length {
        /// `k<n>`.
        id: String,
        /// Curve index.
        l: usize,
        /// The length now.
        value: f64,
        /// The model parameter that drives it.
        param: String,
        /// Where the dimension text sits, relative to the line.
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<[f64; 2]>,
    },
    /// Curves `a` and `b` are tangent where they meet.
    Tangent {
        /// `k<n>`.
        id: String,
        /// Curve index.
        a: usize,
        /// Curve index.
        b: usize,
    },
    /// Curves `a` and `b` are equal (two arcs: one radius).
    Equal {
        /// `k<n>`.
        id: String,
        /// Curve index.
        a: usize,
        /// Curve index.
        b: usize,
    },
    /// Point `p` is the midpoint of line `l`.
    Midpoint {
        /// `k<n>`.
        id: String,
        /// Point index.
        p: usize,
        /// Curve index.
        l: usize,
    },
    /// Arc or circle `c` has a diameter driven by `param`.
    Diameter {
        /// `k<n>`.
        id: String,
        /// Curve index.
        c: usize,
        /// The diameter now.
        value: f64,
        /// The model parameter that drives it.
        param: String,
    },
    /// Point `q` lies `value` along X from point `p`, driven by `param`.
    DistanceX {
        /// `k<n>`.
        id: String,
        /// Point index.
        p: usize,
        /// Point index.
        q: usize,
        /// The distance now.
        value: f64,
        /// The model parameter that drives it.
        param: String,
    },
    /// Point `q` lies `value` along Y from point `p`, driven by `param`.
    DistanceY {
        /// `k<n>`.
        id: String,
        /// Point index.
        p: usize,
        /// Point index.
        q: usize,
        /// The distance now.
        value: f64,
        /// The model parameter that drives it.
        param: String,
    },
    /// Line `b` is turned `value` (radians) from line `a`, driven by `param` (in degrees).
    Angle {
        /// `k<n>`.
        id: String,
        /// Curve index.
        a: usize,
        /// Curve index.
        b: usize,
        /// The angle now, in radians.
        value: f64,
        /// The model parameter that drives it.
        param: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Counters {
    #[serde(skip_serializing_if = "is_zero")]
    a: usize,
    #[serde(skip_serializing_if = "is_zero")]
    j: usize,
    k: usize,
    l: usize,
    p: usize,
}

#[allow(clippy::trivially_copy_pass_by_ref)] // serde passes the field by reference
fn is_zero(n: &usize) -> bool {
    *n == 0
}

/// A rectangle from the origin to `(width, height)` on the XY plane, its sides driven by the
/// model parameters `d1` (along X) and `d2` (along Y). Lines are `l1` (bottom), `l2` (right),
/// `l3` (top) and `l4` (left).
#[must_use]
pub fn rectangle_sketch(width: f64, height: f64) -> Sketch {
    let point = SketchPoint::new;
    let line = |n: usize, a: usize, b: usize| SketchCurve::Line { id: format!("l{n}"), a, b, construction: false, link: None };
    Sketch {
        points: vec![
            point("origin", [0.0, 0.0], true),
            point("p1", [0.0, 0.0], false),
            point("p2", [width, 0.0], false),
            point("p3", [width, height], false),
            point("p4", [0.0, height], false),
        ],
        curves: vec![line(1, 1, 2), line(2, 2, 3), line(3, 3, 4), line(4, 4, 1)],
        constraints: vec![
            Constraint::Horizontal { id: "k1".into(), l: 0 },
            Constraint::Horizontal { id: "k2".into(), l: 2 },
            Constraint::Vertical { id: "k3".into(), l: 1 },
            Constraint::Vertical { id: "k4".into(), l: 3 },
            Constraint::Coincident { id: "k5".into(), p: 0, q: 1 },
            Constraint::Length { id: "k6".into(), l: 0, value: width, param: "d1".into(), text: Some([0.0, -5.0]) },
            Constraint::Length { id: "k7".into(), l: 1, value: height, param: "d2".into(), text: Some([0.0, -5.0]) },
        ],
        links: vec![],
        counters: Counters { a: 0, j: 0, k: 7, l: 4, p: 4 },
    }
}

/// A closed polygon through `corners` (counter-clockwise) on the XY plane. Line `l<k>` runs from
/// corner `k - 1` to corner `k` (wrapping round), point `p<k>` is corner `k - 1`. Lines along X or
/// Y are constrained horizontal or vertical; the corners are otherwise free, as drawn.
#[must_use]
pub fn polygon_sketch(corners: &[[f64; 2]]) -> Sketch {
    polygon_sketch_sized(corners, &[], None)
}

/// Like [`polygon_sketch`], but with a `Length` constraint per side driven by the model parameter
/// `lens[i]`, where `Some`, so the outline re-dimensions in SolidCraft side by side like the
/// rectangle's do, and with an `Angle` constraint on one slanted side when `angle` is `Some`,
/// measured from the sketch's X axis (linked in as a construction line, as in [`slot_sketch`]),
/// so a slanted outline re-angles too. See [`dimensioned_sides`] and [`dimensioned_angle`] for
/// which sides may drive.
#[must_use]
pub fn polygon_sketch_dimensioned(corners: &[[f64; 2]], lens: &[Option<String>], angle: Option<&PolygonAngle>) -> Sketch {
    polygon_sketch_sized(corners, lens, angle)
}

/// What drives a polygon's angle: the side the `Angle` constraint sits on, the angle now (in
/// degrees, the side's own direction from the sketch's X axis, in [0, 360)) and the model
/// parameter driving it.
#[derive(Debug, Clone, PartialEq)]
pub struct PolygonAngle {
    /// The side's line index.
    pub side: usize,
    /// The angle now, in degrees.
    pub degrees: f64,
    /// The model parameter that drives it.
    pub param: String,
}

/// Which side of a closed polygon may get the driving `Angle` constraint: the longest side that
/// is neither horizontal nor vertical. Parallel slanted sides share their angle and a loop's
/// turns sum to 360°, so one angle per polygon is the independent one; the sides' lengths (see
/// [`dimensioned_sides_for_angle`]) fix the shape. `None` when every side is axis-aligned.
#[must_use]
pub fn dimensioned_angle(corners: &[[f64; 2]]) -> Option<usize> {
    let n = corners.len();
    (0..n)
        .filter(|&i| {
            let (a, b) = (corners[i], corners[(i + 1) % n]);
            (a[1] - b[1]).abs() >= 1e-9 && (a[0] - b[0]).abs() >= 1e-9
        })
        .max_by(|&a, &b| {
            let len = |i: usize| {
                let (p, q) = (corners[i], corners[(i + 1) % n]);
                (p[0] - q[0]).hypot(p[1] - q[1])
            };
            len(a).total_cmp(&len(b))
        })
}

/// Which sides of a closed polygon may get a driving `Length` constraint. Sides are grouped by
/// line direction; within a group of two or more parallel sides, loop closure makes one length
/// dependent on the others (for axis-aligned sides, a sum in their direction; for parallel
/// slanted sides, as in a parallelogram, an equality), and driving a dependent one too would
/// leave no solution when one parameter changes. So the longest side of every such group does
/// not drive.
#[allow(clippy::many_single_char_names)] // points a, b, directions d, u, as in the geometry
#[must_use]
pub fn dimensioned_sides(corners: &[[f64; 2]]) -> Vec<bool> {
    let n = corners.len();
    let side = |i: usize| (corners[i], corners[(i + 1) % n]);
    let len = |i: usize| {
        let (a, b) = side(i);
        (a[0] - b[0]).hypot(a[1] - b[1])
    };
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for i in 0..n {
        let (a, b) = side(i);
        let d = [b[0] - a[0], b[1] - a[1]];
        let group = groups.iter_mut().find(|g| {
            let (c, e) = side(g[0]);
            let u = [e[0] - c[0], e[1] - c[1]];
            (d[0] * u[1] - d[1] * u[0]).abs() <= 1e-9 * len(i) * len(g[0])
        });
        match group {
            Some(g) => g.push(i),
            None => groups.push(vec![i]),
        }
    }
    let mut dim = vec![true; n];
    for g in &groups {
        if let Some(&longest) = g.iter().filter(|_| g.len() >= 2).max_by(|&&a, &&b| len(a).total_cmp(&len(b))) {
            dim[longest] = false;
        }
    }
    dim
}

/// Whether side `i` of a closed polygon is axis-aligned, so the sketch holds it horizontal or
/// vertical (as `polygon_sketch_sized` does).
fn axis_aligned(corners: &[[f64; 2]], i: usize) -> bool {
    let n = corners.len();
    let (a, b) = (corners[i], corners[(i + 1) % n]);
    (a[1] - b[1]).abs() < 1e-9 || (a[0] - b[0]).abs() < 1e-9
}

/// Which sides of a closed polygon may get a driving `Length` constraint when side `angle` (see
/// [`dimensioned_angle`]) drives the `Angle` too. As in [`dimensioned_sides`], but the angle
/// side does not drive a length (with the other sides fixed, the angle determines it), and so do
/// no further sides while the driving constraints exceed the polygon's shape degrees of freedom:
/// its `n` corners are free in the plane and two degrees of freedom are its position, so `2n - 2`
/// remain for the shape, and every axis-aligned side takes one for its horizontal or vertical
/// constraint, every driving length one, the angle one. When they exceed that — a right
/// triangle's two legs with their horizontal and vertical constraints already fix the
/// diagonal's direction, so driving the angle too would leave no solution when the angle
/// parameter changes — the longest axis-aligned driving side's length is released (derived
/// through loop closure instead), until they fit.
#[allow(clippy::many_single_char_names)] // points a, b, as in the geometry
#[must_use]
pub fn dimensioned_sides_for_angle(corners: &[[f64; 2]], angle: usize) -> Vec<bool> {
    let n = corners.len();
    let len = |i: usize| {
        let (a, b) = (corners[i], corners[(i + 1) % n]);
        (a[0] - b[0]).hypot(a[1] - b[1])
    };
    let mut dim = dimensioned_sides(corners);
    dim[angle] = false;
    let hv = (0..n).filter(|&i| axis_aligned(corners, i)).count();
    while hv + dim.iter().filter(|&&d| d).count() + 1 > 2 * n - 2 {
        let Some(release) = (0..n).filter(|&i| dim[i]).max_by(|&a, &b| match axis_aligned(corners, a).cmp(&axis_aligned(corners, b)) {
            std::cmp::Ordering::Equal => len(a).total_cmp(&len(b)),
            ord => ord,
        }) else {
            break;
        };
        dim[release] = false;
    }
    dim
}

fn polygon_sketch_sized(corners: &[[f64; 2]], lens: &[Option<String>], angle: Option<&PolygonAngle>) -> Sketch {
    let n = corners.len();
    let mut points = vec![SketchPoint::new("origin", [0.0, 0.0], true)];
    points.extend(corners.iter().enumerate().map(|(i, c)| SketchPoint::new(format!("p{}", i + 1), *c, false)));
    let mut curves: Vec<SketchCurve> = (0..n)
        .map(|i| SketchCurve::Line { id: format!("l{}", i + 1), a: i + 1, b: (i + 1) % n + 1, construction: false, link: None })
        .collect();
    let mut constraints = Vec::new();
    for i in 0..n {
        let (a, b) = (corners[i], corners[(i + 1) % n]);
        if (a[1] - b[1]).abs() < 1e-9 {
            constraints.push(Constraint::Horizontal { id: format!("k{}", constraints.len() + 1), l: i });
        } else if (a[0] - b[0]).abs() < 1e-9 {
            constraints.push(Constraint::Vertical { id: format!("k{}", constraints.len() + 1), l: i });
        }
        if let Some(param) = lens.get(i).and_then(|l| l.as_ref()) {
            constraints.push(Constraint::Length {
                id: format!("k{}", constraints.len() + 1),
                l: i,
                value: snap_dim((a[0] - b[0]).hypot(a[1] - b[1])),
                param: param.clone(),
                text: None,
            });
        }
    }
    let mut links = Vec::new();
    let _axis_line = if let Some(a) = angle {
        let axis = || Some("j1".to_owned());
        points.push(SketchPoint { link: axis(), ..SketchPoint::new(format!("p{}", n + 1), [-100.0, 0.0], true) });
        points.push(SketchPoint { link: axis(), ..SketchPoint::new(format!("p{}", n + 2), [100.0, 0.0], true) });
        curves.push(SketchCurve::Line { id: format!("l{}", n + 1), a: n + 1, b: n + 2, construction: true, link: axis() });
        links.push(Link { id: "j1".into(), kind: "project", source: LinkSource { kind: "sketch_axis", name: "x" } });
        constraints.push(Constraint::Angle {
            id: format!("k{}", constraints.len() + 1),
            a: n,
            b: a.side,
            value: a.degrees.to_radians(),
            param: a.param.clone(),
        });
        Some(n)
    } else {
        None
    };
    // The counters count every allocated ID, the linked axis's `l<n+1>`, `p<n+1>`, `p<n+2>`
    // included, so a later edit allocates fresh IDs (as `slot_sketch`'s counters do).
    let counters = Counters { a: 0, j: links.len(), k: constraints.len(), l: curves.len(), p: points.len() - 1 };
    Sketch { points, curves, constraints, links, counters }
}

/// A sketch on the XY plane of closed polygons (each `corners` counter-clockwise or not), drawn as
/// [`polygon_sketch`] draws its outline: free corners, sides along X or Y held horizontal or
/// vertical.
#[must_use]
pub fn polygons_sketch(polygons: &[Vec<[f64; 2]>]) -> Sketch {
    polygons_sketch_sized(polygons, &[], &[])
}

/// Like [`polygons_sketch`], but with a `Length` constraint per side driven by the model parameter
/// `lens[polygon][side]`, where `Some`, and an `Angle` constraint per polygon where
/// `angles[polygon]` is `Some`, so every side re-dimensions in SolidCraft and slanted polygons
/// re-angle. The angle is measured from one shared X axis, linked in as a construction line, as
/// in [`slot_sketch`]. See [`dimensioned_sides`] and [`dimensioned_angle`] for which sides may
/// drive.
#[must_use]
pub fn polygons_sketch_dimensioned(polygons: &[Vec<[f64; 2]>], lens: &[Vec<Option<String>>], angles: &[Option<PolygonAngle>]) -> Sketch {
    polygons_sketch_sized(polygons, lens, angles)
}

fn polygons_sketch_sized(polygons: &[Vec<[f64; 2]>], lens: &[Vec<Option<String>>], angles: &[Option<PolygonAngle>]) -> Sketch {
    let mut sketch = Sketch {
        points: vec![SketchPoint::new("origin", [0.0, 0.0], true)],
        curves: vec![],
        constraints: vec![],
        links: vec![],
        counters: Counters { a: 0, j: 0, k: 0, l: 0, p: 0 },
    };
    // One construction line along the sketch's X axis, shared by every polygon's angle; its id
    // follows the polygons' lines and points, and the counters count it too, so a later edit
    // allocates fresh IDs (as `slot_sketch`'s counters do).
    let axis_line = if angles.iter().any(Option::is_some) {
        let total: usize = polygons.iter().map(Vec::len).sum();
        let axis = || Some("j1".to_owned());
        sketch.points.push(SketchPoint { link: axis(), ..SketchPoint::new(format!("p{}", total + 1), [-100.0, 0.0], true) });
        sketch.points.push(SketchPoint { link: axis(), ..SketchPoint::new(format!("p{}", total + 2), [100.0, 0.0], true) });
        sketch.curves.push(SketchCurve::Line { id: format!("l{}", total + 1), a: 1, b: 2, construction: true, link: axis() });
        sketch.links.push(Link { id: "j1".into(), kind: "project", source: LinkSource { kind: "sketch_axis", name: "x" } });
        Some(0)
    } else {
        None
    };
    add_polygons(&mut sketch, polygons, lens, angles, axis_line);
    if axis_line.is_some() {
        sketch.counters.p += 2;
        sketch.counters.l += 1;
    }
    sketch.counters.j = sketch.links.len();
    sketch
}

#[allow(clippy::too_many_arguments)] // one sketch's polygons, their lengths and angles, the axis
fn add_polygons(
    sketch: &mut Sketch,
    polygons: &[Vec<[f64; 2]>],
    lens: &[Vec<Option<String>>],
    angles: &[Option<PolygonAngle>],
    axis_line: Option<usize>,
) {
    for (pi, corners) in polygons.iter().enumerate() {
        let n = corners.len();
        let first = sketch.points.len();
        let base_point = sketch.counters.p;
        let base_line = sketch.counters.l;
        for (i, c) in corners.iter().enumerate() {
            sketch.points.push(SketchPoint::new(format!("p{}", base_point + i + 1), *c, false));
        }
        for i in 0..n {
            let line = sketch.curves.len();
            sketch.curves.push(SketchCurve::Line {
                id: format!("l{}", base_line + i + 1),
                a: first + i,
                b: first + (i + 1) % n,
                construction: false,
                link: None,
            });
            let (a, b) = (corners[i], corners[(i + 1) % n]);
            if (a[1] - b[1]).abs() < 1e-9 {
                sketch.constraints.push(Constraint::Horizontal { id: format!("k{}", sketch.counters.k + 1), l: line });
                sketch.counters.k += 1;
            } else if (a[0] - b[0]).abs() < 1e-9 {
                sketch.constraints.push(Constraint::Vertical { id: format!("k{}", sketch.counters.k + 1), l: line });
                sketch.counters.k += 1;
            }
            if let Some(param) = lens.get(pi).and_then(|l| l.get(i)).and_then(|o| o.as_ref()) {
                sketch.constraints.push(Constraint::Length {
                    id: format!("k{}", sketch.counters.k + 1),
                    l: line,
                    value: snap_dim((a[0] - b[0]).hypot(a[1] - b[1])),
                    param: param.clone(),
                    text: None,
                });
                sketch.counters.k += 1;
            }
        }
        if let (Some(a), Some(axis)) = (angles.get(pi).and_then(|o| o.as_ref()), axis_line) {
            sketch.constraints.push(Constraint::Angle {
                id: format!("k{}", sketch.counters.k + 1),
                a: axis,
                b: axis + 1 + base_line + a.side,
                value: a.degrees.to_radians(),
                param: a.param.clone(),
            });
            sketch.counters.k += 1;
        }
        sketch.counters.p += n;
        sketch.counters.l += n;
    }
}

/// Whether the straight side from `line_from` through `corner` is tangent to the arc of radius
/// `r` about `centre` at `corner`. The corner lies on the arc, so the side's line is tangent to
/// the circle exactly when the centre's distance to that line equals `r` (within a small
/// relative tolerance for the snapped output geometry); a merely gentle junction — an arc
/// closing onto its own chord, say — is not tangent and must not get a `Tangent` constraint.
#[must_use]
pub fn line_arc_tangent(line_from: [f64; 2], corner: [f64; 2], centre: [f64; 2], r: f64) -> bool {
    let (dx, dy) = (corner[0] - line_from[0], corner[1] - line_from[1]);
    let dist = ((centre[0] - line_from[0]) * dy - (centre[1] - line_from[1]) * dx).abs() / dx.hypot(dy);
    (r - dist).abs() <= 1e-3 * r.max(1.0)
}

/// A closed polygon through `corners` (counter-clockwise) where side `i` is the arc about
/// `arcs[i].0` with radius `arcs[i].1` when `Some`, else a straight line — like
/// [`polygon_sketch_dimensioned`], with a `Length` constraint on each driving line side
/// (`lens[i]`) and a `Diameter` constraint on each driving arc side (`arc_params[i]`, the model
/// parameter behind the side's `arc_<i>_d` user parameter). An arc side's centre is a free
/// sketch point: the solver places it equidistant from the side's two corners at the driving
/// diameter, so the centre follows the corners when a parameter changes — like a slot's arc
/// centres, which are free points too — and the sketch stays solvable. A straight side whose
/// shared corner with an arc side turns gently is held tangent to the arc (as a slot's sides
/// are), which keeps a driving diameter change solvable to first order even for a
/// semicircular side. Arc side pairs in `arc_equals` share one radius (`Equal`, as a slot's
/// ends do) and only the first of a pair drives — unless they share their centre point (a full
/// circle's two semicircles), when the shared centre ties their radii and no `Equal` is
/// emitted. `arcs` and `arc_params` are parallel to `corners`.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn polygon_sketch_arcs(
    corners: &[[f64; 2]],
    arcs: &[Option<([f64; 2], f64)>],
    lens: &[Option<String>],
    arc_params: &[Option<String>],
    arc_equals: &[(usize, usize)],
) -> Sketch {
    let n = corners.len();
    let mut points = vec![SketchPoint::new("origin", [0.0, 0.0], true)];
    points.extend(corners.iter().enumerate().map(|(i, c)| SketchPoint::new(format!("p{}", i + 1), *c, false)));
    let mut centre_of = vec![None; n];
    for (i, arc) in arcs.iter().enumerate() {
        if let Some((centre, _)) = arc {
            // An arc side whose centre an earlier arc side already has (a full circle's two
            // semicircles) refers to that centre point instead of adding a coincident one.
            let shared = (0..i).find(|&j| {
                arcs[j].is_some_and(|(c, rj)| {
                    (c[0] - centre[0]).hypot(c[1] - centre[1]) <= 1e-9 * rj.max(arc.map_or(0.0, |(_, r)| r)).max(1.0)
                })
            });
            let idx = if let Some(idx) = shared.and_then(|j| centre_of[j]) {
                idx
            } else {
                let idx = points.len();
                points.push(SketchPoint::new(format!("p{idx}"), *centre, false));
                idx
            };
            centre_of[i] = Some(idx);
        }
    }
    let mut curves: Vec<SketchCurve> = Vec::new();
    let mut constraints = Vec::new();
    let mut n_lines = 0;
    let mut n_arcs = 0;
    let mut side_curve = vec![0_usize; n];
    for i in 0..n {
        let (a, b) = (corners[i], corners[(i + 1) % n]);
        if let (Some((_, r)), Some(ci)) = (arcs[i], centre_of[i]) {
            n_arcs += 1;
            let curve = curves.len();
            side_curve[i] = curve;
            curves.push(SketchCurve::Arc { id: format!("a{n_arcs}"), c: ci, a: i + 1, b: (i + 1) % n + 1, construction: false });
            if let Some(param) = arc_params.get(i).and_then(Clone::clone) {
                constraints.push(Constraint::Diameter {
                    id: format!("k{}", constraints.len() + 1),
                    c: curve,
                    value: snap_dim(2.0 * r),
                    param,
                });
            }
        } else {
            n_lines += 1;
            let line = curves.len();
            side_curve[i] = line;
            curves.push(SketchCurve::Line { id: format!("l{n_lines}"), a: i + 1, b: (i + 1) % n + 1, construction: false, link: None });
            if (a[1] - b[1]).abs() < 1e-9 {
                constraints.push(Constraint::Horizontal { id: format!("k{}", constraints.len() + 1), l: line });
            } else if (a[0] - b[0]).abs() < 1e-9 {
                constraints.push(Constraint::Vertical { id: format!("k{}", constraints.len() + 1), l: line });
            }
            if let Some(param) = lens.get(i).and_then(|l| l.as_ref()) {
                constraints.push(Constraint::Length {
                    id: format!("k{}", constraints.len() + 1),
                    l: line,
                    value: snap_dim((a[0] - b[0]).hypot(a[1] - b[1])),
                    param: param.clone(),
                    text: None,
                });
            }
        }
    }
    // A straight side actually tangent to an arc side at their shared corner (within the
    // snapped geometry's tolerance) is held with a `Tangent` constraint, like a slot's sides.
    // Without it, growing a semicircular side's diameter would need its centre to slide off the
    // chord — second order, which no solver does from the snapped position.
    for i in 0..n {
        let Some((centre, r)) = arcs[i] else { continue };
        let prev = (i + n - 1) % n;
        if arcs[prev].is_none() && line_arc_tangent(corners[prev], corners[i], centre, r) {
            constraints.push(Constraint::Tangent { id: format!("k{}", constraints.len() + 1), a: side_curve[prev], b: side_curve[i] });
        }
        let next = (i + 1) % n;
        if arcs[next].is_none() && line_arc_tangent(corners[(i + 2) % n], corners[next], centre, r) {
            constraints.push(Constraint::Tangent { id: format!("k{}", constraints.len() + 1), a: side_curve[i], b: side_curve[next] });
        }
    }
    for &(p, q) in arc_equals {
        // Arc sides sharing their centre point are tied through it already; an `Equal` between
        // them would be redundant.
        if centre_of[p] != centre_of[q] {
            constraints.push(Constraint::Equal { id: format!("k{}", constraints.len() + 1), a: side_curve[p], b: side_curve[q] });
        }
    }
    let counters = Counters { a: n_arcs, j: 0, k: constraints.len(), l: n_lines, p: points.len() - 1 };
    Sketch { points, curves, constraints, links: vec![], counters }
}

/// What drives a slot's sketch: model parameters for the distance between the end centres,
/// the width and the long axis's angle from X, and expressions for where the middle lies.
#[derive(Debug, Clone, PartialEq)]
pub struct SlotDimensions {
    /// Between the end centres.
    pub len: String,
    /// The width (the ends' diameter).
    pub w: String,
    /// The middle's X, an expression (a point placed by it, not a dimension).
    pub x: String,
    /// The middle's Y, as `x`.
    pub y: String,
    /// The long axis from X (in degrees); `None` holds a slot along X or Y horizontal or
    /// vertical instead.
    pub angle: Option<String>,
}

/// A straight slot on the XY plane, as SolidCraft's centre-point slot tool draws it: lines `l1`
/// and `l2` tangent to arcs `a1` (about the end at `+len / 2` along `angle`, in degrees) and
/// `a2`, and the construction line `l3` between the arc centres. Its midpoint `p5` coincides
/// with `p6`, a point placed by the expressions `dims.x` and `dims.y` (SolidCraft's X and Y
/// distances are unsigned, so a dimension could not follow a position across the origin). With
/// an angle, it is measured from the sketch's X axis, linked in as `l4`; without, the slot is
/// held horizontal or vertical.
#[must_use]
pub fn slot_sketch(centre: [f64; 2], len: f64, w: f64, angle: f64, dims: &SlotDimensions) -> Sketch {
    let (sin, cos) = angle.to_radians().sin_cos();
    let (u, v, r) = ([cos, sin], [-sin, cos], w / 2.0);
    let at = |s: f64, t: f64| [centre[0] + s * u[0] + t * v[0], centre[1] + s * u[1] + t * v[1]];
    let half = len / 2.0;
    let mut points = vec![
        SketchPoint::new("origin", [0.0, 0.0], true),
        SketchPoint::new("p1", at(-half, -r), false),
        SketchPoint::new("p2", at(half, -r), false),
        SketchPoint::new("p3", at(half, r), false),
        SketchPoint::new("p4", at(-half, r), false),
        SketchPoint::new("a1.center", at(half, 0.0), false),
        SketchPoint::new("a2.center", at(-half, 0.0), false),
        SketchPoint::new("p5", centre, false),
        SketchPoint { exprs: Some(PointExprs { x: dims.x.clone(), y: dims.y.clone() }), ..SketchPoint::new("p6", centre, true) },
    ];
    let line = |n: usize, a: usize, b: usize, construction: bool| SketchCurve::Line { id: format!("l{n}"), a, b, construction, link: None };
    let mut curves = vec![
        line(1, 1, 2, false),
        SketchCurve::Arc { id: "a1".into(), c: 5, a: 2, b: 3, construction: false },
        line(2, 3, 4, false),
        SketchCurve::Arc { id: "a2".into(), c: 6, a: 4, b: 1, construction: false },
        line(3, 6, 5, true),
    ];
    let mut constraints: Vec<Constraint> = Vec::new();
    let mut add = |make: &dyn Fn(String) -> Constraint| {
        let id = format!("k{}", constraints.len() + 1);
        constraints.push(make(id));
    };
    add(&|id| Constraint::Tangent { id, a: 0, b: 1 });
    add(&|id| Constraint::Tangent { id, a: 2, b: 1 });
    add(&|id| Constraint::Tangent { id, a: 0, b: 3 });
    add(&|id| Constraint::Tangent { id, a: 2, b: 3 });
    add(&|id| Constraint::Equal { id, a: 3, b: 1 });
    add(&|id| Constraint::Midpoint { id, p: 7, l: 4 });
    add(&|id| Constraint::Length { id, l: 0, value: len, param: dims.len.clone(), text: None });
    add(&|id| Constraint::Diameter { id, c: 1, value: w, param: dims.w.clone() });
    add(&|id| Constraint::Coincident { id, p: 7, q: 8 });
    let mut links = vec![];
    match &dims.angle {
        None if sin.abs() < 0.5 => add(&|id| Constraint::Horizontal { id, l: 4 }),
        None => add(&|id| Constraint::Vertical { id, l: 4 }),
        Some(param) => {
            let axis = || Some("j1".to_owned());
            points.push(SketchPoint { link: axis(), ..SketchPoint::new("p7", [-100.0, 0.0], true) });
            points.push(SketchPoint { link: axis(), ..SketchPoint::new("p8", [100.0, 0.0], true) });
            curves.push(SketchCurve::Line { id: "l4".into(), a: 9, b: 10, construction: true, link: axis() });
            links.push(Link { id: "j1".into(), kind: "project", source: LinkSource { kind: "sketch_axis", name: "x" } });
            add(&|id| Constraint::Angle { id, a: 5, b: 4, value: angle.to_radians(), param: param.clone() });
        }
    }
    let counters = Counters { a: 2, j: links.len(), k: constraints.len(), l: curves.len() - 2, p: points.len() - 3 };
    Sketch { points, curves, constraints, links, counters }
}
