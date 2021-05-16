use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SearchFilter {
    pub value: Option<String>,
    pub property: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct SearchSort {
    pub direction: Option<Direction>,
    pub timestamp: Option<SearchTime>,
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
