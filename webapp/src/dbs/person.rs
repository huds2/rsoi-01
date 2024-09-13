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

pub fn add_id(person: PersonNoId, id: i32) -> Person {
    Person { id, name: person.name, age: person.age, address: person.address, work: person.work }
}
