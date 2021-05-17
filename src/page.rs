use crate::{rich_text::RichTextObject, time::Time, user::UserObject};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct PageObject {
    pub id: String,
    pub created_time: Time,
    pub last_edited_time: Time,
    pub archived: bool,
    pub parent: Parent,
    pub properties: HashMap<String, PageProperty>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Parent {
    DatabaseId { database_id: String },
    PageId { page_id: String },
    Workspace,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum PageProperty {
    RichText {
        id: String,
        rich_text: Vec<RichTextObject>,
    },
    Number {
        id: String,
        number: f64,
    },
    Select {
        id: String,
        select: PageSelectOptions,
    },
    MultiSelect {
        id: String,
        multi_select: Vec<PageMultiSelectOptions>,
    },
    Date {
        id: String,
        date: PageDate,
    },
    Formula {
        id: String,
        formula: PageFormula,
    },
    Relation {
        id: String,
        relation: Vec<PageRelation>,
    },
    Rollup {
        id: String,
        rollup: PageRollup,
    },
    Title {
        id: String,
        title: Vec<RichTextObject>,
    },
    People {
        id: String,
        people: Vec<UserObject>,
    },
    Files {
        id: String,
        files: Vec<PageFileReference>,
    },
    Checkbox {
        id: String,
        checkbox: bool,
    },
    Url {
        id: String,
        url: String,
    },
    Email {
        id: String,
        email: String,
    },
    PhoneNumber {
        id: String,
        phone_number: String,
    },
    CreatedTime {
        id: String,
        created_time: Time,
    },
    CreatedBy {
        id: String,
        created_by: UserObject,
    },
    LastEditedTime {
        id: String,
        last_edited_time: Time,
    },
    LastEditedBy {
        id: String,
        last_edited_by: UserObject,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageFileReference {
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PageDate {
    pub start: Time,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Time>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum PageFormula {
    String {
        #[serde(skip_serializing_if = "Option::is_none")]
        string: Option<String>,
    },
    Number {
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<f64>,
    },
    Boolean {
        #[serde(skip_serializing_if = "Option::is_none")]
        boolean: Option<bool>,
    },
    Date {
        date: Time,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageRelation {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum PageRollup {
    Number {
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<f64>,
    },
    Date {
        date: Time,
    },
    Array {
        array: Vec<PageRollupProperty>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum PageRollupProperty {
    RichText {
        rich_text: Vec<RichTextObject>,
    },
    Number {
        number: f64,
    },
    Select {
        select: PageSelectOptions,
    },
    MultiSelect {
        multi_select: Vec<PageMultiSelectOptions>,
    },
    Date {
        date: PageDate,
    },
    Formula {
        formula: PageFormula,
    },
    Relation {
        relation: Vec<PageRelation>,
    },
    Rollup {
        rollup: PageRollup,
    },
    Title {
        title: Vec<RichTextObject>,
    },
    People {
        people: Vec<UserObject>,
    },
    Files {
        files: Vec<PageFileReference>,
    },
    Checkbox {
        checkbox: bool,
    },
    Url {
        url: String,
    },
    Email {
        email: String,
    },
    PhoneNumber {
        phone_number: String,
    },
    CreatedTime {
        created_time: Time,
    },
    CreatedBy {
        created_by: UserObject,
    },
    LastEditedTime {
        last_edited_time: Time,
    },
    LastEditedBy {
        last_edited_by: UserObject,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageSelectOptions {
    pub name: String,
    pub id: String,
    pub color: PageColor,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PageMultiSelectOptions {
    pub name: String,
    pub id: String,
    pub color: PageColor,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PageColor {
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
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PagePropertyValue {
    RichText {
        rich_text: Vec<RichTextObject>,
    },
    Number {
        number: f64,
    },
    Select {
        select: PageSelectOptions,
    },
    MultiSelect {
        multi_select: Vec<PageMultiSelectOptions>,
    },
    Date {
        date: PageDate,
    },
    Formula {
        formula: PageFormula,
    },
    Relation {
        relation: Vec<PageRelation>,
    },
    Rollup {
        rollup: PageRollup,
    },
    Title {
        title: Vec<RichTextObject>,
    },
    People {
        people: Vec<UserObject>,
    },
    Files {
        files: Vec<PageFileReference>,
    },
    Checkbox {
        checkbox: bool,
    },
    Url {
        url: String,
    },
    Email {
        email: String,
    },
    PhoneNumber {
        phone_number: String,
    },
    CreatedTime {
        created_time: Time,
    },
    CreatedBy {
        created_by: UserObject,
    },
    LastEditedTime {
        last_edited_time: Time,
    },
    LastEditedBy {
        last_edited_by: UserObject,
    },
}
