use ayvu::character::CharacterRegistry;
use ayvu::parser::parse_dialogue_script;
use ayvu::renderer::svg::render_comic_strip_to_svg;

#[test]
fn test_aly_manifest_has_accurate_anatomical_anchors() {
    let registry = CharacterRegistry::default();
    let aly = registry
        .get("aly")
        .expect("Aly deve estar presente no registro padrão de personagens");

    assert_eq!(aly.id, "aly");
    assert_eq!(aly.name, "Aly");

    // Validação das âncoras anatômicas calibradas para a jaqueta, cachos e mão
    let balloon_anchor = aly.balloon_anchor;
    assert!(
        (balloon_anchor.x - 0.50).abs() < 0.05,
        "balloon_anchor.x deve estar centralizado na cabeça (~0.50), obtido {}",
        balloon_anchor.x
    );
    assert!(
        (balloon_anchor.y - 0.18).abs() < 0.05,
        "balloon_anchor.y deve estar no topo dos cachos (~0.18), obtido {}",
        balloon_anchor.y
    );

    let hand_anchor = aly.hand_anchor;
    assert!(
        (hand_anchor.x - 0.68).abs() < 0.05,
        "hand_anchor.x deve apontar para a mão direita anatômica (~0.68), obtido {}",
        hand_anchor.x
    );
    assert!(
        (hand_anchor.y - 0.49).abs() < 0.05,
        "hand_anchor.y deve apontar para a mão na altura do tórax/cintura (~0.49), obtido {}",
        hand_anchor.y
    );

    // Descrição não deve mais conter o placeholder da estampa "A" no peito
    assert!(
        !aly.description.contains("estampa A"),
        "A descrição da Aly não deve conter referências a placeholders 'estampa A'"
    );
}

#[test]
fn test_aly_has_eight_authentic_ink_poses_without_placeholder_badge() {
    let registry = CharacterRegistry::default();
    let aly = registry
        .get("aly")
        .expect("Aly deve estar registrada no catálogo");

    let expected_poses = [
        "neutral", "happy", "laughing", "shouting", "thinking", "angry", "sad", "scared",
    ];

    for pose_name in &expected_poses {
        assert!(
            aly.poses.contains_key(*pose_name),
            "Aly deve conter a pose '{}'",
            pose_name
        );
        let svg_data = &aly.poses[*pose_name];

        // Não deve conter a estampa artificial com a letra "A"
        assert!(
            !svg_data.contains(">A</text>"),
            "Pose '{}' da Aly ainda contém badge artificial com letra 'A'",
            pose_name
        );
        assert!(
            !svg_data.contains("cx=\"80\" cy=\"180\" r=\"16\""),
            "Pose '{}' da Aly ainda contém círculo do badge provisório",
            pose_name
        );

        // Deve conter elementos característicos da identidade visual em nanquim
        assert!(
            svg_data.contains("aly-jacket"),
            "Pose '{}' da Aly deve conter a classe da jaqueta esportiva em nanquim",
            pose_name
        );
        assert!(
            svg_data.contains("aly-hair-curly"),
            "Pose '{}' da Aly deve conter a classe dos cabelos cacheados volumosos",
            pose_name
        );
        assert!(
            svg_data.contains("aly-jeans"),
            "Pose '{}' da Aly deve conter a classe da calça jeans",
            pose_name
        );
    }
}

#[test]
fn test_render_dialogue_with_aly_and_prop() {
    let script = r#"
[prop: cafe@Aly]
Aly: Bom dia, pessoal! Prontos para a aula no HU?
"#;

    let strip = parse_dialogue_script(script, Some("Aula no HU")).expect("Script com Aly e prop deve ser parseado");
    let svg = render_comic_strip_to_svg(&strip);

    assert!(
        svg.contains("data-speaker=\"aly\""),
        "SVG final deve renderizar o grupo da Aly com atributo de speaker"
    );
    assert!(
        svg.contains("aly-jacket"),
        "SVG final renderizado deve conter a jaqueta em nanquim da Aly"
    );
    assert!(
        svg.contains("prop-anchored"),
        "SVG final renderizado deve conter o prop de café ancorado"
    );
}
