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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PanelWidth {
    Narrow,
    Standard,
    Wide,
    Panoramic,
    Custom(u32),
}

impl PanelWidth {
    pub fn to_pixels(&self) -> f64 {
        match self {
            PanelWidth::Narrow => 210.0,
            PanelWidth::Standard => 280.0,
            PanelWidth::Wide => 360.0,
            PanelWidth::Panoramic => 450.0,
            PanelWidth::Custom(px) => *px as f64,
        }
    }

    pub fn from_str_loose(s: &str) -> Option<Self> {
        let clean = s.trim().to_lowercase();
        match clean.as_str() {
            "narrow" | "estreito" | "small" | "pequeno" | "curto" => Some(PanelWidth::Narrow),
            "standard" | "padrao" | "padrão" | "normal" | "medium" | "medio" | "médio" => {
                Some(PanelWidth::Standard)
            }
            "wide" | "largo" | "grande" | "large" => Some(PanelWidth::Wide),
            "panoramic" | "panoramico" | "panorâmico" | "extra-wide" | "extrawide" => {
                Some(PanelWidth::Panoramic)
            }
            _ => {
                let num_str = clean.trim_end_matches("px").trim();
                if let Ok(val) = num_str.parse::<u32>() {
                    Some(PanelWidth::Custom(val.clamp(140, 800)))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PropType {
    Coffee,
    Book,
    Laptop,
    Erlenmeyer,
    Chalk,
    Stethoscope,
    Microphone,
    Custom(u32),
}

impl PropType {
    pub fn from_str_loose(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "cafe" | "café" | "coffee" | "xicara" | "xícara" | "mug" | "cup" => {
                Some(PropType::Coffee)
            }
            "livro" | "book" | "caderno" | "notebook_paper" | "leitura" => Some(PropType::Book),
            "laptop" | "notebook" | "computador" | "pc" => Some(PropType::Laptop),
            "erlenmeyer" | "quimica" | "química" | "laboratorio" | "frasco" | "potion" => {
                Some(PropType::Erlenmeyer)
            }
            "giz" | "chalk" => Some(PropType::Chalk),
            "estetoscopio" | "estetoscópio" | "stethoscope" | "medico" | "médico" => {
                Some(PropType::Stethoscope)
            }
            "microfone" | "mic" | "microphone" => Some(PropType::Microphone),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PropType::Coffee => "cafe",
            PropType::Book => "livro",
            PropType::Laptop => "laptop",
            PropType::Erlenmeyer => "erlenmeyer",
            PropType::Chalk => "giz",
            PropType::Stethoscope => "estetoscopio",
            PropType::Microphone => "microfone",
            PropType::Custom(_) => "custom",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropItem {
    pub prop_type: PropType,
    pub target: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Panel {
    pub id: usize,
    pub dialogues: Vec<DialogueLine>,
    #[serde(default)]
    pub effects: Vec<VisualEffect>,
    #[serde(default)]
    pub characters: Vec<String>,
    #[serde(default)]
    pub backdrop: Option<String>,
    #[serde(default)]
    pub width: Option<PanelWidth>,
    #[serde(default)]
    pub props: Vec<PropItem>,
}

impl Panel {
    pub fn resolve_width(&self) -> f64 {
        if let Some(w) = self.width {
            return w.to_pixels();
        }

        let mut distinct_speakers = self.characters.clone();
        for d in &self.dialogues {
            if !distinct_speakers
                .iter()
                .any(|c| c.eq_ignore_ascii_case(&d.speaker))
            {
                distinct_speakers.push(d.speaker.clone());
            }
        }
        let char_count = distinct_speakers.len();
        let total_chars: usize = self.dialogues.iter().map(|d| d.text.len()).sum();

        if char_count >= 4 {
            PanelWidth::Panoramic.to_pixels()
        } else if char_count == 3 {
            PanelWidth::Wide.to_pixels()
        } else if char_count == 2 {
            if total_chars > 90 {
                330.0
            } else {
                PanelWidth::Standard.to_pixels()
            }
        } else if char_count == 1 {
            if total_chars > 25 {
                PanelWidth::Standard.to_pixels()
            } else {
                PanelWidth::Narrow.to_pixels()
            }
        } else {
            if self.backdrop.is_some() || !self.effects.is_empty() {
                PanelWidth::Standard.to_pixels()
            } else {
                PanelWidth::Narrow.to_pixels()
            }
        }
    }
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
