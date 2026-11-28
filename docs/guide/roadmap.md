# Roadmap

Done items carry the release they shipped in (`vX.Y.Z`); work merged after the latest release
(v0.3.0) is marked **unreleased** and gets its tag when release-please cuts the next one (v0.4.0).
Tags are read from the `chore: release` commits and `CHANGELOG.md` — never invented.

## Open points, prioritised

1. **Item 5.2 remainder** — D-shaped cut-outs, rounded-corner cut-outs and concave arc sides
   still import as a mesh (round outlines are done, **unreleased**). Next in the roadmap's own
   sequence (reworkable sketches are what sets revmesh apart beyond mesh2cad parity) and builds
   directly on the arc sides 5.2 just added.
2. **Converting a mesh body already in the open design** ([issue #70](https://github.com/stritti/revmesh/issues/70)):
   the spike ([research/convert-mesh-body.md](https://github.com/stritti/revmesh/blob/main/research/convert-mesh-body.md))
   showed this needs a second `command-plugin` package — a separate feature track after the
   sketch work.

## Items (stable numbering — referenced as `roadmap X.Y`)

1. **Skeleton** (done in v0.1.0-beta.1): plugin pipeline, document writer, mesh fallback, quality gates,
   releases.
2. **Plates** (done in v0.1.0-beta.1): any outline of straight sides with round through holes and rounded
   vertical corners.
3. **Parity with mesh2cad**, one recogniser per pull request, each compared with mesh2cad's
   output (`scripts/compare-mesh2cad.sh`, see [Development](/development)):
   1. (done in v0.1.0-beta.2) parameter names as mesh2cad writes them (`base_t`, `fillet_r`, `extrude_len`, …),
      and the part kept where it is in the file
   2. (done in v0.1.0-beta.2) hole rows, grids and bolt circles as SolidCraft patterns
   3. (done in v0.1.0-beta.2) counterbored and countersunk holes
   4. (done in v0.1.0-beta.2) blind holes
   5. (done in v0.1.0-beta.2) slots
   6. (done in v0.1.0-beta.2) bosses on the top face
   7. (done in v0.1.0-beta.2) polygonal cut-outs
   8. (done in v0.1.0-beta.2) pads and rectangular pockets (steps and other pockets later)
   9. (done in v0.1.0-beta.4) extrusions along X and Y
   10. (done in v0.1.0-beta.4) boxes with every edge rounded (`edge_fillet_r`), with through holes in them, and
       (done in v0.3.0: a side wall's round boss is stripped with the wall rotated to face up
       (`plate/bosses.rs::strip_side_bosses`) and written as a `cylinder` joined on the wall's
       `base_*` expression with the wall's outward normal as its axis) bosses on side faces
   11. (done in v0.1.0-beta.4–v0.1.0-beta.5) chamfers, drafts and the rounds at a boss's foot or a hole's rim
   12. combined extrusions: (done in v0.1.0-beta.5) the intersection of outlines extruded along X, Y and Z, with
       openings through them, straight blind cuts into them and round holes, and (done in v0.1.0-beta.5) unions
       of outlines stacked along an axis; (done in v0.3.0) round holes that cross inside a combined part —
       the bores are split out of their corner-sharing groups by circle consensus
       (`combined/holes.rs::crossed_bore`), a bore is accepted with a gap in its wall where the
       gap lies inside another bore's cylinder, and the volume takes the crossing once — and
       (done in v0.3.0) roofs and bevels on a stack: the top band's top edges may all carry one chamfer
       (`w` in on the top face, `h` down the walls — a bevel when small, a roof when the
       chamfers meet), recognised in `stack/mod.rs::recognize_chamfer` on a convex top outline
       and written as one `Chamfer1` with `chamfer` (45°) or `chamfer_w`/`chamfer_h`
4. **Checking** (done in v0.1.0-beta.4 as `cargo xtask check`, and at import time in v0.2.0 through the
   in-sandbox self-check of item 7.1): compare the rebuilt part with the mesh through SolidCraft's `evaluate` (volume
   and deviation), as mesh2cad does with CadQuery.
5. **Reworkable sketches** — beyond parity: every sketch fully dimensioned and constrained, so it
   can be reworked in SolidCraft. The concepts behind this and their sources (products and
   papers) are mapped in [Comparable tools → Concepts from the list, applied to
   revmesh](/guide/comparison#concepts-from-the-list-applied-to-revmesh):
   1. (done in v0.1.0-beta.6–v0.2.0 for plate outlines and cut-outs, and for the stack's and combined's sketches: a
      stack band `k`'s outline as `outline_<k>_<i>_len`/`_angle`, a combined extrusion along
      axis `a` as `outline_<a>_<i>_len`/`_angle`, their openings as `cutout_<n>_<i>_len`/`_angle`
      and a combined cut's outline as `cut_<n>_<i>_len`/`_angle`) a
      `Length` constraint per side with its own parameter (`outline_<i>_len`,
      `cutout_<n>_<i>_len`) and an `Angle` from the sketch's X axis on the longest slanted side
      with its own parameter (`outline_<i>_angle`, `cutout_<n>_<i>_angle`) in
      `polygon_sketch_dimensioned`/`polygons_sketch_dimensioned`
      (`crates/revmesh-core/src/design/sketch.rs`; parallel slanted sides share their angle and a
      loop's turns sum to 360°, so one angle per polygon is the independent one — and that side's
      length is then derived, driving both would over-constrain the sketch; likewise, when the
      remaining lengths and orientations already fix that side's direction, as a right triangle's
      legs held horizontal and vertical do for the diagonal, the longest axis-aligned side's
      length is derived too (`dimensioned_sides_for_angle`), so the angle parameter stays
      editable)
   2. (done in v0.1.0-beta.6–v0.2.0 for outlines and cut-outs) arc sides in outlines and in cut-outs as
      `SketchCurve::Arc` with a `Diameter` constraint driven by `arc_<i>_d` (an outline's) or
      `cutout_<n>_arc_<i>_d` (a cut-out's), the centre a free sketch point the solver
      places equidistant from the side's corners (like a slot's arc centres)
      (`polygon_sketch_arcs`, `crates/revmesh-core/src/design/sketch.rs`; recognised as
      `ProfileSpec::arcs` in `plate/outline.rs::outline_shape` and as `CutoutSpec::arcs` in
      `plate/holes.rs::cutout_shape`, which reads a cut-out's loop like an outline and accepts
      it only with arc sides and no rounded corners) — round outlines (done, **unreleased**: a
      run all the way round one circle is read as two opposite corners with a semicircular arc
      side either way, the two arcs sharing one centre point and one `arc_1_d` driving the
      `Diameter`) — D-shaped cut-outs, rounded-corner cut-outs and concave arc sides stay a mesh
   3. (done in v0.1.0-beta.6–v0.2.0 for pockets of any straight-sided outline, for steps down from the top face
      several levels deep along Z, for headed holes opening on a step's level — measured from
      the level and drilled from it, a rounded rim filleted on the pocket's start edge — and
      for steps along X or Y, which the stack recogniser reads as outlines stacked along that
      axis) one sketch per face level: generalise `split_faces` beyond
      bottom and top — arbitrary pockets and steps, both written as `Pocket<n>Sketch` +
      `Pocket<n>` cut with `pocket_<n>_<i>_len` per side
      (`recognize/to_design.rs::block_features`; the levels in `plate/prism.rs::step_levels`,
      each top loop's level tagged in `face_outlines`, heads measured from their mouth's face in
      `stepped_hole`),
      and with 3.12 combined extrusions
   4. (done in v0.1.0-beta.6–v0.2.0 for every recognised kind) per-face surface deviation in the import report,
      computed in the sandbox at import: each recognised face's label with the farthest its mesh
      points lie from the ideal surface (`recognize::deviations`: `plate::face_deviations` for
      prisms, a box rounded all over reports its rounded envelope as "rounded surface",
      `plate::walls_deviations` per extrusion and per band for combined and stacked parts,
      labelled `extrude <a> side <i>` / `band <k> side <i>`; `Walls::deviation`; the volume
      within 1% stays the gate, Hausdorff in `cargo xtask check`)
6. **More parts, fitted robustly** — from the code review of the recognisers against the
   references (the extracted know-how is packaged in the `revmesh-recognition-knowhow` agent
   skill; the concept → code mapping with the sources is [Comparable tools → Concepts from the
   list, applied to revmesh](/guide/comparison#concepts-from-the-list-applied-to-revmesh)):
   1. (done in v0.3.0: `prism_axes` in `recognize/axis.rs` fits the dominant parallel plane pairs by
      fixed-seed RANSAC (Schnabel et al. 2007, as `ransac_circle` does for circles) — one facet
      per 3-point sample, inliers within the tolerance scored by area, the opposite-facing
      plane as the dominant cluster of the anti-parallel facets, the part required to lie
      between the two planes and the axis off every world axis (up to three candidate planes,
      largest first, so a long extrusion's dominant wall does not hide its caps) — and returns
      the proper rotation `Frame` taking that axis to +Z; when no principal axis fits,
      `recognize` runs the Z pipeline in that frame and returns
      `Recognition::Slanted { prism, frame, offset }`.
      Recognition-side only: SolidCraft 0.51.2's origin-plane sketches and translate-only moves
      cannot place a slanted prism at the pinned version, so the import keeps the mesh and
      reports what was recognised (`revmesh.slanted`); writing it parametrically needs
      construction planes or a newer pin)
   2. (done in v0.2.0: a fixed-seed RANSAC circle in `plate/geometry.rs::ransac_circle` augmenting the
      algebraic `fit_circle` — 3-point minimal samples, inliers within tolerance, the
      iteration count adapted to the inlier ratio — behind `round_hole` in `plate/holes.rs`:
      a rim loop with a few corners off the circle is fitted on its inliers when at least
      `MIN_RING_SEGMENTS` (6) of them are a strict majority of the loop, their steps stay even
      and the excluded corners lie within one facet's sagitta of the circle, while the ring's
      own edges keep the original 50° gate, so an exporter's uneven rim reads as a round
      hole but a real flat or lobe stays refused: in a face's own loops fewer than
      `MIN_HOLE_SEGMENTS` (8) corners are read as a polygonal cut-out first, so a hexagon stays
      a cut-out there, and a D-shape's flat side and an omitted facet's long step are still
      one long step) robust circle fits on rim loops
   3. (done in v0.2.0: a torus about the known axis is a circle in the (radial, axial) profile plane, so
      `plate/geometry.rs::fit_torus` fits it as one — least-squares circle through the round's
      band corners, refused when they lie on no circle — in `rounded_rim` for hole rims
      (`plate/prism.rs`, with tangency to the bore's wall and to the face checked) and in
      `round_foot` for boss feet (`plate/bosses.rs`), replacing the per-corner averaging) analytic
      torus detection for the rounds at hole rims and boss feet
   4. (done in v0.3.0: `outline_shape` in `plate/outline.rs` returns one fitted radius per rounded corner and
      the design writes one `Fillet<g>` per radius group with `fillet_r_<g>` / `outline_fillet_r_<g>`
      — a single shared radius keeps the plain `fillet_r` / `outline_fillet_r` and `Fillet1`)
      per-corner radii in outlines
   5. (done in v0.3.0: `hole_groups` in `crates/revmesh-core/src/recognize/to_design.rs` writes a row whose
      angle is not 0° or ±90° as single holes, as a pattern's direction is not an expression in
      SolidCraft; an axis-aligned row stays a `HoleRow<n>` pattern) hole rows at any angle (a
      pattern's direction is not an expression in SolidCraft — write them as single holes or on
      a rotated sketch)
   6. (done in v0.3.0: `closed_loops` in `crates/revmesh-core/src/recognize/mod.rs` refuses, before any axis
      is tried, a mesh whose edges are not covered by two facets in opposite directions —
      corners welded on a sub-tolerance grid, an exporter's T-junctions counted by their sub-edges'
      length — so the recognised faces close into loops; and `check_walls` in `plate/prism.rs`
      refuses a wall facet whose plan is not a straight line within the tolerance plus the round
      walls' sagitta, so walls meet their face at 90°) ComplexGen-style topology gates
      alongside exact-or-nothing and the volume check: recognised faces close into loops, walls
      meet their face at 90°
7. **Import and conversion**:
   1. (done in v0.2.0: the manifest requests `evaluate@0.2`; a recognised design is staged
      and rebuilt in the app's private sandbox (`document.inspect`) before it is written, and a
      design the app cannot open falls back to the mesh body with the reason in the loss report,
      code `revmesh.selfcheck`; mesh-body imports are not checked — `self_check` in
      `crates/revmesh-plugin/src/lib.rs`, closing item 4 at import time) an in-sandbox self-check:
      request the `evaluate@0.2` capability, stage the produced design
      and rebuild it, and on feature errors fall back to the mesh body with the reason in the
      report
   2. (done in v0.3.0: `plugin.json` accepts `stl`, `obj` and `ply`; `format.rs` tells the format by
      the file's name or its first bytes, and `MeshReader` streams all three in chunks with the same
      size limits as `StlReader`; `obj.rs` and `ply.rs` (ASCII and binary little-endian) parse
      into the same `Mesh` with localised line/record errors; `import`, the plugin's `read_mesh`
      and `cargo xtask check` take any of them; the plugin-test fixtures `block-60x40x6.obj`,
      `block-60x40x6.ply` and `block-60x40x6-bin.ply` import the STL block unchanged, and
      `obj-bad.obj`/`ply-bad.ply` cover the error cases) OBJ and PLY input next to STL
   3. (done in v0.3.0) OBJ, PLY and ASCII STL accept finite double-precision coordinates. The mesh
      fallback preserves coordinates beyond the single-precision range. Parametric feature
      recognition uses its existing coordinate range.
   4. (done in v0.3.0: files up to 32 MiB are accepted (`MAX_BYTES` in `crates/revmesh-plugin/src/lib.rs`)
      and recognition is gated at `MAX_RECOGNITION_TRIANGLES` (≈ 335k triangles ≈ 16 MiB of binary
      STL) in `revmesh-core` — `import_mesh` takes the mesh by reference and clones only in the
      mesh/unrecognized fallback paths, and `Faces.chords` is slimmed from (a, b, middle, length)
      to (a, b, mid_z, length); measured peak ≈ 90 MiB at 16 MiB, inside the 128 MiB budget; more
      would need index-based face storage) a higher recognition ceiling (335k triangles today,
      `revmesh-core`) by slimming the face and wall indexes recognition
      builds
   5. (done in v0.3.0: ASCII-STL syntax errors carry a `SyntaxReason` (`stl.rs`) rendered in the import
      language via `stl.syntax.reason.*` in the locale catalogs; the dialog's `units` option
      (`plugin.json`) reads inch files as inches and scales them to millimetres in place
      (`convert_units`, range-checked afterwards), noted in the report as `revmesh.units`).
   6. (done in v0.3.0: the import dialog's `mode` option (`auto`/`mesh`) is gone — `plugin.json` only asks
      for `units`; the import always tries recognition and falls back to the mesh body where
      nothing is recognised (`revmesh.unrecognized`), the app's self-check fails
      (`revmesh.selfcheck`) or the mesh exceeds `MAX_RECOGNITION_TRIANGLES` (`revmesh.too_large`),
      and the loss report says which one happened) an import dialog without a mode choice

Converting a mesh body that is already in the open design (not just an STL file) is tracked in
[issue #70](https://github.com/stritti/revmesh/issues/70); the spike's findings — second package
with the `command-plugin` world, read the body via `model.mesh`, write back via `timeline.add` /
`timeline.delete`, selection via `x-solidcraft-selection` — are in
[research/convert-mesh-body.md](https://github.com/stritti/revmesh/blob/main/research/convert-mesh-body.md).

Contributions are very welcome: bug reports, STL files that import badly, recognisers, docs.
Start with [CONTRIBUTING.md](https://github.com/stritti/revmesh/blob/main/CONTRIBUTING.md).
