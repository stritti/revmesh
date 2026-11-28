# Vergleichbare Werkzeuge

revmesh macht aus einem STL-Dreiecksnetz eine parametrische, bearbeitbare Konstruktion in SolidCraft.
Diese Seite listet vergleichbare Lösungen — Werkzeuge, die ein Dreiecksnetz (STL, OBJ, PLY) oder eine
Punktwolke in ein bearbeitbares CAD-Modell verwandeln — mit Link, Lizenz und dem, worin jedes Produkt
besonders ist. Unter den Übersichtstabellen vergleicht eine [Feature-Matrix](#feature-matrix) die
Fähigkeiten im Detail.

Lizenzen sind so angegeben, wie jedes Projekt sie in seiner eigenen Lizenzdatei nennt; Links und Preise
wurden im November 2026 geprüft und können sich ändern. revmesh ist ein unabhängiges Projekt und mit
keinem der aufgeführten Produkte verbunden.

## Automatische Erkennung, Open Source

| Werkzeug | Lizenz | Kosten | Besonderheiten | Feature-Historie? |
|---|---|---|---|---|
| [revmesh (dieses Projekt)](https://github.com/stritti/revmesh) | MIT | kostenlos | SolidCraft-Plugin, das in der WebAssembly-Sandbox läuft — kein Python, numpy oder OpenCascade; schreibt SolidCrafts Feature-Historie mit mesh2cads Parameternamen; was nicht erkannt wird, bleibt ein Mesh-Body, mit dem Grund im Importbericht | Ja: Platten und Konturen entlang X, Y oder Z extrudiert, Durchgangs-, Sackloch-, Senk- und Kegelsenkbohrungen, Langlöcher, Zapfen, Aufsätze, Taschen, Bohrungsreihen/-gitter/-lochkreise, Verrundungen |
| [mesh2cad](https://github.com/emirmrkaljevic/mesh2cad) | MIT | kostenlos (`pip`) | Python auf numpy, trimesh und OpenCascade; CLI plus MCP-Server; schreibt ein exaktes B-rep-STEP und parametrische CadQuery-, OpenSCAD- oder FreeCAD-Skripte; Fidelitätsbericht (Hausdorff, RMS, Volumenfehler); lehnt Teile ab, die es nicht ausdrücken kann | Ja — die Teilklassen und Parameternamen, die revmesh portiert |
| [FreeCAD Reverse Engineering workbench](https://wiki.freecad.org/Reverse_Engineering_Workbench) | LGPL v2+ (Teil von [FreeCAD](https://www.freecad.org)) | kostenlos | passt B-Spline-Flächen an Netze und Punktwolken an und approximiert sie; Segmentierung, Oberflächen-Rekonstruktion | Teilweise — Flächen, kein automatischer Feature-Baum |
| [Detessellate](https://github.com/DesignWeaver3D/Detessellate) (FreeCAD-Add-on) | LGPL-2.1 | kostenlos | algorithmisch unterstützter, skizzenbasierter Workflow für STL/OBJ/3MF: Netz-Querschnitte zu ausgerichteten, loft-bereiten Splines | geführt, nicht automatisch |
| [MeshRemodel](https://github.com/mwganson/MeshRemodel) (FreeCAD-Add-on) | LGPL-2.1-or-later | kostenlos | Remodelling-Assistent: Netz-Eckpunkte werden zu Punkten, dann Skizzen; Netz-Reparaturwerkzeuge | Nein — unterstützt manuelles Remodelling |
| [CAD Sketcher](https://github.com/hlorus/CAD_Sketcher) (Blender-Add-on) | GPL-3.0 | kostenlos | bedingungsbasierter parametrischer Sketcher (Solvespace-Löser) in Blender; projiziert Netz-Flächen in Skizzen, extrudiert/revolviert dann | keine automatische Erkennung des Netzes selbst |

## Forschungsprototypen (Machine Learning)

| Werkzeug | Lizenz | Kosten | Besonderheiten | Feature-Historie? |
|---|---|---|---|---|
| [CAD-Recode](https://cad-recode.github.io/) ([Paper](https://arxiv.org/abs/2412.14042), [Code](https://github.com/filaPro/cad-recode)) | CC-BY-NC-4.0 (nur nicht-kommerziell) | kostenlos | Punktwolke → CadQuery-Python-Code über ein leichtes Netzwerk plus ein vortrainiertes LLM; Stand der Technik auf den DeepCAD-, Fusion-360- und CC3D-Benchmarks | Ja, als Code — Prototyp, braucht eine GPU |
| [Point2CAD](https://www.obukhov.ai/point2cad.html) ([Code](https://github.com/prs-eth/point2cad)) | Apache-2.0 | kostenlos | Punktwolke → komplettes CAD-Modell samt Freiformflächen (implizite neuronale Repräsentation), Kanten und Ecken; Stand der Technik auf dem ABC-Benchmark | Nein — rekonstruierte Flächen, Kanten und Ecken, keine Konstruktionshistorie; Prototyp, braucht eine GPU, Eingabe Punktwolke |

## Kommerzielle Reverse-Engineering-Suiten

| Werkzeug | Lizenz | Kosten | Besonderheiten | Feature-Historie? |
|---|---|---|---|---|
| [Geomagic Design X](https://hexagon.com/products/geomagic-design-x) (Hexagon) | proprietär | Abo oder unbefristet; Go ab ≈ USD 1.900, Pro ≈ USD 22.000+ | Industriestandard für Scan-to-CAD: automatische Feature-Extraktion plus voller historiebasierter parametrischer Baum; LiveTransfer schiebt den Baum in SOLIDWORKS, NX, Creo, Inventor oder Solid Edge; Accuracy Analyser zeigt die Abweichung zum Scan in Echtzeit | Ja, voller Feature-Baum |
| [ZEISS REVERSE ENGINEERING](https://www.gom.com/en/products/zeiss-quality-suite/zeiss-reverse-engineering) | proprietär | kostenpflichtig | Netz/Punktwolke (STL, PLY, ASCII) → CAD, exportiert als STEP/IGES/SAT; geführter Workflow mit Best-Fit, Extrusion und Lofting; Ein-Klick-Übergabe aus ZEISS INSPECT | geführt, halbautomatisch |
| [PolyWorks\|Modeler](https://www.innovmetric.com/products/polyworks-modeler) (InnovMetric) | proprietär | Enterprise | NURBS-Flächenerzeugung und parametrisches Skizzieren direkt auf Polygonmodellen; CAD-Add-ins für CATIA, NX, Creo, SolidWorks und Inventor; verarbeitet Milliarden-Punktwolken | Teilweise — Skizzen und Flächen werden ins CAD-System übertragen |
| [QUICKSURFACE](https://www.quicksurface.com/) (KVS) | proprietär | Personal/Lite/Pro-Stufen | hybrides parametrisches + Freiform-Modellieren auf dem Parasolid-Kernel; bedingte Primitiv-Extraktion (Ebenen, Zylinder, Kegel, Kugeln); automatische Flächenerzeugung; QSConnect-Add-in für SOLIDWORKS; STEP/IGES/Parasolid-Export | geführt |
| [Artec Studio](https://www.artec3d.com/3d-software/artec-studio) | proprietär | Abo oder lebenslang; Lite ab ≈ USD 480/Jahr, Pro ≈ USD 1.700/Jahr; 30 Tage Pro-Test | Scanner-Suite mit Netz-Bereinigung, Vermessung und Netz-zu-CAD-Abweichung; einfaches Scan-to-CAD (Auto-Surfacing, Booleans; seit v20 auch Fase-, Verrundungs- und Schrägen-Erkennung); Ein-Klick-Export von Netz und angepassten Primitiven zu SOLIDWORKS | Teilweise |
| [EXModel](https://www.shining3d.com/reverse-engineering/exmodel) (Shining 3D) | proprietär | kostenpflichtige Lizenzen ab ≈ USD 225 (Personal, 1 Jahr) bis EXModel Pro | Netz → CAD-Volumenkörper in wenigen Schritten: Primitiv-Extraktion, Auto-Surfacing, bedingtes 2D-Skizzieren (Pro), STEP/IGES-Export; Ein-Klick-Übergabe aus den Scannern EXScan/FreeScan | geführt, in Pro parametrisch |
| [peel 3.CAD](https://store.peel-3d.com/software/peel-cad/) | proprietär | Scanner-+Software-Bundles, ≈ USD 12.000 und mehr | schlüsselfertiges Scan-to-CAD mit peel-3D-Scannern: Feature-Extraktion, fortgeschrittenes Skizzieren, Echtzeit-Abweichung gegen den Scan | geführt |
| [MeshWright](https://www.meshwright.io/) (meshwright.io) | proprietär | Seat USD 99, Pro USD 199 pro Nutzer/Monat, Site nach Vereinbarung; KI-Nutzung separat beim KI-Anbieter abgerechnet | Windows-App mit Fusion-Add-in und CLI: dein eigener KI-Assistent (Claude Code, Codex) baut einen nativen, parametrischen Fusion-Feature-Baum aus einem Scan (STL, OBJ, PLY, 3MF); erkannte Bohrungen, Zapfen und Flächen werden aus dem Netz vermessen; die Verifikation vergleicht das neu aufgebaute Modell mit dem Scan und speichert F3D plus STEP; Jobs bleiben lokal, Lizenzprüfungen gehen online | Ja — nativer Fusion-Feature-Baum, KI-geführt |

> **Namenshinweis:** MeshWright (meshwright.io) ist das Produkt eines anderen Anbieters und nicht mit
> revmesh verwandt. Dieses Repository wurde genau deshalb von *meshwright* in *revmesh* umbenannt
> (siehe [PR #34](https://github.com/stritti/revmesh/pull/34)). meshwright.dev und mehrere GitHub-Projekte
> namens *Meshwright* (Mesh-Reparatur für den 3D-Druck) sind weitere unverwandte Produkte.

## Netz zu B-rep in CAD-Apps (bearbeitbarer Volumenkörper, keine Feature-Historie)

| Werkzeug | Lizenz | Kosten | Besonderheiten | Feature-Historie? |
|---|---|---|---|---|
| [SOLIDWORKS ScanTo3D](https://help.solidworks.com/2025/English/SolidWorks/ScanTo3D/c_Scanto3d_overview.htm) | proprietäres Add-in von SOLIDWORKS | in SOLIDWORKS Professional, Premium und Ultimate enthalten | öffnet STL/OBJ/PLY und Punktwolken; Mesh Prep, Curve und Surface Wizards passen analytische oder nicht-analytische Flächen an; direkt gegen das Netz skizzieren | Nein — ein Volumenkörper/eine Fläche, die du dann von Hand bearbeitest |
| [Autodesk Fusion 360](https://www.autodesk.com/products/fusion-360) | proprietär | Abo, für private Nutzung kostenlos | Convert Mesh mit drei Methoden: faceted (eine B-rep-Fläche pro Dreieck), prismatic (analytische Anpassung, am besten für mechanische Teile) und T-Spline (organische Formen); die direkte Konvertierung ist auf etwa 10.000 Dreiecke begrenzt | Nein |
| [Rhino 3D](https://www.rhino3d.com/) | proprietär | einmalig, ≈ USD 1.000 | MeshToNURB/ToNURBS wandelt jede Polygonfläche in eine NURBS-Fläche — Fläche für Fläche, keine smarte Rekonstruktion; SubD-Workflow, kostenpflichtige Plug-ins wie RhinoResurf erweitern mehr | Nein |

## Rund ums Netz (keine Erkennung, aber Teil des Workflows)

| Werkzeug | Lizenz | Kosten | Besonderheiten | Feature-Historie? |
|---|---|---|---|---|
| [OpenSCAD](https://openscad.org/) | GPL-2.0 | kostenlos | skriptbasierter CSG-Modellierer; `import()` liest eine STL als Referenz ins Skript (mesh2cad kann `.scad` exportieren) | Nein — das Modell ist das Skript |
| [CadQuery](https://github.com/CadQuery/cadquery) | Apache-2.0 | kostenlos (`pip`) | parametrisches Python-CAD auf dem OpenCascade-Kernel (OCP); mesh2cad kann CadQuery-Skripte mit passenden Parametern exportieren | Nein — du skriptest das Modell |
| [build123d](https://build123d.readthedocs.io/) | Apache-2.0 | kostenlos (`pip`) | modernes Python-B-rep-Framework über OpenCascade; der Builder-Modus behält eine Konstruktionshistorie mit benannten Maßen | Nein — du skriptest das Modell |
| [Blender](https://www.blender.org/) | GPL-3.0 | kostenlos | Netz-Bearbeitung, Reparatur und Sculpting; kein B-rep und keine Feature-Historie | Nein |
| [MeshLab](https://www.meshlab.net/) | GPL-3.0 | kostenlos | Netz-Bereinigung, Reparatur, Dezimierung und Formatkonvertierung (STL, OBJ, PLY, …); PyMeshLab für Stapelverarbeitung | Nein |

## Feature-Matrix

Legende: ✓ unterstützt · ~ teilweise, geführt oder kostenpflichtige Stufe · – nicht unterstützt. Die
ML-Prototypen (CAD-Recode, Point2CAD) fehlen hier: sie nehmen Punktwolken, nicht Netzdateien, und
brauchen eine GPU.

### Erkannte Geometrie (automatische Werkzeuge)

| Feature | revmesh | mesh2cad | MeshWright (meshwright.io) | FreeCAD-RE-Workbench | Detessellate | MeshRemodel | CAD Sketcher |
|---|---|---|---|---|---|---|---|
| Eingabenetze | STL, OBJ, PLY (≤ 16 MiB in auto, 32 MiB nur Netz) | STL, OBJ, PLY | STL, OBJ, PLY, 3MF | Netz + Punktwolke | STL, OBJ, 3MF | Netz (STL, OBJ, …) | Netz als Skizzen-Referenz |
| Platte/Kontur entlang Z extrudiert | ✓ | ✓ | ~ KI-gebaute Features | – | ~ Querschnitte | ~ manuell | ~ projizierte Skizze |
| Extrusion entlang X oder Y | ✓ | ✓ (probiert X, Y, Z) | ~ | – | – | – | ~ |
| Durchgangsbohrungen | ✓ | ✓ | ✓ aus dem Netz angepasst | – | – | – | – |
| Sacklochbohrungen, Senkung, Kegelsenkung | ✓ alle drei | ✓ Senkung + Kegelsenkung | ~ | – | – | – | – |
| Langlöcher | ✓ | ~ | – | – | – | – | – |
| Zapfen, Aufsätze, Taschen, Ausschnitte | ✓ | ✓ Zapfen, ~ Rest | ✓ Zapfen erkannt | – | – | – | – |
| Bohrungsmuster (Reihe, Gitter, Lochkreis) | ✓ als SolidCraft-Muster | ✓ hole_row / hole_grid | – | – | – | – | ✓ Array-Werkzeuge |
| Verrundungen mit einem Radius | ✓ vertikale Ecken, beliebige Kontur | ✓ vertikale Ecken + alle Kanten (`edge_fillet_r`) | ~ Verrundungsradien als Parameter | – | – | – | ✓ bedingungsgetrieben |
| Fasen, Formschrägen | – (Roadmap) | ~ | ~ | – | – | – | ✓ |
| Freiform-/NURBS-Flächen aus dem Netz | – Netz-Fallback | – faceted/hybrider Fallback | ~ Referenzanpassung, nicht der Feature-Baum | ✓ B-Spline-Anpassung | ~ Loft-Splines | – | – |

### Workflow und Plattform (automatische Werkzeuge)

| Feature | revmesh | mesh2cad | MeshWright (meshwright.io) | FreeCAD-RE-Workbench | Detessellate | MeshRemodel | CAD Sketcher |
|---|---|---|---|---|---|---|---|
| Parametrische Feature-Historie heraus | ✓ SolidCraft-Konstruktion + benannte Parameter | ✓ CadQuery-/OpenSCAD-/FreeCAD-Skripte + exaktes STEP | ✓ nativer Fusion-Feature-Baum | – | ~ Skizzen | ~ manuell | ✓ Blender-Bedingungen |
| Fidelitätsbericht gegen das Netz | ✓ Importbericht, Neuaufbau-Prüfung | ✓ Hausdorff, RMS, Volumen; lehnt Fehler ab | ✓ Verifikation gegen den Scan, F3D + STEP gespeichert | ~ Abweichung | – | – | – |
| Netz-Fallback, wenn nichts gefunden | ✓ Mesh-Body + Grund | ✓ faceted/hybrider Volumenkörper | – manuelle Korrekturen | ✓ (Flächenanpassung) | ✓ | ✓ | n/a |
| Automatisierung | vollautomatisch, deterministisch | vollautomatisch CLI + MCP-Server | KI-Assistent-geführt (Claude Code, Codex) | halbautomatisch | geführt | manuelle Hilfe | manuell |
| Umgebung | WebAssembly-Plugin in SolidCraft | Desktop-CLI / Python | Windows + Fusion-Add-in / CLI | FreeCAD-Desktop | FreeCAD-Add-on | FreeCAD-Add-on | Blender-Add-on |
| Funktioniert offline | ✓ | ✓ | – Lizenzprüfung online; die KI-App überträgt evtl. Daten | ✓ | ✓ | ✓ | ✓ |
| Lizenz, Kosten | MIT, kostenlos | MIT, kostenlos | proprietär, USD 99–199/Nutzer/Monat + KI-Nutzung | LGPL, kostenlos | LGPL, kostenlos | LGPL, kostenlos | GPL-3.0, kostenlos |

### Kommerzielle Suiten

| Feature | Geomagic Design X | ZEISS REVERSE ENGINEERING | PolyWorks Modeler | QUICKSURFACE | Artec Studio | EXModel | peel 3.CAD | MeshWright |
|---|---|---|---|---|---|---|---|---|
| Primitiv-Anpassung (Ebene, Zylinder, Kegel, Kugel) | ✓ automatische Extraktion | ✓ | ✓ bedingtes Best-Fit | ✓ | ✓ | ✓ | ✓ | ✓ aus dem Netz vermessen |
| Bedingtes 2D-Skizzieren | ✓ Auto-Sketch | ✓ | ✓ parametrische Skizzen | ✓ | – | ✓ (Pro) | ✓ | ~ über den KI-Assistenten |
| NURBS-/Freiform-Flächenerzeugung | ✓ Exact Surfacing | ✓ | ✓ | ✓ automatische Flächenerzeugung | ✓ Auto-Surfacing | ✓ | – | ~ Referenzanpassung |
| Feature-Baum ins CAD übertragen | ✓ LiveTransfer (SOLIDWORKS, NX, Creo, Inventor, Solid Edge) | – exportiert STEP/IGES/SAT | ~ parametrische Skizzen über Add-ins (CATIA, NX, Creo, SOLIDWORKS, Inventor); Extrudieren/Loften in der CAD-App | ✓ QSConnect für SOLIDWORKS | ~ Netze + angepasste Primitive zu SOLIDWORKS — Geometrie, kein Feature-Baum | ~ parametrisches STEP | ~ | ✓ nativer Fusion-Baum |
| Echtzeit-Abweichung zum Scan | ✓ Accuracy Analyser | ✓ Best-Fit | ✓ | ✓ Abweichungsanalyse | ✓ Compare Map | ✓ | ✓ | ✓ Verifikationsprotokoll |
| KI-Assistent / MCP-Integration | – | – | – | – | – | – | – | ✓ Claude Code, Codex |
| Scanner-Anbieter-Integration | ✓ Plug-ins (Faro, Hexagon, Aicon) | ✓ Ein-Klick aus ZEISS INSPECT | – | – | ✓ Artec-Scanner | ✓ Ein-Klick aus EXScan | ✓ peel-Scanner | – |
| Stapelverarbeitung / Automatisierung | ✓ Pro-Stapelverarbeitung | ~ | ✓ Makros | – | ✓ Workflows | – | – | ✓ Site-Plan |
| Funktioniert offline | ✓ unbefristet/offline-Aktivierung | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | – Lizenzprüfung online |
| Preissegment | ≈ USD 1.900–22.000+ | Enterprise | Enterprise | Lite bis Pro | ≈ USD 1.700/Jahr Pro | ab ≈ USD 495 | ≈ USD 12.000+ Bundle | USD 99–199/Nutzer/Monat |

### Netz zu B-rep in CAD-Apps

| Feature | SOLIDWORKS ScanTo3D | Autodesk Fusion 360 | Rhino 3D |
|---|---|---|---|
| Konvertierungsmethoden | Surface Wizard (analytische + nicht-analytische Anpassung), Curve Wizard, Mesh Prep | Convert Mesh: faceted, prismatic, T-Spline | MeshToNURB / ToNURBS (eine NURBS-Fläche pro Polygon), SubD |
| Ergebnis | Volumenkörper oder Flächenkörper, dann manuelle Features | B-rep-Volumenkörper/-Fläche; Timeline-Eintrag im parametrischen Modus | Polysurface; bei hohen Flächenzahlen unbrauchbar |
| Flächenzahl-Limit für direkte Konvertierung | Netz-Reparatur zuerst, dann Wizard | ≈ 10.000 Dreiecke | jenseits von Low-Poly-Netzen unpraktikabel |
| Organische Formen | ~ | ✓ T-Spline-Weg über den Form-Arbeitsbereich | ~ Plug-ins wie RhinoResurf |
| Feature-Historie aus dem Netz | – | – | – |

## Forschung und Literatur {#research-and-literature}

Paper zum Rekonstruieren von CAD aus Netzen/Punktwolken und zu dessen Auswertung — gruppiert danach,
wie nah sie an revmeshs Ansatz sind. Die neuronalen Verfahren brauchen eine GPU und Python, daher können
sie nicht in SolidCrafts WebAssembly-Sandbox laufen; die klassischen (RANSAC-Formerkennung,
STL-Feature-Erkennung, geschlossenes Extrusions-Anpassen) sind die direkt portierbaren Bausteine, und
die Datensätze am Ende sind die üblichen Testkorpora (zu STL tesselliert üben sie einen Erkenner so aus,
wie revmesh getestet wird).

### Überblicks- und Review-Artikel

| Paper | Venue | Links | Relevanz |
|---|---|---|---|
| A Survey on Deep Learning in 3D CAD Reconstruction (Lin et al.) | Appl. Sci. 15(12):6681, 2025 | [DOI](https://doi.org/10.3390/app15126681) | Einstieg in DL-basierte CAD-Rekonstruktion aus Scans, Netzen, Punktwolken |
| Integrating Reverse Engineering for Digital Model Reconstruction and Remanufacturing of Mechanical Components: A Systematic Review | Metrology 5(4):66, 2025 | [DOI](https://doi.org/10.3390/metrology5040066) | Systematischer Review des Reverse Engineering mechanischer Teile aus 3D-Scans, 2005–2025 — die industrielle Seite dieser Seite |
| Machine learning in feature recognition for manufacturing: taxonomy, analytical review, comparisons, trends, challenges, and outlook | Int. J. Manuf. Res., 2026 | [DOI](https://doi.org/10.1080/00207543.2026.2675460) | Taxonomie der (ML-basierten) Feature-Erkennung aus CAD-Modellen — Bohrungen, Langlöcher, Taschen |

### Netz / Punktwolke zu parametrischem CAD

| Paper | Venue | Links | Relevanz |
|---|---|---|---|
| Point2Cyl: Reverse Engineering 3D Objects from Point Clouds to Extrusion Cylinders (Uy et al.) | CVPR 2022 | [arXiv](https://arxiv.org/abs/2112.09329), [Projekt](https://point2cyl.github.io/), [Code](https://github.com/mikacuy/point2cyl) | zerlegt eine Punktwolke in Extrusionszylinder (2D-Skizze + Achse + Ausmaß) mit geschlossenem differenzierbarem Anpassen — das nächstliegende lernbasierte Analogon zur Prismen-Erkennung von revmesh |
| ComplexGen: CAD Reconstruction by B-Rep Chain Complex Generation (Guo et al.) | SIGGRAPH 2022, ACM TOG 41(4) | [arXiv](https://arxiv.org/abs/2205.14573), [Code](https://github.com/guohaoxiang/ComplexGen) | rekonstruiert ein volles B-rep mit expliziter Topologie; seine globalen Konsistenzbedingungen (geschlossene Flächenschleifen, Zylinder-Senkrechtigkeit) sind Ideen zum Validieren erkannter Teile |
| Point2CAD (Liu et al.) | CVPR 2024 | [arXiv](https://arxiv.org/abs/2312.04962), [Code](https://github.com/prs-eth/point2cad) | in den Tabellen oben aufgeführt: Punktwolke → CAD mit Freiformflächen, Kanten, Ecken |
| CAD-Recode (Rukhovich et al.) | ICCV 2025 | [arXiv](https://arxiv.org/abs/2412.14042), [Code](https://github.com/filaPro/cad-recode) | in den Tabellen oben aufgeführt: Punktwolke → CadQuery-Python-Code |

### Primitiv- und Kantenerkennung (Bausteine für Erkenner)

| Paper | Venue | Links | Relevanz |
|---|---|---|---|
| Efficient RANSAC for Point-Cloud Shape Detection (Schnabel, Wahl, Klein) | CGF 26(2):214–226, 2007 | [DOI](https://doi.org/10.1111/j.1467-8659.2007.01016.x) | erkennt Ebenen, Kugeln, Zylinder, Kegel, Tori in ungeordneten Punkten — die klassische Basis von revmeshs fixed-seed-deterministischem RANSAC (siehe [Architektur](/de/architecture)) |
| SPFN: Supervised Fitting of Geometric Primitives to 3D Point Clouds (Li et al.) | CVPR 2019 | [arXiv](https://arxiv.org/abs/1811.08988), [Code](https://github.com/lingxiaoli94/SPFN) | gelerntes Primitiv-Anpassen, gegen Efficient RANSAC auf mechanischen Teilen benchmarked |
| CPFN: Cascaded Primitive Fitting Networks for High-Resolution Point Clouds | 2021 | [arXiv](https://arxiv.org/abs/2109.00113) | SPFN kaskadiert für 100k+-Punkt-Scans — kleine Features (Bohrungen) bei hoher Auflösung |
| ParSeNet: A Parametric Surface Fitting Network for 3D Point Clouds (Sharma et al.) | CVPR 2021 | [arXiv](https://arxiv.org/abs/2003.12145) | passt parametrische Patches samt B-Splines an — die Freiform-Seite der Erkennung |
| HPNet: Deep Primitive Segmentation Using Hybrid Representations (Yan et al.) | ICCV 2021 | [arXiv](https://arxiv.org/abs/2105.10620) | segmentiert Primitive mit explizit scharfen Kanten — relevant für revmeshs Wand-/Flächenklassifikation |
| PIE-Net: Parametric Inference of Point Cloud Edges (Wang et al.) | NeurIPS 2020 | [arXiv](https://arxiv.org/abs/2007.04883) | erschließt Feature-Kanten als parametrische Kurven (Linien, Kreise, B-Splines) — das Analogon zum Lesen von Bohrungsrändern und Konturen aus einem Netz |

### Feature-Erkennung aus STL-Netzen (klassisch, am nächsten an revmeshs Eingabe)

| Paper | Venue | Links | Relevanz |
|---|---|---|---|
| Automatic recognition of features from freeform surface CAD models (Sunil, Pande) | Computer-Aided Design 40(4):502–517, 2008 | [Record](https://www.semanticscholar.org/paper/Automatic-recognition-of-features-from-freeform-CAD-Sunil-Pande/81a304bfe4043aaa87b1e936942fefe2fe6dcddc) | segmentiert STL-Netze nach Diederwinkel und Krümmung, erkennt dann Bohrungen, Langlöcher, Stege, Spitzen — direkt auf revmeshs Eingabeformat |
| A learning-based approach to feature recognition of Engineering shapes (Muraleedharan, Muthuganapathy) | 2021 | [arXiv](https://arxiv.org/abs/2112.07962) | klassifiziert Bohrungen/Langlöcher in CAD-Netzen (Random Forest auf einer diskreten Gauss-Abbildung); neuronale, aber die Netz-Deskriptoren sind portierbar |

### Datensätze und Benchmarks

| Datensatz | Paper | Links | Relevanz |
|---|---|---|---|
| ABC-Datensatz (~1 Mio. CAD-Modelle, parametrischer Ground Truth) | Koch et al., CVPR 2019 | [arXiv](https://arxiv.org/abs/1812.06216), [Projekt](https://deep-geometry.github.io/abc-dataset/) | Benchmark von ParSeNet, HPNet, Point2CAD; zu STL tesselliert ein großes Testkorpus für Erkenner |
| DeepCAD (178k Onshape-Modelle + Konstruktionssequenzen) | Wu et al., ICCV 2021 | [arXiv](https://arxiv.org/abs/2105.09492), [Code](https://github.com/rundiwu/DeepCAD) | CAD-Skizze-Extrudieren-Sequenzen als Ground Truth für Rekonstruktion; von CAD-Recode und Point2Cyl genutzt |
| Fusion 360 Gallery (8.625 menschliche Skizze-Extrudieren-Sequenzen) | Willis et al., SIGGRAPH 2021, ACM TOG 40(4):54 | [arXiv](https://arxiv.org/abs/2010.02392), [DOI](https://doi.org/10.1145/3450626.3459818), [Daten](https://github.com/AutodeskAILab/Fusion360GalleryDataset) | definiert „CAD-Rekonstruktion" — ein CAD-Programm aus Zielgeometrie zurückgewinnen — dieselbe Aufgabe, die revmesh für STL-Eingabe löst |

## Wo revmesh passt

Die Open-Source-Erkenner oben laufen alle auf einem Desktop mit Python und OpenCascade. revmesh ist der
einzige, der in SolidCrafts WebAssembly-Plugin-Sandbox läuft und SolidCrafts eigene Feature-Historie
direkt schreibt, wobei das Netz — mit dem Grund im Importbericht — überall dort bleibt, wo keine
Features gefunden werden. MeshWright (meshwright.io) ist die KI-geführte Alternative: ein Assistent
baut einen parametrischen Feature-Baum in Fusion, geführt von Maßen aus dem Netz, braucht aber Windows,
Fusion, ein KI-Konto und eine online Lizenzprüfung. Wer bereits in Fusion 360, SOLIDWORKS oder Rhino
arbeitet, sind die Netz-zu-B-rep-Konvertierungen dort der pragmatische Weg zu einem bearbeitbaren
Volumenkörper, ohne Feature-Historie. Für 3D-Scandaten mit organischen Formen bleiben die kommerziellen
Suiten (Geomagic Design X, ZEISS REVERSE ENGINEERING, PolyWorks\|Modeler, …) zu Enterprise-Preisen die
vollständigsten.

## Konzepte aus der Liste, auf revmesh angewendet {#concepts-from-the-list-applied-to-revmesh}

Die Sandbox entscheidet, welche Konzepte adoptierbar sind: keine GPU, kein Python, kein Netzwerk (siehe
[Architektur](/de/architecture)), daher können die neuronalen Anpaser und Rekonstruktoren (SPFN, CPFN,
HPNet, ParSeNet, Point2CAD, CAD-Recode, MeshWrights KI-Assistent) nicht im Plugin laufen. Portierbar
sind ihre *Ausgaberepräsentation* — parametrische Skizzen mit Bedingungen, Extrusionszylinder,
parametrische Kanten — und die klassischen deterministischen Bausteine: fixed-seed-RANSAC-Primitive,
kantenbasierte Netz-Segmentierung, Topologie-Prüfungen. Measure-the-features-first und dann die Historie
bauen (wie MeshWright mit einem KI-Assistenten, mesh2cad mit einem Skript) ist schon revmeshs Ansatz,
minus die KI.

| Konzept | Kommt von | revmesh heute | Konkreter nächster Schritt |
|---|---|---|---|
| Querschnitte werden zu bedingten Skizzen | Geomagic Auto-Sketch, PolyWorks Modeler, QUICKSURFACE, EXModel (Pro), Detessellate, ZEISS | `plate::faces::split_faces` (`crates/revmesh-core/src/plate/faces.rs:33`) nimmt genau zwei Flächenebenen, unten und oben; Stufen und nicht-rechteckige Taschen werden abgelehnt | auf jede Flächenebene verallgemeinern: das Netz auf jeder Ebene schneiden, `boundary_loops` (`plate/geometry.rs`) pro Ebene nehmen, eine Skizze + Extrusion/Schnitt pro Ebene schreiben — beliebige Taschen und Stufen, wo heute nur `box`-Features geschrieben werden (`to_design.rs:343`) |
| Skizze-Extrudieren mit Bedingungen als CAD-Repräsentation | DeepCAD, Fusion 360 Gallery, ABC-Datensatz | Skizzen-Schreiber in `crates/revmesh-core/src/design/sketch.rs`: `rectangle_sketch` (Zeile 239) voll bemaßt, `slot_sketch` (Zeile 364) voll bestimmt, aber `polygon_sketch`/`add_polygons` (Zeilen 269, 308) schreiben freie Punkte mit nur `Horizontal`/`Vertical` — die Kontur (`to_design.rs:166`) und die Ausschnitte (`to_design.rs:265`) tragen keine Bemaßungen | in `polygon_sketch`/`add_polygons` eine `Length`- oder `Angle`-Bedingung pro Seite (oder `DistanceX`/`DistanceY` pro Ecke) ergänzen, jede mit eigenem Modell-Parameter; der Erkenner misst heute schon jede Seite (`plate/outline.rs`), daher lässt sich jede Kontur wie `base_x` neu bemaßen |
| Extrusionszylinder: 2D-Profil + Achse + Ausmaß | Point2Cyl | ein Prisma, die Achse entlang X, Y und Z probiert (`plate/prism.rs`) | in mehrere Extrusionszylinder zerlegen, jeder eigene Skizze + Extrusion (kombinierte Extrusionen); geschlossenes Anpassen hält es deterministisch |
| Parametrische Kanten aus dem Netz lesen | PIE-Net (neuronal); klassisch Kreis-Anpassungen auf Rand-Schleifen | Bohrungen und Zapfen sind Kreise, an ihre Rand-Schleifen angepasst (`fit_circle` in `plate/geometry.rs`, genutzt aus `plate/holes.rs` und `plate/bosses.rs`); Konturen mit Bogen-Seiten und runde Konturen nutzen `SketchCurve::Arc` (`sketch.rs:69`) mit einem `Diameter` pro Bogen-Seite — eine runde Kontur ist zwei Bögen um einen gemeinsamen Mittelpunkt, getrieben von einem `arc_1_d` | D-förmige und Ausschnitte mit verrundeten Ecken sowie konkave Bogen-Seiten als `Arc`s in der Skizze eines Ausschnitts lesen — heute bleiben sie ein Netz ([noch nicht erkannt](/de/guide/recognition#not-recognised-yet)) |
| Segmentierung nach scharfen Kanten / Diederwinkel | Sunil & Pande 2008, Muraleedharan & Muthuganapathy 2021, HPNets explizite Kanten | `faces.rs`/`walls.rs` klassifizieren Flächen und Wände; Facetten-Ecken werden gegen die Wände geprüft, nicht gegen Facetten-Normalen | mehr Feature-Klassen aus derselben Segmentierung: Fasen, Formschrägen, Stege, Sackloch- und schräge Langlöcher |
| Primitiv-Erkennung samt Tori | Efficient RANSAC (Schnabel et al. 2007) | fixed-seed RANSAC für Ebenen und Zylinder; Verrundungen an Bohrungsrändern und Zapfenfüßen werden als Tori angepasst — ein Kreis in der Profilebene um die bekannte Achse (`fit_torus` in `plate/geometry.rs`, genutzt von `rounded_rim` und `round_foot`) | analytische Torus-Erkennung mit Ausreißer-Verwerfung (fixed-seed RANSAC auf dem Profil), und Tori um unbekannte Achsen |
| Topologie als Gültigkeits-Gate | ComplexGen (geschlossene Flächenschleifen, Zylinder-Senkrechtigkeit) | exact-or-nothing (Toleranz `max(1 µm, 10⁻⁵ · Größe)`), Volumen innerhalb `MAX_VOLUME_ERROR = 1%` (`plate/mod.rs`), plus ComplexGen-artige Gates: geschlossene Kantenschleifen (`closed_loops`, `recognize/mod.rs`) und Wände, die ihre Fläche rechtwinklig treffen (`check_walls`, `plate/prism.rs`) | weitere ComplexGen-Bedingungen (volle B-Rep-Kanten-/Ecken-Konsistenz), um verdrehte oder bandierte Fälschungen noch früher abzulehnen |
| Abweichung gegen das Netz als Bericht | Geomagic Accuracy Analyser, PolyWorks, mesh2cads Hausdorff/RMS | Volumen innerhalb 1 % beim Import; Hausdorff nur in `cargo xtask check` (`check.rs`) | die Oberflächenabweichung pro Fläche in der Sandbox berechnen und in den Importbericht aufnehmen |

### Hin zu nacharbeitbaren Skizzen, konkret

Eine SolidCraft-Skizze ist Punkte + `Line`/`Arc`-Kurven + Bedingungen, und die Bedingungen `Length`,
`Diameter`, `DistanceX`, `DistanceY` und `Angle` werden von Modell-Parametern getrieben (`Constraint` in
`crates/revmesh-core/src/design/sketch.rs:102`; siehe [Das SolidCraft-Design-Format](/de/design-format)).
Was revmeshs Skizzen-Schreiber heute erzeugen:

| Skizze | Geschrieben von | Bedingungen | Nacharbeitbar? |
|---|---|---|---|
| Platten-Rechteck | `rectangle_sketch` (`sketch.rs:239`, genutzt bei `to_design.rs:70`) | horizontal/vertikal, koinzident, zwei `Length`, getrieben von den Modell-Parametern `d1`/`d2` (`to_design.rs:67`), deren Ausdrücke `base_x`/`base_y` sind | ja |
| Allgemeine Kontur | `polygon_sketch` (`sketch.rs:269`, genutzt bei `to_design.rs:166`) | freie Ecken, `Horizontal`/`Vertical` nur auf achsparallelen Seiten | nein — die Form steckt in den Punkten |
| Ausschnitte | `polygons_sketch` (`sketch.rs:294`, genutzt bei `to_design.rs:265`) | wie die Kontur | nein |
| Langloch | `slot_sketch` (`sketch.rs:364`, genutzt bei `to_design.rs:244`) | tangent ×4, gleich, Mittelpunkt, `Length`, `Diameter`, koinzidente Position, `Angle` — alle parametergetrieben | ja |

Die Lücke zur Vision nacharbeitbarer Skizzen sind genau die Kontur- und Ausschnitt-Skizzen. Die
Schritte sind [Punkt 5 der Roadmap](/de/guide/roadmap): jede Kontur-Seite mit eigenem Modell-Parameter in
`polygon_sketch`/`add_polygons` bemaßen (keine neue Bedingungsart nötig — `Length`, `Angle`,
`DistanceX`/`DistanceY` existieren alle im `Constraint`-Enum und SolidCraft liest sie, und der Erkenner
misst heute schon jede Seite in `plate/outline.rs`), verrundete Eckenläufe als
`SketchCurve::Arc` mit `Diameter`-Parameter schreiben (`outline_shape` passt heute schon einen Kreis pro
Ecke an, `CircleSums` bei `plate/outline.rs:165`), `split_faces` auf eine Skizze + Extrusion oder Schnitt
pro Flächenebene verallgemeinern und die Abweichung pro Fläche in den Importbericht aufnehmen — alles
innerhalb der deterministischen Gates (exact-or-nothing, Volumen innerhalb 1 %, ComplexGen-artige
Topologie-Prüfungen; die neuronalen Rekonstruktoren bleiben draußen: ihre Repräsentation ist das Ziel,
ihr Rechenaufwand nicht).
