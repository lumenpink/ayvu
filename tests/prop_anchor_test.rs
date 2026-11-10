use ayvu::character::CharacterRegistry;
use ayvu::model::{Emotion, PropType};
use ayvu::parser::parse_dialogue_script;
use ayvu::renderer::svg::render_comic_strip_to_svg;

#[test]
fn test_character_manifest_and_pack_hand_anchors() {
    let registry = CharacterRegistry::default();
    let arti = registry.get("arti").expect("arti exists");

    // Deve possuir hand_anchor padrao
    let default_anchor = arti.hand_anchor;
    assert!(default_anchor.x > 0.0 && default_anchor.x <= 1.0);
    assert!(default_anchor.y > 0.0 && default_anchor.y <= 1.0);

    // Deve suportar consulta de hand_anchor por emocao
    let neutral_anchor = arti.get_hand_anchor(&Emotion::Neutral);
    assert_eq!(default_anchor, neutral_anchor);
}

#[test]
fn test_prop_type_grip_offset() {
    let coffee = PropType::Coffee;
    let (gx, gy) = coffee.grip_offset();
    assert!(gx.abs() >= 0.0);
    assert!(gy.abs() >= 0.0);

    let mic = PropType::Microphone;
    let (_mx, my) = mic.grip_offset();
    assert!(
        my > 0.0,
        "Microphone grip must offset vertically for handle"
    );
}

#[test]
fn test_rendered_prop_anchoring_coordinates() {
    let script = r#"
Arti (neutral): Olá, café!
[prop: coffee@Arti]
"#;
    let strip = parse_dialogue_script(script, None).expect("parse");
    let svg = render_comic_strip_to_svg(&strip);

    // Deve renderizar com classe de prop ancorado
    assert!(
        svg.contains("prop-anchored"),
        "Prop direcionado a personagem deve ter classe prop-anchored"
    );
    assert!(svg.contains("prop-cafe"), "Prop cafe deve estar presente");
}

#[test]
fn test_flipped_character_prop_anchoring() {
    let script = r#"
Arti (neutral): Oi Sombra!
Sombra (happy): Olá Arti!
[prop: livro@Sombra]
"#;
    let strip = parse_dialogue_script(script, None).expect("parse");
    let svg = render_comic_strip_to_svg(&strip);

    assert!(
        svg.contains("prop-anchored"),
        "Prop livro ancorado na Sombra deve ter classe prop-anchored"
    );
    assert!(svg.contains("prop-livro"), "Prop livro deve estar presente");
    // Sombra esta na direita (flipped), portanto o transform do prop deve conter escala negativa em x
    assert!(
        svg.contains("scale(-0.95, 0.95)"),
        "Prop em personagem espelhado deve ser renderizado com transform flipped"
    );
}
