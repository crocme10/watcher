use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Screaming Snake Case is GraphQL convention for enums.
// But the document must be written in lowercase (feels more natural)
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE", deserialize = "lowercase"))]
pub enum DocKind {
    Doc,
    Post,
}

// Screaming Snake Case is GraphQL convention for enums.
// But the document must be written in lowercase (feels more natural)
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE", deserialize = "lowercase"))]
pub enum DocGenre {
    Tutorial,
    Howto,
    Background,
    Reference,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub fullname: String,
    pub resource: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub title: String,
    pub resource: String,
    pub author: Author,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Front {
    pub title: String,
    #[serde(rename = "abstract")]
    pub outline: String,
    pub author: Author,
    pub tags: Vec<String>,
    pub image: Image,
    #[serde(default = "default_kind")]
    pub kind: DocKind,
    #[serde(default = "default_genre")]
    pub genre: DocGenre,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn default_kind() -> DocKind {
    DocKind::Doc
}

pub fn default_genre() -> DocGenre {
    DocGenre::Tutorial
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Doc {
    pub id: Uuid,
    pub front: Front,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleDocResponseBody {
    pub doc: Option<Doc>,
}

// I haven't found a way to have struct that can be both GraphQLInputObject and GraphQLObject.
// I would have like to use Doc to create a new document, but it doesn't work. So this is
// the I don't want to think about it solution...
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocSpec {
    pub id: Uuid,
    pub title: String,
    pub outline: String,
    pub author_fullname: String,
    pub author_resource: String,
    pub tags: Vec<String>,
    pub image_title: String,
    pub image_resource: String,
    pub image_author_fullname: String,
    pub image_author_resource: String,
    pub kind: DocKind,
    pub genre: DocGenre,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentRequestBody {
    pub doc: DocSpec,
}
