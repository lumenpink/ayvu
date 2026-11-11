use ayvu::backdrop::{get_vector_backdrop_svg, list_available_backdrops};
use ayvu::parser::parse_dialogue_script;
use ayvu::renderer::svg::render_comic_strip_to_svg;

#[test]
fn test_all_eighteen_backdrops_registered() {
    let list = list_available_backdrops();
    assert_eq!(
        list.len(),
        18,
        "Deve conter exatamente 18 cenários cadastrados, encontrou: {:?}",
        list
    );

    let expected_new = [
        "bosque_cfh",
        "reitoria",
        "pista_atletismo",
        "centro_eventos",
    ];
    for b in &expected_new {
        assert!(
            list.contains(b),
            "Lista de cenários deve conter o novo cenário '{}'",
            b
        );
    }
}

#[test]
fn test_new_scenarios_render_valid_rich_svg() {
    let new_scenarios = [
        ("bosque_cfh", "backdrop-bosque-cfh"),
        ("reitoria", "backdrop-reitoria"),
        ("pista_atletismo", "backdrop-pista-atletismo"),
        ("centro_eventos", "backdrop-centro-eventos"),
    ];

    for (name, expected_class) in &new_scenarios {
        let svg_opt = get_vector_backdrop_svg(name, 10.0, 10.0, 200.0, 300.0);
        assert!(
            svg_opt.is_some(),
            "Cenário '{}' deve retornar SVG renderizado",
            name
        );
        let svg = svg_opt.unwrap();
        assert!(
            svg.contains(expected_class),
            "SVG do cenário '{}' deve conter a classe '{}'",
            name,
            expected_class
        );
        assert!(
            svg.contains("<path") || svg.contains("<line") || svg.contains("<g"),
            "Cenário '{}' deve conter elementos vetoriais desenhados",
            name
        );
    }
}

#[test]
fn test_render_strip_with_new_scenarios() {
    let script = r#"
[backdrop: bosque_cfh]
Arti (thinking): Os eucaliptos filtram a luz da tarde no CFH.
Capivara (happy): Melhor lugar para descansar entre os seminários.
---
[backdrop: reitoria]
Jacaré (neutral): Passando em frente à Reitoria a caminho do córrego.
"#;
    let strip = parse_dialogue_script(script, Some("Tirinha Campus UFSC")).expect("parse");
    let svg = render_comic_strip_to_svg(&strip);

    assert!(
        svg.contains("backdrop-bosque-cfh"),
        "SVG da tirinha deve conter o cenário do Bosque do CFH"
    );
    assert!(
        svg.contains("backdrop-reitoria"),
        "SVG da tirinha deve conter o cenário da Reitoria"
    );
}

#[test]
fn test_all_backdrops_follow_comic_chat_ink_standards() {
    let list = list_available_backdrops();
    let forbidden_paint_colors = [
        "#2ea44f", "#ffe58f", "#d0e1fd", "#388bfd", "#ff7b72", "#e3b341", "#54aeff", "#1f6feb",
        "#85e89d", "#ffd33d",
    ];

    for name in list {
        let svg = get_vector_backdrop_svg(name, 0.0, 0.0, 200.0, 300.0)
            .unwrap_or_else(|| panic!("Cenário '{}' não retornou SVG", name));

        // Sem textos de sistema artificiais simulando placas
        assert!(
            !svg.contains("<text"),
            "Cenário '{}' viola padrão Comic Chat ao conter tag <text (deve usar desenho vetorial puro): {}",
            name,
            svg
        );

        // Sem cores berrantes de ferramenta Paint
        for color in &forbidden_paint_colors {
            assert!(
                !svg.to_lowercase().contains(color),
                "Cenário '{}' viola padrão nanquim ao usar cor saturada de Paint '{}'",
                name,
                color
            );
        }

        // Deve conter linhas ou hachuras de nanquim
        assert!(
            svg.contains("#111111")
                || svg.contains("#1a1a1a")
                || svg.contains("#222222")
                || svg.contains("#24292e"),
            "Cenário '{}' deve utilizar traços de nanquim escuro",
            name
        );
    }
}
