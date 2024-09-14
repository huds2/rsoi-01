use std::error::Error;
use async_trait::async_trait;
use super::person::{Person, PersonNoId};
use crate::dbs::WebappError;
use tokio_postgres::{Client, NoTls};

#[async_trait]
pub trait PersonRepository: Sync + Send {
    async fn init(&mut self) ->  Result<(), Box<dyn Error>>;
    async fn create(&mut self, person: PersonNoId) ->  Result<i32, Box<dyn Error>>;
    async fn list(&mut self) -> Result<Vec<Person>, Box<dyn Error>>;
    async fn get(&mut self, id: i32) ->  Result<Person, Box<dyn Error>>;
    async fn update(&mut self, person: Person) ->  Result<Person, Box<dyn Error>>;
    async fn delete(&mut self, id: i32) -> Result<(), Box<dyn Error>>;
}

pub struct Repository {
    client: Client
}

impl Repository {
    pub async fn new(connection_str: &str) -> Result<Self,  Box<dyn Error>> {
        let (client, connection) = tokio_postgres::connect(connection_str, NoTls).await?;
        tokio::spawn(async move {
            connection.await
        });
        Ok(Self { 
            client
        })
    }
    async fn get_min_id(&mut self) -> Result<i32, Box<dyn Error>> {
        if let Ok(rows) = self.client.query("
            SELECT case when min(id) = 0 then 0 else max(id) end FROM person;
        ", &[]).await {
            for row in rows {
                let min_id: i32 = row.get(0);
                return Ok(min_id + 1);
            }
            return Ok(1);
        }
        else {
            return Ok(1);
        }
    }
}

#[async_trait]
impl PersonRepository for Repository {
    async fn init(&mut self) ->  Result<(), Box<dyn Error>> {
        self.client.execute("
            CREATE TABLE IF NOT EXISTS person (
                id              INTEGER PRIMARY KEY,
                name            VARCHAR NOT NULL,
                age             INTEGER NOT NULL,
                address         VARCHAR NOT NULL,
                work            VARCHAR NOT NULL
                )
        ", &[]).await?;
        Ok(())
    }
    async fn get(&mut self, id: i32) ->  Result<Person, Box<dyn Error>> {
        for row in self.client.query(&format!("
            SELECT * FROM person WHERE id = {}
        ", id), &[]).await? {
            return Ok(Person {
                id: row.get(0),
                name: row.get(1),
                age: row.get(2),
                address: row.get(3),
                work: row.get(4)
            })
        }
        Err(WebappError::NotFoundError.into())
    }
    async fn list(&mut self) -> Result<Vec<Person>, Box<dyn Error>> {
        let mut list = vec![];
        for row in self.client.query("
            SELECT * FROM person
        ", &[]).await? {
            list.push(Person {
                id: row.get(0),
                name: row.get(1),
                age: row.get(2),
                address: row.get(3),
                work: row.get(4)
            })
        }
        Ok(list)
    }
    async fn create(&mut self, person: PersonNoId) ->  Result<i32, Box<dyn Error>> {
        let id = self.get_min_id().await?;
        self.client.batch_execute(&format!("
            INSERT INTO person VALUES
                ({}, '{}', {}, '{}', '{}')
        ", id.clone(), person.name, person.age, person.address, person.work)).await?;
        Ok(id)
    }
    async fn update(&mut self, person: Person) ->  Result<Person, Box<dyn Error>> {
        self.client.batch_execute(&format!("
            UPDATE person SET
                name = '{}',
                age = {},
                address = '{}',
                work = '{}'
            WHERE id = {} 
        ", person.name, person.age, person.address, person.work, person.id)).await?;
        self.get(person.id).await
    }
    async fn delete(&mut self, id: i32) -> Result<(), Box<dyn Error>> {
        self.client.batch_execute(&format!("
            DELETE FROM person WHERE id = {}
        ", id)).await?;
        Ok(())
    }
}
