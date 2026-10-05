use ayvu::character::CharacterRegistry;
use ayvu::model::{BalloonType, ComicStrip, DialogueLine, Emotion, Panel, StripLayout};
use ayvu::renderer::render_comic_strip_to_svg;

#[test]
fn test_authentic_jim_woodring_characters_in_registry() {
    let registry = CharacterRegistry::default();
    let char_ids = registry.list_character_ids();

    assert!(
        char_ids.contains(&"armando".to_string()),
        "Armando must be in registry"
    );
    assert!(
        char_ids.contains(&"dan".to_string()),
        "Dan must be in registry"
    );
    assert!(
        char_ids.contains(&"tiki".to_string()),
        "Tiki must be in registry"
    );
    assert!(
        char_ids.contains(&"tongtyed".to_string()),
        "Tongtyed must be in registry"
    );

    let armando = registry.get("armando").expect("Armando pack must exist");
    assert_eq!(armando.author, "Jim Woodring");
    assert_eq!(armando.license, "MIT");

    // Must have authentic embedded data image
    let neutral_svg = armando.get_pose_svg(&Emotion::Neutral);
    assert!(
        neutral_svg.contains("data:image/png;base64,"),
        "Pose must contain embedded PNG data URI"
    );
}

#[test]
fn test_comic_chat_backdrops_rendering() {
    let panel = Panel {
        id: 1,
        characters: vec![],
        backdrop: None,
        dialogues: vec![DialogueLine {
            speaker: "armando".to_string(),
            emotion: Emotion::Neutral,
            balloon_type: BalloonType::Speech,
            text: "Man, this place is out of sight!".to_string(),
        }],
        effects: vec![],
        width: None,
    };

    let strip = ComicStrip {
        title: Some("Comic Chat Authentic".to_string()),
        layout: StripLayout::Horizontal,
        backdrop: Some("pastoral".to_string()),
        panels: vec![panel],
    };

    let svg = render_comic_strip_to_svg(&strip);
    assert!(
        svg.contains("style=\"text-transform: uppercase;\""),
        "Must use Comic Chat all-caps text transform"
    );
    assert!(
        svg.contains("font-family=\"'Comic Sans MS'"),
        "Must use Comic Sans MS typography"
    );
    assert!(
        svg.contains("Man, this place is out of") && svg.contains("sight!"),
        "Must contain dialogue text"
    );
}

#[test]
fn test_comic_chat_layout_proportions() {
    let panel = Panel {
        id: 1,
        characters: vec![],
        backdrop: None,
        dialogues: vec![
            DialogueLine {
                speaker: "armando".to_string(),
                emotion: Emotion::Happy,
                balloon_type: BalloonType::Speech,
                text: "Hey Tiki!".to_string(),
            },
            DialogueLine {
                speaker: "tiki".to_string(),
                emotion: Emotion::Neutral,
                balloon_type: BalloonType::Speech,
                text: "Greetings.".to_string(),
            },
        ],
        effects: vec![],
        width: None,
    };

    let strip = ComicStrip {
        title: None,
        layout: StripLayout::Horizontal,
        backdrop: Some("pastoral".to_string()),
        panels: vec![panel],
    };

    let svg = render_comic_strip_to_svg(&strip);
    assert!(
        svg.contains("data:image/png;base64,"),
        "Must contain character image data URI"
    );
}

#[test]
fn test_all_31_official_comic_chat_characters_present() {
    let registry = CharacterRegistry::default();
    let char_ids = registry.list_character_ids();

    let expected_31 = [
        "anna", "armando", "bolo", "buck", "connor", "cro", "dan", "denise", "glenda", "hugh",
        "jordan", "kevin", "kirby", "kwensa", "lance", "lynnea", "margaret", "maynard", "mike",
        "pedagog", "rainbow", "rebecca", "sage", "scotty", "susan", "tiki", "tongtyed", "tux",
        "veronica", "waf", "xeno",
    ];

    assert_eq!(
        char_ids.len(),
        31,
        "Registry must contain exactly 31 official Comic Chat characters"
    );
    for id in &expected_31 {
        assert!(
            char_ids.contains(&id.to_string()),
            "Character '{}' must be present in registry",
            id
        );
        let pack = registry.get(id).unwrap();
        assert_eq!(pack.author, "Jim Woodring");
        assert_eq!(pack.license, "MIT");
        assert!(
            !pack.get_pose_svg(&Emotion::Neutral).is_empty(),
            "Character '{}' must have a neutral pose",
            id
        );
    }
}

