use ayvu::character::CharacterRegistry;
use ayvu::parser::parse_dialogue_script;
use ayvu::renderer::svg::render_comic_strip_to_svg;

#[test]
fn test_capivara_has_authentic_ink_and_no_colored_paint_fills() {
    let registry = CharacterRegistry::default();
    let capivara = registry
        .get("capivara")
        .expect("Capivara deve estar registrada no catálogo");

    let required_poses = [
        "neutral", "happy", "laughing", "shouting", "thinking", "angry", "sad", "scared",
    ];

    for pose_name in &required_poses {
        assert!(
            capivara.poses.contains_key(*pose_name),
            "Capivara deve conter a pose '{}'",
            pose_name
        );
        let svg = &capivara.poses[*pose_name];

        // NÃO pode conter os preenchimentos marrons simplificados do Paint
        assert!(
            !svg.contains("#2b231d") && !svg.contains("#3a2f27") && !svg.contains("#4a3c32"),
            "Pose '{}' da Capivara ainda contém cores chapadas marrons do Paint",
            pose_name
        );

        // Deve conter nanquim preto e branco autêntico (Comic Chat)
        assert!(
            svg.contains("<image href=\"data:image/png;base64,")
                || (svg.contains("stroke=\"#111111\"") && !svg.contains("fill=\"#")),
            "Pose '{}' deve seguir o padrão autêntico de nanquim do Comic Chat",
            pose_name
        );
    }
}

#[test]
fn test_capivara_manifest_and_anchors() {
    let registry = CharacterRegistry::default();
    let capivara = registry
        .get("capivara")
        .expect("Capivara deve estar registrada");

    assert_eq!(capivara.id, "capivara");
    assert_eq!(capivara.name, "Capivara");

    let b_anchor = capivara.balloon_anchor;
    assert!(
        (b_anchor.x - 0.50).abs() < 0.10,
        "balloon_anchor.x deve estar centrado (~0.50), obtido {}",
        b_anchor.x
    );
    assert!(
        (b_anchor.y - 0.15).abs() < 0.10,
        "balloon_anchor.y deve estar no topo da cabeça (~0.15), obtido {}",
        b_anchor.y
    );

    let h_anchor = capivara.hand_anchor;
    assert!(
        (h_anchor.x - 0.70).abs() < 0.15,
        "hand_anchor.x deve apontar para a pata (~0.70), obtido {}",
        h_anchor.x
    );
}

#[test]
fn test_render_dialogue_with_capivara() {
    let script = r#"
[backdrop: bosque]
Capivara (neutral): ...
"#;
    let strip = parse_dialogue_script(script, Some("Tarde no Bosque")).expect("parse");
    let svg = render_comic_strip_to_svg(&strip);

    assert!(
        svg.contains("data-speaker=\"Capivara\"") || svg.contains("id=\"char-capivara\""),
        "SVG deve renderizar a capivara"
    );
}
