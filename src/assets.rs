//! Embedded native character packs for Ayvu (Arti, Sombra, Billi, Aly, Lu, Raio)

use crate::character::{Anchor, CharacterPack};
use std::collections::HashMap;

pub fn list_available_characters() -> Vec<&'static str> {
    vec![
        "aly", "arti", "billi", "capivara", "jacare", "lu", "raio", "sombra",
    ]
}

pub fn get_character(id: &str) -> Option<CharacterPack> {
    match id.to_lowercase().as_str() {
        "arti" => Some(load_arti()),
        "sombra" => Some(load_sombra()),
        "billi" => Some(load_billi()),
        "aly" => Some(load_aly()),
        "lu" => Some(load_lu()),
        "raio" => Some(load_raio()),
        "capivara" => Some(load_capivara()),
        "jacare" | "jacaré" => Some(load_jacare()),
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
        load_capivara(),
        load_jacare(),
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
        hand_anchor: Anchor { x: 0.65, y: 0.55 },
        pose_hand_anchors: HashMap::new(),
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
        hand_anchor: Anchor { x: 0.71, y: 0.50 },
        pose_hand_anchors: HashMap::new(),
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
        hand_anchor: Anchor { x: 0.71, y: 0.50 },
        pose_hand_anchors: HashMap::new(),
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
        hand_anchor: Anchor { x: 0.71, y: 0.50 },
        pose_hand_anchors: HashMap::new(),
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
        hand_anchor: Anchor { x: 0.71, y: 0.50 },
        pose_hand_anchors: HashMap::new(),
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
        hand_anchor: Anchor { x: 0.71, y: 0.50 },
        pose_hand_anchors: HashMap::new(),
        poses,
    }
}

fn load_capivara() -> CharacterPack {
    let mut poses = HashMap::new();
    poses.insert(
        "neutral".to_string(),
        include_str!("../assets/characters/capivara/neutral.svg").to_string(),
    );
    poses.insert(
        "happy".to_string(),
        include_str!("../assets/characters/capivara/happy.svg").to_string(),
    );
    poses.insert(
        "laughing".to_string(),
        include_str!("../assets/characters/capivara/laughing.svg").to_string(),
    );
    poses.insert(
        "shouting".to_string(),
        include_str!("../assets/characters/capivara/shouting.svg").to_string(),
    );
    poses.insert(
        "thinking".to_string(),
        include_str!("../assets/characters/capivara/thinking.svg").to_string(),
    );
    poses.insert(
        "angry".to_string(),
        include_str!("../assets/characters/capivara/angry.svg").to_string(),
    );
    poses.insert(
        "sad".to_string(),
        include_str!("../assets/characters/capivara/sad.svg").to_string(),
    );
    poses.insert(
        "scared".to_string(),
        include_str!("../assets/characters/capivara/scared.svg").to_string(),
    );

    CharacterPack {
        id: "capivara".to_string(),
        name: "Capivara".to_string(),
        author: "Lumen Pink".to_string(),
        license: "CC-BY".to_string(),
        description: "Capivara da Trindade, mascote da UFSC de porte sereno e traço em nanquim"
            .to_string(),
        balloon_anchor: Anchor { x: 0.72, y: 0.28 },
        hand_anchor: Anchor { x: 0.68, y: 0.62 },
        pose_hand_anchors: HashMap::new(),
        poses,
    }
}

fn load_jacare() -> CharacterPack {
    let mut poses = HashMap::new();
    poses.insert(
        "neutral".to_string(),
        include_str!("../assets/characters/jacare/neutral.svg").to_string(),
    );
    poses.insert(
        "happy".to_string(),
        include_str!("../assets/characters/jacare/happy.svg").to_string(),
    );
    poses.insert(
        "laughing".to_string(),
        include_str!("../assets/characters/jacare/laughing.svg").to_string(),
    );
    poses.insert(
        "shouting".to_string(),
        include_str!("../assets/characters/jacare/shouting.svg").to_string(),
    );
    poses.insert(
        "thinking".to_string(),
        include_str!("../assets/characters/jacare/thinking.svg").to_string(),
    );
    poses.insert(
        "angry".to_string(),
        include_str!("../assets/characters/jacare/angry.svg").to_string(),
    );
    poses.insert(
        "sad".to_string(),
        include_str!("../assets/characters/jacare/sad.svg").to_string(),
    );
    poses.insert(
        "scared".to_string(),
        include_str!("../assets/characters/jacare/scared.svg").to_string(),
    );

    CharacterPack {
        id: "jacare".to_string(),
        name: "Jacaré".to_string(),
        author: "Lumen Pink".to_string(),
        license: "CC-BY".to_string(),
        description:
            "Jacaré-do-papo-amarelo do Córrego da Trindade, ícone lendário da UFSC em nanquim"
                .to_string(),
        balloon_anchor: Anchor { x: 0.75, y: 0.32 },
        hand_anchor: Anchor { x: 0.64, y: 0.58 },
        pose_hand_anchors: HashMap::new(),
        poses,
    }
}
