use std::fs;
use std::sync::Mutex;
use tempfile::TempDir;

use ayvu::assets::{discover_external_characters, get_xdg_data_dirs, load_backdrop_from_disk};
use ayvu::backdrop::get_vector_backdrop_svg;
use ayvu::character::CharacterRegistry;

static ENV_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_xdg_data_dirs_hierarchy() {
    let _guard = ENV_MUTEX.lock().unwrap();

    let temp = TempDir::new().expect("tempdir");
    let home_dir = temp.path().join("local_share");
    let sys_dir1 = temp.path().join("usr_local_share");
    let sys_dir2 = temp.path().join("usr_share");

    std::env::set_var("XDG_DATA_HOME", &home_dir);
    std::env::set_var(
        "XDG_DATA_DIRS",
        format!("{}:{}", sys_dir1.display(), sys_dir2.display()),
    );

    let dirs = get_xdg_data_dirs();

    assert!(
        dirs.len() >= 3,
        "Deve conter ao menos data_home e as duas entradas de data_dirs: {:?}",
        dirs
    );

    assert_eq!(
        dirs[0],
        home_dir.join("ayvu"),
        "A primeira prioridade deve ser XDG_DATA_HOME/ayvu"
    );
    assert_eq!(
        dirs[1],
        sys_dir1.join("ayvu"),
        "A segunda prioridade deve ser a primeira entrada de XDG_DATA_DIRS/ayvu"
    );
    assert_eq!(
        dirs[2],
        sys_dir2.join("ayvu"),
        "A terceira prioridade deve ser a segunda entrada de XDG_DATA_DIRS/ayvu"
    );
}

#[test]
fn test_discover_custom_character_from_xdg_data_home() {
    let _guard = ENV_MUTEX.lock().unwrap();

    let temp = TempDir::new().expect("tempdir");
    let home_ayvu = temp.path().join("ayvu");
    let char_dir = home_ayvu.join("characters").join("robozinho");
    fs::create_dir_all(&char_dir).expect("mkdir char_dir");

    let manifest_json = r#"{
        "id": "robozinho",
        "name": "Robozinho",
        "description": "Um robo curioso criado no XDG",
        "gender": "neutral",
        "author": "Comunidade Ayvu",
        "version": "1.0.0",
        "hand_anchor": [40.0, 110.0],
        "poses": {
            "neutral": "neutral.svg",
            "happy": "happy.svg"
        }
    }"#;
    fs::write(char_dir.join("manifest.json"), manifest_json).expect("write manifest");

    let svg_neutral = r##"<svg viewBox="0 0 100 200"><circle cx="50" cy="50" r="30" fill="#111111" /></svg>"##;
    let svg_happy = r##"<svg viewBox="0 0 100 200"><rect x="20" y="20" width="60" height="60" fill="#111111" /></svg>"##;
    fs::write(char_dir.join("neutral.svg"), svg_neutral).expect("write neutral");
    fs::write(char_dir.join("happy.svg"), svg_happy).expect("write happy");

    std::env::set_var("XDG_DATA_HOME", temp.path());
    std::env::set_var("XDG_DATA_DIRS", "");

    let custom_packs = discover_external_characters();
    let found = custom_packs.iter().find(|p| p.manifest.id == "robozinho");

    assert!(
        found.is_some(),
        "Robozinho deve ter sido descoberto em XDG_DATA_HOME"
    );
    let robo = found.unwrap();
    assert_eq!(robo.manifest.name, "Robozinho");
    assert!(robo.poses.contains_key("neutral"));
    assert!(robo.poses.contains_key("happy"));
    assert!(robo.poses["neutral"].contains(r#"circle cx="50""#));

    // Agora registra no CharacterRegistry e valida resolução
    let mut registry = CharacterRegistry::new();
    for pack in custom_packs {
        registry.register_pack(pack);
    }
    let resolved = registry.resolve_speaker("robozinho");
    assert!(
        resolved.is_some(),
        "Registry deve resolver personagem descoberto dinamicamente"
    );
}

#[test]
fn test_discover_custom_backdrop_from_xdg() {
    let _guard = ENV_MUTEX.lock().unwrap();

    let temp = TempDir::new().expect("tempdir");
    let backdrops_dir = temp.path().join("ayvu").join("backdrops");
    fs::create_dir_all(&backdrops_dir).expect("mkdir backdrops");

    let joaquina_svg = r##"<g class="backdrop-vector backdrop-joaquina"><rect x="0" y="0" width="100" height="100" fill="#ffffff" stroke="#111111" /></g>"##;
    fs::write(backdrops_dir.join("praia_joaquina.svg"), joaquina_svg).expect("write backdrop svg");

    std::env::set_var("XDG_DATA_HOME", temp.path());
    std::env::set_var("XDG_DATA_DIRS", "");

    // Teste direto de carregamento em disco
    let loaded = load_backdrop_from_disk("praia_joaquina");
    assert!(
        loaded.is_some(),
        "Deve carregar 'praia_joaquina' dinamicamente do disco XDG"
    );
    assert!(loaded.unwrap().contains("backdrop-joaquina"));

    // Teste integrado através de get_vector_backdrop_svg
    let rendered = get_vector_backdrop_svg("praia_joaquina", 0.0, 0.0, 200.0, 300.0);
    assert!(
        rendered.is_some(),
        "get_vector_backdrop_svg deve integrar resolução de backdrops externos em disco"
    );
    assert!(rendered.unwrap().contains("backdrop-joaquina"));
}
