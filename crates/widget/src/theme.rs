use std::{fmt, fs, path::Path};

use gpui::{Rgba, rgba};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

/// A color encoded in theme JSON as `#RRGGBB` or `#RRGGBBAA`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThemeColor(u32);

impl ThemeColor {
    pub const fn rgb(value: u32) -> Self {
        Self((value << 8) | 0xff)
    }

    pub const fn rgba(value: u32) -> Self {
        Self(value)
    }

    pub fn gpui(self) -> Rgba {
        rgba(self.0)
    }
}

impl Serialize for ThemeColor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("#{:08X}", self.0))
    }
}

impl<'de> Deserialize<'de> for ThemeColor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        let hex = raw.strip_prefix('#').unwrap_or(&raw);
        let value = u32::from_str_radix(hex, 16)
            .map_err(|_| de::Error::custom("color must be #RRGGBB or #RRGGBBAA"))?;
        match hex.len() {
            6 => Ok(Self::rgb(value)),
            8 => Ok(Self::rgba(value)),
            _ => Err(de::Error::custom("color must be #RRGGBB or #RRGGBBAA")),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct TabBarTheme {
    pub background: ThemeColor,
    pub border: ThemeColor,
    pub tab_background: ThemeColor,
    pub tab_hover_background: ThemeColor,
    pub tab_active_background: ThemeColor,
    pub text: ThemeColor,
    pub icon: ThemeColor,
    pub close_hover_background: ThemeColor,
}

impl Default for TabBarTheme {
    fn default() -> Self {
        Self {
            background: ThemeColor::rgb(0xf8fafc),
            border: ThemeColor::rgb(0xcbd5e1),
            tab_background: ThemeColor::rgba(0x00000000),
            tab_hover_background: ThemeColor::rgb(0xe2e8f0),
            tab_active_background: ThemeColor::rgb(0xffffff),
            text: ThemeColor::rgb(0x0f172a),
            icon: ThemeColor::rgb(0x475569),
            close_hover_background: ThemeColor::rgb(0xcbd5e1),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct TextFieldTheme {
    pub background: ThemeColor,
    pub hover_background: ThemeColor,
    pub disabled_background: ThemeColor,
    pub text: ThemeColor,
    pub placeholder: ThemeColor,
    pub border: ThemeColor,
    pub focused_border: ThemeColor,
    pub disabled_border: ThemeColor,
    pub selection_background: ThemeColor,
}

impl Default for TextFieldTheme {
    fn default() -> Self {
        Self {
            background: ThemeColor::rgb(0xffffff),
            hover_background: ThemeColor::rgb(0xf8fafc),
            disabled_background: ThemeColor::rgb(0xe2e8f0),
            text: ThemeColor::rgb(0x0f172a),
            placeholder: ThemeColor::rgba(0x737373cc),
            border: ThemeColor::rgb(0x64748b),
            focused_border: ThemeColor::rgb(0x2563eb),
            disabled_border: ThemeColor::rgba(0x94a3b8aa),
            selection_background: ThemeColor::rgba(0x3b82f64d),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct TextEditorTheme {
    #[serde(flatten)]
    pub field: TextFieldTheme,
    pub scrollbar_track: ThemeColor,
    pub scrollbar_thumb: ThemeColor,
    pub scrollbar_thumb_hover: ThemeColor,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct DataGridTheme {
    pub header_background: ThemeColor,
    pub row_background: ThemeColor,
    pub text: ThemeColor,
    pub divider: ThemeColor,
    pub scrollbar_track: ThemeColor,
    pub scrollbar_thumb: ThemeColor,
    pub scrollbar_thumb_hover: ThemeColor,
}

impl Default for DataGridTheme {
    fn default() -> Self {
        Self {
            header_background: ThemeColor::rgb(0xf1f5f9),
            row_background: ThemeColor::rgb(0xffffff),
            text: ThemeColor::rgb(0x475569),
            divider: ThemeColor::rgb(0xcbd5e1),
            scrollbar_track: ThemeColor::rgb(0xe2e8f0),
            scrollbar_thumb: ThemeColor::rgb(0x94a3b8),
            scrollbar_thumb_hover: ThemeColor::rgb(0x64748b),
        }
    }
}

impl Default for TextEditorTheme {
    fn default() -> Self {
        Self {
            field: TextFieldTheme::default(),
            scrollbar_track: ThemeColor::rgba(0x0f172a14),
            scrollbar_thumb: ThemeColor::rgba(0x64748baa),
            scrollbar_thumb_hover: ThemeColor::rgb(0x64748b),
        }
    }
}

/// Visual settings shared by all widgets. Missing JSON properties use defaults.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Theme {
    pub tab_bar: TabBarTheme,
    pub text_input: TextFieldTheme,
    pub text_editor: TextEditorTheme,
    pub data_grid: DataGridTheme,
}

#[derive(Debug)]
pub enum ThemeLoadError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for ThemeLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "could not read theme: {e}"),
            Self::Json(e) => write!(f, "invalid theme JSON: {e}"),
        }
    }
}

impl std::error::Error for ThemeLoadError {}

impl Theme {
    pub fn from_json(json: &str) -> Result<Self, ThemeLoadError> {
        serde_json::from_str(json).map_err(ThemeLoadError::Json)
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, ThemeLoadError> {
        let json = fs::read_to_string(path).map_err(ThemeLoadError::Io)?;
        Self::from_json(&json)
    }
}
