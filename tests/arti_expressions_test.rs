use ayvu::character::CharacterRegistry;
use ayvu::parser::parse_dialogue_script;
use ayvu::renderer::svg::render_comic_strip_to_svg;

#[test]
fn test_arti_has_eight_distinct_expressive_poses() {
    let registry = CharacterRegistry::default();
    let arti = registry
        .get("arti")
        .expect("Arti deve estar registrado no catálogo");

    let required_poses = [
        "neutral", "happy", "laughing", "shouting", "thinking", "angry", "sad", "scared",
    ];

    for pose_name in &required_poses {
        assert!(
            arti.poses.contains_key(*pose_name),
            "Arti deve conter a pose '{}'",
            pose_name
        );
        let svg = &arti.poses[*pose_name];
        assert!(
            !svg.is_empty(),
            "Pose '{}' do Arti não pode ser vazia",
            pose_name
        );

        // Deve conter marcação expressiva específica da pose em nanquim
        let expected_class = format!("arti-{}", pose_name);
        assert!(
            svg.contains(&expected_class),
            "Pose '{}' do Arti deve conter a classe específica de pose '{}'",
            pose_name,
            expected_class
        );
    }
}

#[test]
fn test_render_dialogue_with_arti_expressions() {
    let script = r#"
[backdrop: reitoria]
Arti (happy): Olá! Que ótimo dia na UFSC.
---
Arti (shouting): Ei, cuidado com a capivara!
---
Arti (laughing): Hahaha, ela só queria um café!
"#;

    let strip = parse_dialogue_script(script, Some("Dia na Reitoria")).expect("parse");
    let svg = render_comic_strip_to_svg(&strip);

    assert!(
        svg.contains("arti-happy"),
        "Tirinha deve renderizar Arti com pose happy"
    );
    assert!(
        svg.contains("arti-shouting"),
        "Tirinha deve renderizar Arti com pose shouting"
    );
    assert!(
        svg.contains("arti-laughing"),
        "Tirinha deve renderizar Arti com pose laughing"
    );
}
