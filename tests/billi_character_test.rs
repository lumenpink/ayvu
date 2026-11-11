use ayvu::character::CharacterRegistry;
use ayvu::parser::parse_dialogue_script;
use ayvu::renderer::svg::render_comic_strip_to_svg;

#[test]
fn test_billi_has_eight_authentic_ink_poses_without_placeholder_badge() {
    let registry = CharacterRegistry::default();
    let billi = registry.get("billi").expect("Billi deve estar registrada");

    let required_poses = [
        "neutral", "happy", "laughing", "shouting", "thinking", "angry", "sad", "scared",
    ];

    for pose in &required_poses {
        assert!(
            billi.poses.contains_key(*pose),
            "Billi deve conter a pose '{}'",
            pose
        );
        let svg = &billi.poses[*pose];

        // NÃO deve conter a estampa provisória de silhueta com letra "B"
        assert!(
            !svg.contains(r#">B</text>"#),
            "Pose '{}' da Billi não deve conter estampa provisória com letra 'B'",
            pose
        );

        // Deve conter elementos da ilustração em nanquim (piercings, tatuagem, alfaiataria)
        assert!(
            svg.contains("piercing")
                || svg.contains("butterfly")
                || svg.contains("mariposa")
                || svg.contains("blazer"),
            "Pose '{}' deve conter elementos anatômicos em nanquim (piercings/tatuagem/blazer)",
            pose
        );
    }
}

#[test]
fn test_billi_manifest_has_accurate_anatomical_anchors() {
    let registry = CharacterRegistry::default();
    let billi = registry.get("billi").expect("Billi deve estar registrada");

    let balloon = billi.balloon_anchor;
    assert!(
        (balloon.x - 0.5).abs() < 0.05,
        "balloon_anchor.x próximo de 0.5"
    );
    assert!(
        (balloon.y - 0.18).abs() < 0.05,
        "balloon_anchor.y próximo de 0.18"
    );

    let hand = billi.hand_anchor;
    assert!(
        (hand.x - 0.65).abs() < 0.05,
        "hand_anchor.x próximo de 0.65"
    );
    assert!(
        (hand.y - 0.48).abs() < 0.05,
        "hand_anchor.y próximo de 0.48"
    );
}

#[test]
fn test_render_dialogue_with_billi_and_prop() {
    let script = r#"
[backdrop: cafe_ccs]
[prop: cafe@Billi]
Billi (happy): O café do CCS hoje está excelente!
Arti (thinking): Concordo, o expresso duplo ajuda no raciocínio.
"#;
    let strip = parse_dialogue_script(script, Some("Encontro no Café")).expect("parse");
    let svg = render_comic_strip_to_svg(&strip);

    assert!(
        svg.contains("billi-blazer") || svg.contains("data-speaker=\"billi\""),
        "SVG da tirinha deve renderizar a personagem Billi"
    );
    assert!(
        svg.contains("prop-anchored"),
        "SVG deve ancorar o prop de café na mão da Billi"
    );
}
