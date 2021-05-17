use crate::{rich_text::RichTextObject, time::Time};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BlockObject {
    Paragraph {
        id: String,
        created_time: Time,
        last_edited_time: Time,
        has_children: bool,
        paragraph: Paragraph,
    },
    #[serde(rename = "heading_1")]
    Heading1 {
        id: String,
        created_time: Time,
        last_edited_time: Time,
        has_children: bool,
        heading_1: Heading1,
    },
    #[serde(rename = "heading_2")]
    Heading2 {
        id: String,
        created_time: Time,
        last_edited_time: Time,
        has_children: bool,
        heading_2: Heading2,
    },
    #[serde(rename = "heading_3")]
    Heading3 {
        id: String,
        created_time: Time,
        last_edited_time: Time,
        has_children: bool,
        heading_3: Heading3,
    },
    BulletedListItem {
        id: String,
        created_time: Time,
        last_edited_time: Time,
        has_children: bool,
        bulleted_list_item: BulletedListItem,
    },
    ToDo {
        id: String,
        created_time: Time,
        last_edited_time: Time,
        has_children: bool,
        to_do: ToDo,
    },
    Toggle {
        id: String,
        created_time: Time,
        last_edited_time: Time,
        has_children: bool,
        toggle: Toggle,
    },
    ChildPage {
        id: String,
        created_time: Time,
        last_edited_time: Time,
        has_children: bool,
        child_page: ChildPage,
    },
    Unsupported {
        id: String,
        created_time: Time,
        last_edited_time: Time,
        has_children: bool,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Paragraph {
    pub text: Vec<RichTextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<BlockObject>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Heading1 {
    pub text: Vec<RichTextObject>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Heading2 {
    pub text: Vec<RichTextObject>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Heading3 {
    pub text: Vec<RichTextObject>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct BulletedListItem {
    pub text: Vec<RichTextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<BlockObject>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NumberedListItem {
    pub text: Vec<RichTextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<BlockObject>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ToDo {
    pub text: Vec<RichTextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<BlockObject>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Toggle {
    pub text: Vec<RichTextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<BlockObject>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ChildPage {
    pub title: String,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BlockObjectInput {
    Paragraph {
        object: String,
        paragraph: Paragraph,
    },
    #[serde(rename = "heading_1")]
    Heading1 {
        object: String,
        heading_1: Heading1,
    },
    #[serde(rename = "heading_2")]
    Heading2 {
        object: String,
        heading_2: Heading2,
    },
    #[serde(rename = "heading_3")]
    Heading3 {
        object: String,
        heading_3: Heading3,
    },
    BulletedListItem {
        object: String,
        bulleted_list_item: BulletedListItem,
    },
    ToDo {
        object: String,
        to_do: ToDo,
    },
    Toggle {
        object: String,
        toggle: Toggle,
    },
    ChildPage {
        object: String,
        child_page: ChildPage,
    },
}
