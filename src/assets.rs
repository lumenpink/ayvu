//! Embedded native character packs for Ayvu (Arti, Sombra, Billi, Aly, Lu, Raio)

use crate::character::{Anchor, CharacterPack};
use std::collections::HashMap;

pub fn list_available_characters() -> Vec<&'static str> {
    vec!["aly", "arti", "billi", "lu", "raio", "sombra"]
}

pub fn get_character(id: &str) -> Option<CharacterPack> {
    match id.to_lowercase().as_str() {
        "arti" => Some(load_arti()),
        "sombra" => Some(load_sombra()),
        "billi" => Some(load_billi()),
        "aly" => Some(load_aly()),
        "lu" => Some(load_lu()),
        "raio" => Some(load_raio()),
        _ => None,
    }
}

pub fn load_official_packs() -> Vec<CharacterPack> {
    vec![
        load_arti(),
        load_sombra(),
        load_billi(),
        load_aly(),
        load_lu(),
        load_raio(),
    ]
}

pub fn get_backdrop_b64(_name: &str) -> Option<&'static str> {
    None
}

fn load_arti() -> CharacterPack {
    let mut poses = HashMap::new();
    poses.insert(
        "neutral".to_string(),
        include_str!("../assets/characters/arti/neutral.svg").to_string(),
    );
    poses.insert(
        "happy".to_string(),
        include_str!("../assets/characters/arti/happy.svg").to_string(),
    );
    poses.insert(
        "laughing".to_string(),
        include_str!("../assets/characters/arti/laughing.svg").to_string(),
    );
    poses.insert(
        "shouting".to_string(),
        include_str!("../assets/characters/arti/shouting.svg").to_string(),
    );
    poses.insert(
        "thinking".to_string(),
        include_str!("../assets/characters/arti/thinking.svg").to_string(),
    );
    poses.insert(
        "angry".to_string(),
        include_str!("../assets/characters/arti/angry.svg").to_string(),
    );
    poses.insert(
        "sad".to_string(),
        include_str!("../assets/characters/arti/sad.svg").to_string(),
    );
    poses.insert(
        "scared".to_string(),
        include_str!("../assets/characters/arti/scared.svg").to_string(),
    );

    CharacterPack {
        id: "arti".to_string(),
        name: "Arti".to_string(),
        author: "Lumen Pink".to_string(),
        license: "CC-BY".to_string(),
        description:
            "Arti, o acadêmico misterioso de sobretudo preto, cabelos longos lisos, óculos de grau e calça saruel"
                .to_string(),
        balloon_anchor: Anchor { x: 0.45, y: 0.12 },
        poses,
    }
}

fn load_sombra() -> CharacterPack {
    let mut poses = HashMap::new();
    poses.insert(
        "neutral".to_string(),
        include_str!("../assets/characters/sombra/neutral.svg").to_string(),
    );
    poses.insert(
        "happy".to_string(),
        include_str!("../assets/characters/sombra/happy.svg").to_string(),
    );
    poses.insert(
        "laughing".to_string(),
        include_str!("../assets/characters/sombra/laughing.svg").to_string(),
    );
    poses.insert(
        "shouting".to_string(),
        include_str!("../assets/characters/sombra/shouting.svg").to_string(),
    );
    poses.insert(
        "thinking".to_string(),
        include_str!("../assets/characters/sombra/thinking.svg").to_string(),
    );
    poses.insert(
        "angry".to_string(),
        include_str!("../assets/characters/sombra/angry.svg").to_string(),
    );
    poses.insert(
        "sad".to_string(),
        include_str!("../assets/characters/sombra/sad.svg").to_string(),
    );
    poses.insert(
        "scared".to_string(),
        include_str!("../assets/characters/sombra/scared.svg").to_string(),
    );

    CharacterPack {
        id: "sombra".to_string(),
        name: "Sombra".to_string(),
        author: "Lumen Pink".to_string(),
        license: "CC-BY".to_string(),
        description: "Silhueta humanoide feminina em nanquim com estampa S no peito".to_string(),
        balloon_anchor: Anchor { x: 0.50, y: 0.19 },
        poses,
    }
}

fn load_billi() -> CharacterPack {
    let mut poses = HashMap::new();
    poses.insert(
        "neutral".to_string(),
        include_str!("../assets/characters/billi/neutral.svg").to_string(),
    );
    poses.insert(
        "happy".to_string(),
        include_str!("../assets/characters/billi/happy.svg").to_string(),
    );
    poses.insert(
        "laughing".to_string(),
        include_str!("../assets/characters/billi/laughing.svg").to_string(),
    );
    poses.insert(
        "shouting".to_string(),
        include_str!("../assets/characters/billi/shouting.svg").to_string(),
    );
    poses.insert(
        "thinking".to_string(),
        include_str!("../assets/characters/billi/thinking.svg").to_string(),
    );
    poses.insert(
        "angry".to_string(),
        include_str!("../assets/characters/billi/angry.svg").to_string(),
    );
    poses.insert(
        "sad".to_string(),
        include_str!("../assets/characters/billi/sad.svg").to_string(),
    );
    poses.insert(
        "scared".to_string(),
        include_str!("../assets/characters/billi/scared.svg").to_string(),
    );

    CharacterPack {
        id: "billi".to_string(),
        name: "Billi".to_string(),
        author: "Lumen Pink".to_string(),
        license: "CC-BY".to_string(),
        description: "Billi, silhueta humanoide feminina em nanquim com estampa B no peito"
            .to_string(),
        balloon_anchor: Anchor { x: 0.50, y: 0.19 },
        poses,
    }
}

