use ayvu::character::CharacterRegistry;
use ayvu::parser::parse_dialogue_script;
use ayvu::renderer::svg::render_comic_strip_to_svg;

#[test]
fn test_ground_line_and_shadows() {
    let script = "Alice: Olá mundo!\nBob: Oi Alice!";
    let strip = parse_dialogue_script(script, None).expect("Parse strip");
    let svg = render_comic_strip_to_svg(&strip);

    // Deve conter linha de chão e sombras de contato
    assert!(
        svg.contains("panel-floor"),
        "SVG deve conter a linha de chão/piso para ancoragem física"
    );
    assert!(
        svg.contains("contact-shadow"),
        "SVG deve conter sombras de contato sob os personagens"
    );
}

#[test]
fn test_no_artificial_ui_labels() {
    let script = "Alice: Olá!\nBob: Tudo bem?";
    let strip = parse_dialogue_script(script, None).expect("Parse strip");
    let svg = render_comic_strip_to_svg(&strip);

    // Não deve poluir o quadrinho com rótulos artificiais de depuração ou UI
    assert!(
        !svg.contains("emotion-badge"),
        "SVG não deve conter rótulo textual de emoção tipo '(neutral)'"
    );
    assert!(
        !svg.contains("speaker-label"),
        "SVG não deve conter rótulo textual do nome do personagem embaixo do avatar"
    );
}

#[test]
fn test_hand_inked_panel_borders() {
    let script = "Alice: Que belo dia!";
    let strip = parse_dialogue_script(script, None).expect("Parse strip");
    let svg = render_comic_strip_to_svg(&strip);

    // Deve conter moldura de nanquim e cantos de prancheta
    assert!(
        svg.contains("panel-frame-ink") || svg.contains("panel-corner-tick"),
        "SVG deve conter moldura de nanquim orgânica com cantos de prancheta"
    );
    assert!(
        !svg.contains("rx=\"4\""),
        "SVG não deve usar cantos arredondados digitais de CSS rx=4"
    );
}

#[test]
fn test_anti_crossing_balloons() {
    let script = "Alice: Primeiro balão na esquerda!\nBob: Segundo balão na direita!";
    let strip = parse_dialogue_script(script, None).expect("Parse strip");
    let svg = render_comic_strip_to_svg(&strip);

    // O primeiro balão e o segundo balão não devem ter a mesma coordenada X fixa de largura total
    // Devem existir origens diferenciadas entre o interlocutor da esquerda e da direita
    assert!(
        svg.contains("balloon-left")
            || svg.contains("balloon-right")
            || svg.contains("anti-crossing"),
        "SVG deve aplicar posicionamento de balão anti-crossing dedicado"
    );
}

#[test]
fn test_classic_comic_chat_characters_available() {
    let registry = CharacterRegistry::default();
    assert!(
        registry.get("tiki").is_some(),
        "Deve conter a personagem clássica Tiki"
    );
    assert!(
        registry.get("jordan").is_some(),
        "Deve conter o personagem clássico Jordan"
    );
    assert!(
        registry.get("armando").is_some(),
        "Deve conter o personagem clássico Armando"
    );
}
