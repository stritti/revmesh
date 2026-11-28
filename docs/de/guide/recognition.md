# Was der Import erkennt

| In der STL | In SolidCraft | Parameter |
|---|---|---|
| Eine rechteckige Platte, Dicke entlang Z | `Sketch1` (Rechteck auf XY), `Extrude1` und `Move1` (dorthin, wo die Platte liegt) | `base_x`, `base_y`, `base_t`; die Mitte `base_cx`, `base_cy` und die Unterseite `base_z0` |
| Eine Platte mit einer beliebigen anderen Kontur aus geraden Seiten | `Sketch1` (die Kontur auf XY, Seiten entlang X oder Y bleiben horizontal bzw. vertikal, jede Seite eine `Length`-Bemaßung, die längste schräge Seite eine `Angle` von der X-Achse), `Extrude1` und `Move1` | `extrude_len` (Dicke), `extrude_0` (Unterseite), `outline_<i>_len` pro Seite, `outline_<i>_angle` an der längsten schrägen Seite |
| Eine Kontur mit Kreisbogen-Seiten (eine Wölbung, das runde Ende eines Stadions) | `Sketch1` (die Kontur auf XY, jede Bogen-Seite eine `arc`-Kurve mit einer `Diameter`-Bemaßung, jede gerade Seite eine `Length`-Bemaßung), `Extrude1` und `Move1` | `extrude_len`, `extrude_0`, `outline_<i>_len` pro gerader Seite, `arc_<i>_d` pro Bogen-Seite (deren Mittelpunkt ein freier Skizzenpunkt) |
| Eine runde Kontur (ein Kreis ganz herum) | `Sketch1` (der Kreis auf XY als zwei `arc`-Kurven um einen gemeinsamen Mittelpunkt, eine `Diameter`-Bemaßung an der ersten), `Extrude1` und `Move1` | `extrude_len`, `extrude_0`, `arc_1_d` |
| Runde vertikale Durchgangsbohrungen (8 oder mehr Facetten, die gleichmäßig um den Kreis laufen; ein Rand mit wenigen Ecken neben dem Kreis wird auf dem Rest per fixed-seed RANSAC-Kreis eingepasst) | `Hole1`, `Hole2`, … von links nach rechts nummeriert | `hole_<n>_x`, `hole_<n>_y` (wo die Bohrung in der Datei liegt), `hole_<n>_d` |
| Eine Durchgangsbohrung mit Senkung (Counterbore), deren Senkung auf der Ober- oder Unterseite sitzt | `Hole<n>` vom Typ Counterbore (von dieser Fläche gebohrt) | `hole_<n>_d`, `hole_<n>_cbore_d`, `hole_<n>_cbore_depth` (von dieser Fläche) |
| Eine Durchgangsbohrung mit Kegelsenkung (Countersink) auf der Ober- oder Unterseite | `Hole<n>` vom Typ Countersink | `hole_<n>_d`, `hole_<n>_csk_d` (an der Fläche), `hole_<n>_csk_angle` (der volle Kegelwinkel) |
| Eine Durchgangsbohrung mit verrundetem Rand auf der Ober- oder Unterseite | `Hole<n>` und `Hole<n>Rim` (eine Verrundung der Kante zwischen Fläche und Bohrung) | `hole_<n>_d`, `hole_<n>_rim_r` (oben) bzw. `hole_<n>_bot_rim_r` (unten) |
| Drei oder mehr Bohrungen eines Durchmessers auf einer Geraden mit gleichem Abstand entlang X oder Y | `HoleRow<n>` und `HoleRow<n>Pattern` (ein rechteckiges Muster entlang der Reihe) | `hole_row_<n>_n`, `_pitch`, `_d` sowie die erste Bohrung `_x`, `_y` |
| Dieselbe Reihe in einem beliebigen anderen Winkel (die Richtung eines Musters ist in SolidCraft kein Ausdruck) | eine `Hole<n>` pro Bohrung | `hole_<n>_x`, `hole_<n>_y`, `hole_<n>_d` |
| Bohrungen eines Durchmessers in einem achsparallel Gitter von mindestens 2 x 2 | `HoleGrid<n>` und `HoleGrid<n>Pattern` | `hole_grid_<n>_nx`, `_ny`, `_pitch_x`, `_pitch_y`, `_d` sowie die Eckbohrung `_x`, `_y` |
| Drei oder mehr Bohrungen eines Durchmessers gleichmäßig auf einem Kreis verteilt (ein Lochkreis) | `HoleCircle<n>` und `HoleCircle<n>Pattern` (ein kreisförmiges Muster um den Kreis-Mittelpunkt) | `hole_circle_<n>_n`, `_d`, `_r` (Mittelpunkt zu Bohrung), `_cx`, `_cy`, `_angle` (erste Bohrung, von +X) |
| Eine runde Sacklochbohrung mit flachem Boden, von der Ober- oder Unterseite gebohrt | `Hole<n>` mit einer Tiefe (von dieser Fläche gebohrt) | `hole_<n>_d`, `hole_<n>_depth` (von dieser Fläche) |
| Ein gerades Langloch durch die Platte (zwei runde Enden einer Breite), entlang X, Y oder in einem beliebigen Winkel | `Slot<n>Sketch` (das Langloch auf XY) und `Slot<n>` (ein Schnitt durch alles) | `slot_<n>_len` (zwischen den End-Mittelpunkten), `_w`, die Mitte `_x`, `_y` und `_angle` (der langen Achse, von +X) |
| Ein gerader runder Zapfen an einer Seitenwand (der X- oder Y-Wand einer rechteckigen Platte), entlang der Wand-Normale vorstehend | `Boss<n>` (ein `cylinder`, mit der Platte verbunden, seine Basis an der Wand-`base_*`-Ausdruck, seine Achse die Wand-Außennormale), nach den Ober-Zapfen nummeriert | `boss_<n>_d`, `boss_<n>_h` (aus der Wand), die Achse quer zur Wand `boss_<n>_y`/`_z` (an einer X-Wand) bzw. `boss_<n>_x`/`_z` (an einer Y-Wand) |
| Ein runder Zapfen auf der Oberseite, gerade, mit Formschräge (nach oben schmaler) oder mit gefaster Oberkante | `Boss<n>` (ein Zylinder, mit der Platte verbunden, auf ihrer Oberseite) | `boss_<n>_d`, `boss_<n>_h` (über der Oberseite), `boss_<n>_fillet` (eine Verrundung an seinem Fuß, falls vorhanden; `Boss<n>Fillet`), `boss_<n>_draft_deg` (ein Zapfen mit Formschräge ist ein Kegel), `boss_<n>_chamfer` (45°) oder `boss_<n>_chamfer_h` und `_w` (`Boss<n>Chamfer`), die Mitte `boss_<n>_x`, `boss_<n>_y` |
| Ein Ausschnitt aus geraden Seiten durch die Platte (3 bis 24 Ecken, jede ein echter Knick) | `CutoutsSketch` (jeder Ausschnitt auf XY, jede Seite eine `Length`-Bemaßung) und `Cutouts` (ein Schnitt durch alles, nach der Verrundung der Kontur) | `cutout_<n>_<i>_len` pro Seite (mesh2cad schreibt die Ecken als Punktliste `cutout_<n>`) |
| Ein rechteckiger Block (Aufsatz) entlang X und Y auf der Oberseite | `Pad<n>` (eine mit der Platte verbundene Box) | `pad_<n>_w` (entlang X), `pad_<n>_l` (entlang Y), `pad_<n>_h`, die Mitte `pad_<n>_x`, `pad_<n>_y` |
| Eine rechteckige Sackloch-Tasche entlang X und Y mit flachem Boden in der Ober- oder Unterseite | `Pocket<n>` (eine aus der Platte ausgeschnittene Box) | `pocket_<n>_w`, `pocket_<n>_l`, `pocket_<n>_depth` (von dieser Fläche), die Mitte `pocket_<n>_x`, `pocket_<n>_y` |
| Eine Sackloch-Tasche mit beliebiger anderer Kontur aus geraden Seiten (3 bis 24 Ecken, jede ein echter Knick) mit flachem Boden in der Ober- oder Unterseite | `Pocket<n>Sketch` (die Mund-Kontur auf XY, jede Seite eine `Length`-Bemaßung) und `Pocket<n>` (ein Schnitt von dieser Fläche um `pocket_<n>_depth` nach unten), nach den rechteckigen Taschen nummeriert | `pocket_<n>_depth`, `pocket_<n>_<i>_len` pro Seite |
| Eine Stufe von der Oberseite entlang Z nach unten (eine niedrigere Ebene, deren Bereich eine Kante mit der Kontur teilt) | wie eine Tasche dieser Bereichs-Kontur geschrieben: `Pocket<n>Sketch` und `Pocket<n>`, von oben bis zur Ebene geschnitten | `pocket_<n>_depth` (die Stufenhöhe), `pocket_<n>_<i>_len` pro Seite |
| Die vier vertikalen Ecken einer rechteckigen Platte verrundet, jede mit eigenem Radius | ein `Fillet<g>` pro Radius-Gruppe | `fillet_r` (alle Ecken ein Radius) oder `fillet_r_<g>` pro Gruppe |
| Jede vertikale Ecke einer beliebigen anderen Kontur verrundet, jede mit eigenem Radius, Innen- und Außenecken gleichermaßen | ein `Fillet<g>` pro Radius-Gruppe (vor den Bohrungen, sodass eine Bohrung in der Verrundung einer Innenecke liegen darf) | `outline_fillet_r` (alle Ecken ein Radius) oder `outline_fillet_r_<g>` pro Gruppe |
| Eine rechteckige Box, deren zwölf Kanten alle mit einem Radius verrundet sind, mit Durchgangsbohrungen | `Fillet1` auf allen zwölf Kanten der Extrusion (den vier `side|side`, den vier `side|top`, den vier `side|start`), vor den Bohrungen | `edge_fillet_r` |
| Der Schnitt von Konturen aus geraden Seiten, die das Teil entlang zweier oder dreier von X, Y und Z rechtwinklig durchziehen (eine von oben gesehene Platte, geschnitten von einer von der Seite gesehenen Rampe; jede Fläche eben und parallel zu mindestens einer Achse) | `Sketch<k>` (jede Kontur auf der Ebene ihrer Achse) und `Extrude<k>` (die erste ein neuer Body, jede weitere `intersect`), `Move1` | `extrude_<a>_len` (die Länge entlang Achse `a`), `extrude_<a>_0` (wo sie entlang dieser beginnt) für die Achse jeder Kontur; die Ecken der Konturen wie in der Datei (mesh2cads Punktlisten `outline_<a>`); Öffnungen aus geraden Seiten quer durch eine Kontur (ein Fenster durch das Teil entlang dieser Achse) werden in ihrer Skizze gezeichnet und sind die Löcher des extrudierten Profils (mesh2cads `outline_<a>_cutout_<i>`); gerade Sackloch-Schnitte und Taschen aus geraden Seiten entlang X, Y oder Z sind `Cut<n>Sketch` und `Cut<n>` (eine Schnitt-Extrusion von ihrem Start), `cut_<n>_len`, `cut_<n>_0` (wo sie in der Datei beginnt), wobei die längste der Achsen gewinnt, in die eine rechteckige Tasche passt; runde Bohrungen entlang X, Y oder Z, durch das Teil oder sacklochartig von einer Fläche — auch zwei, die einander innen kreuzen — sind `Hole<n>` (ein herausgeschnittener Zylinder), `hole_<n>_d`, die zwei Koordinaten der Bohrung quer zu ihrer Achse (`hole_<n>_x`, `hole_<n>_y` entlang Z), `hole_<n>_depth` bei einer Sacklochbohrung |
| Entlang X, Y oder Z gestapelte Konturen, jede über ihren eigenen Bereich extrudiert (eine Basis, ein kleinerer Block auf einem Teil davon, darauf ein weiterer), wenn kein Schnitt von Konturen passt: jede Fläche winklig zur Achse oder entlang ihr, oder eine Fase an allen Oberkanten des obersten Bands (eine Fase, oder ein Dach) | pro Band `Sketch<k>` (ihr Schnitt auf der Ebene der Achse) und `Extrude<k>` von wo das Band darunter endet (das erste ein neuer Body, die anderen verbunden), `Move1`, und die Fase oben als `Chamfer1` | `extrude_<a><k>_len` und `extrude_<a><k>_0` (mesh2cads Namen; ein Band beginnt bei `extrude_<a><k-1>_0 + extrude_<a><k-1>_len`, daher hebt eine neue Höhe eines Bandes die Bänder darüber an); von den Achsen, entlang derer es stapelt, die mit den wenigsten Bändern |

