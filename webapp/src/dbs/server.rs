use std::{convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::{reject, reply::{self, Reply}, Filter, Rejection};
use super::{PersonNoId, PersonRepository, PersonPatch};

pub type WebResult<T> = std::result::Result<T, Rejection>;

async fn list_handler(person_repository: Arc<Mutex<dyn PersonRepository>>) -> WebResult<impl Reply> {
    match person_repository.lock().await.list().await {
        Ok(persons) => {
            return Ok(reply::json(&persons));
        },
        Err(_) => {
            return Err(reject::not_found());
        }
    }
}

async fn get_handler(id: i32,
                     person_repository: Arc<Mutex<dyn PersonRepository>>) -> WebResult<impl Reply> {
    match person_repository.lock().await.get(id).await {
        Ok(person) => {
            return Ok(reply::json(&person));
        },
        Err(_) => {
            return Err(reject::not_found());
        }
    }
}

async fn post_handler(body: PersonNoId,
                      person_repository: Arc<Mutex<dyn PersonRepository>>) -> WebResult<impl Reply> {
    match person_repository.lock().await.create(body).await {
        Ok(id) => {
            let reply = warp::reply::with_status("Created person", warp::http::StatusCode::CREATED);
            let reply = warp::reply::with_header(reply, "Location", &format!("/api/v1/persons/{}", id));
            return Ok(reply)
        },
        Err(_) => {
            return Err(reject::reject());
        }
    }
}

async fn delete_handler(id: i32,
                        person_repository: Arc<Mutex<dyn PersonRepository>>) -> WebResult<impl Reply> {
    match person_repository.lock().await.delete(id).await {
        Ok(_) => {
            return Ok(warp::reply::with_status("Deleted person", warp::http::StatusCode::NO_CONTENT))
        },
        Err(_) => {
            return Err(reject::not_found());
        }
    }
}

async fn patch_handler(id: i32,
                       body: PersonPatch,
                       person_repository: Arc<Mutex<dyn PersonRepository>>) -> WebResult<impl Reply> {
    let mut current_person = match person_repository.lock().await.get(id).await {
        Ok(person) => {
            person
        },
        Err(_) => {
            return Err(reject::not_found());
        }
    };
    if let Some(name) = body.name {
        current_person.name = name;
    }
    if let Some(age) = body.age {
        current_person.age = age;
    }
    if let Some(work) = body.work {
        current_person.work = work;
    }
    if let Some(address) = body.address {
        current_person.address = address;
    }
    match person_repository.lock().await.update(current_person).await {
        Ok(person) => {
            return Ok(reply::json(&person));
        },
        Err(_) => {
            return Err(reject::reject());
        }
    }
}

fn with_arc<T: Send + ?Sized>(arc: Arc<Mutex<T>>) -> impl Filter<Extract = (Arc<Mutex<T>>,), Error = Infallible> + Clone {
    warp::any().map(move || arc.clone())
}

pub async fn run_server(repository: Arc<Mutex<dyn PersonRepository>>) {
    let list_route = warp::path("api")
        .and(warp::path("v1"))
        .and(warp::path!("persons"))
        .and(warp::get())
        .and(with_arc(repository.clone()))
        .and_then(list_handler);
    let create_route = warp::path("api")
        .and(warp::path("v1"))
        .and(warp::path!("persons"))
        .and(warp::body::json())
        .and(warp::post())
        .and(with_arc(repository.clone()))
        .and_then(post_handler);
    let get_route = warp::path("api")
        .and(warp::path("v1"))
        .and(warp::path!("persons" / i32))
        .and(warp::get())
        .and(with_arc(repository.clone()))
        .and_then(get_handler);
    let delete_route = warp::path("api")
        .and(warp::path("v1"))
        .and(warp::path!("persons" / i32))
        .and(warp::delete())
        .and(with_arc(repository.clone()))
        .and_then(delete_handler);
    let patch_route = warp::path("api")
        .and(warp::path("v1"))
        .and(warp::path!("persons" / i32))
        .and(warp::body::json())
        .and(warp::patch())
        .and(with_arc(repository.clone()))
        .and_then(patch_handler);
    let routes = create_route
        .or(list_route)
        .or(get_route)
        .or(delete_route)
        .or(patch_route);
    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080))
        .await
}
