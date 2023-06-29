use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let todos: Vec<Todo> = client
    .get("https://jsonplaceholder.typicode.com/todos?userId=1")
    .send()
    .await?
    .json()
    .await?;

    println!("{:#?}", todos);

    

    Ok(())
}