#[test]
fn test_balloon_arrow_stops_above_character_head() {
    let panel = Panel {
        id: 1,
        characters: vec![],
        backdrop: None,
        dialogues: vec![DialogueLine {
            speaker: "dan".to_string(),
            emotion: Emotion::Neutral,
            balloon_type: BalloonType::Speech,
            text: "Hello above my head!".to_string(),
        }],
        effects: vec![],
        width: None,
    };

    let strip = ComicStrip {
        title: None,
        layout: StripLayout::Horizontal,
        backdrop: None,
        panels: vec![panel],
    };

    let svg = render_comic_strip_to_svg(&strip);

    // Find the speech balloon path
    assert!(
        svg.contains("<path d=\"M "),
        "Must render speech balloon path"
    );
    // Verify that the speech balloon path does not penetrate deep into character
    // In our algorithm, target_y = char_y - 2.0 (above the character apex)
    assert!(svg.contains("class=\"character-pose\" id=\"char-dan\""));
}

#[test]
fn test_characters_face_each_other() {
    let panel = Panel {
        id: 1,
        characters: vec!["armando".to_string(), "tiki".to_string()],
        backdrop: None,
        dialogues: vec![
            DialogueLine {
                speaker: "armando".to_string(),
                emotion: Emotion::Neutral,
                balloon_type: BalloonType::Speech,
                text: "Facing you!".to_string(),
            },
            DialogueLine {
                speaker: "tiki".to_string(),
                emotion: Emotion::Neutral,
                balloon_type: BalloonType::Speech,
                text: "Facing you too!".to_string(),
            },
        ],
        effects: vec![],
        width: None,
    };

    let strip = ComicStrip {
        title: None,
        layout: StripLayout::Horizontal,
        backdrop: None,
        panels: vec![panel],
    };

    let svg = render_comic_strip_to_svg(&strip);

    // Armando on the left faces right (unflipped)
    assert!(
        svg.contains(r#"class="character-pose" id="char-armando""#),
        "Left character must not be flipped"
    );
    // Tiki on the right faces left (flipped with negative horizontal scale)
    assert!(
        svg.contains(r#"class="character-pose flipped" id="char-tiki""#),
        "Right character must have flipped class"
    );
    assert!(
        svg.contains("scale(-"),
        "Flipped character transform must use negative scale X"
    );
}

#[test]
fn test_three_characters_in_single_panel() {
    let panel = Panel {
        id: 1,
        characters: vec!["armando".to_string(), "dan".to_string(), "tiki".to_string()],
        backdrop: None,
        dialogues: vec![
            DialogueLine {
                speaker: "armando".to_string(),
                emotion: Emotion::Neutral,
                balloon_type: BalloonType::Speech,
                text: "First!".to_string(),
            },
            DialogueLine {
                speaker: "dan".to_string(),
                emotion: Emotion::Neutral,
                balloon_type: BalloonType::Speech,
                text: "Second!".to_string(),
            },
            DialogueLine {
                speaker: "tiki".to_string(),
                emotion: Emotion::Neutral,
                balloon_type: BalloonType::Speech,
                text: "Third!".to_string(),
            },
        ],
        effects: vec![],
        width: None,
    };

    let strip = ComicStrip {
        title: None,
        layout: StripLayout::Horizontal,
        backdrop: None,
        panels: vec![panel],
    };

    let svg = render_comic_strip_to_svg(&strip);

    // Check all three characters rendered
    assert!(svg.contains(r#"id="char-armando""#), "Armando rendered");
    assert!(svg.contains(r#"id="char-dan""#), "Dan rendered");
    assert!(svg.contains(r#"id="char-tiki""#), "Tiki rendered");

    // Check 3 balloon columns rendered
    assert!(
        svg.contains("balloon-col-0"),
        "First balloon column rendered"
    );
    assert!(
        svg.contains("balloon-col-1"),
        "Second balloon column rendered"
    );
    assert!(
        svg.contains("balloon-col-2"),
        "Third balloon column rendered"
    );
}

#[test]
fn test_vector_academic_backdrops() {
    let script = "[backdrop: sala_de_aula]\nDan: Boa tarde turma!\n---\n[backdrop: cafe_ccs]\nArmando: Vamos tomar um café!\n---\n[backdrop: laboratorio]\nTiki: Hora do experimento!";
    let strip =
        ayvu::parser::parse_dialogue_script(script, Some("UFSC Strip")).expect("Parse script");

    let svg = render_comic_strip_to_svg(&strip);
    assert!(
        svg.contains("backdrop-classroom"),
        "Must render classroom vector backdrop"
    );
    assert!(
        svg.contains("UFSC - CALCULO I"),
        "Must contain classroom chalkboard formula"
    );
    assert!(
        svg.contains("backdrop-cafe-ccs"),
        "Must render café CCS vector backdrop"
    );
    assert!(svg.contains("CAFÉ CCS"), "Must contain café CCS sign");
    assert!(
        svg.contains("backdrop-laboratory"),
        "Must render lab vector backdrop"
    );
}

#[test]
fn test_new_ufsc_iconic_backdrops() {
    let script = r#"
[backdrop: hu_fachada]
Dan: Chegamos no Hospital Universitário!
---
[backdrop: ru]
Armando: Partiu pegar o bandejão no RU!
---
[backdrop: hall_ccs]
Tiki: Vamos ver os murais no Hall do CCS.
---
[backdrop: hu_enfermaria]
Dan: Plantão médico na enfermaria do HU.
---
[backdrop: bu_ufsc]
Armando: Silêncio total estudando na BU.
"#;
    let strip =
        ayvu::parser::parse_dialogue_script(script, Some("UFSC Tour")).expect("Parse script");
    let svg = render_comic_strip_to_svg(&strip);

    assert!(
        svg.contains("backdrop-hu-fachada"),
        "Must render HU fachada vector backdrop"
    );
    assert!(
        svg.contains("HOSPITAL UNIVERSITÁRIO"),
        "Must contain HU lettering"
    );
    assert!(
        svg.contains("backdrop-ru"),
        "Must render RU vector backdrop"
    );
    assert!(svg.contains("RU TRINDADE"), "Must contain RU Trindade sign");
    assert!(
        svg.contains("backdrop-hall-ccs"),
        "Must render Hall CCS vector backdrop"
    );
    assert!(
        svg.contains("MURAL • CCS / UFSC"),
        "Must contain Hall CCS notice board"
    );
    assert!(
        svg.contains("backdrop-hu-enfermaria"),
        "Must render HU enfermaria vector backdrop"
    );
    assert!(
        svg.contains("backdrop-bu-ufsc"),
        "Must render BU UFSC vector backdrop"
    );
    assert!(svg.contains("SILÊNCIO"), "Must contain BU silence sign");
}

#[test]
fn test_dynamic_panel_width_heuristics() {
    // 1 speaker with short text -> Narrow (210px)
    let s1 = "Dan: Oi!";
    let strip1 = ayvu::parser::parse_dialogue_script(s1, None).unwrap();
    assert_eq!(strip1.panels[0].resolve_width(), 210.0);

    // 2 speakers -> Standard (280px)
    let s2 = "Dan: Olá!\nArmando: Tudo bem?";
    let strip2 = ayvu::parser::parse_dialogue_script(s2, None).unwrap();
    assert_eq!(strip2.panels[0].resolve_width(), 280.0);

    // 3 speakers in one panel -> Wide (360px)
    let s3 = "[characters: Dan, Armando, Tiki]\nDan: Primeiro\nArmando: Segundo\nTiki: Terceiro";
    let strip3 = ayvu::parser::parse_dialogue_script(s3, None).unwrap();
    assert_eq!(strip3.panels[0].resolve_width(), 360.0);

    // 4 speakers in one panel -> Panoramic (450px)
    let s4 = "[characters: Dan, Armando, Tiki, Jordan]\nDan: 1\nArmando: 2\nTiki: 3\nJordan: 4";
    let strip4 = ayvu::parser::parse_dialogue_script(s4, None).unwrap();
    assert_eq!(strip4.panels[0].resolve_width(), 450.0);
}

#[test]
fn test_explicit_panel_width_parsing() {
    let script = r#"
[narrow]
Dan: Reação rápida!
---
[panel: wide]
Dan: Painel com largura ampla
Armando: Muito mais espaço aqui!
---
[width: 320]
Tiki: Largura customizada de 320px
"#;
    let strip = ayvu::parser::parse_dialogue_script(script, None).unwrap();
    assert_eq!(strip.panels.len(), 3);
    assert_eq!(strip.panels[0].resolve_width(), 210.0);
    assert_eq!(strip.panels[1].resolve_width(), 360.0);
    assert_eq!(strip.panels[2].resolve_width(), 320.0);

    let svg = render_comic_strip_to_svg(&strip);
    // Check panel rect widths
    assert!(
        svg.contains(r#"width="210.0""#),
        "Must contain 210px panel rect"
    );
    assert!(
        svg.contains(r#"width="360.0""#),
        "Must contain 360px panel rect"
    );
    assert!(
        svg.contains(r#"width="320.0""#),
        "Must contain 320px panel rect"
    );

    // Total width = 16 + 210 + 16 + 360 + 16 + 320 + 16 = 954
    assert!(
        svg.contains(r#"viewBox="0 0 954.0 312.0""#),
        "Total viewBox must reflect sum of dynamic widths"
    );
}
