use ayvu::character::CharacterRegistry;
use ayvu::parser::parse_dialogue_script;
use ayvu::renderer::svg::render_comic_strip_to_svg;

#[test]
fn test_jacare_has_authentic_ink_and_no_colored_paint_fills() {
    let registry = CharacterRegistry::default();
    let jacare = registry
        .get("jacare")
        .expect("Jacaré deve estar registrado no catálogo");

    let required_poses = [
        "neutral", "happy", "laughing", "shouting", "thinking", "angry", "sad", "scared",
    ];

    for pose_name in &required_poses {
        assert!(
            jacare.poses.contains_key(*pose_name),
            "Jacaré deve conter a pose '{}'",
            pose_name
        );
        let svg = &jacare.poses[*pose_name];

        // NÃO pode conter os preenchimentos verdes e amarelos do Paint antigo
        assert!(
            !svg.contains("#243823")
                && !svg.contains("#2d472c")
                && !svg.contains("#c9a84a")
                && !svg.contains("#f0c808"),
            "Pose '{}' do Jacaré ainda contém cores chapadas do Paint",
            pose_name
        );

        // Deve conter nanquim preto e branco autêntico do Comic Chat
        assert!(
            svg.contains("<image href=\"data:image/png;base64,")
                || (svg.contains("stroke=\"#111111\"") && !svg.contains("fill=\"#")),
            "Pose '{}' deve seguir o padrão autêntico de nanquim do Comic Chat",
            pose_name
        );
    }
}

#[test]
fn test_jacare_manifest_and_anchors() {
    let registry = CharacterRegistry::default();
    let jacare = registry
        .get("jacare")
        .expect("Jacaré deve estar registrado");

    assert_eq!(jacare.id, "jacare");
    assert_eq!(jacare.name, "Jacaré");

    let b_anchor = jacare.balloon_anchor;
    assert!(
        (b_anchor.x - 0.50).abs() < 0.15,
        "balloon_anchor.x deve estar no topo da cabeça (~0.50), obtido {}",
        b_anchor.x
    );

    let h_anchor = jacare.hand_anchor;
    assert!(
        (h_anchor.x - 0.68).abs() < 0.15,
        "hand_anchor.x deve apontar para a pata/garra (~0.68), obtido {}",
        h_anchor.x
    );
}

#[test]
fn test_render_dialogue_with_jacare() {
    let script = r#"
[backdrop: pantanal]
Jacaré (neutral): ...
"#;
    let strip = parse_dialogue_script(script, Some("No Pantanal da UFSC")).expect("parse");
    let svg = render_comic_strip_to_svg(&strip);

    assert!(
        svg.contains("data-speaker=\"Jacaré\"")
            || svg.contains("data-speaker=\"Jacare\"")
            || svg.contains("id=\"char-jacare\""),
        "SVG deve renderizar o jacaré"
    );
}
