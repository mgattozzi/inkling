use crate::user::UserObject;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RichTextObject {
    Text {
        plain_text: String,
        href: Option<String>,
        annotations: Annotations,
        text: Text,
    },
    Mention {
        plain_text: String,
        href: Option<String>,
        annotations: Annotations,
        mention: Mention,
    },
    Equation {
        plain_text: String,
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
    id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MentionDate {
    start: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Equation {
    expression: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    content: String,
    link: Option<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    r#type: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Annotations {
    bold: bool,
    italic: bool,
    strikethrough: bool,
    underline: bool,
    code: bool,
    color: RichTextColor,
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
