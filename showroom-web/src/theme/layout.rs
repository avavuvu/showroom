use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Layout {
    #[default]
    Default,
    Compact,
    Centred,
    Letter,
}

impl Layout {
    pub const ALL: [Layout; 4] = [Layout::Default, Layout::Compact, Layout::Centred, Layout::Letter];

    pub fn key(self) -> &'static str {
        match self {
            Layout::Default => "default",
            Layout::Compact => "compact",
            Layout::Centred => "centred",
            Layout::Letter => "letter",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Layout::Default => "Default",
            Layout::Compact => "Compact",
            Layout::Centred => "Centred",
            Layout::Letter => "Letter",
        }
    }

    pub fn from_key(key: &str) -> Option<Layout> {
        Layout::ALL.into_iter().find(|layout| layout.key() == key)
    }

    pub fn is_default(&self) -> bool {
        *self == Layout::Default
    }

    pub(super) fn css(self) -> &'static str {
        match self {
            Layout::Default => "",
            Layout::Compact => {
                "--prose-font-size:0.875rem;--prose-h1-size:1em;--prose-h2-size:1em;--prose-h3-size:1em;--article-header-align:right;"
            }
            Layout::Centred => {
                "--article-header-align:center;--article-date-align:center;--article-header-rule:block;--prose-heading-align:center;"
            }
            Layout::Letter => "--article-title-size:1em;--article-date-display:none;--article-handle-display:none;",
        }
    }
}
