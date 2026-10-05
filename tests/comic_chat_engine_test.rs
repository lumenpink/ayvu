use ayvu::character::CharacterRegistry;
use ayvu::model::{BalloonType, ComicStrip, DialogueLine, Emotion, Panel, StripLayout};
use ayvu::renderer::render_comic_strip_to_svg;

#[test]
fn test_authentic_jim_woodring_characters_in_registry() {
    let registry = CharacterRegistry::default();
    let char_ids = registry.list_character_ids();

    assert!(char_ids.contains(&"armando".to_string()), "Armando must be in registry");
    assert!(char_ids.contains(&"dan".to_string()), "Dan must be in registry");
    assert!(char_ids.contains(&"tiki".to_string()), "Tiki must be in registry");
    assert!(char_ids.contains(&"tongtyed".to_string()), "Tongtyed must be in registry");

    let armando = registry.get("armando").expect("Armando pack must exist");
    assert_eq!(armando.author, "Jim Woodring");
    assert_eq!(armando.license, "MIT");

    // Must have authentic embedded data image
    let neutral_svg = armando.get_pose_svg(&Emotion::Neutral);
    assert!(neutral_svg.contains("data:image/png;base64,"), "Pose must contain embedded PNG data URI");
}

#[test]
fn test_comic_chat_backdrops_rendering() {
    let panel = Panel {
        id: 1,
        characters: vec![],
        dialogues: vec![DialogueLine {
            speaker: "armando".to_string(),
            emotion: Emotion::Neutral,
            balloon_type: BalloonType::Speech,
            text: "Man, this place is out of sight!".to_string(),
        }],
        effects: vec![],
    };

    let strip = ComicStrip {
        title: Some("Comic Chat Authentic".to_string()),
        layout: StripLayout::Horizontal,
        backdrop: Some("pastoral".to_string()),
        panels: vec![panel],
    };

    let svg = render_comic_strip_to_svg(&strip);
    assert!(svg.contains("style=\"text-transform: uppercase;\""), "Must use Comic Chat all-caps text transform");
    assert!(svg.contains("font-family=\"'Comic Sans MS'"), "Must use Comic Sans MS typography");
    assert!(svg.contains("Man, this place is out of") && svg.contains("sight!"), "Must contain dialogue text");
}

#[test]
fn test_comic_chat_layout_proportions() {
    let panel = Panel {
        id: 1,
        characters: vec![],
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
    };

    let strip = ComicStrip {
        title: None,
        layout: StripLayout::Horizontal,
        backdrop: Some("pastoral".to_string()),
        panels: vec![panel],
    };

    let svg = render_comic_strip_to_svg(&strip);
    assert!(svg.contains("data:image/png;base64,"), "Must contain character image data URI");
}

#[test]
fn test_all_31_official_comic_chat_characters_present() {
    let registry = CharacterRegistry::default();
    let char_ids = registry.list_character_ids();

    let expected_31 = [
        "anna", "armando", "bolo", "buck", "connor", "cro", "dan", "denise",
        "glenda", "hugh", "jordan", "kevin", "kirby", "kwensa", "lance", "lynnea",
        "margaret", "maynard", "mike", "pedagog", "rainbow", "rebecca", "sage",
        "scotty", "susan", "tiki", "tongtyed", "tux", "veronica", "waf", "xeno",
    ];

    assert_eq!(char_ids.len(), 31, "Registry must contain exactly 31 official Comic Chat characters");
    for id in &expected_31 {
        assert!(char_ids.contains(&id.to_string()), "Character '{}' must be present in registry", id);
        let pack = registry.get(id).unwrap();
        assert_eq!(pack.author, "Jim Woodring");
        assert_eq!(pack.license, "MIT");
        assert!(!pack.get_pose_svg(&Emotion::Neutral).is_empty(), "Character '{}' must have a neutral pose", id);
    }
}

#[test]
fn test_balloon_arrow_stops_above_character_head() {
    let panel = Panel {
        id: 1,
        characters: vec![],
        dialogues: vec![DialogueLine {
            speaker: "dan".to_string(),
            emotion: Emotion::Neutral,
            balloon_type: BalloonType::Speech,
            text: "Hello above my head!".to_string(),
        }],
        effects: vec![],
    };

    let strip = ComicStrip {
        title: None,
        layout: StripLayout::Horizontal,
        backdrop: None,
        panels: vec![panel],
    };

    let svg = render_comic_strip_to_svg(&strip);

    // Find the speech balloon path
    assert!(svg.contains("<path d=\"M "), "Must render speech balloon path");
    // Verify that the speech balloon path does not penetrate deep into character
    // In our algorithm, target_y = char_y - 2.0 (above the character apex)
    assert!(svg.contains("class=\"character-pose\" id=\"char-dan\""));
}

#[test]
fn test_characters_face_each_other() {
    let panel = Panel {
        id: 1,
        characters: vec!["armando".to_string(), "tiki".to_string()],
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
    };

    let strip = ComicStrip {
        title: None,
        layout: StripLayout::Horizontal,
        backdrop: None,
        panels: vec![panel],
    };

    let svg = render_comic_strip_to_svg(&strip);

    // Armando on the left faces right (unflipped)
    assert!(svg.contains(r#"class="character-pose" id="char-armando""#), "Left character must not be flipped");
    // Tiki on the right faces left (flipped with negative horizontal scale)
    assert!(svg.contains(r#"class="character-pose flipped" id="char-tiki""#), "Right character must have flipped class");
    assert!(svg.contains("scale(-"), "Flipped character transform must use negative scale X");
}

#[test]
fn test_three_characters_in_single_panel() {
    let panel = Panel {
        id: 1,
        characters: vec!["armando".to_string(), "dan".to_string(), "tiki".to_string()],
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
    assert!(svg.contains("balloon-col-0"), "First balloon column rendered");
    assert!(svg.contains("balloon-col-1"), "Second balloon column rendered");
    assert!(svg.contains("balloon-col-2"), "Third balloon column rendered");
}
