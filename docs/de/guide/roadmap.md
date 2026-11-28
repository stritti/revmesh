# Roadmap

Erledigte Punkte tragen das Release, in dem sie erschienen sind (`vX.Y.Z`); Arbeit, die nach dem
neuesten Release (v0.3.0) gemerged wurde, ist als **unveröffentlicht** markiert und bekommt ihren Tag,
sobald release-please das nächste Release (v0.4.0) schneidet. Die Tags liest man aus den
`chore: release`-Commits und der `CHANGELOG.md` — nie erfinden.

## Offene Punkte, priorisiert

1. **Rest von Punkt 5.2** — D-förmige Ausschnitte, Ausschnitte mit verrundeten Ecken und
   konkave Bogen-Seiten importieren noch als Netz (runde Konturen sind erledigt,
   **unveröffentlicht**). Als Nächstes in der Reihenfolge der Roadmap selbst (nacharbeitbare
   Skizzen sind das, was revmesh jenseits der mesh2cad-Parität ausmacht) und baut direkt auf den
   soeben hinzugekommenen Bogen-Seiten von 5.2 auf.
2. **Konvertieren eines Mesh-Bodies, der bereits in der offenen Konstruktion liegt**
   ([Issue #70](https://github.com/stritti/revmesh/issues/70)): der Spike
   ([research/convert-mesh-body.md](https://github.com/stritti/revmesh/blob/main/research/convert-mesh-body.md))
   hat gezeigt, dass das ein zweites `command-plugin`-Paket braucht — ein eigener Feature-Track
   nach der Skizzen-Arbeit.

## Punkte (stabile Nummerierung — referenziert als `roadmap X.Y`)

1. **Gerüst** (erledigt in v0.1.0-beta.1): Plugin-Pipeline, Dokument-Schreiber, Netz-Fallback, Qualitäts-Gates,
   Releases.
2. **Platten** (erledigt in v0.1.0-beta.1): beliebige Kontur aus geraden Seiten mit runden Durchgangsbohrungen und
   verrundeten vertikalen Ecken.
3. **Parität mit mesh2cad**, ein Erkenner pro Pull Request, jeder mit mesh2cads Ausgabe verglichen
   (`scripts/compare-mesh2cad.sh`, siehe [Entwicklung](/de/development)):
   1. (erledigt in v0.1.0-beta.2) Parameternamen, wie mesh2cad sie schreibt (`base_t`, `fillet_r`, `extrude_len`, …),
      und das Teil bleibt an seiner Stelle in der Datei
   2. (erledigt in v0.1.0-beta.2) Bohrungsreihen, -gitter und -lochkreise als SolidCraft-Muster
   3. (erledigt in v0.1.0-beta.2) Bohrungen mit Senkung und mit Kegelsenkung
   4. (erledigt in v0.1.0-beta.2) Sacklochbohrungen
   5. (erledigt in v0.1.0-beta.2) Langlöcher
   6. (erledigt in v0.1.0-beta.2) Zapfen auf der Oberseite
   7. (erledigt in v0.1.0-beta.2) polygonale Ausschnitte
   8. (erledigt in v0.1.0-beta.2) Aufsätze und rechteckige Taschen (Stufen und andere Taschen später)
   9. (erledigt in v0.1.0-beta.4) Extrusionen entlang X und Y
   10. (erledigt in v0.1.0-beta.4) Boxen, deren Kanten alle verrundet sind (`edge_fillet_r`), mit Durchgangsbohrungen
       darin, und (erledigt in v0.3.0: ein runder Zapfen an einer Seitenwand wird abgezogen, indem die Wand
       nach oben gedreht wird (`plate/bosses.rs::strip_side_bosses`), und als `cylinder` geschrieben,
       der an der Wand-`base_*`-Ausdruck mit der Wand-Außennormale als Achse verbunden wird)
       Zapfen an Seitenflächen
   11. (erledigt in v0.1.0-beta.4–v0.1.0-beta.5) Fasen, Formschrägen und die Verrundungen an einem Zapfenfuß oder einem Bohrungsrand
   12. kombinierte Extrusionen: (erledigt in v0.1.0-beta.5) der Schnitt von Konturen, die entlang X, Y und Z extrudiert
       werden, mit Öffnungen durch sie, geraden Sackloch-Schnitten hinein und runden Bohrungen, und
       (erledigt in v0.1.0-beta.5) Vereinigungen von Konturen, die entlang einer Achse gestapelt sind;
       (erledigt in v0.3.0) sich kreuzende runde Bohrungen in einem kombinierten Teil — die Bohrungen werden per
       Kreis-Konsens aus ihren ecken-teilenden Gruppen gelöst (`combined/holes.rs::crossed_bore`),
       eine Bohrung mit Lücke in ihrer Wand wird akzeptiert, wenn die Lücke im Zylinder einer
       anderen Bohrung liegt, und das Volumen rechnet die Kreuzung einmal — und (erledigt in v0.3.0)
       Dächer und Fasen auf einem Stapel: die Oberkanten des obersten Bands dürfen alle eine Fase tragen
       (`w` auf der Oberseite hinein, `h` an den Wänden hinab — eine Fase, wenn klein, ein Dach,
       wenn die Fasen sich treffen), erkannt in `stack/mod.rs::recognize_chamfer` an einer konvexen
       Oberkante und als ein `Chamfer1` mit `chamfer` (45°) oder `chamfer_w`/`chamfer_h` geschrieben
4. **Prüfung** (erledigt in v0.1.0-beta.4 als `cargo xtask check`, und zur Importzeit in v0.2.0 über die
   In-Sandbox-Selbstprüfung von Punkt 7.1): das neu aufgebaute Teil mit dem Netz durch SolidCrafts `evaluate` vergleichen (Volumen
   und Abweichung), wie mesh2cad es mit CadQuery tut.
5. **Nacharbeitbare Skizzen** — jenseits der Parität: jede Skizze vollständig bemaßt und bestimmt, sodass
   sie in SolidCraft nachgearbeitet werden kann. Die Konzepte dahinter und ihre Quellen (Produkte und
   Paper) sind unter [Vergleichbare Werkzeuge → Konzepte aus der Liste, auf revmesh angewendet](/de/guide/comparison#concepts-from-the-list-applied-to-revmesh)
   kartiert:
   1. (erledigt in v0.1.0-beta.6–v0.2.0 für Plattenkonturen und Ausschnitte sowie für die Skizzen von Stapel und Schnitt: eine
      Kontur des Stapel-Bands `k` als `outline_<k>_<i>_len`/`_angle`, eine kombinierte Extrusion entlang
      Achse `a` als `outline_<a>_<i>_len`/`_angle`, ihre Öffnungen als `cutout_<n>_<i>_len`/`_angle` und
      die Kontur eines kombinierten Schnitts als `cut_<n>_<i>_len`/`_angle`) eine
      `Length`-Bedingung pro Seite mit eigenem Parameter (`outline_<i>_len`,
      `cutout_<n>_<i>_len`) und eine `Angle` von der X-Achse der Skizze an der längsten schrägen Seite
      mit eigenem Parameter (`outline_<i>_angle`, `cutout_<n>_<i>_angle`) in
      `polygon_sketch_dimensioned`/`polygons_sketch_dimensioned`
      (`crates/revmesh-core/src/design/sketch.rs`; parallele schräge Seiten teilen ihren Winkel und die
      Knicke eines Rings summieren sich zu 360°, daher ist ein Winkel pro Polygon der unabhängige — und
      die Länge dieser Seite ist dann abgeleitet, beide zu treiben würde die Skizze überbestimmen;
      ebenso, wenn die übrigen Längen und Orientierungen die Richtung dieser Seite bereits festlegen,
      wie die Katheten eines rechtwinkligen Dreiecks, horizontal und vertikal gehalten, es für die
      Diagonale tun, ist auch die Länge der längsten achsparallelen Seite abgeleitet
      (`dimensioned_sides_for_angle`), sodass der Winkel-Parameter bearbeitbar bleibt)
   2. (erledigt in v0.1.0-beta.6–v0.2.0 für Konturen und Ausschnitte) Bogen-Seiten in Konturen und Ausschnitten als
      `SketchCurve::Arc` mit einer `Diameter`-Bedingung, getrieben von `arc_<i>_d` (einer Kontur) bzw.
      `cutout_<n>_arc_<i>_d` (eines Ausschnitts), der Mittelpunkt ein freier Skizzenpunkt, den der
      Löser im Abstand des treibenden Durchmessers von den beiden Ecken der Seite hält (wie die
      Bogen-Mittelpunkte eines Langlochs)
      (`polygon_sketch_arcs`, `crates/revmesh-core/src/design/sketch.rs`; erkannt als
      `ProfileSpec::arcs` in `plate/outline.rs::outline_shape` und als `CutoutSpec::arcs` in
      `plate/holes.rs::cutout_shape`, das einen Ausschnitt-Ring wie eine Kontur liest und ihn nur mit
      Bogen-Seiten und ohne verrundete Ecken annimmt) — runde Konturen (erledigt,
      **unveröffentlicht**: ein Lauf einmal ganz um einen Kreis herum wird als zwei
      gegenüberliegende Ecken mit je einer Halbkreis-Bogen-Seite gelesen, wobei die beiden Bögen
      einen Mittelpunkt teilen und ein `arc_1_d` den `Diameter` treibt) — D-förmige Ausschnitte,
      Ausschnitte mit verrundeten Ecken und konkave Bogen-Seiten bleiben ein Netz
   3. (erledigt in v0.1.0-beta.6–v0.2.0 für Taschen beliebiger geradliniger Kontur, für Stufen von der Oberseite mehrere
      Ebenen tief entlang Z, für gesenkte Bohrungen, die auf der Ebene einer Stufe öffnen — von der
      Ebene gemessen und von ihr gebohrt, eine verrundete Kante auf der Startkante der Tasche verrundet
      — und für Stufen entlang X oder Y, die der Stapel-Erkenner als entlang dieser Achse gestapelte
      Konturen liest) eine Skizze pro Flächenebene: `split_faces` über Unter- und Oberseite hinaus
      verallgemeinern — beliebige Taschen und Stufen, beide als `Pocket<n>Sketch` +
      `Pocket<n>`-Schnitt mit `pocket_<n>_<i>_len` pro Seite geschrieben
      (`recognize/to_design.rs::block_features`; die Ebenen in `plate/prism.rs::step_levels`,
      jede Oberseiten-Schleife in `face_outlines` mit ihrer Ebene versehen, Köpfe von ihrer Mund-Fläche
      aus gemessen in `stepped_hole`), und mit 3.12 kombinierten Extrusionen
   4. (erledigt in v0.1.0-beta.6–v0.2.0 für jede erkannte Art) Oberflächenabweichung pro Fläche im Importbericht, in der Sandbox
      beim Import berechnet: jede erkannte Fläche mit ihrem Label und der weitesten Entfernung ihrer
      Netz-Punkte von der idealen Fläche (`recognize::deviations`: `plate::face_deviations` für Prismen,
      eine rundum verrundete Box meldet ihre verrundete Hülle als "rounded surface",
      `plate::walls_deviations` pro Extrusion und pro Band bei kombinierten und gestapelten Teilen,
      beschriftet `extrude <a> side <i>` / `band <k> side <i>`; `Walls::deviation`; das Volumen
      innerhalb von 1 % bleibt das Gate, Hausdorff in `cargo xtask check`)
6. **Mehr Teile, robust eingepasst** — aus dem Code-Review der Erkenner gegen die Referenzen (das
   extrahierte Know-how ist im Agent-Skill `revmesh-recognition-knowhow` paketiert; die Konzept → Code
   Kartierung mit den Quellen ist [Vergleichbare Werkzeuge → Konzepte aus der Liste, auf revmesh
   angewendet](/de/guide/comparison#concepts-from-the-list-applied-to-revmesh)):
   1. (erledigt in v0.3.0: `prism_axes` in `recognize/axis.rs` passt die dominanten parallelen
      Ebenenpaare per fixed-seed RANSAC an (Schnabel et al. 2007, wie `ransac_circle` für Kreise) —
      eine Facette pro 3-Punkt-Probe, Inlier innerhalb der Toleranz nach Fläche bewertet, die
      gegenüberliegende Ebene als dominante Menge der antiparallelen Facetten, das Teil muss zwischen
      den beiden Ebenen liegen und die Achse abseits jeder Welt-Achse (bis zu drei Kandidaten-Ebenen,
      größte zuerst, damit die dominante Wand einer langen Extrusion nicht ihre Kappen versteckt) —
      und liefert die eigentliche Drehung `Frame`, die diese Achse auf +Z bringt; wenn keine
      Hauptachse passt, läuft `recognize` die Z-Pipeline in diesem Frame und liefert
      `Recognition::Slanted { prism, frame, offset }`.
      Nur auf Erkennungsseite: SolidCraft 0.51.2s Skizzen auf Ursprungsebenen und
      translate-only-Verschiebungen können ein schräges Prisma in der gepinnten Version nicht
      platzieren, daher behält der Import das Netz und meldet, was erkannt wurde (`revmesh.slanted`);
      parametrisches Schreiben braucht Konstruktionsebenen oder einen neueren Pin)
   2. (erledigt in v0.2.0: ein fixed-seed RANSAC-Kreis in `plate/geometry.rs::ransac_circle`, der den algebraischen
      `fit_circle` ergänzt — 3-Punkt-Minimalproben, Inlier innerhalb der Toleranz, die Iterationszahl an
      das Inlier-Verhältnis angepasst — hinter `round_hole` in `plate/holes.rs`: ein Rand-Ring mit
      wenigen Ecken neben dem Kreis wird auf seinen Inliern eingepasst, wenn mindestens
      `MIN_RING_SEGMENTS` (6) von ihnen eine strikte Mehrheit des Rings sind, ihre Schritte gleichmäßig
      bleiben und die ausgeschlossenen Ecken innerhalb einer Facetten-Pfeilhöhe des Kreises liegen,
      während die eigenen Kanten des Rings das ursprüngliche 50°-Gate behalten, sodass ein ungleichmäßiger
      Exporteur-Rand als runde Bohrung gelesen wird, eine echte Fläche oder Lappung aber abgelehnt
      bleibt: in den eigenen Schleifen einer Fläche werden weniger als `MIN_HOLE_SEGMENTS` (8) Ecken
      zuerst als polygonaler Ausschnitt gelesen, daher bleibt ein Sechseck dort ein Ausschnitt, und die
      flache Seite eines D und der lange Schritt einer ausgelassenen Facette sind weiter ein langer
      Schritt) robuste Kreis-Einpassungen auf Rand-Ringen
   3. (erledigt in v0.2.0: ein Torus um die bekannte Achse ist ein Kreis in der (radialen, axialen)
      Profilebene, daher passt `plate/geometry.rs::fit_torus` ihn als einen an — kleinstes-Quadrate-Kreis
      durch die Ecken des Verrundungs-Bands, abgelehnt, wenn sie auf keinem Kreis liegen — in
      `rounded_rim` für Bohrungsränder (`plate/prism.rs`, mit Tangente an die Bohrungswand und an die
      Fläche geprüft) und in `round_foot` für Zapfenfüße (`plate/bosses.rs`), statt des
      Mittelwerts pro Ecke) analytische Torus-Erkennung für die Verrundungen an Bohrungsrändern und
      Zapfenfüßen
   4. (erledigt in v0.3.0: `outline_shape` in `plate/outline.rs` liefert einen angepassten Radius pro verrundeter
      Ecke und das Design schreibt ein `Fillet<g>` pro Radius-Gruppe mit `fillet_r_<g>` /
      `outline_fillet_r_<g>` — ein gemeinsamer Radius behält das schlichte `fillet_r` /
      `outline_fillet_r` und `Fillet1`) pro-Ecken-Radien in Konturen
   5. (erledigt in v0.3.0: `hole_groups` in `crates/revmesh-core/src/recognize/to_design.rs` schreibt eine Reihe,
      deren Winkel nicht 0° oder ±90° ist, als einzelne Bohrungen, da die Richtung eines Musters in
      SolidCraft kein Ausdruck ist; eine achsparallele Reihe bleibt ein `HoleRow<n>`-Muster)
      Bohrungsreihen in beliebigem Winkel (die Richtung eines Musters ist in SolidCraft kein Ausdruck —
      als einzelne Bohrungen oder auf einer gedrehten Skizze schreiben)
   6. (erledigt in v0.3.0: `closed_loops` in `crates/revmesh-core/src/recognize/mod.rs` lehnt, bevor eine Achse
      probiert wird, ein Netz ab, dessen Kanten nicht von zwei Facetten in Gegenrichtung bedeckt
      sind — Ecken auf einem Sub-Toleranz-Raster verschweißt, T-Junctions eines Exporters über
      ihre Subkanten-Längen gezählt — erkannte Flächen schließen also zu Schleifen; und `check_walls`
      in `plate/prism.rs` lehnt eine Wandfacette ab, deren Grundriss nicht innerhalb der Toleranz
      plus der Pfeilhöhe der runden Wände eine Gerade ist, Wände also ihre Fläche rechtwinklig
      treffen) ComplexGen-artige Topologie-Gates neben exact-or-nothing und der Volumen-Prüfung:
      erkannte Flächen schließen zu Schleifen, Wände treffen ihre Fläche rechtwinklig
7. **Import und Konvertierung**:
   1. (erledigt in v0.2.0: das Manifest fordert `evaluate@0.2`; wird eine erkannte Konstruktion
      in der privaten Sandbox der App gestaget und dort neu aufgebaut (`document.inspect`), bevor sie
      geschrieben wird, und eine Konstruktion, die die App nicht öffnen kann, fällt mit dem Grund im
      Verlustbericht auf den Mesh-Body zurück, Code `revmesh.selfcheck`; Mesh-Body-Importe werden nicht
      geprüft — `self_check` in `crates/revmesh-plugin/src/lib.rs`, schließt Punkt 4 zur Importzeit)
      eine Selbstprüfung in der App-Sandbox: die Capability `evaluate@0.2` anfordern, die erzeugte
      Konstruktion stagen und neu aufbauen, und bei Feature-Fehlern mit dem Grund im Bericht auf den
      Mesh-Body zurückfallen
   2. (erledigt in v0.3.0: `plugin.json` akzeptiert `stl`, `obj` und `ply`; `format.rs` erkennt das Format an
      Dateiname oder den ersten Bytes, und `MeshReader` streamt alle drei gepuffert mit denselben
      Größenlimits wie `StlReader`; `obj.rs` und `ply.rs` (ASCII und binary little-endian) parsen
      in dasselbe `Mesh` mit lokalisierten Zeilen-/Datensatz-Fehlern; `import`, `read_mesh` des
      Plugins und `cargo xtask check` nehmen alle drei; die Plugin-Test-Fixtures
      `block-60x40x6.obj`, `block-60x40x6.ply` und `block-60x40x6-bin.ply` importieren den
      STL-Block unverändert, `obj-bad.obj`/`ply-bad.ply` decken die Fehlerfälle ab) OBJ- und
      PLY-Eingabe neben STL
   3. (erledigt in v0.3.0) OBJ, PLY und ASCII-STL akzeptieren endliche Koordinaten mit doppelter
      Genauigkeit. Der Netz-Fallback erhält Koordinaten auch jenseits des einfachgenauen
      Wertebereichs. Für die parametrische Erkennung gilt der bisherige Koordinatenbereich.
   4. (erledigt in v0.3.0: Dateien bis 32 MiB werden akzeptiert (`MAX_BYTES` in
      `crates/revmesh-plugin/src/lib.rs`) und die Erkennung ist bei `MAX_RECOGNITION_TRIANGLES`
      (≈ 335.000 Dreiecke ≈ 16 MiB binäres STL) in `revmesh-core` begrenzt — `import_mesh` nimmt das
      Netz als Referenz und klont nur in den Mesh-/Unrecognized-Fallback-Pfaden, und `Faces.chords`
      ist von (a, b, middle, length) auf (a, b, mid_z, length) verschlankt; gemessene Spitze ≈ 90 MiB
      bei 16 MiB, innerhalb des 128-MiB-Budgets; mehr bräuchte indexbasierte Flächenspeicherung)
      eine höhere Erkennungsgrenze (heute 335k Dreiecke, `revmesh-core`), indem
      die Indexe, die die Erkennung über Flächen und Wände baut, schlanker werden
   5. (erledigt in v0.3.0: ASCII-STL-Syntaxfehler tragen einen `SyntaxReason` (`stl.rs`), gerendert in der
      Importsprache über `stl.syntax.reason.*` in den Lokalisierungskatalogen; die Dialogoption
      `units` (`plugin.json`) liest Zoll-Dateien als Zoll und skaliert sie direkt auf Millimeter
      (`convert_units`, danach bereichsgeprüft), vermerkt im Bericht als `revmesh.units`)
   6. (erledigt in v0.3.0: die Dialogoption `mode` (`auto`/`mesh`) ist entfallen — `plugin.json` fragt nur
      noch nach `units`; der Import versucht immer die Erkennung und fällt auf den Mesh-Body
      zurück, wo nichts erkannt wird (`revmesh.unrecognized`), die Selbstprüfung der App
      fehlschlägt (`revmesh.selfcheck`) oder das Netz `MAX_RECOGNITION_TRIANGLES` überschreitet
      (`revmesh.too_large`), und der Verlustbericht sagt, welcher Fall eingetreten ist) einen
      Import-Dialog ohne Moduswahl

Ein Mesh-Body, der bereits in der offenen Konstruktion liegt, direkt konvertieren (nicht nur eine
STL-Datei) ist in [Issue #70](https://github.com/stritti/revmesh/issues/70) getrackt; die
Ergebnisse des Spikes — zweites Paket mit der `command-plugin`-World, Body über `model.mesh` lesen,
über `timeline.add`/`timeline.delete` zurückschreiben, Auswahl über `x-solidcraft-selection` —
stehen in
[research/convert-mesh-body.md](https://github.com/stritti/revmesh/blob/main/research/convert-mesh-body.md).

Beiträge sind sehr willkommen: Bug-Reports, STL-Dateien, die schlecht importieren, Erkenner, Doku.
Fang an mit [CONTRIBUTING.md](https://github.com/stritti/revmesh/blob/main/CONTRIBUTING.md).
