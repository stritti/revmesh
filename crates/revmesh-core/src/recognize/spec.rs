use crate::design::fmt_number;
use crate::i18n::Locale;

/// A rectangular plate from the origin, extruded up, with vertical through holes.
#[derive(Debug, Clone, PartialEq)]
pub struct PlateSpec {
    /// Size along X.
    pub x: f64,
    /// Size along Y.
    pub y: f64,
    /// Thickness along Z.
    pub z: f64,
    /// Through holes.
    pub holes: Vec<HoleSpec>,
    /// Straight through slots.
    pub slots: Vec<SlotSpec>,
    /// Round bosses standing on the top face.
    pub bosses: Vec<BossSpec>,
    /// Round bosses standing on a side wall, protruding along that wall's outward normal
    /// (roadmap 3.10).
    pub side_bosses: Vec<SideBossSpec>,
    /// Rectangular blocks standing on the top face.
    pub pads: Vec<PadSpec>,
    /// Rectangular blind pockets in the top or the bottom face.
    pub pockets: Vec<PocketSpec>,
    /// Openings through the plate, each a polygon of straight sides or of straight and arc
    /// sides.
    pub cutouts: Vec<CutoutSpec>,
    /// One radius per vertical corner edge, parallel to the corners `[0, 0]`, `[x, 0]`,
    /// `[x, y]`, `[0, y]`; empty when the corners are sharp. All four corners are rounded or
    /// none.
    pub outline_fillet_rs: Vec<f64>,
    /// The radius of all twelve edges (the four vertical corner edges and the eight on the top
    /// and bottom faces), if the box is rounded all over; mesh2cad's `edge_fillet_r`. Never set
    /// together with `outline_fillet_rs`.
    pub edge_fillet_r: Option<f64>,
}

/// A rectangular block standing on the top face, its sides along X and Y: `w` along X, `l`
/// along Y and `h` high, about its middle `(x, y)`.
#[derive(Debug, Clone, PartialEq)]
pub struct PadSpec {
    /// Middle X.
    pub x: f64,
    /// Middle Y.
    pub y: f64,
    /// Size along X.
    pub w: f64,
    /// Size along Y.
    pub l: f64,
    /// Height above the top face.
    pub h: f64,
}

/// A rectangular blind pocket with a flat floor, its sides along X and Y: `w` along X, `l`
/// along Y and `depth` deep from the face it opens on, about its middle `(x, y)`.
#[derive(Debug, Clone, PartialEq)]
pub struct PocketSpec {
    /// Middle X.
    pub x: f64,
    /// Middle Y.
    pub y: f64,
    /// Size along X.
    pub w: f64,
    /// Size along Y.
    pub l: f64,
    /// Depth from the face.
    pub depth: f64,
    /// Whether it opens on the bottom face (else on the top).
    pub bottom: bool,
}

/// A blind pocket whose mouth is any polygon of straight sides (not just an axis-aligned
/// rectangle), cut from the top or the bottom face down to a flat floor.
#[derive(Debug, Clone, PartialEq)]
pub struct OutlinePocketSpec {
    /// The mouth's corners, counter-clockwise from the lowest, the leftmost of those.
    pub corners: Vec<[f64; 2]>,
    /// Depth from the face.
    pub depth: f64,
    /// Whether it opens on the bottom face (else on the top).
    pub bottom: bool,
}