fn load_aly() -> CharacterPack {
    let mut poses = HashMap::new();
    poses.insert(
        "neutral".to_string(),
        include_str!("../assets/characters/aly/neutral.svg").to_string(),
    );
    poses.insert(
        "happy".to_string(),
        include_str!("../assets/characters/aly/happy.svg").to_string(),
    );
    poses.insert(
        "laughing".to_string(),
        include_str!("../assets/characters/aly/laughing.svg").to_string(),
    );
    poses.insert(
        "shouting".to_string(),
        include_str!("../assets/characters/aly/shouting.svg").to_string(),
    );
    poses.insert(
        "thinking".to_string(),
        include_str!("../assets/characters/aly/thinking.svg").to_string(),
    );
    poses.insert(
        "angry".to_string(),
        include_str!("../assets/characters/aly/angry.svg").to_string(),
    );
    poses.insert(
        "sad".to_string(),
        include_str!("../assets/characters/aly/sad.svg").to_string(),
    );
    poses.insert(
        "scared".to_string(),
        include_str!("../assets/characters/aly/scared.svg").to_string(),
    );

    CharacterPack {
        id: "aly".to_string(),
        name: "Aly".to_string(),
        author: "Lumen Pink".to_string(),
        license: "CC-BY".to_string(),
        description: "Aly, silhueta humanoide feminina em nanquim com estampa A no peito"
            .to_string(),
        balloon_anchor: Anchor { x: 0.50, y: 0.19 },
        poses,
    }
}

fn load_lu() -> CharacterPack {
    let mut poses = HashMap::new();
    poses.insert(
        "neutral".to_string(),
        include_str!("../assets/characters/lu/neutral.svg").to_string(),
    );
    poses.insert(
        "happy".to_string(),
        include_str!("../assets/characters/lu/happy.svg").to_string(),
    );
    poses.insert(
        "laughing".to_string(),
        include_str!("../assets/characters/lu/laughing.svg").to_string(),
    );
    poses.insert(
        "shouting".to_string(),
        include_str!("../assets/characters/lu/shouting.svg").to_string(),
    );
    poses.insert(
        "thinking".to_string(),
        include_str!("../assets/characters/lu/thinking.svg").to_string(),
    );
    poses.insert(
        "angry".to_string(),
        include_str!("../assets/characters/lu/angry.svg").to_string(),
    );
    poses.insert(
        "sad".to_string(),
        include_str!("../assets/characters/lu/sad.svg").to_string(),
    );
    poses.insert(
        "scared".to_string(),
        include_str!("../assets/characters/lu/scared.svg").to_string(),
    );

    CharacterPack {
        id: "lu".to_string(),
        name: "Lu".to_string(),
        author: "Lumen Pink".to_string(),
        license: "CC-BY".to_string(),
        description: "Lu, silhueta humanoide feminina em nanquim com estampa L no peito"
            .to_string(),
        balloon_anchor: Anchor { x: 0.50, y: 0.19 },
        poses,
    }
}

fn load_raio() -> CharacterPack {
    let mut poses = HashMap::new();
    poses.insert(
        "neutral".to_string(),
        include_str!("../assets/characters/raio/neutral.svg").to_string(),
    );
    poses.insert(
        "happy".to_string(),
        include_str!("../assets/characters/raio/happy.svg").to_string(),
    );
    poses.insert(
        "laughing".to_string(),
        include_str!("../assets/characters/raio/laughing.svg").to_string(),
    );
    poses.insert(
        "shouting".to_string(),
        include_str!("../assets/characters/raio/shouting.svg").to_string(),
    );
    poses.insert(
        "thinking".to_string(),
        include_str!("../assets/characters/raio/thinking.svg").to_string(),
    );
    poses.insert(
        "angry".to_string(),
        include_str!("../assets/characters/raio/angry.svg").to_string(),
    );
    poses.insert(
        "sad".to_string(),
        include_str!("../assets/characters/raio/sad.svg").to_string(),
    );
    poses.insert(
        "scared".to_string(),
        include_str!("../assets/characters/raio/scared.svg").to_string(),
    );

    CharacterPack {
        id: "raio".to_string(),
        name: "Raio".to_string(),
        author: "Lumen Pink".to_string(),
        license: "CC-BY".to_string(),
        description: "Raio, silhueta humanoide masculina em branco com contorno preto e estampa R"
            .to_string(),
        balloon_anchor: Anchor { x: 0.50, y: 0.19 },
        poses,
    }
}
