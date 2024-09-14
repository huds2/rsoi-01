use std::error::Error;
use crate::dbs::{Person, PersonNoId, WebappError, PersonRepository, router};
use crate::arc;
use async_trait::async_trait;


struct MockRepository {
    persons: Vec<Person>
}

impl MockRepository {
    pub fn new() -> Self {
        MockRepository {
            persons: vec![]
        }
    }
    pub fn get_id(&self) -> i32 {
        if self.persons.len() == 0 {
            return 1;
        }
        let mut max = self.persons[0].id;
        for person in &self.persons {
            if max < person.id {
                max = person.id;
            }
        }
        return max + 1;
    }
}

#[async_trait]
impl PersonRepository for MockRepository {
    async fn init(&mut self) ->  Result<(), Box<dyn Error>> {
        Ok(())
    }
    async fn get(&mut self, id: i32) ->  Result<Person, Box<dyn Error>> {
        for person in &self.persons {
            if person.id == id {
                return Ok(person.clone())
            }
        }
        Err(WebappError::NotFoundError.into())
    }
    async fn list(&mut self) -> Result<Vec<Person>, Box<dyn Error>> {
        Ok(self.persons.clone())
    }
    async fn create(&mut self, person: PersonNoId) ->  Result<i32, Box<dyn Error>> {
        let id = self.get_id();
        let person = Person {
            id: id.clone(),
            name: person.name,
            age: person.age,
            work: person.work,
            address: person.address
        };
        self.persons.push(person);
        println!("{}", id);
        Ok(id)
    }
    async fn update(&mut self, person: Person) ->  Result<Person, Box<dyn Error>> {
        let index = self.persons.iter().position(|x| x.id == person.id).unwrap();
        self.persons[index] = person.clone();
        Ok(person)
    }
    async fn delete(&mut self, id: i32) -> Result<(), Box<dyn Error>> {
        self.persons.retain(|x| x.id != id);
        Ok(())
    }
}


#[tokio::test]
async fn create_person() {
    let repository = arc!(MockRepository::new());
    let router = router(repository);
    let res = warp::test::request()
        .method("POST")
        .path("/api/v1/persons")
        .body("{\"name\": \"john\", \"age\": 32, \"address\": \"somewhere\", \"work\": \"somehow\"}")
        .reply(&router).await;
    assert_eq!(res.status(), 201);
    let resource_path = res.headers().get("Location").unwrap().to_str().unwrap();
    let res = warp::test::request()
        .method("GET")
        .path(resource_path)
        .reply(&router).await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), "{\"id\":1,\"name\":\"john\",\"age\":32,\"address\":\"somewhere\",\"work\":\"somehow\"}");
}

#[tokio::test]
async fn list_persons() {
    let repository = arc!(MockRepository::new());
    let router = router(repository);
    let res = warp::test::request()
        .method("POST")
        .path("/api/v1/persons")
        .body("{\"name\": \"john\", \"age\": 32, \"address\": \"somewhere\", \"work\": \"somehow\"}")
        .reply(&router).await;
    assert_eq!(res.status(), 201);
    let res = warp::test::request()
        .method("GET")
        .path("/api/v1/persons")
        .reply(&router).await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), "[{\"id\":1,\"name\":\"john\",\"age\":32,\"address\":\"somewhere\",\"work\":\"somehow\"}]");
}

#[tokio::test]
async fn delete_persons() {
    let repository = arc!(MockRepository::new());
    let router = router(repository);
    let res = warp::test::request()
        .method("POST")
        .path("/api/v1/persons")
        .body("{\"name\": \"john\", \"age\": 32, \"address\": \"somewhere\", \"work\": \"somehow\"}")
        .reply(&router).await;
    assert_eq!(res.status(), 201);
    let resource_path = res.headers().get("Location").unwrap().to_str().unwrap();
    let res = warp::test::request()
        .method("GET")
        .path("/api/v1/persons")
        .reply(&router).await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), "[{\"id\":1,\"name\":\"john\",\"age\":32,\"address\":\"somewhere\",\"work\":\"somehow\"}]");
    let res = warp::test::request()
        .method("DELETE")
        .path(resource_path)
        .reply(&router).await;
    assert_eq!(res.status(), 204);
    let res = warp::test::request()
        .method("GET")
        .path("/api/v1/persons")
        .reply(&router).await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), "[]");
}

#[tokio::test]
async fn update_persons() {
    let repository = arc!(MockRepository::new());
    let router = router(repository);
    let res = warp::test::request()
        .method("POST")
        .path("/api/v1/persons")
        .body("{\"name\": \"john\", \"age\": 32, \"address\": \"somewhere\", \"work\": \"somehow\"}")
        .reply(&router).await;
    assert_eq!(res.status(), 201);
    let resource_path = res.headers().get("Location").unwrap().to_str().unwrap();
    let res = warp::test::request()
        .method("PATCH")
        .path(resource_path)
        .body("{\"name\": \"johny\", \"age\": 33}")
        .reply(&router).await;
    assert_eq!(res.status(), 200);
    let res = warp::test::request()
        .method("GET")
        .path("/api/v1/persons")
        .reply(&router).await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), "[{\"id\":1,\"name\":\"johny\",\"age\":33,\"address\":\"somewhere\",\"work\":\"somehow\"}]");
}
