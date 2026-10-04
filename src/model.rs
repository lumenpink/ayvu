use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Emotion {
    Neutral,
    Happy,
    Laughing,
    Sad,
    Crying,
    Angry,
    Shouting,
    Thinking,
    Skeptical,
    Surprised,
    Custom(String),
}

impl Emotion {
    pub fn from_str_loose(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "neutral" | "neutro" | "normal" | "default" => Emotion::Neutral,
            "happy" | "alegre" | "feliz" | "smile" => Emotion::Happy,
            "laughing" | "rindo" | "gargalhada" | "lol" | "riso" => Emotion::Laughing,
            "sad" | "triste" | "down" => Emotion::Sad,
            "crying" | "chorando" | "pranto" | "choro" => Emotion::Crying,
            "angry" | "bravo" | "zangado" | "furious" | "furia" => Emotion::Angry,
            "shouting" | "gritando" | "grito" | "shout" => Emotion::Shouting,
            "thinking" | "pensando" | "pensativo" | "thought" => Emotion::Thinking,
            "skeptical" | "cético" | "cetico" | "duvidoso" | "sus" => Emotion::Skeptical,
            "surprised" | "surpreso" | "espantado" | "wow" => Emotion::Surprised,
            other => Emotion::Custom(other.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Emotion::Neutral => "neutral",
            Emotion::Happy => "happy",
            Emotion::Laughing => "laughing",
            Emotion::Sad => "sad",
            Emotion::Crying => "crying",
            Emotion::Angry => "angry",
            Emotion::Shouting => "shouting",
            Emotion::Thinking => "thinking",
            Emotion::Skeptical => "skeptical",
            Emotion::Surprised => "surprised",
            Emotion::Custom(s) => s.as_str(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BalloonType {
    Speech,
    Thought,
    Shout,
    Whisper,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StripLayout {
    #[default]
    Horizontal,
    Grid,
    Vertical,
}

impl StripLayout {
    pub fn from_str_loose(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "grid" | "2x2" | "quad" => StripLayout::Grid,
            "vertical" | "webtoon" | "column" => StripLayout::Vertical,
            _ => StripLayout::Horizontal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StickerType {
    Sweat,
    Shock,
    Heart,
    Idea,
    Question,
}

impl StickerType {
    pub fn from_str_loose(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "sweat" | "gota" => Some(StickerType::Sweat),
            "shock" | "raio" | "alert" | "exclamation" => Some(StickerType::Shock),
            "heart" | "coracao" | "love" => Some(StickerType::Heart),
            "idea" | "lampada" | "eureka" => Some(StickerType::Idea),
            "question" | "duvida" | "interrogacao" => Some(StickerType::Question),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            StickerType::Sweat => "sweat",
            StickerType::Shock => "shock",
            StickerType::Heart => "heart",
            StickerType::Idea => "idea",
            StickerType::Question => "question",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisualEffect {
    Onomatopoeia(String),
    Sticker(StickerType),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogueLine {
    pub speaker: String,
    pub emotion: Emotion,
    pub balloon_type: BalloonType,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Panel {
    pub id: usize,
    pub dialogues: Vec<DialogueLine>,
    #[serde(default)]
    pub effects: Vec<VisualEffect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComicStrip {
    pub title: Option<String>,
    #[serde(default)]
    pub layout: StripLayout,
    #[serde(default)]
    pub backdrop: Option<String>,
    pub panels: Vec<Panel>,
}

