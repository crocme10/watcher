use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// enums need to be serialized in uppercase because the resulting string is
// sent to a GraphQL interface, which expects all enums to be uppercase. On
// the other hand, the deserialize is lowercase, because we expect the enum
// to be written in lowercase in the front matter of the markdown.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "lowercase"))]
pub enum DocKind {
    Doc,
    Post,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "lowercase"))]
pub enum DocGenre {
    Tutorial,
    Howto,
    Background,
    Reference,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Doc {
    pub id: Uuid,
    pub title: String,
    pub outline: String,
    pub author: String,
    pub tags: Vec<String>,
    pub image: String,
    pub kind: DocKind,
    pub genre: DocGenre,
    pub content: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Front {
    pub title: String,
    #[serde(rename = "abstract")]
    pub outline: String,
    pub author: String,
    pub tags: Vec<String>,
    pub image: String,
    #[serde(default = "default_kind")]
    pub kind: DocKind,
    #[serde(default = "default_genre")]
    pub genre: DocGenre,
    pub updated_at: DateTime<Utc>,
}

pub fn default_kind() -> DocKind {
    DocKind::Doc
}

pub fn default_genre() -> DocGenre {
    DocGenre::Tutorial
}
