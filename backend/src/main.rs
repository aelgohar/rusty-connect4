#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate bson;
#[macro_use] extern crate rocket;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

// routing for the index file
#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("target/deploy/index.html")).ok()
}

// routing for static files
#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("target/deploy/").join(file)).ok()
}

mod games {
    use bson::Bson;
    use mongodb::Collection;
    use rocket::State;
    use rocket_contrib::json::Json;
    use serde::{Serialize, Deserialize};

    // Rust representation of game objects
    #[derive(Serialize, Deserialize, Debug)]
    #[allow(non_snake_case)]
    pub struct Game {
        gameNumber: String,
        gameType: String,
        Player1Name: String,
        Player2Name: String,
        WinnerName: String,
        GameDate: i64,
    }

    #[get("/games")]
    pub fn get(collection: State<Collection>) -> Json<Vec<Game>> {
        let mut games = Vec::new();
        
        // get all the documents in the collection
        if let Ok(cursor) = collection.find(doc!{}, None) {
            for document in cursor.filter_map(Result::ok) {
                // if document can be converted from BSON, add to vec
                if let Ok(game) = bson::from_bson(Bson::Document(document)) {
                    games.push(game);
                }
            }
        }
        
        Json(games)
    }

    #[post("/games", format="json", data="<game>")]
    pub fn post(game: Json<Game>, collection: State<Collection>) {
        // if game can be converted to document, store in db
        if let Ok(Bson::Document(document)) = bson::to_bson(&game.into_inner()) {
            collection.insert_one(document, None).unwrap();
        }
    }
}

fn rocket() -> Result<rocket::Rocket, mongodb::error::Error> {
    // create a connection to the games collection
    let collection = mongodb::Client::with_uri_str("mongodb://localhost:27017/")?
        .database("Connect4DB")
        .collection("games");

    // create the web server object
    Ok(rocket::ignite()
        .manage(collection)
        .mount("/", routes![
            index,
            files,
            games::get, 
            games::post,
        ]))
}

fn main() {
    // launch server or report error
    match rocket() {
        Ok(rocket) => {
            let error = rocket.launch();
            eprintln!("Failed to launch server: {}", error);
        }
        Err(error) => eprintln!("Failed to create server: {}", error),
    }
}
