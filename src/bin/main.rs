use tasks_authenticated::{App, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    App::run().await
}
