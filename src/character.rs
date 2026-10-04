use crate::model::Emotion;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Anchor {
    pub x: f64,
    pub y: f64,
}

impl Default for Anchor {
    fn default() -> Self {
        Self { x: 0.5, y: 0.35 }
    }
}

fn default_anchor() -> Anchor {
    Anchor::default()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterManifest {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub license: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_anchor")]
    pub balloon_anchor: Anchor,
    pub poses: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterMetadata {
    pub id: String,
    pub name: String,
    pub author: String,
    pub license: String,
    pub description: String,
    pub emotions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CharacterPack {
    pub id: String,
    pub name: String,
    pub author: String,
    pub license: String,
    pub description: String,
    pub balloon_anchor: Anchor,
    pub poses: HashMap<String, String>,
}

impl CharacterPack {
    pub fn get_pose_svg(&self, emotion: &Emotion) -> &str {
        let key = emotion.as_str();
        if let Some(svg) = self.poses.get(key) {
            return svg.as_str();
        }

        // Semantic emotion fallbacks
        let fallback_key = match emotion {
            Emotion::Crying => "sad",
            Emotion::Skeptical => "thinking",
            Emotion::Shouting => "angry",
            Emotion::Laughing => "happy",
            _ => "neutral",
        };

        if let Some(svg) = self.poses.get(fallback_key) {
            return svg.as_str();
        }

        if let Some(svg) = self.poses.get("neutral") {
            return svg.as_str();
        }

        ""
    }

    pub fn dimensions(&self) -> (f64, f64) {
        match self.id.to_lowercase().as_str() {
            "armando" => (150.0, 390.0),
            "dan" => (200.0, 390.0),
            "tiki" => (180.0, 440.0),
            "tongtyed" => (210.0, 450.0),
            _ => (100.0, 120.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CharacterRegistry {
    packs: HashMap<String, CharacterPack>,
    ordered_ids: Vec<String>,
    speaker_cache: HashMap<String, String>,
}

impl Default for CharacterRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        // Load embedded presets
        for pack in load_embedded_presets() {
            registry.register_pack(pack);
        }
        // Also check runtime filesystem if assets directory exists
        let assets_dir = Path::new("assets/characters");
        if assets_dir.is_dir() {
            let _ = registry.load_from_directory(assets_dir);
        }
        registry
    }
}

impl CharacterRegistry {
    pub fn new() -> Self {
        Self {
            packs: HashMap::new(),
            ordered_ids: Vec::new(),
            speaker_cache: HashMap::new(),
        }
    }

    pub fn register_pack(&mut self, pack: CharacterPack) {
        let id_lower = pack.id.to_lowercase();
        if !self.ordered_ids.contains(&id_lower) {
            self.ordered_ids.push(id_lower.clone());
        }
        self.packs.insert(id_lower, pack);
    }

    pub fn contains_id(&self, id: &str) -> bool {
        self.packs.contains_key(&id.to_lowercase())
    }

    pub fn get(&self, id: &str) -> Option<&CharacterPack> {
        self.packs.get(&id.to_lowercase())
    }

    pub fn list_character_ids(&self) -> Vec<String> {
        self.ordered_ids.clone()
    }

    pub fn list_metadata(&self) -> Vec<CharacterMetadata> {
        let mut list = Vec::new();
        for id in &self.ordered_ids {
            if let Some(pack) = self.packs.get(id) {
                let mut emotions: Vec<String> = pack.poses.keys().cloned().collect();
                emotions.sort();
                list.push(CharacterMetadata {
                    id: pack.id.clone(),
                    name: pack.name.clone(),
                    author: pack.author.clone(),
                    license: pack.license.clone(),
                    description: pack.description.clone(),
                    emotions,
                });
            }
        }
        list
    }

    pub fn resolve_speaker(&mut self, speaker: &str) -> CharacterPack {
        let speaker_trimmed = speaker.trim();
        let speaker_lower = speaker_trimmed.to_lowercase();

        // 1. Check if speaker is already cached
        if let Some(id) = self.speaker_cache.get(&speaker_lower) {
            if let Some(pack) = self.packs.get(id) {
                return pack.clone();
            }
        }

        // 2. Check if speaker directly matches a character ID or character name
        for (id, pack) in &self.packs {
            if id == &speaker_lower || pack.name.to_lowercase() == speaker_lower {
                self.speaker_cache.insert(speaker_lower.clone(), id.clone());
                return pack.clone();
            }
        }

        // 3. Fallback: deterministic round-robin based on count of assigned unknown speakers
        if self.ordered_ids.is_empty() {
            panic!("CharacterRegistry has no registered character packs");
        }
        let index = self.speaker_cache.len() % self.ordered_ids.len();
        let assigned_id = self.ordered_ids[index].clone();
        self.speaker_cache
            .insert(speaker_lower, assigned_id.clone());
        self.packs.get(&assigned_id).unwrap().clone()
    }

    pub fn load_from_directory(&mut self, base_dir: &Path) -> Result<usize, std::io::Error> {
        let mut loaded = 0;
        if !base_dir.is_dir() {
            return Ok(0);
        }

        for entry in fs::read_dir(base_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let manifest_path = path.join("manifest.json");
                if manifest_path.is_file() {
                    if let Ok(content) = fs::read_to_string(&manifest_path) {
                        if let Ok(manifest) = serde_json::from_str::<CharacterManifest>(&content) {
                            let mut poses = HashMap::new();
                            for (emotion_name, file_rel) in &manifest.poses {
                                let pose_file = path.join(file_rel);
                                if let Ok(pose_svg) = fs::read_to_string(&pose_file) {
                                    poses.insert(
                                        emotion_name.clone(),
                                        clean_or_extract_svg(&pose_svg),
                                    );
                                }
                            }
                            if !self.contains_id(&manifest.id) {
                                self.register_pack(CharacterPack {
                                    id: manifest.id,
                                    name: manifest.name,
                                    author: manifest.author,
                                    license: manifest.license,
                                    description: manifest.description,
                                    balloon_anchor: manifest.balloon_anchor,
                                    poses,
                                });
                                loaded += 1;
                            }
                        }
                    }
                }
            }
        }
        Ok(loaded)
    }
}

pub fn clean_or_extract_svg(raw_svg: &str) -> String {
    let trimmed = raw_svg.trim();
    // If it contains <svg ...> and </svg>, extract the inner content
    if let Some(start_idx) = trimmed.find("<svg") {
        if let Some(tag_end) = trimmed[start_idx..].find('>') {
            let content_start = start_idx + tag_end + 1;
            if let Some(end_idx) = trimmed.rfind("</svg>") {
                if content_start <= end_idx {
                    return trimmed[content_start..end_idx].trim().to_string();
                }
            }
        }
    }
    trimmed.to_string()
}

fn load_embedded_presets() -> Vec<CharacterPack> {
    let mut presets = crate::assets::load_woodring_characters();
    presets.push(load_preset_jordan());
    presets.push(load_preset_tuffy());
    presets.push(load_preset_peep());
    presets
}


fn load_preset_tuffy() -> CharacterPack {
    let mut poses = HashMap::new();
    poses.insert(
        "neutral".into(),
        clean_or_extract_svg(include_str!("../assets/characters/tuffy/neutral.svg")),
    );
    poses.insert(
        "happy".into(),
        clean_or_extract_svg(include_str!("../assets/characters/tuffy/happy.svg")),
    );
    poses.insert(
        "sad".into(),
        clean_or_extract_svg(include_str!("../assets/characters/tuffy/sad.svg")),
    );
    poses.insert(
        "angry".into(),
        clean_or_extract_svg(include_str!("../assets/characters/tuffy/angry.svg")),
    );
    poses.insert(
        "surprised".into(),
        clean_or_extract_svg(include_str!("../assets/characters/tuffy/surprised.svg")),
    );
    poses.insert(
        "shouting".into(),
        clean_or_extract_svg(include_str!("../assets/characters/tuffy/shouting.svg")),
    );
    poses.insert(
        "thinking".into(),
        clean_or_extract_svg(include_str!("../assets/characters/tuffy/thinking.svg")),
    );
    poses.insert(
        "laughing".into(),
        clean_or_extract_svg(include_str!("../assets/characters/tuffy/laughing.svg")),
    );
    poses.insert(
        "whispering".into(),
        clean_or_extract_svg(include_str!("../assets/characters/tuffy/whispering.svg")),
    );

    CharacterPack {
        id: "tuffy".into(),
        name: "Tuffy".into(),
        author: "Ayvu Project (inspired by Jim Woodring / MS Comic Chat)".into(),
        license: "GPL-3.0-or-later".into(),
        description: "Expressive vintage cartoon dog with floppy ears and snout".into(),
        balloon_anchor: Anchor { x: 0.5, y: 0.35 },
        poses,
    }
}

fn load_preset_jordan() -> CharacterPack {
    let mut poses = HashMap::new();
    poses.insert(
        "neutral".into(),
        clean_or_extract_svg(include_str!("../assets/characters/jordan/neutral.svg")),
    );
    poses.insert(
        "happy".into(),
        clean_or_extract_svg(include_str!("../assets/characters/jordan/happy.svg")),
    );
    poses.insert(
        "sad".into(),
        clean_or_extract_svg(include_str!("../assets/characters/jordan/sad.svg")),
    );
    poses.insert(
        "angry".into(),
        clean_or_extract_svg(include_str!("../assets/characters/jordan/angry.svg")),
    );
    poses.insert(
        "surprised".into(),
        clean_or_extract_svg(include_str!("../assets/characters/jordan/surprised.svg")),
    );
    poses.insert(
        "shouting".into(),
        clean_or_extract_svg(include_str!("../assets/characters/jordan/shouting.svg")),
    );
    poses.insert(
        "thinking".into(),
        clean_or_extract_svg(include_str!("../assets/characters/jordan/thinking.svg")),
    );
    poses.insert(
        "laughing".into(),
        clean_or_extract_svg(include_str!("../assets/characters/jordan/laughing.svg")),
    );
    poses.insert(
        "whispering".into(),
        clean_or_extract_svg(include_str!("../assets/characters/jordan/whispering.svg")),
    );

    CharacterPack {
        id: "jordan".into(),
        name: "Jordan".into(),
        author: "Jim Woodring / Microsoft Comic Chat / Ayvu".into(),
        license: "MIT".into(),
        description:
            "Intellectual comic character with round spectacles, shaggy hair and green coat".into(),
        balloon_anchor: Anchor { x: 0.5, y: 0.45 },
        poses,
    }
}

fn load_preset_peep() -> CharacterPack {
    let mut poses = HashMap::new();
    poses.insert(
        "neutral".into(),
        clean_or_extract_svg(include_str!("../assets/characters/peep/neutral.svg")),
    );
    poses.insert(
        "happy".into(),
        clean_or_extract_svg(include_str!("../assets/characters/peep/happy.svg")),
    );
    poses.insert(
        "sad".into(),
        clean_or_extract_svg(include_str!("../assets/characters/peep/sad.svg")),
    );
    poses.insert(
        "angry".into(),
        clean_or_extract_svg(include_str!("../assets/characters/peep/angry.svg")),
    );
    poses.insert(
        "surprised".into(),
        clean_or_extract_svg(include_str!("../assets/characters/peep/surprised.svg")),
    );
    poses.insert(
        "shouting".into(),
        clean_or_extract_svg(include_str!("../assets/characters/peep/shouting.svg")),
    );
    poses.insert(
        "thinking".into(),
        clean_or_extract_svg(include_str!("../assets/characters/peep/thinking.svg")),
    );
    poses.insert(
        "laughing".into(),
        clean_or_extract_svg(include_str!("../assets/characters/peep/laughing.svg")),
    );
    poses.insert(
        "whispering".into(),
        clean_or_extract_svg(include_str!("../assets/characters/peep/whispering.svg")),
    );

    CharacterPack {
        id: "peep".into(),
        name: "Peep".into(),
        author: "Ayvu Project".into(),
        license: "GPL-3.0-or-later".into(),
        description: "Cheerful curious avian cartoon character".into(),
        balloon_anchor: Anchor { x: 0.5, y: 0.35 },
        poses,
    }
}
