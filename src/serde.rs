use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Ability {
    name: String,
    url: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct Pokemon {
    name: String,
    height: u32,
    weight: u32,
    abilities: Vec<Ability>,
}

impl Pokemon{
    fn _new(name: String, height: u32, weight: u32) -> Self{
        Self{
            name,
            height,
            weight,
            abilities: Vec::new(),
        }
    }
    fn display(&self){
        println!("Pokemon: {}", self.name);
        println!("Height: {}", self.height);
        println!("Weight: {}", self.weight);
    }
}

#[tokio::main]
async fn serde(){
    let url = "https://pokeapi.co/api/v2/pokemon/2";
    let resp = reqwest::get(url).await.unwrap();
    let pokemon: Pokemon = resp.json().await.unwrap();
    pokemon.display();
}