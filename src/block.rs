use crate::rich_text::RichTextObject;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BlockObject {
    Paragraph {
        object: String,
        id: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
        paragraph: Paragraph,
    },
    Heading1 {
        object: String,
        id: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
        heading_1: Heading1,
    },
    Heading2 {
        object: String,
        id: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
        heading_2: Heading2,
    },
    Heading3 {
        object: String,
        id: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
        heading_3: Heading3,
    },
    BulletedListItem {
        object: String,
        id: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
        bulleted_list_item: BulletedListItem,
    },
    ToDo {
        object: String,
        id: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
        to_do: ToDo,
    },
    Toggle {
        object: String,
        id: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
        toggle: Toggle,
    },
    ChildPage {
        object: String,
        id: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
        child_page: ChildPage,
    },
    Unsupported {
        object: String,
        id: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Paragraph {
    text: Vec<RichTextObject>,
    children: Vec<BlockObject>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Heading1 {
    text: Vec<RichTextObject>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Heading2 {
    text: Vec<RichTextObject>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Heading3 {
    text: Vec<RichTextObject>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct BulletedListItem {
    text: Vec<RichTextObject>,
    children: Vec<BlockObject>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NumberedListItem {
    text: Vec<RichTextObject>,
    children: Vec<BlockObject>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ToDo {
    text: Vec<RichTextObject>,
    checked: Option<bool>,
    children: Vec<BlockObject>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Toggle {
    text: Vec<RichTextObject>,
    children: Vec<BlockObject>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ChildPage {
    title: String,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BlockObjectInput {
    Paragraph {
        object: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
        paragraph: Paragraph,
    },
    Heading1 {
        object: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
        heading_1: Heading1,
    },
    Heading2 {
        object: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
        heading_2: Heading2,
    },
    Heading3 {
        object: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
        heading_3: Heading3,
    },
    BulletedListItem {
        object: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
        bulleted_list_item: BulletedListItem,
    },
    ToDo {
        object: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
        to_do: ToDo,
    },
    Toggle {
        object: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
        toggle: Toggle,
    },
    ChildPage {
        object: String,
        created_time: DateTime<FixedOffset>,
        last_edited_time: DateTime<FixedOffset>,
        has_children: bool,
        child_page: ChildPage,
    },
}