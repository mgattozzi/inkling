use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SearchFilter {
    value: Option<String>,
    property: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct SearchSort {
    direction: Option<Direction>,
    timestamp: Option<SearchTime>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Ascending,
    Descending,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SearchTime {
    LastEditedTime,
}
