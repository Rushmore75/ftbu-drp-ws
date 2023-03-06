use crate::rest::pages::{team_leave, team_join, version_check, player_message};
use rocket::routes;



#[rocket::main]
pub async fn start_rocket() -> Result<(), rocket::Error> {
    println!("Starting rocket...");

    let _rocket = rocket::build()
        .mount("/", routes![team_leave, team_join, version_check, player_message])
        .launch()
        .await?;
    Ok(())
 
}
    