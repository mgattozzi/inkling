use crate::rich_text::RichTextObject;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseObject {
    id: String,
    created_time: DateTime<FixedOffset>,
    last_edited_time: DateTime<FixedOffset>,
    properties: HashMap<String, DatabaseProperty>,
    title: Vec<RichTextObject>,
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
    format: NumberFormat,
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
    options: Vec<SelectOptions>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SelectOptions {
    name: String,
    id: String,
    color: DatabaseColor,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MultiSelect {
    options: Vec<MultiSelectOptions>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MultiSelectOptions {
    name: String,
    id: String,
    color: DatabaseColor,
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
    expression: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Relation {
    database_id: String,
    synced_property_name: Option<String>,
    synced_property_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Rollup {
    relation_property_name: String,
    relation_property_id: String,
    rollup_property_name: String,
    rollup_property_id: String,
    function: Function,
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
    equals: Option<String>,
    does_not_equal: Option<String>,
    contains: Option<String>,
    does_not_contain: Option<String>,
    starts_with: Option<String>,
    ends_with: Option<String>,
    is_empty: Option<bool>,
    is_not_empty: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct NumberFilter {
    equals: Option<String>,
    does_not_equal: Option<String>,
    greater_than: Option<String>,
    less_than: Option<String>,
    greater_than_or_equal_to: Option<String>,
    less_than_or_equal_to: Option<String>,
    is_empty: Option<bool>,
    is_not_empty: Option<bool>,
}
#[derive(Serialize, Debug)]
pub struct CheckBoxFilter {
    equals: Option<String>,
    does_not_equal: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct SelectFilter {
    equals: Option<String>,
    does_not_equal: Option<String>,
    is_empty: Option<bool>,
    is_not_empty: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct MultiSelectFilter {
    contains: Option<String>,
    does_not_contain: Option<String>,
    is_empty: Option<bool>,
    is_not_empty: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct DateFilter {
    equals: Option<DateTime<FixedOffset>>,
    before: Option<DateTime<FixedOffset>>,
    after: Option<DateTime<FixedOffset>>,
    on_or_before: Option<DateTime<FixedOffset>>,
    on_or_after: Option<DateTime<FixedOffset>>,
    is_empty: Option<bool>,
    is_not_empty: Option<bool>,
    past_week: Option<HashMap<(), ()>>,
    past_month: Option<HashMap<(), ()>>,
    past_year: Option<HashMap<(), ()>>,
    next_week: Option<HashMap<(), ()>>,
    next_month: Option<HashMap<(), ()>>,
    next_year: Option<HashMap<(), ()>>,
}

#[derive(Serialize, Debug)]
pub struct PersonFilter {
    contains: Option<String>,
    does_not_contain: Option<String>,
    is_empty: Option<bool>,
    is_not_empty: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct FilesFilter {
    contains: Option<String>,
    does_not_contain: Option<String>,
    is_empty: Option<bool>,
    is_not_empty: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct RelationFilter {
    contains: Option<String>,
    does_not_contain: Option<String>,
    is_empty: Option<bool>,
    is_not_empty: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct FormulaFilter {
    text: Option<TextFilter>,
    checkbox: Option<CheckBoxFilter>,
    number: Option<NumberFilter>,
    date: Option<DateFilter>,
}

#[derive(Serialize, Debug)]
pub struct QueryDatabaseSort {
    property: Option<String>,
    timestamp: Option<QueryDatabaseTime>,
    direction: Option<Direction>,
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