impl OutlinePocketSpec {
    /// Whether `p` lies inside the mouth's polygon, or within `tol` of its edge.
    #[must_use]
    #[allow(clippy::many_single_char_names)] // p, n and the edge vectors, as in the geometry
    pub fn contains(&self, p: [f64; 2], tol: f64) -> bool {
        let ring = &self.corners;
        let n = ring.len();
        let on_edge = ring.iter().zip(ring.iter().cycle().skip(1)).any(|(a, b)| {
            let (u, v) = ([b[0] - a[0], b[1] - a[1]], [p[0] - a[0], p[1] - a[1]]);
            let len2 = u[0] * u[0] + u[1] * u[1];
            let t = if len2 <= 0.0 { 0.0 } else { ((u[0] * v[0] + u[1] * v[1]) / len2).clamp(0.0, 1.0) };
            ((v[0] - t * u[0]).powi(2) + (v[1] - t * u[1]).powi(2)).sqrt() <= tol
        });
        if on_edge {
            return true;
        }
        // Even-odd ray casting along +X.
        let mut inside = false;
        for i in 0..n {
            let (a, b) = (ring[i], ring[(i + 1) % n]);
            if (a[1] > p[1]) != (b[1] > p[1]) && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0] {
                inside = !inside;
            }
        }
        inside
    }
}

/// A round boss standing on the top face: a cylinder `d` wide and `h` high about `(x, y)`.
#[derive(Debug, Clone, PartialEq)]
pub struct BossSpec {
    /// Centre X.
    pub x: f64,
    /// Centre Y.
    pub y: f64,
    /// Diameter.
    pub d: f64,
    /// Height above the top face.
    pub h: f64,
    /// The radius of the round where the boss meets the top face, if it has one.
    pub fillet: Option<f64>,
    /// The taper of a drafted boss (a cone narrowing upwards), degrees from the axis: `d` is
    /// then its diameter at the top face.
    pub draft_deg: Option<f64>,
    /// A chamfer of the boss's top edge: `w` in from the side, `h` down from the top.
    pub chamfer: Option<BossChamfer>,
}

/// A side wall of a plate standing along Z (the frame the part is recognised in): the wall a
/// [`SideBossSpec`] protrudes from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Wall {
    /// The wall at `x = size X`, protruding along +X.
    PosX,
    /// The wall at `x = 0`, protruding along −X.
    NegX,
    /// The wall at `y = size Y`, protruding along +Y.
    PosY,
    /// The wall at `y = 0`, protruding along −Y.
    NegY,
}

/// A round boss standing on a side wall: a cylinder `d` wide and `h` out along the wall's
/// outward normal, its axis through `at` — the two frame coordinates across the wall
/// (`[y, z]` on an X wall, `[x, z]` on a Y wall), the part's corner at the origin.
#[derive(Debug, Clone, PartialEq)]
pub struct SideBossSpec {
    /// The wall it stands on.
    pub wall: Wall,
    /// The axis's two coordinates across the wall, as above.
    pub at: [f64; 2],
    /// Diameter.
    pub d: f64,
    /// How far it protrudes from the wall.
    pub h: f64,
}

impl SideBossSpec {
    /// The volume of the boss, in mm³: the whole cylinder lies outside the wall.
    #[must_use]
    pub fn volume(&self) -> f64 {
        std::f64::consts::PI * (self.d / 2.0).powi(2) * self.h
    }
}

/// A chamfer of a boss's top edge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BossChamfer {
    /// How far in from the side it reaches on the top.
    pub w: f64,
    /// How far down from the top it reaches on the side.
    pub h: f64,
}

impl BossSpec {
    /// The boss's volume, in mm³, with the material its foot's round adds: the ring a quarter
    /// circle's outside sweeps round the cylinder (Pappus: the square of side `f` beside the
    /// wall, less the quarter disk, about the axis).
    #[must_use]
    pub fn volume(&self) -> f64 {
        use std::f64::consts::PI;
        let r = self.d / 2.0;
        let ring = self.fillet.map_or(0.0, |f| 2.0 * PI * (f * f * (r + f / 2.0) - PI * f * f / 4.0 * (r + f) + f.powi(3) / 3.0));
        if let Some(draft) = self.draft_deg {
            // A frustum from the full radius at the face to the top's.
            let top = r - self.h * draft.to_radians().tan();
            return PI * self.h / 3.0 * (r * r + r * top + top * top) + ring;
        }
        // A chamfer takes off the triangle `w` x `h` swept about the axis at its centroid.
        let chamfer = self.chamfer.map_or(0.0, |c| 2.0 * PI * (r - c.w / 3.0) * c.w * c.h / 2.0);
        PI * r * r * self.h + ring - chamfer
    }
}

