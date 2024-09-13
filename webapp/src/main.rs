use std::error::Error;
use dbs::{run_server, PersonRepository, Repository};
use std::env;

mod dbs;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>> {
    let connection_str = env::var("PSQL_CONNECTION")?;
    let repository = arc!(Repository::new(&connection_str).await?);
    repository.lock().await.init().await?;
    run_server(repository).await;
    Ok(())
}
