use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SearchFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct SearchSort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
