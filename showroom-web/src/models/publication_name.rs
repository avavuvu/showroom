use sea_orm::{DeriveActiveEnum, EnumIter, sea_query::StringLen};
use serde::Deserialize;
use strum::{AsRefStr, EnumMessage, EnumString};

pub const MAX_LABEL_LEN: usize = 40;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, EnumIter, AsRefStr, EnumString, EnumMessage)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum Article {
    #[default]
    #[strum(message = "–")]
    None,
    #[strum(message = "A")]
    A,
    #[strum(message = "An")]
    An,
    #[strum(message = "The")]
    The,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, EnumIter, AsRefStr, EnumMessage)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Possessive {
    #[default]
    #[strum(message = "\u{2019}s")]
    Apostrophes,
    #[strum(message = "\u{2019}")]
    Apostrophe,
    #[strum(message = "–")]
    None,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, EnumIter, AsRefStr, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum Noun {
    Week,
    Month,
    Archive,
    Blog,
    Forum,
    Home,
    House,
    Journal,
    Page,
    #[default]
    Room,
    Site,
    Weblog,
    Zone,
    #[strum(message = "–")]
    None,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, EnumIter, DeriveActiveEnum, AsRefStr, EnumMessage)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(16))", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum NameCase {
    #[strum(message = "Sentence case")]
    Sentence,
    #[strum(message = "Title Case")]
    Title,
    #[strum(message = "UPPERCASE")]
    Upper,
    #[default]
    #[strum(message = "As typed")]
    Typed,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PublicationName {
    pub article: Article,
    pub label: String,
    pub possessive: Possessive,
    pub noun: Noun,
}

impl Possessive {
    fn suffix(self) -> &'static str {
        match self {
            Possessive::Apostrophes => "\u{2019}s",
            Possessive::Apostrophe => "\u{2019}",
            Possessive::None => "",
        }
    }
}

impl PublicationName {
    pub fn for_slug(slug: &str) -> Self {
        Self { label: slug.to_string(), ..Self::default() }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        match self.label.chars().count() {
            0 => Err("A name is required"),
            length if length > MAX_LABEL_LEN => Err("The name is too long"),
            _ => Ok(()),
        }
    }

    pub fn render(&self, case: NameCase) -> String {
        let typed = self.typed();
        match case {
            NameCase::Typed => typed,
            NameCase::Sentence => capitalize(&typed),
            NameCase::Title => typed.split(' ').map(capitalize).collect::<Vec<_>>().join(" "),
            NameCase::Upper => typed.to_uppercase(),
        }
    }

    pub fn parse(name: &str) -> Self {
        let mut words: Vec<&str> = name.split_whitespace().collect();

        let article = words.first().and_then(|word| word.parse().ok()).filter(|article| *article != Article::None && words.len() > 2);
        if article.is_some() {
            words.remove(0);
        }

        let noun = words
            .last()
            .and_then(|word| word.parse().ok())
            .filter(|noun| *noun != Noun::None && words.len() > 1);
        if noun.is_some() {
            words.pop();
        }

        let label = words.join(" ");
        let (label, possessive) = match label.strip_suffix(['s', 'S']).and_then(|rest| rest.strip_suffix(['\u{2019}', '\''])) {
            Some(rest) => (rest, Possessive::Apostrophes),
            None => match label.strip_suffix(['\u{2019}', '\'']) {
                Some(rest) => (rest, Possessive::Apostrophe),
                None => (label.as_str(), Possessive::None),
            },
        };

        Self {
            article: article.unwrap_or_default(),
            label: label.to_string(), possessive,
            noun: noun.unwrap_or(Noun::None),
        }
    }

    fn typed(&self) -> String {
        let label = format!("{}{}", self.label, self.possessive.suffix());
        let article = (self.article != Article::None).then_some(self.article.as_ref());
        let noun = (self.noun != Noun::None).then_some(self.noun.as_ref());
        [article, Some(label.as_str()), noun].into_iter().flatten().collect::<Vec<_>>().join(" ")
    }
}

fn capitalize(text: &str) -> String {
    let mut characters = text.chars();
    characters.next().map(|first| first.to_uppercase().chain(characters).collect()).unwrap_or_default()
}
