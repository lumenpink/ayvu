use ayvu::backdrop::get_vector_backdrop_svg;
use ayvu::parser::parse_dialogue_script;
use ayvu::renderer::svg::render_comic_strip_to_svg;

#[test]
fn test_hu_backdrop_contains_authentic_reference_elements() {
    let svg = get_vector_backdrop_svg("hu_fachada", 0.0, 0.0, 300.0, 300.0)
        .expect("Backdrop hu_fachada deve estar disponível");

    // Elementos da arquitetura real do HU (fotos hu-ufsc.jpg e HU2.jpg)
    assert!(
        svg.contains("hu-monumental-sign"),
        "Backdrop do HU deve conter a placa monumental frontal com suporte"
    );
    assert!(
        svg.contains("hu-water-tower"),
        "Backdrop do HU deve conter a torre cilíndrica da caixa d'água à esquerda"
    );
    assert!(
        svg.contains("hu-hospital-blocks"),
        "Backdrop do HU deve conter os blocos de pavilhões hospitalares em perspectiva"
    );
    assert!(
        svg.contains("hu-mesh-fence"),
        "Backdrop do HU deve conter a cerca aramada em losangos"
    );
}

#[test]
fn test_render_strip_with_hu_backdrop() {
    let script = r#"
[backdrop: hu_fachada]
Arti (neutral): Chegando para o estágio no HU.
"#;

    let strip = parse_dialogue_script(script, Some("Estágio no HU")).expect("parse");
    let svg = render_comic_strip_to_svg(&strip);

    assert!(
        svg.contains("backdrop-hu-fachada"),
        "SVG da tira deve conter o backdrop do HU"
    );
    assert!(
        svg.contains("hu-monumental-sign"),
        "SVG da tira deve exibir a placa monumental do HU"
    );
}