/// A straight obround (stadium) slot through the plate along Z: two round ends `w` wide whose
/// centres are `len` apart, about the middle `(x, y)`, its long axis `angle` degrees from X
/// (0 up to but not including 180).
#[derive(Debug, Clone, PartialEq)]
pub struct SlotSpec {
    /// Middle X.
    pub x: f64,
    /// Middle Y.
    pub y: f64,
    /// Between the end centres.
    pub len: f64,
    /// Width, the ends' diameter.
    pub w: f64,
    /// The long axis from X, degrees.
    pub angle: f64,
}

impl SlotSpec {
    /// The slot's area seen from above, in mm².
    #[must_use]
    pub fn area(&self) -> f64 {
        self.len * self.w + std::f64::consts::PI * self.w * self.w / 4.0
    }
}

/// A vertical hole, through the plate or blind.
#[derive(Debug, Clone, PartialEq)]
pub struct HoleSpec {
    /// Centre X.
    pub x: f64,
    /// Centre Y.
    pub y: f64,
    /// Diameter (of the bore, below a counterbore or countersink).
    pub d: f64,
    /// A counterbore or countersink at one end, if it has one.
    pub head: Option<HoleHead>,
    /// Where a blind hole stops; `None` for a hole through the plate.
    pub blind: Option<Blind>,
}

/// A blind hole's end: a flat floor `depth` below the face it is drilled from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Blind {
    /// Depth from the face.
    pub depth: f64,
    /// Whether it is drilled from the bottom face (else from the top).
    pub bottom: bool,
}

/// The wider mouth of a hole, at the top or the bottom face.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HoleHead {
    /// A flat-bottomed recess: `d` wide and `depth` deep from the face it opens on.
    Counterbore {
        /// Diameter.
        d: f64,
        /// Depth from the face.
        depth: f64,
        /// Whether it opens on the bottom face (else on the top).
        bottom: bool,
    },
    /// A cone: `d` wide at the face it opens on, narrowing at `angle` (degrees, the cone's full
    /// angle) down to the bore.
    Countersink {
        /// Diameter at the face.
        d: f64,
        /// The cone's full angle, degrees.
        angle: f64,
        /// Whether it opens on the bottom face (else on the top).
        bottom: bool,
    },
    /// A round of radius `r` where the bore meets the face: the mouth is `2 r` wider.
    Round {
        /// The round's radius.
        r: f64,
        /// Whether it is on the bottom face's rim (else on the top's).
        bottom: bool,
    },
}

impl HoleHead {
    /// Whether it opens on the bottom face (else on the top's).
    #[must_use]
    pub fn bottom(&self) -> bool {
        match self {
            Self::Counterbore { bottom, .. } | Self::Countersink { bottom, .. } | Self::Round { bottom, .. } => *bottom,
        }
    }
}

impl HoleSpec {
    /// The material the hole removes from a plate `z` thick, in mm³.
    #[must_use]
    pub fn volume(&self, z: f64) -> f64 {
        let area = |d: f64| std::f64::consts::PI * d * d / 4.0;
        let bore = area(self.d) * self.blind.map_or(z, |b| b.depth);
        match self.head {
            None => bore,
            Some(HoleHead::Counterbore { d, depth, .. }) => bore + (area(d) - area(self.d)) * depth,
            // The round takes off the corner between the bore and the face: the square of side
            // `r` beside the bore less the quarter disk, swept about the axis (Pappus).
            Some(HoleHead::Round { r, .. }) => {
                use std::f64::consts::PI;
                let b = self.d / 2.0;
                bore + 2.0 * PI * (r * r * (b + r / 2.0) - PI * r * r / 4.0 * (b + r) + r.powi(3) / 3.0)
            }
            Some(HoleHead::Countersink { d, angle, .. }) => {
                // The frustum from the face down to where the cone meets the bore, less the bore
                // it overlaps.
                let (big, small) = (d / 2.0, self.d / 2.0);
                let h = (big - small) / (angle.to_radians() / 2.0).tan();
                bore + std::f64::consts::PI * h / 3.0 * (big * big + big * small + small * small) - area(self.d) * h
            }
        }
    }
}

