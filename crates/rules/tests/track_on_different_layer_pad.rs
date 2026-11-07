use pcbcraft_doc::{Footprint, LibraryFootprint, Net, Pad, Project, Track, component_template};
use pcbcraft_geom::Point;
use pcbcraft_library::{FootprintDefinition, FootprintPad};
use pcbcraft_rules::drc;

fn track_on_different_layer_pad_board() -> Result<Project, String> {
    let mut p = Project::default();
    p.nets.push(Net {
        id: 1,
        name: "SIG".into(),
        ..Default::default()
    });
    // Component takes id 2 (net 1 is taken).
    let mut component = component_template("resistor", 2, "U1".into(), Point::new(20.0, 20.0))
        .ok_or("component template")?;
    component.pins.truncate(1);
    component.pins[0].net = Some(1);
    // Front-only SMD pad.
    let geometry = FootprintPad {
        number: "1".into(),
        shape: "circle".into(),
        pad_type: "smd".into(),
        position: Point::default(),
        rotation: 0.0,
        size: Point::new(1.2, 1.2),
        layers: vec!["F.Cu".into(), "F.Mask".into()],
        drill: None,
        roundrect_ratio: None,
        chamfer: None,
        polygon: vec![],
        raw_source: String::new(),
    };
    component.library_footprint = Some(LibraryFootprint {
        library: "test".into(),
        definition: FootprintDefinition {
            name: "pad".into(),
            pads: vec![geometry.clone()],
            ..Default::default()
        },
    });
    p.components.push(component);
    p.board.footprints.push(Footprint {
        id: 10,
        component: 2,
        position: Point::new(20.0, 20.0),
        rotation: 0.0,
        locked: None,
        side: None,
        pads: vec![Pad {
            number: "1".into(),
            offset: Point::default(),
            net: Some(1),
            diameter: 1.2,
            castellated: None,
            geometry: Some(geometry),
        }],
        preserved: Default::default(),
    });
    // Bottom-layer track ending at the front-only pad location.
    p.board.tracks.push(Track {
        id: 12,
        net: 1,
        start: Point::new(15.0, 20.0),
        end: Point::new(20.0, 20.0),
        width: 0.2,
        layer: pcbcraft_doc::Layer::Back,
        arc: None,
        preserved: Default::default(),
    });
    p.validate()?;
    Ok(p)
}

#[test]
fn track_ending_on_different_layer_pad_is_flagged() -> Result<(), String> {
    let board = track_on_different_layer_pad_board()?;
    let findings = drc(&board);
    assert!(
        findings
            .iter()
            .any(|f| f.code == "track_on_different_layer_pad"),
        "track on B.Cu ending at F.Cu-only pad is flagged"
    );
    Ok(())
}
