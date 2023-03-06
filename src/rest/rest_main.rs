use crate::{rest::pages::{team_leave, team_join, version_check, player_message, listen_for_chats}, minecraft::MinecraftMsg};
use rocket::{routes, State};
use tokio::sync::broadcast::{channel, Sender};

pub static mut STATE: Option<*const State<Sender<MinecraftMsg>>> = None;

#[rocket::main]
pub async fn start_rocket() -> Result<(), rocket::Error> {
    println!("Starting rocket...");

    let rocket = rocket::build()
        .manage(channel::<MinecraftMsg>(1024).0)
        .mount("/", routes![team_leave, team_join, version_check, player_message, listen_for_chats]);

    let y: &State<Sender<MinecraftMsg>> = State::get(&rocket).expect("Failed to get state.");


    unsafe {
        let ptr = std::ptr::addr_of!(*y);
        STATE = Some(ptr);
    }

    let _r = rocket.launch().await?;

    unsafe { STATE = None; }

    Ok(())
 
}
    