impl PlateSpec {
    /// The same plate as an outline: `extrude_len` thick with its four corners as the outline,
    /// the way mesh2cad's extrusion module writes a rectangle extruded along X or Y (its box
    /// vocabulary is a Z extrusion's).
    #[must_use]
    pub fn into_profile(self) -> ProfileSpec {
        ProfileSpec {
            corners: vec![[0.0, 0.0], [self.x, 0.0], [self.x, self.y], [0.0, self.y]],
            z: self.z,
            holes: self.holes,
            slots: self.slots,
            bosses: self.bosses,
            pads: self.pads,
            pockets: self.pockets,
            outline_pockets: Vec::new(),
            cutouts: self.cutouts,
            outline_fillet_rs: self.outline_fillet_rs,
            // A box rounded all over is a Z plate (mesh2cad's box vocabulary is a Z
            // extrusion's); recognize() tries Z first and never converts it to a profile.
            // Side bosses only attach in the Z frame, so they are always empty here.
            arcs: Vec::new(),
        }
    }

    /// One line describing the plate, for the loss report.
    #[must_use]
    pub fn describe(&self) -> String {
        self.describe_in(Locale::En)
    }

    /// [`Self::describe`] in `locale`'s words.
    #[must_use]
    pub fn describe_in(&self, locale: Locale) -> String {
        describe(
            locale,
            [self.x, self.y, self.z],
            false,
            None,
            [
                self.holes.len(),
                self.slots.len(),
                self.bosses.len() + self.side_bosses.len(),
                self.cutouts.len(),
                self.pads.len(),
                self.pockets.len(),
            ],
            &self.outline_fillet_rs,
            self.edge_fillet_r,
        )
    }

    /// The volume of the plate this spec describes, in mm³: what SolidCraft measures once it has
    /// rebuilt the design.
    #[must_use]
    pub fn volume(&self) -> f64 {
        let base = if let Some(r) = self.edge_fillet_r {
            self.x * self.y * self.z - crate::plate::edge_fillet_removed(self.x, self.y, self.z, r)
        } else {
            // Each rounded corner loses its own `fillet_loss` (a 90° corner's square minus its
            // quarter circle), in the corners' order `[0,0]`, `[x,0]`, `[x,y]`, `[0,y]`.
            let corners = [[0.0, 0.0], [self.x, 0.0], [self.x, self.y], [0.0, self.y]];
            let loss: f64 = self
                .outline_fillet_rs
                .iter()
                .enumerate()
                .map(|(i, &r)| {
                    let (a, c, b) = (corners[(i + 3) % 4], corners[i], corners[(i + 1) % 4]);
                    crate::plate::fillet_loss(a, c, b, r)
                })
                .sum();
            (self.x * self.y - loss) * self.z
        };
        let holes: f64 = self.holes.iter().map(|h| h.volume(self.z)).sum();
        let slots: f64 = self.slots.iter().map(SlotSpec::area).sum();
        let bosses: f64 =
            self.bosses.iter().map(BossSpec::volume).sum::<f64>() + self.side_bosses.iter().map(SideBossSpec::volume).sum::<f64>();
        let cutouts: f64 = self.cutouts.iter().map(|c| polygon_area_arcs(&c.corners, c.arc_sides()).abs()).sum();
        base - (slots + cutouts) * self.z - holes + bosses + blocks(&self.pads, &self.pockets)
    }
}

