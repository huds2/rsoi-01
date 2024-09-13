use std::error::Error;
use async_trait::async_trait;
use super::person::Person;
use crate::dbs::WebappError;
use tokio_postgres::{Client, NoTls};

#[async_trait]
pub trait PersonRepository: Sync + Send {
    async fn init(&mut self) ->  Result<(), Box<dyn Error>>;
    async fn create(&mut self, person: Person) ->  Result<(), Box<dyn Error>>;
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
    async fn create(&mut self, person: Person) ->  Result<(), Box<dyn Error>> {
        if let Ok(_) = self.get(person.id).await { return Err(WebappError::AlreadyExistsError.into()) }
        self.client.batch_execute(&format!("
            INSERT INTO person VALUES
                ({}, '{}', {}, '{}', '{}')
        ", person.id, person.name, person.age, person.address, person.work)).await?;
        Ok(())
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
