use std::str::FromStr;

use ureq::serde::{Deserialize, Serialize};
use ureq::serde_json::Value;

use crate::responses::Format::{Anime, Manga, Novel};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Root {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "Page")]
    pub page: Page,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page {
    pub media: Vec<Medum>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Medum {
    pub format: Format,
    pub id: u64,
    #[serde(rename = "isLicensed")]
    pub is_licensed: bool,
    #[serde(rename = "mediaListEntry")]
    pub media_list_entry: Value,
    pub synonyms: Vec<String>,
    pub title: Title,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "MANGA")]
    Manga,
    #[serde(rename = "NOVEL")]
    Novel,
    #[serde(rename = "ANIME")]
    Anime,
    #[serde(rename = "ONE_SHOT")]
    OneShot,
    #[default]
    Unknown,
}

impl FromStr for Format {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "MANGA" => Manga,
            "NOVEL" => Novel,
            "Anime" => Anime,
            unknown => panic!("{unknown}"),
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Title {
    pub english: Option<String>,
    pub native: Option<String>,
    pub romaji: Option<String>,
    #[serde(rename = "userPreferred")]
    pub user_preferred: String,
}