/// An arc side of an outline or a cut-out: the side from corner `i` to corner `i + 1` (wrapping
/// round) is the arc of the circle about `centre` with radius `r`, bulging away from the
/// polygon's interior (the loop runs counter-clockwise).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArcSide {
    /// The arc's centre.
    pub centre: [f64; 2],
    /// The arc's radius.
    pub r: f64,
}

/// An opening through the plate: a polygon of straight sides, or of straight and arc sides, its
/// corners counter-clockwise from the lowest, the leftmost of those.
#[derive(Debug, Clone, PartialEq)]
pub struct CutoutSpec {
    /// The corners as if every side were straight.
    pub corners: Vec<[f64; 2]>,
    /// `arcs[i]` is the arc side from corner `i` to corner `i + 1` when that side is curved,
    /// `None` when it is a straight line. Empty when every side is straight.
    pub arcs: Vec<Option<ArcSide>>,
}

impl CutoutSpec {
    /// The arc sides, parallel to `corners` (`arcs[i]` from corner `i` to corner `i + 1`); an
    /// empty slice when every side is straight.
    #[must_use]
    pub fn arc_sides(&self) -> &[Option<ArcSide>] {
        if self.arcs.len() == self.corners.len() { &self.arcs } else { &[] }
    }

    /// Whether any side is an arc side.
    #[must_use]
    pub fn has_arcs(&self) -> bool {
        self.arc_sides().iter().any(Option::is_some)
    }
}

/// A plate whose outline is any polygon of straight sides (counter-clockwise, in the first
/// quadrant), extruded up from the XY plane, with vertical through holes.
#[derive(Debug, Clone, PartialEq)]
pub struct ProfileSpec {
    /// The outline's corners as if they were sharp, counter-clockwise.
    pub corners: Vec<[f64; 2]>,
    /// `arcs[i]` is the arc side from corner `i` to corner `i + 1` when that side is curved,
    /// `None` when it is a straight line. Empty when every side is straight.
    pub arcs: Vec<Option<ArcSide>>,
    /// Thickness along Z.
    pub z: f64,
    /// Through holes.
    pub holes: Vec<HoleSpec>,
    /// Straight through slots.
    pub slots: Vec<SlotSpec>,
    /// Round bosses standing on the top face.
    pub bosses: Vec<BossSpec>,
    /// Rectangular blocks standing on the top face.
    pub pads: Vec<PadSpec>,
    /// Rectangular blind pockets in the top or the bottom face.
    pub pockets: Vec<PocketSpec>,
    /// Blind pockets of any outline of straight sides in the top or the bottom face.
    pub outline_pockets: Vec<OutlinePocketSpec>,
    /// Openings through the plate, each a polygon of straight sides or of straight and arc
    /// sides.
    pub cutouts: Vec<CutoutSpec>,
    /// One radius per vertical corner edge, parallel to `corners`, inside and outside corners
    /// alike; empty when the corners are sharp. All corners are rounded or none.
    pub outline_fillet_rs: Vec<f64>,
}

impl ProfileSpec {
    /// The arc sides, parallel to `corners` (`arcs[i]` from corner `i` to corner `i + 1`); an
    /// empty slice when every side is straight.
    #[must_use]
    pub fn arc_sides(&self) -> &[Option<ArcSide>] {
        if self.arcs.len() == self.corners.len() { &self.arcs } else { &[] }
    }

    /// Whether the outline is a full circle: every side an arc about one and the same centre
    /// (recognised as two corners with a semicircle between them either way).
    #[must_use]
    pub fn is_round(&self) -> bool {
        let arcs = self.arc_sides();
        arcs.len() == self.corners.len()
            && self.corners.len() >= 2
            && arcs.windows(2).all(|w| match (w[0], w[1]) {
                (Some(a), Some(b)) => (a.centre[0] - b.centre[0]).hypot(a.centre[1] - b.centre[1]) <= 1e-9 * a.r.max(b.r).max(1.0),
                _ => false,
            })
    }

