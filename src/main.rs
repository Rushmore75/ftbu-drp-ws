use std::env;

use diesel::{PgConnection, Connection};
use dotenvy::dotenv;
use rocket::{routes, fs::FileServer, serde::json::{Json, serde_json::json, Value}, get};


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    println!("Hello, world!");

   dotenv().ok();

   let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
   PgConnection::establish(&database_url)
       .unwrap_or_else(|_| panic!("Error Conencting to {}", database_url));

    let _rocket = rocket::build()
        .mount("/", routes![example])
        .launch()
        .await?;
    Ok(())


}

#[get("/test")]
fn example() -> Json<Value> {
    let x = json!({
            "id": 20,
            "status": "Active",
            "type": "AIR CONDITIONING",
            "category": "",
            "subcategory": "",
            "item": "Orifice Tube",
            "description": "Orifice Tube",
            "descriptionfull": "Orifice Tube  38623",
            "qoh": 18,
            "cost": 1.32,
            "price": 11.06
        });
    
    Json(x)
}
