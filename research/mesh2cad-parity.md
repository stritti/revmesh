# mesh2cad parity map

Source: [mesh2cad](https://github.com/emirmrkaljevic/mesh2cad) (MIT), read at commit
`701308f96e932ac59f2a26dd55402752aa7720ac` (2026-11-20; checked out 2026-11-26). revmesh ports its
approach, the part classes it targets and its parameter names — in Rust, so it runs inside
SolidCraft's WebAssembly sandbox without Python, numpy or OpenCascade. This file maps mesh2cad's
modules to ours and lists where we deliberately differ. To compare both tools' output on the same
STL files, run `scripts/compare-mesh2cad.sh` (see
[docs/development.md](../docs/development.md#comparing-with-mesh2cad)); a pull request that adds or
changes a recogniser includes that comparison table.

## Module map

| mesh2cad (`src/mesh2cad/`) | revmesh (`crates/revmesh-core/src/`) | Notes |
|---|---|---|
| `io.py` mesh loading and cleaning | `stl.rs`, `mesh.rs` | chunked `StlReader` streaming in ≤ 1 MiB host reads instead of trimesh |
| `segment.py` planar face segmentation | `plate/faces.rs`, `plate/walls.rs` | |
| `fit.py` plane/cylinder/sphere/cone fitting | `plate/geometry.rs`, `plate/outline.rs`, `plate/holes.rs` | |
| `extrude.py` straight extrusions of a 2D outline, bosses along an axis | `plate/prism.rs`, `plate/bosses.rs`, `recognize/axis.rs` | prism tried along Z, then X, then Y (see deviation 7) |
| `patterns.py` rows, grids and bolt circles of plain through holes | `patterns.rs` | written as SolidCraft patterns, not repeated holes |
| `slots.py` straight through slots | `plate/holes.rs`, `design/sketch.rs` | `Slot<n>Sketch` + `Slot<n>` cut |
| `csg.py` booleans of straight extrusions | `combined/` (`mod.rs`, `silhouette.rs`, `cuts.rs`, `holes.rs`, `volume.rs`) | intersection of outlines along X, Y and Z |
| `stack.py` outlines stacked along an axis | `stack/`, `recognize/stack.rs` | union of bands extruded over their own span |
| `refine.py` fallback outline and chamfer cuts on a combined extrusion | `combined/silhouette.rs`, `combined/cuts.rs` | |
| `parametric.py` shared parametric vocabulary | `recognize/spec.rs` | the `PlateSpec` … types |
| `cadquery_code.py`, `openscad_code.py`, `freecad_code.py` script emitters | `recognize/to_design.rs`, `design/` | one writer: the SolidCraft design JSON (`docs/design-format.md`) |
| `params.py` reading/editing named parameters | `design/`, `report.rs` | parameters live in SolidCraft's parameter panel |
| `report.py` fidelity report | `report.rs`, `check.rs` | import report with volume change and per-face surface deviation |
| `brep.py` exact B-rep / STEP export | — | no STEP: see deviation 8 |
| `cli.py`, `mcp_server.py` entry points | `crates/revmesh-plugin/` | one WIT importer entry (`craft:plugin@0.2.0`, `importer-plugin` world) |

## Parameter names

Kept as mesh2cad writes them, so a part rebuilt by either tool reads the same: `base_*` (a box
prism along Z), `extrude_len` / `extrude_0` and `outline_<a>` / `outline_<a>_cutout_<i>` per axis,
`hole_<n>_*`, `slot_<n>_*`, `boss_<n>_*`, `pad_<n>_*`, `pocket_<n>_*`, `cut_<n>_*`,
`cutout_<n>`, `hole_row_<n>_*`, `hole_grid_<n>_*`, `hole_circle_<n>_*`, `fillet_r`,
`outline_fillet_r`, `edge_fillet_r`. The full table of what maps to which SolidCraft feature is
the README's [What `auto` recognises](../README.md#what-auto-recognises).

## Deliberate deviations

1. **Hole numbering.** The two tools may number holes in a different order; compare the sets
   (`scripts/compare-mesh2cad.sh` does).
2. **L-brackets.** mesh2cad writes an L with every corner rounded alike as `base_x`, `base_y`,
   `leg_x`, `leg_y`, `fillet_r` and `base_hole_N_*` / `leg_hole_N_*`; revmesh writes it as any other
   outline (`extrude_len`, `outline_fillet_r`, `hole_N_*`) — SolidCraft has no L primitive, the
   outline is a sketch.
3. **The outline itself.** mesh2cad's `outline` is a list of points; SolidCraft has no list
   parameters, so revmesh's outline lives in its sketch, each straight side a `Length` dimension
   (`outline_<i>_len`), each arc side a `Diameter` (`arc_<i>_d`), the longest slanted side an
   `Angle`.
4. **`hole_row_N_angle` is not a parameter.** mesh2cad writes the row's direction; a SolidCraft
   pattern direction is not an expression, so the row's direction is fixed at import.
5. **Pattern precedence.** A grid wins over a bolt circle (four corner holes are both); a complete
   grid can also be found alongside one same-diameter hole that shares neither a row nor a column
   with it; two holes are never a pattern; a pattern with one hole missing or out of place stays
   single holes. mesh2cad patterns only plain through holes (`_PLAIN` in `patterns.py`); so do we
   — counterbored, countersunk and blind holes stay single holes in both.
6. **Quality gate.** The rebuilt part must match the mesh's volume within 1 %, otherwise the mesh is
   kept and the import report says so. mesh2cad gates its generated scripts the same way through
   CadQuery (deviation from the mesh); revmesh checks inside SolidCraft's sandbox
   (`evaluate@0.2`, `check.rs`) and also reports the surface deviation per recognised face.
7. **Axis tries above 200,000 triangles.** A rotated X/Y recognition attempt would hold a second
   mesh copy plus the face/wall indexes (measured peak ~133 MiB at 333k triangles), over the
   128 MiB budget `plugin.json` asks for — so above 200,000 triangles `recognize` only tries Z and a
   larger part lying along X or Y falls back to a mesh body. mesh2cad tries X, Y and Z always.
8. **No STEP.** mesh2cad's `brep.py` exports an exact B-rep STEP for what it cannot express
   parametrically; revmesh keeps the mesh body inside the SolidCraft design instead, with the
   reason in the import report.
9. **Not recognised yet** (kept as mesh, with the reason): curved sides that are not circular arcs,
   rounded corners mixed with arc sides, only some corners
   rounded, D-shaped and other curved cut-outs, blind or slanted slots, corners rounded almost
   all the way along their sides, steps along X or Y, pockets with curved sides, bosses with a
   draft, a round or a hole, other shapes on the top face, a counterbore/countersink/rounded rim
   opening on a step's level, blind holes with a drill point or a counterbore, parts rotated off
   X, Y and Z.

## Adding a recogniser

Open an issue first (so two people don't port the same thing), then port the mesh2cad module from
the map above, keep its parameter names, and include the `scripts/compare-mesh2cad.sh` table for
your test files in the pull request. See [CONTRIBUTING.md](../CONTRIBUTING.md) and
[docs/development.md](../docs/development.md).