    /// One line describing the plate, for the loss report.
    #[must_use]
    pub fn describe(&self) -> String {
        self.describe_in(Locale::En)
    }

    /// [`Self::describe`] in `locale`'s words.
    #[must_use]
    pub fn describe_in(&self, locale: Locale) -> String {
        let size = self.size();
        describe(
            locale,
            [size[0], size[1], self.z],
            self.is_round(),
            Some(self.corners.len()),
            [
                self.holes.len(),
                self.slots.len(),
                self.bosses.len(),
                self.cutouts.len(),
                self.pads.len(),
                self.pockets.len() + self.outline_pockets.len(),
            ],
            &self.outline_fillet_rs,
            None,
        )
    }

    /// The size of the outline's bounding box along X and Y, arc sides' bulges included: an arc
    /// can reach past its endpoints' box, and the reported dimensions must cover it.
    #[must_use]
    pub fn size(&self) -> [f64; 2] {
        use std::f64::consts::PI;
        let n = self.corners.len();
        let mut min = [f64::INFINITY; 2];
        let mut max = [f64::NEG_INFINITY; 2];
        let mut take = |p: [f64; 2]| {
            for (axis, c) in p.iter().enumerate() {
                min[axis] = min[axis].min(*c);
                max[axis] = max[axis].max(*c);
            }
        };
        for c in &self.corners {
            take(*c);
        }
        // An arc side's circle extrema that lie within its counter-clockwise sweep from one
        // corner to the next bound the outline too.
        for (i, arc) in self.arc_sides().iter().enumerate() {
            let Some(arc) = arc else { continue };
            let (a, b) = (self.corners[i], self.corners[(i + 1) % n]);
            let angle = |p: [f64; 2]| (p[1] - arc.centre[1]).atan2(p[0] - arc.centre[0]);
            let (from, sweep) = (angle(a), (angle(b) - angle(a)).rem_euclid(2.0 * PI));
            for k in 0..4 {
                let t = f64::from(k) * PI / 2.0;
                if (t - from).rem_euclid(2.0 * PI) <= sweep {
                    take([arc.centre[0] + arc.r * t.cos(), arc.centre[1] + arc.r * t.sin()]);
                }
            }
        }
        [max[0] - min[0], max[1] - min[1]]
    }

    /// The volume of the plate this spec describes, in mm³.
    #[must_use]
    #[allow(clippy::many_single_char_names)] // n and the corners a, c, b, as in the geometry
    pub fn volume(&self) -> f64 {
        let n = self.corners.len();
        let mut area = polygon_area_arcs(&self.corners, self.arc_sides());
        for i in 0..n {
            let r = self.outline_fillet_rs.get(i).copied().unwrap_or(0.0);
            if r > 0.0 {
                let (a, c, b) = (self.corners[(i + n - 1) % n], self.corners[i], self.corners[(i + 1) % n]);
                area -= crate::plate::fillet_loss(a, c, b, r);
            }
        }
        let holes: f64 = self.holes.iter().map(|h| h.volume(self.z)).sum();
        let slots: f64 = self.slots.iter().map(SlotSpec::area).sum();
        let bosses: f64 = self.bosses.iter().map(BossSpec::volume).sum();
        let cutouts: f64 = self.cutouts.iter().map(|c| polygon_area_arcs(&c.corners, c.arc_sides()).abs()).sum();
        let outline_pockets: f64 = self.outline_pockets.iter().map(|p| polygon_area(&p.corners) * p.depth).sum();
        (area - slots - cutouts) * self.z - holes - outline_pockets + bosses + blocks(&self.pads, &self.pockets)
    }
}

/// "A 70 x 40 x 6 mm plate with a 6-sided outline, 2 holes and 3 mm rounded corners".
/// The area a polygon encloses, either way round.
fn polygon_area(corners: &[[f64; 2]]) -> f64 {
    let n = corners.len();
    ((0..n).map(|i| corners[i][0] * corners[(i + 1) % n][1] - corners[(i + 1) % n][0] * corners[i][1]).sum::<f64>() / 2.0).abs()
}

