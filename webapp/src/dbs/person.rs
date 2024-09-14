use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub address: String,
    pub work: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonNoId {
    pub name: String,
    pub age: i32,
    pub address: String,
    pub work: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonPatch {
    pub name: Option<String>,
    pub age: Option<i32>,
    pub address: Option<String>,
    pub work: Option<String>
}
