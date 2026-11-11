use ayvu::character::CharacterRegistry;
use ayvu::model::Emotion;
use ayvu::parser::parse_dialogue_script;
use ayvu::renderer::svg::render_comic_strip_to_svg;

#[test]
fn test_billi_all_poses_follow_comic_chat_ink_pipeline() {
    let registry = CharacterRegistry::default();
    let billi = registry
        .get("billi")
        .expect("Personagem billi deve existir");

    let emotions = [
        (Emotion::Neutral, "neutral"),
        (Emotion::Happy, "happy"),
        (Emotion::Laughing, "laughing"),
        (Emotion::Shouting, "shouting"),
        (Emotion::Thinking, "thinking"),
        (Emotion::Angry, "angry"),
        (Emotion::Sad, "sad"),
        (Emotion::Scared, "scared"),
    ];

    for (emotion, name) in &emotions {
        let svg = billi.get_pose_svg(emotion);

        let expected_class = format!("billi-ink billi-{name}");
        assert!(
            svg.contains(&expected_class),
            "SVG da Billi na pose '{name}' deve conter a classe de nanquim '{expected_class}'"
        );

        // Deve utilizar o pipeline autêntico do Comic Chat com nimbus mask e nanquim embutido
        assert!(
            svg.contains(r#"<image href="data:image/png;base64,"#),
            "Pose '{name}' da Billi deve utilizar imagem embutida do pipeline Comic Chat"
        );

        // Não deve conter preenchimentos chapados do vetor simplificado anterior
        assert!(
            !svg.contains("#1e232a"),
            "Pose '{name}' não deve conter cor chapada antiga de calça"
        );
        assert!(
            !svg.contains("#f8eedf"),
            "Pose '{name}' não deve conter cor chapada antiga de pele"
        );
    }
}

#[test]
fn test_billi_poses_have_distinct_expressions() {
    let registry = CharacterRegistry::default();
    let billi = registry
        .get("billi")
        .expect("Personagem billi deve existir");

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

    let svgs: Vec<&str> = emotions.iter().map(|e| billi.get_pose_svg(e)).collect();

    // Nenhuma pose pode ter exatamente o mesmo conteúdo
    for i in 0..svgs.len() {
        for j in (i + 1)..svgs.len() {
            assert_ne!(
                svgs[i], svgs[j],
                "As poses {:?} e {:?} da Billi devem ter expressões faciais e gestos distintos",
                emotions[i], emotions[j]
            );
        }
    }
}

#[test]
fn test_render_strip_with_billi_comic_chat_pipeline() {
    let script = r#"
[backdrop: cafe_ccs]
Billi (happy): O café do CCS é o melhor refúgio do campus!
Arti (thinking): Depende do referencial termodinâmico, Billi.
---
Billi (laughing): Sempre com uma equação na manga, né Arti?
"#;

    let strip = parse_dialogue_script(script, Some("Encontro no Café")).expect("parse");
    let svg = render_comic_strip_to_svg(&strip);

    assert!(
        svg.contains("billi-ink"),
        "SVG da tira deve conter a classe de nanquim da Billi"
    );
    assert!(
        svg.contains("arti-ink"),
        "SVG da tira deve conter a classe de nanquim do Arti"
    );
}