/// The signed area a closed polygon of `corners` (counter-clockwise: positive) with arc sides
/// `arcs` encloses: the chord polygon's plus each arc side's circular segment between chord and
/// arc.
fn polygon_area_arcs(corners: &[[f64; 2]], arcs: &[Option<ArcSide>]) -> f64 {
    let n = corners.len();
    let mut area = (0..n).map(|i| corners[i][0] * corners[(i + 1) % n][1] - corners[(i + 1) % n][0] * corners[i][1]).sum::<f64>() / 2.0;
    for (i, arc) in arcs.iter().enumerate() {
        let Some(arc) = arc else { continue };
        let (a, b) = (corners[i], corners[(i + 1) % n]);
        let (ca, cb) = ([a[0] - arc.centre[0], a[1] - arc.centre[1]], [b[0] - arc.centre[0], b[1] - arc.centre[1]]);
        let theta = (cb[1].atan2(cb[0]) - ca[1].atan2(ca[0])).rem_euclid(2.0 * std::f64::consts::PI);
        area += 0.5 * arc.r * arc.r * (theta - theta.sin());
    }
    area
}

/// The volume pads add and pockets take away, in mm³.
fn blocks(pads: &[PadSpec], pockets: &[PocketSpec]) -> f64 {
    pads.iter().map(|p| p.w * p.l * p.h).sum::<f64>() - pockets.iter().map(|p| p.w * p.l * p.depth).sum::<f64>()
}

/// `counts` are the holes, the slots, the bosses, the cut-outs, the pads and the pockets. A
/// `round` outline (a full circle) is described as such, not as an `n`-sided one.
fn describe(
    locale: Locale,
    size: [f64; 3],
    round: bool,
    sides: Option<usize>,
    counts: [usize; 6],
    fillet_rs: &[f64],
    edge_r: Option<f64>,
) -> String {
    let number = |v: f64| locale.decimal(&fmt_number(v));
    let [x, y, z] = size.map(number);
    let mut parts = Vec::new();
    if round {
        parts.push(locale.text("summary.round_outline").to_owned());
    } else if let Some(n) = sides {
        parts.push(locale.format("summary.outline", &[("n", &n)]));
    }
    for (n, kind) in counts.into_iter().zip(["hole", "slot", "boss", "cutout", "pad", "pocket"]) {
        match n {
            0 => {}
            1 => parts.push(locale.text(&format!("summary.{kind}.one")).to_owned()),
            n => parts.push(locale.format(&format!("summary.{kind}.other"), &[("n", &n)])),
        }
    }
    // One radius reads as before ("3 mm rounded corners"); different radii list them.
    let mut distinct: Vec<f64> = Vec::new();
    for &r in fillet_rs {
        if !distinct.contains(&r) {
            distinct.push(r);
        }
    }
    match distinct.len() {
        0 => {}
        1 => parts.push(locale.format("summary.rounded_corners", &[("r", &number(distinct[0]))])),
        _ => {
            let rs = distinct.iter().map(|&r| number(r)).collect::<Vec<_>>().join(", ");
            parts.push(locale.format("summary.rounded_corners_rs", &[("rs", &rs)]));
        }
    }
    if let Some(r) = edge_r {
        parts.push(locale.format("summary.rounded_edges", &[("r", &number(r))]));
    }
    let plate = locale.format("summary.plate", &[("x", &x), ("y", &y), ("z", &z)]);
    let Some((last, rest)) = parts.split_last() else {
        return plate;
    };
    let parts = if rest.is_empty() { last.clone() } else { locale.format("summary.list", &[("rest", &rest.join(", ")), ("last", last)]) };
    locale.format("summary.with", &[("plate", &plate), ("parts", &parts)])
}
