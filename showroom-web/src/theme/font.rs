use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Font {
    Arial,
    CourierNew,
    Georgia,
    LucidaConsole,
    Palatino,
    Times,
    Verdana,
    Garamond,
}

impl Font {
    pub const ALL: [Font; 8] = [
        Font::Arial,
        Font::CourierNew,
        Font::Georgia,
        Font::LucidaConsole,
        Font::Palatino,
        Font::Times,
        Font::Verdana,
        Font::Garamond,
    ];

    pub fn key(self) -> &'static str {
        match self {
            Font::Arial => "arial",
            Font::CourierNew => "courier-new",
            Font::Georgia => "georgia",
            Font::LucidaConsole => "lucida-console",
            Font::Palatino => "palatino",
            Font::Times => "times",
            Font::Verdana => "verdana",
            Font::Garamond => "garamond",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Font::Arial => "Arial",
            Font::CourierNew => "Courier New",
            Font::Georgia => "Georgia",
            Font::LucidaConsole => "Lucida Console",
            Font::Palatino => "Palatino",
            Font::Times => "Times",
            Font::Verdana => "Verdana",
            Font::Garamond => "Garamond",
        }
    }

    pub fn stack(self) -> &'static str {
        match self {
            Font::Arial => r#""Arial", "Helvetica", sans-serif"#,
            Font::CourierNew => r#""Courier New", "Courier", monospace"#,
            Font::Georgia => r#""Georgia", serif"#,
            Font::LucidaConsole => r#""Lucida Console", "Monaco", monospace"#,
            Font::Palatino => r#""Palatino Linotype", "Palatino", "Book Antiqua", serif"#,
            Font::Times => r#""Times", "Times New Roman", serif"#,
            Font::Verdana => r#""Verdana", "Geneva", sans-serif"#,
            Font::Garamond => r#""Garamond", "Baskerville", "Baskerville Old Face", serif"#,
        }
    }

    pub fn from_key(key: &str) -> Option<Font> {
        Font::ALL.into_iter().find(|font| font.key() == key)
    }
}

impl Serialize for Font {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.key())
    }
}

impl<'de> Deserialize<'de> for Font {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Font, D::Error> {
        let key = String::deserialize(deserializer)?;
        Font::from_key(&key).ok_or_else(|| serde::de::Error::custom(format!("unknown font {key:?}")))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Fonts {
    pub title: Font,
    pub body: Font,
}

impl Default for Fonts {
    fn default() -> Self {
        Fonts { title: Font::Georgia, body: Font::Times }
    }
}

impl Fonts {
    pub fn is_default(&self) -> bool {
        *self == Fonts::default()
    }
}
