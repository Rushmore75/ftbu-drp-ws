use serenity::builder::CreateApplicationCommand;
use serenity::model::Permissions;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(options: &[CommandDataOption]) -> String {
    let x = &options.iter().fold(String::new(),|mut s, f| {
        match &f.value {
            Some(e) => s = e.to_string(),
            None => {}, 
        };   
        s
    });
    x.to_string()
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