Die [Beispiele](/de/guide/examples) zeigen das an konkreten STL-Dateien. Die Parameternamen sind die von
mesh2cad, und das Teil bleibt an seiner Stelle in der Datei. Eine gefaste Ecke ist einfach eine weitere
gerade Seite, kommt also exakt rüber.

## Prüfungen

Das neu aufgebaute Teil muss dem Netz volumenmäßig innerhalb von 1 % entsprechen, sonst bleibt das
Netz. Der Importbericht vermerkt, um wie viel sich das Volumen geändert hat, weil runde Facetten zu
echten Kreisen wurden.

Davor lehnen Topologie-Gates ein Netz rundweg ab: Jede Kante muss zu zwei Flächen gehören (die
erkannten Flächen schließen zu Schleifen; T-Junctions eines Exporters zählen), und eine Seitenwand
muss ihre Fläche rechtwinklig treffen. Ein Netz, das beides nicht erfüllt, bleibt ein Netz, mit dem
Grund im Bericht.

## Noch nicht erkannt {#not-recognised-yet}

Diese bleiben ein Netz, mit dem Grund im Bericht:

- gekrümmte Seiten, die keine Kreisbögen sind
- verrundete Ecken gemischt mit Bogen-Seiten in einer Kontur
- nur einige Ecken verrundet
- D-förmige und andere gekrümmte Ausschnitte, Sackloch- oder schräge Langlöcher
- Ecken, die fast ihre ganze Seite entlang verrundet sind
- Taschen mit gekrümmten Seiten, andere Formen auf der Oberseite und Zapfen mit Formschräge, einer Verrundung oder einer Bohrung (und alles außer einem geraden runden Zapfen an einer Seitenwand)
- Sacklochbohrungen mit Bohrerspitze oder Senkung
- Teile, die aus den Achsen gedreht sind
- kombinierte Extrusionen mit runden Bohrungen, die in einer schrägen Fläche enden (sie bleiben Öffnungen ihrer Facetten), gesenkte oder gefaste Bohrungen, Schnitte, die einander berühren oder Inseln haben; Stapel mit einer Fase nur auf einigen Oberkanten, einer schrägen Fläche unterhalb der Oberseite, einer nicht konvexen gefasten Oberseite oder Bohrungen

Die [Roadmap](/de/guide/roadmap) listet die Reihenfolge, in der sie kommen.
