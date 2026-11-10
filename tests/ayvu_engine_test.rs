use ayvu::assets::get_character;
use ayvu::model::Emotion;
use ayvu::parser::parse_dialogue_script;
use ayvu::renderer::render_comic_strip_to_svg;

#[test]
fn test_native_roster_composition() {
    let characters = ayvu::assets::list_available_characters();

    // Must contain exactly the 6 native characters
    let expected = vec!["aly", "arti", "billi", "lu", "raio", "sombra"];
    for id in &expected {
        assert!(
            characters.contains(id),
            "Native roster must contain character '{}'",
            id
        );
    }
    assert_eq!(
        characters.len(),
        6,
        "Roster must have exactly 6 characters, got: {:?}",
        characters
    );

    // Must NOT contain legacy Comic Chat characters
    let legacy = ["armando", "tiki", "dan", "anna", "bolo", "sage"];
    for id in &legacy {
        assert!(
            !characters.contains(id),
            "Roster must not contain legacy character '{}'",
            id
        );
    }
}

#[test]
fn test_all_native_characters_have_eight_poses() {
    let ids = ["arti", "sombra", "raio", "billi", "aly", "lu"];
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

    for id in &ids {
        let pack = get_character(id).unwrap_or_else(|| panic!("Character '{}' not found", id));
        for emotion in &emotions {
            let pose = pack.get_pose_svg(emotion);
            assert!(
                !pose.is_empty(),
                "Character '{}' must have pose for emotion {:?}",
                id,
                emotion
            );
            assert!(
                pose.contains("<svg") || pose.contains("<image") || pose.contains("<g") || pose.contains("<path"),
                "Character '{}' pose for {:?} must be valid SVG markup",
                id,
                emotion
            );
        }
    }
}

#[test]
fn test_render_dialogue_with_native_characters() {
    let script = r#"
[backdrop: sala_de_aula]
Arti (thinking): As teorias sobre o motor foram comprovadas.
Billi (happy): Sim! Agora o traço em nanquim é totalmente nativo!
Aly (laughing): E sem resquícios de 1996!
Lu (shouting): Avante com a nova arquitetura!
"#;
    let strip = parse_dialogue_script(script, Some("Ayvu Native")).expect("parse");
    let svg = render_comic_strip_to_svg(&strip);

    assert!(svg.contains("<svg"));
    assert!(svg.contains("Arti"));
    assert!(svg.contains("Billi"));
    assert!(svg.contains("As teorias sobre o motor"));
    assert!(svg.contains("totalmente nativo"));
}
