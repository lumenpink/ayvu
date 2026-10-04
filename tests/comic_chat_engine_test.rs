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
