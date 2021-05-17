use crate::{time::Time, user::UserObject};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RichTextObject {
    Text {
        plain_text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        href: Option<String>,
        annotations: Annotations,
        text: Text,
    },
    Mention {
        plain_text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        href: Option<String>,
        annotations: Annotations,
        mention: Mention,
    },
    Equation {
        plain_text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        href: Option<String>,
        annotations: Annotations,
        equation: Equation,
    },
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Mention {
    User { user: UserObject },
    Page { page: Page },
    Database { database: Database },
    Date { page: MentionDate },
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MentionDate {
    pub start: Time,
    pub end: Time,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Equation {
    pub expression: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub r#type: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Annotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: RichTextColor,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RichTextColor {
    Default,
    Gray,
    Brown,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Red,
    GrayBackground,
    BrownBackground,
    OrangeBackground,
    YellowBackground,
    GreenBackground,
    BlueBackground,
    PurpleBackground,
    PinkBackground,
    RedBackground,
}
