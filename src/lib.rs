pub mod block;
pub mod database;
pub mod error;
pub mod page;
pub mod rich_text;
pub mod search;
pub mod user;

use self::{
    block::{BlockObject, BlockObjectInput},
    database::{DatabaseObject, QueryDatabaseFilter, QueryDatabaseSort},
    error::ErrorObject,
    page::{PageObject, PagePropertyValue, Parent},
    search::{SearchFilter, SearchSort},
    user::UserObject,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, future::Future};

const BASE_URL: &'static str = "https://api.notion.com/v1/";
pub struct Client {
    reqwest: reqwest::Client,
    auth_token: String,
}

impl Client {
    const NOTION_VERSION: &'static str = "2021-05-13";
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            auth_token: token.into(),
        }
    }

    fn get(&self, url: String) -> impl Future<Output = Result<NotionObject, Box<dyn Error>>> + '_ {
        async move {
            Ok(serde_json::from_str(
                &self
                    .reqwest
                    .get(url)
                    .header("Notion-Version", Self::NOTION_VERSION)
                    .header("Content-Type", "application/json")
                    .bearer_auth(&self.auth_token)
                    .send()
                    .await?
                    .text()
                    .await?,
            )?)
        }
    }

    pub fn get_database(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<NotionObject, Box<dyn Error>>> + '_ {
        self.get(BASE_URL.to_string() + "databases/" + id)
    }

    pub fn list_databases(
        &self,
        start_cursor: Option<String>,
        page_size: Option<i32>,
    ) -> impl Future<Output = Result<NotionObject, Box<dyn Error>>> + '_ {
        let query = match (start_cursor, page_size) {
            (Some(c), Some(p)) => format!("?start_cursor={}&page_size={}", c, p),
            (Some(c), None) => format!("?start_cursor={}", c),
            (None, Some(p)) => format!("?page_size={}", p),
            (None, None) => String::new(),
        };
        self.get(BASE_URL.to_string() + "databases" + &query)
    }

    pub fn get_user(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<NotionObject, Box<dyn Error>>> + '_ {
        self.get(BASE_URL.to_string() + "users/" + id)
    }

    pub fn list_users(
        &self,
        start_cursor: Option<String>,
        page_size: Option<i32>,
    ) -> impl Future<Output = Result<NotionObject, Box<dyn Error>>> + '_ {
        let query = match (start_cursor, page_size) {
            (Some(c), Some(p)) => format!("?start_cursor={}&page_size={}", c, p),
            (Some(c), None) => format!("?start_cursor={}", c),
            (None, Some(p)) => format!("?page_size={}", p),
            (None, None) => String::new(),
        };
        self.get(BASE_URL.to_string() + "users" + &query)
    }

    pub fn get_block_children(
        &self,
        id: &str,
        start_cursor: Option<String>,
        page_size: Option<i32>,
    ) -> impl Future<Output = Result<NotionObject, Box<dyn Error>>> + '_ {
        let query = match (start_cursor, page_size) {
            (Some(c), Some(p)) => format!("?start_cursor={}&page_size={}", c, p),
            (Some(c), None) => format!("?start_cursor={}", c),
            (None, Some(p)) => format!("?page_size={}", p),
            (None, None) => String::new(),
        };
        self.get(BASE_URL.to_string() + "blocks/" + id + "/children" + &query)
    }

    pub fn get_page(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<NotionObject, Box<dyn Error>>> + '_ {
        self.get(BASE_URL.to_string() + "pages/" + id)
    }

    fn patch(
        &self,
        url: String,
        obj: Vec<u8>,
    ) -> impl Future<Output = Result<NotionObject, Box<dyn Error>>> + '_ {
        async move {
            Ok(serde_json::from_str(
                &self
                    .reqwest
                    .patch(url)
                    .header("Notion-Version", Self::NOTION_VERSION)
                    .header("Content-Type", "application/json")
                    .bearer_auth(&self.auth_token)
                    .body(obj)
                    .send()
                    .await?
                    .text()
                    .await?,
            )?)
        }
    }

    pub fn update_page_properties(
        &self,
        id: &str,
        properties: HashMap<String, PagePropertyValue>,
    ) -> impl Future<Output = Result<NotionObject, Box<dyn Error>>> + '_ {
        #[derive(Serialize)]
        struct PageProperties {
            properties: HashMap<String, PagePropertyValue>,
        }

        let id_str = id.to_string();
        async move {
            Ok(self
                .patch(
                    BASE_URL.to_string() + "pages/" + &id_str,
                    serde_json::to_vec(&PageProperties { properties })?,
                )
                .await?)
        }
    }

    pub fn append_block_children(
        &self,
        id: &str,
        children: Vec<BlockObject>,
    ) -> impl Future<Output = Result<NotionObject, Box<dyn Error>>> + '_ {
        #[derive(Serialize)]
        struct BlockChildren {
            children: Vec<BlockObject>,
        }

        let id_str = id.to_string();
        async move {
            Ok(self
                .patch(
                    BASE_URL.to_string() + "blocks/" + &id_str + "/children",
                    serde_json::to_vec(&BlockChildren { children })?,
                )
                .await?)
        }
    }

    fn post(
        &self,
        url: String,
        obj: Vec<u8>,
    ) -> impl Future<Output = Result<NotionObject, Box<dyn Error>>> + '_ {
        async move {
            Ok(serde_json::from_str(
                &self
                    .reqwest
                    .post(url)
                    .header("Notion-Version", Self::NOTION_VERSION)
                    .header("Content-Type", "application/json")
                    .bearer_auth(&self.auth_token)
                    .body(obj)
                    .send()
                    .await?
                    .text()
                    .await?,
            )?)
        }
    }

    pub fn create_page(
        &self,
        id: &str,
        parent: Parent,
        properties: HashMap<String, PagePropertyValue>,
        children_blocks: Option<Vec<BlockObjectInput>>,
    ) -> impl Future<Output = Result<NotionObject, Box<dyn Error>>> + '_ {
        #[derive(Serialize)]
        struct NewPage {
            parent: Parent,
            properties: HashMap<String, PagePropertyValue>,
            children_blocks: Option<Vec<BlockObjectInput>>,
        }

        let id_str = id.to_string();
        async move {
            Ok(self
                .post(
                    BASE_URL.to_string() + "pages/" + &id_str,
                    serde_json::to_vec(&NewPage {
                        parent,
                        properties,
                        children_blocks,
                    })?,
                )
                .await?)
        }
    }

    pub fn query_database(
        &self,
        id: &str,
        filter: Option<QueryDatabaseFilter>,
        sorts: Option<Vec<QueryDatabaseSort>>,
        start_cursor: Option<String>,
        page_size: Option<i32>,
    ) -> impl Future<Output = Result<NotionObject, Box<dyn Error>>> + '_ {
        #[derive(Serialize)]
        struct DatabaseQuery {
            filter: Option<QueryDatabaseFilter>,
            sorts: Option<Vec<QueryDatabaseSort>>,
            start_cursor: Option<String>,
            page_size: Option<i32>,
        }

        let id_str = id.to_string();
        async move {
            Ok(self
                .post(
                    BASE_URL.to_string() + "pages/" + &id_str,
                    serde_json::to_vec(&DatabaseQuery {
                        filter,
                        sorts,
                        start_cursor,
                        page_size,
                    })?,
                )
                .await?)
        }
    }

    pub fn search(
        &self,
        query: Option<String>,
        filter: Option<SearchFilter>,
        sort: Option<SearchSort>,
        start_cursor: Option<String>,
        page_size: Option<i32>,
    ) -> impl Future<Output = Result<NotionObject, Box<dyn Error>>> + '_ {
        #[derive(Serialize)]
        struct Search {
            query: Option<String>,
            filter: Option<SearchFilter>,
            sort: Option<SearchSort>,
            start_cursor: Option<String>,
            page_size: Option<i32>,
        }

        async move {
            Ok(self
                .post(
                    BASE_URL.to_string() + "search",
                    serde_json::to_vec(&Search {
                        query,
                        filter,
                        sort,
                        start_cursor,
                        page_size,
                    })?,
                )
                .await?)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "object")]
#[serde(rename_all = "snake_case")]
pub enum NotionObject {
    Database(DatabaseObject),
    Page(PageObject),
    Block(BlockObject),
    User(UserObject),
    List(List),
    Error(ErrorObject),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    pub results: Vec<NotionObject>,
    pub next_cursor: String,
    pub has_more: bool,
}
