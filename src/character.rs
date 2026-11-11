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

fn default_hand_anchor() -> Anchor {
    Anchor { x: 0.65, y: 0.58 }
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
    #[serde(default = "default_hand_anchor")]
    pub hand_anchor: Anchor,
    #[serde(default)]
    pub pose_hand_anchors: HashMap<String, Anchor>,
    pub poses: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterMetadata {
    pub id: String,
    pub name: String,
    pub author: String,
    pub license: String,
    pub description: String,
    pub width: f64,
    pub height: f64,
    pub balloon_anchor: Anchor,
    pub hand_anchor: Anchor,
    pub emotions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDetail {
    pub id: String,
    pub name: String,
    pub author: String,
    pub license: String,
    pub description: String,
    pub width: f64,
    pub height: f64,
    pub balloon_anchor: Anchor,
    pub hand_anchor: Anchor,
    pub emotions: Vec<String>,
    pub poses: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct CharacterPack {
    pub id: String,
    pub name: String,
    pub author: String,
    pub license: String,
    pub description: String,
    pub balloon_anchor: Anchor,
    pub hand_anchor: Anchor,
    pub pose_hand_anchors: HashMap<String, Anchor>,
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
            Emotion::Surprised => "scared",
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
            "arti" => (157.0, 396.0),
            "capivara" => (180.0, 360.0),
            "jacare" => (190.0, 380.0),
            "sombra" | "billi" | "aly" | "lu" | "raio" => (160.0, 440.0),
            _ => (160.0, 440.0),
        }
    }

    pub fn get_hand_anchor(&self, emotion: &Emotion) -> Anchor {
        let key = emotion.as_str();
        if let Some(anchor) = self.pose_hand_anchors.get(key) {
            *anchor
        } else {
            self.hand_anchor
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
        // 1. Discover external character packs from XDG and local runtime directories
        for pack in crate::assets::discover_external_characters() {
            registry.register_pack(pack);
        }
        // 2. Load embedded official presets for any characters not overridden on disk
        for pack in load_embedded_presets() {
            if !registry.contains_id(&pack.id) {
                registry.register_pack(pack);
            }
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
                let (width, height) = pack.dimensions();
                list.push(CharacterMetadata {
                    id: pack.id.clone(),
                    name: pack.name.clone(),
                    author: pack.author.clone(),
                    license: pack.license.clone(),
                    description: pack.description.clone(),
                    width,
                    height,
                    balloon_anchor: pack.balloon_anchor,
                    hand_anchor: pack.hand_anchor,
                    emotions,
                });
            }
        }
        list
    }

    pub fn get_detail(&self, id: &str) -> Option<CharacterDetail> {
        if let Some(pack) = self.get(id) {
            let (width, height) = pack.dimensions();
            let mut emotions: Vec<String> = pack.poses.keys().cloned().collect();
            emotions.sort();
            Some(CharacterDetail {
                id: pack.id.clone(),
                name: pack.name.clone(),
                author: pack.author.clone(),
                license: pack.license.clone(),
                description: pack.description.clone(),
                width,
                height,
                balloon_anchor: pack.balloon_anchor,
                hand_anchor: pack.hand_anchor,
                emotions,
                poses: pack.poses.clone(),
            })
        } else {
            None
        }
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
            let unaccented = speaker_lower.replace('é', "e").replace('á', "a");
            if id == &speaker_lower
                || pack.name.to_lowercase() == speaker_lower
                || id == &unaccented
                || pack.name.to_lowercase().replace('é', "e") == unaccented
            {
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
                                    hand_anchor: manifest.hand_anchor,
                                    pose_hand_anchors: manifest.pose_hand_anchors,
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
    crate::assets::load_official_packs()
}
