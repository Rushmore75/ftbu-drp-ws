use serenity::builder::CreateApplicationCommand;
use serenity::model::Permissions;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tracing::info;

use crate::bot::bot_main::{RELAY_CHANNEL, self,};

pub async fn run(options: &[CommandDataOption], channel_id: &u64) -> String {
    let mut x = RELAY_CHANNEL.write().await;
    *x = Some(*channel_id);
    info!("Set relay channel to {}", channel_id);

    let _ = &options.iter().for_each(|f| {
        match &f.value {
            Some(uuid) => {
                let string = uuid.to_string();
                let len = string.len(); 
                
                // remove the quotes around value
                unsafe { bot_main::MC_UNIVERSE = Some(string[1..len-1].to_string()); }
                info!("Set universe to {}", uuid.to_string());
            },
            None => {}, 
        };   
    });

    // tokio::fs::write("./config.json", )

    String::from("Noted.")
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("relay-chat-here")
        .description("Set the channel in which to relay MC chat.")
        .default_member_permissions(Permissions::MANAGE_CHANNELS)
        .create_option(|option| {
            option
                .name("uuid")
                .description("Minecraft Universe UUID")
                .kind(serenity::model::prelude::command::CommandOptionType::String)
                .required(true)
        })
}
