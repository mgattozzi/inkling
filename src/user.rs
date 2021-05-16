use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum UserObject {
    Person {
        object: String,
        id: String,
        name: Option<String>,
        avatar_url: Option<String>,
        person: Person,
    },
    Bot {
        object: String,
        id: String,
        name: Option<String>,
        avatar_url: Option<String>,
        bot: Bot,
    },
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub email: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Bot {
    pub bot: Option<HashMap<(), ()>>,
}
