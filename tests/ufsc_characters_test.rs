use ayvu::character::CharacterRegistry;
use ayvu::model::Emotion;
use ayvu::parser::parse_dialogue_script;
use ayvu::renderer::svg::render_comic_strip_to_svg;

#[test]
fn test_ufsc_characters_available_in_registry() {
    let registry = CharacterRegistry::default();
    assert!(
        registry.get("capivara").is_some(),
        "Capivara deve estar disponivel no CharacterRegistry"
    );
    assert!(
        registry.get("jacare").is_some(),
        "Jacare deve estar disponivel no CharacterRegistry"
    );
}

#[test]
fn test_ufsc_characters_have_eight_poses() {
    let registry = CharacterRegistry::default();
    let emotions = [
        Emotion::Neutral,
        Emotion::Happy,
        Emotion::Laughing,
        Emotion::Shouting,
        Emotion::Thinking,
        Emotion::Angry,
        Emotion::Sad,
        Emotion::Scared,
    ];

    for id in &["capivara", "jacare"] {
        let pack = registry
            .get(id)
            .unwrap_or_else(|| panic!("Personagem {} nao encontrado", id));

        assert!(pack.balloon_anchor.x > 0.0);
        assert!(pack.balloon_anchor.y > 0.0);
        assert!(pack.hand_anchor.x > 0.0);
        assert!(pack.hand_anchor.y > 0.0);

        for emotion in &emotions {
            let svg = pack.get_pose_svg(emotion);
            assert!(
                !svg.is_empty(),
                "Pose {:?} de {} nao pode ser vazia",
                emotion,
                id
            );
            assert!(
                svg.contains("<svg") || svg.contains("<path") || svg.contains("<g"),
                "Pose {:?} de {} deve ser markup SVG valido",
                emotion,
                id
            );
        }
    }
}

#[test]
fn test_render_ufsc_characters_dialogue_and_prop() {
    let script = r#"
[backdrop: pantanal]
Capivara (thinking): A tranquilidade do campus antes da aula...
Jacaré (happy): E a água do córrego está perfeita!
[prop: coffee@Capivara]
"#;
    let strip = parse_dialogue_script(script, Some("Campus Pantanal")).expect("parse");
    let svg = render_comic_strip_to_svg(&strip);

    assert!(svg.contains("<svg"));
    assert!(svg.contains("Capivara"));
    assert!(svg.contains("Jacaré") || svg.contains("Jacare"));
    assert!(svg.contains("tranquilidade"));
    assert!(svg.contains("prop-cafe"));
    assert!(svg.contains("prop-anchored"));
}
