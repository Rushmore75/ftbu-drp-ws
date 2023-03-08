use std::{env, sync::Arc};

use lazy_static::lazy_static;
use rocket::{async_trait};
use serenity::{Client, prelude::{GatewayIntents, EventHandler, Context}, model::prelude::{interaction::{Interaction, InteractionResponseType}, Ready, GuildId, ChannelId, Message}, utils::MessageBuilder, http::Http};
use tokio::sync::RwLock;
use tracing::{error, warn, info, debug};

use crate::{bot::commands::{ping, relay_messages_here}, minecraft::MinecraftMsg, rest::rest_main};

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            // Read the slash command
            let content = match command.data.name.as_str() {
                "ping" => ping::run(&command.data.options),
                "relay-chat-here" => relay_messages_here::run(&command.data.options, command.channel_id.as_u64()).await,
                _ => "not implemented".to_string(),
            };

            // Respond to the slash command
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| msg.content(content))
                }).await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
    
    // on message event
    async fn message(&self, _: Context, message: Message) {
        let cnl = *RELAY_CHANNEL.read().await;
        
        if message.author.bot { return; }

        // Make sure the relay channel as been set
        match cnl {
            Some(channel) => {
                // Make sure *this* message is in the relay channel
                if message.channel_id == channel {
                    // Try to get the state queue from rocket
                    match unsafe { rest_main::STATE } {
                        Some(e) => {
                            match unsafe { &MC_UNIVERSE } {
                                Some(universe) => {
                                    // Send message to the queue
                                    match unsafe { e.as_ref() } {
                                        Some(x) => {
                                            let content = message.content;
                                            let sender = message.author.name;
                                                
                                                debug!("Attempting to forward Discord message...");
                                                match x.send(MinecraftMsg::fake_message(sender, content, universe.to_string())) {
                                                    Ok(_) => {},
                                                    Err(err) => {
                                                        error!("Sending to State errored: {}. This is probably because the server has no one listening for messages.", err)
                                                    },
                                                }
                                            },
                                            None => { warn!("State is missing, did rocket crash?") },
                                        }
                                    }
                                },
                                None => { warn!("Universe is missing, has `/relay-chat-here` been run?") }
                            }
                        },
                        None => { warn!("State is missing, has rocket started?") },
                    }
                }
            },
            None => { /* Message was in irrelevant channel, ignore */ },
        }

    }

    async fn ready(&self, context: Context, _: Ready) {

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("GUILD_ID needs to be set")
                .parse()
                .expect("GUILD_ID needs to be an int"),
        );

        // Register commands
        let commands = GuildId::set_application_commands(&guild_id, &context.http, |commands| {
            commands
                .create_application_command(|cmd| ping::register(cmd))
                .create_application_command(|cmd| relay_messages_here::register(cmd))
        }).await;

        // Register http context for the relay to use
        let context_http = unsafe { &mut CONTEXT_HTTP };
        context_http.replace(context.http.clone());

        println!{"{:?} command", commands}
    }
}
    
/// A http context that the message relay can use
static mut CONTEXT_HTTP: Option<Arc<Http>> = None ; // This should really have a rw lock on it
/// The current mc universe it is using
pub static mut MC_UNIVERSE: Option<String> = None ; // This is a bottle neck point for multi-server use. Might
                                                    // want to hashmap pointers to the queue to the universe uuid.
lazy_static!(
    // TODO read this from a config file (as well)
    /// The channel that Minecraft messages get relayed to
    pub static ref RELAY_CHANNEL: RwLock<Option<u64>> = RwLock::new(None);
);


pub async fn send_msg_to_discord(message: &MinecraftMsg) {
    let response = MessageBuilder::new()
        .push_bold(format!("<{}> ", message.sender.player_name))
        .push(message.msg.to_string())
        .build();

    // Check if the http context has been set up
    match unsafe { &CONTEXT_HTTP } {
        Some(http) => {
            // see if the relay channel as been set up
            match *RELAY_CHANNEL.read().await {
                Some(id) => {
                    // relay message
                    ChannelId(id)
                        .say(http, &response)
                        .await
                        .expect("Failed to send message to discord.");
                },
                None => {warn!("Relay Channel not setup!")},
            }
        },
        None => {}
    }
}



pub async fn start_bot() {
    println!("Starting bot...");

    tracing_subscriber::fmt::init();
    // build client
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN");
    // Should probably not do all intents, but it's easier
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        // .event_handler(cRelay)
        .await
        .expect("Error creating client");
    
    // start client
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    };
}
