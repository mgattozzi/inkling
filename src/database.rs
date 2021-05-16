use crate::rich_text::RichTextObject;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseObject {
    pub id: String,
    pub created_time: DateTime<FixedOffset>,
    pub last_edited_time: DateTime<FixedOffset>,
    pub properties: HashMap<String, DatabaseProperty>,
    pub title: Vec<RichTextObject>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum DatabaseProperty {
    Title {
        id: String,
        title: Option<HashMap<(), ()>>,
    },
    RichText {
        id: String,
        rich_text: Option<HashMap<(), ()>>,
    },
    Number {
        id: String,
        number: Number,
    },
    Select {
        id: String,
        select: Select,
    },
    MultiSelect {
        id: String,
        multi_select: MultiSelect,
    },
    Date {},
    People {
        id: String,
        people: Option<HashMap<(), ()>>,
    },
    Files {
        id: String,
        files: Option<HashMap<(), ()>>,
    },
    Checkbox {
        id: String,
        checkbox: Option<HashMap<(), ()>>,
    },
    Url {
        id: String,
        url: Option<HashMap<(), ()>>,
    },
    Email {
        id: String,
        email: Option<HashMap<(), ()>>,
    },
    PhoneNumber {
        id: String,
        phone_number: Option<HashMap<(), ()>>,
    },
    Formula {
        id: String,
        formula: Expression,
    },
    Relation {
        id: String,
        relation: Option<HashMap<(), ()>>,
    },
    Rollup {
        id: String,
        rollup: Option<HashMap<(), ()>>,
    },
    CreatedTime {
        id: String,
        created_time: Option<HashMap<(), ()>>,
    },
    CreatedBy {
        id: String,
        created_by: Option<HashMap<(), ()>>,
    },
    LastEditedTime {
        id: String,
        last_edited_time: Option<HashMap<(), ()>>,
    },
    LastEditedBy {
        id: String,
        last_edited_by: Option<HashMap<(), ()>>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Number {
    pub format: NumberFormat,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum NumberFormat {
    Number,
    NumberWithCommas,
    Percent,
    Dollar,
    Euro,
    Pound,
    Yen,
    Ruble,
    Rupee,
    Won,
    Yuan,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Select {
    pub options: Vec<SelectOptions>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SelectOptions {
    pub name: String,
    pub id: String,
    pub color: DatabaseColor,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MultiSelect {
    pub options: Vec<MultiSelectOptions>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MultiSelectOptions {
    pub name: String,
    pub id: String,
    pub color: DatabaseColor,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseColor {
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
pub struct Expression {
    pub expression: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Relation {
    pub database_id: String,
    pub synced_property_name: Option<String>,
    pub synced_property_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Rollup {
    pub relation_property_name: String,
    pub relation_property_id: String,
    pub rollup_property_name: String,
    pub rollup_property_id: String,
    pub function: Function,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Function {
    CountAll,
    CountValues,
    CountUniqueValues,
    CountEmpty,
    CountNotEmpty,
    PercentEmpty,
    PercentNotEmpty,
    Sum,
    Average,
    Median,
    Min,
    Max,
    Range,
}

#[derive(Serialize, Debug)]
pub enum QueryDatabaseFilter {
    Or {
        or: Vec<QueryDatabaseFilter>,
    },
    And {
        and: Vec<QueryDatabaseFilter>,
    },
    Title {
        property: String,
        title: TextFilter,
    },
    RichText {
        property: String,
        rich_text: TextFilter,
    },
    Url {
        property: String,
        url: TextFilter,
    },
    Email {
        property: String,
        email: TextFilter,
    },
    Phone {
        property: String,
        fhone: TextFilter,
    },
    Number {
        property: String,
        number: NumberFilter,
    },
    CheckBoxFilter {
        property: String,
        check_box: CheckBoxFilter,
    },
    SelectFilter {
        property: String,
        select: SelectFilter,
    },
    MultiSelectFilter {
        property: String,
        multi_select: MultiSelectFilter,
    },
    Date {
        property: String,
        date: DateFilter,
    },
    CreatedTime {
        property: String,
        created_time: DateFilter,
    },
    LastEditedTime {
        property: String,
        created_time: DateFilter,
    },
    PersonDate {
        property: String,
        date: PersonFilter,
    },
    CreatedBy {
        property: String,
        created_by: PersonFilter,
    },
    LastEditedBy {
        property: String,
        created_by: PersonFilter,
    },
    Files {
        property: String,
        files: FilesFilter,
    },
    Relation {
        property: String,
        relation: RelationFilter,
    },
    Formula {
        property: String,
        formula: FormulaFilter,
    },
}

#[derive(Serialize, Debug)]
pub struct TextFilter {
    pub equals: Option<String>,
    pub does_not_equal: Option<String>,
    pub contains: Option<String>,
    pub does_not_contain: Option<String>,
    pub starts_with: Option<String>,
    pub ends_with: Option<String>,
    pub is_empty: Option<bool>,
    pub is_not_empty: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct NumberFilter {
    pub equals: Option<String>,
    pub does_not_equal: Option<String>,
    pub greater_than: Option<String>,
    pub less_than: Option<String>,
    pub greater_than_or_equal_to: Option<String>,
    pub less_than_or_equal_to: Option<String>,
    pub is_empty: Option<bool>,
    pub is_not_empty: Option<bool>,
}
#[derive(Serialize, Debug)]
pub struct CheckBoxFilter {
    pub equals: Option<String>,
    pub does_not_equal: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct SelectFilter {
    pub equals: Option<String>,
    pub does_not_equal: Option<String>,
    pub is_empty: Option<bool>,
    pub is_not_empty: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct MultiSelectFilter {
    pub contains: Option<String>,
    pub does_not_contain: Option<String>,
    pub is_empty: Option<bool>,
    pub is_not_empty: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct DateFilter {
    pub equals: Option<DateTime<FixedOffset>>,
    pub before: Option<DateTime<FixedOffset>>,
    pub after: Option<DateTime<FixedOffset>>,
    pub on_or_before: Option<DateTime<FixedOffset>>,
    pub on_or_after: Option<DateTime<FixedOffset>>,
    pub is_empty: Option<bool>,
    pub is_not_empty: Option<bool>,
    pub past_week: Option<HashMap<(), ()>>,
    pub past_month: Option<HashMap<(), ()>>,
    pub past_year: Option<HashMap<(), ()>>,
    pub next_week: Option<HashMap<(), ()>>,
    pub next_month: Option<HashMap<(), ()>>,
    pub next_year: Option<HashMap<(), ()>>,
}

#[derive(Serialize, Debug)]
pub struct PersonFilter {
    pub contains: Option<String>,
    pub does_not_contain: Option<String>,
    pub is_empty: Option<bool>,
    pub is_not_empty: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct FilesFilter {
    pub contains: Option<String>,
    pub does_not_contain: Option<String>,
    pub is_empty: Option<bool>,
    pub is_not_empty: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct RelationFilter {
    pub contains: Option<String>,
    pub does_not_contain: Option<String>,
    pub is_empty: Option<bool>,
    pub is_not_empty: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct FormulaFilter {
    pub text: Option<TextFilter>,
    pub checkbox: Option<CheckBoxFilter>,
    pub number: Option<NumberFilter>,
    pub date: Option<DateFilter>,
}

#[derive(Serialize, Debug)]
pub struct QueryDatabaseSort {
    pub property: Option<String>,
    pub timestamp: Option<QueryDatabaseTime>,
    pub direction: Option<Direction>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryDatabaseTime {
    CreatedTime,
    LastEditedTime,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Ascending,
    Descending,
}
