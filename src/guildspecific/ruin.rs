use std::{collections::HashMap, sync::Arc};
use lazy_static::lazy_static;
use once_cell::sync::Lazy;

use poise::serenity_prelude::{Context, Message, GuildChannel, UserId, ActionRowComponent, InteractionCreateEvent, MessageComponentInteraction, Interaction, MessageId, RoleId, MessageUpdateEvent, AttachmentType};
use serenity::{utils::MessageBuilder, builder::{CreateComponents, CreateActionRow, CreateButton}};
use tokio::sync::Mutex;
use crate::{ Error, SessionData, structs::multi_key_map::MultiKeyMap };

#[derive(Debug, Clone)]
struct PendingContainer {
    original_message: MessageId,
    crosspost_message: MessageId,
    user_id: UserId
}


const RUIN_GUILD_ID: &'static u64 = &933683767908913222;
const REGISTERED_OC_ROLE_ID: &'static u64 = &934564453461131324;
const LISTENER_CHANNEL_ID: &'static u64 = &960791596100648990;
const PENDING_CHANNEL_ID: &'static u64 = &1031739800862081084;
// const PENDING: Lazy<Mutex<HashMap<MessageId, PendingContainer>>> = Lazy::new(|| Mutex::new(HashMap::new()));

lazy_static! {
    static ref PENDING: Mutex<HashMap<MessageId, PendingContainer>> = Mutex::new(HashMap::new());
}

pub async fn handle_ruin_message_update(ctx: &Context, old: &Option<Message>, new: Arc<&Option<Message>>, event: &MessageUpdateEvent, data: &SessionData) -> Result<(), Error> {

    if !PENDING.lock().await.contains_key(&event.id) {
        return Ok(());
    }

    let pending = PENDING.lock().await.get(&event.id).unwrap().clone();
    let original_message = ctx.http.get_message(LISTENER_CHANNEL_ID.clone(), pending.original_message.0).await;

    // We know it's being deleted
    if new.is_none() {
        PENDING.lock().await.remove(&pending.crosspost_message);
        PENDING.lock().await.remove(&pending.original_message);

        if original_message.is_ok() {
            original_message.unwrap().delete(&ctx.http).await.unwrap();
        }

        return Ok(());
    }

    let original_message = original_message.unwrap();
    let mut crosspost_message = ctx.http.get_message(PENDING_CHANNEL_ID.clone(), pending.crosspost_message.0).await.unwrap();

    crosspost_message.edit(&ctx.http, |m| {
        m.content(trim_content(&event.content.clone().unwrap_or("".to_string())));
        for a in event.attachments.clone().unwrap() {
            m.attachment(AttachmentType::Image(reqwest::Url::parse(a.url.clone().as_str()).unwrap()));
        }
        m
    }).await.ok();

    Ok(())
}

pub async fn handle_ruin_registry_interaction(ctx: &Context, base_interaction: &Interaction, data: &SessionData) -> Result<(), Error> {
    if base_interaction.clone().kind() != poise::serenity_prelude::InteractionType::MessageComponent {
        return Ok(());
    }

    let interaction = base_interaction.clone().message_component().unwrap();
    let id = interaction.data.custom_id.as_str();
    if !id.starts_with("ruin_") {
        return Ok(());
    }

    // let pending = PENDING.try_lock().unwrap().get(&interaction.message.id).unwrap().clone();

    let pending = match PENDING.try_lock().unwrap().get(&interaction.message.id) {
        Some(pending) => pending.clone(),
        None => return Ok(())
    };

    let original_message = ctx.http.get_message(LISTENER_CHANNEL_ID.clone(), pending.original_message.0).await;
    let crosspost_message = ctx.http.get_message(PENDING_CHANNEL_ID.clone(), pending.crosspost_message.0).await;
    let guild = ctx.http.get_guild(RUIN_GUILD_ID.clone()).await.unwrap();
    let guildmember = guild.member(&ctx.http, pending.user_id).await.unwrap();
    

    if original_message.is_err() || crosspost_message.is_err() {
        if original_message.is_ok() {
            original_message.unwrap().delete(&ctx.http).await.unwrap();
        }

        if crosspost_message.is_ok() {
            crosspost_message.unwrap().delete(&ctx.http).await.unwrap();
        }

        return Ok(());
    }

    let original_message = original_message.unwrap();
    let crosspost_message = crosspost_message.unwrap();

    match id {
        "ruin_accept" => {
            crosspost_message.delete(&ctx.http).await.unwrap();
            original_message.reply(&ctx.http, "Your character has been approved! Post it in <#1001500979755241603>").await.unwrap();
            guild.edit_member(&ctx.http, original_message.author.id, |member|
                member.roles(
                    vec![guildmember.roles, vec![RoleId(REGISTERED_OC_ROLE_ID.clone())]].concat()
                )
            ).await.ok();
        }

        "ruin_deny" => {
            crosspost_message.delete(&ctx.http).await.unwrap();
            original_message.reply(&ctx.http, "Your character has been denied.").await.unwrap();
        }

        _ => {
            panic!("Invalid interaction ID in Ruin Registry");
        }
    }

    // Remove both keys from the map.
    PENDING.lock().await.remove(&interaction.message.id);
    PENDING.lock().await.remove(&pending.crosspost_message);

    Ok(())
}

pub async fn handle_ruin_registry_message(ctx: &Context, message: &Message, _data: &SessionData) -> Result<(), Error> {
    // Doesn't meet the decoration requirement for the message.
    if message.guild_id.is_none() || 
       &message.guild_id.unwrap().0 != RUIN_GUILD_ID ||
       &message.channel_id.0 != LISTENER_CHANNEL_ID ||
       !message.content.starts_with("╭──────────────────.★..─╮") ||
       !message.content.ends_with("╰─..★.──────────────────╯") 
    {
        return Ok(());
    }

    let pending_channel: GuildChannel = ctx.cache.guild_channel(PENDING_CHANNEL_ID.clone()).unwrap();
    let listened_channel: GuildChannel = ctx.cache.guild_channel(LISTENER_CHANNEL_ID.clone()).unwrap();

    let mut components = CreateComponents::default();
    components.create_action_row(|create_action_row|
        create_action_row.create_button(|button1|
            button1.style(poise::serenity_prelude::ButtonStyle::Primary)
                .label("Accept")
                .custom_id("ruin_accept")
        ).create_button(|button2|
            button2.style(poise::serenity_prelude::ButtonStyle::Danger)
                .label("Deny")
                .custom_id("ruin_deny")
        )
    );

    let crosspost_message = pending_channel.send_message(&ctx.http, |b| {
            b.content(trim_content(&message.content))
            .add_embed(|e| {
                crate::components::embeds::default(e);
                e.field("Account Created", message.author.created_at().to_rfc2822(), true);
                e.field("Message Link", format!("[Here]({})", message.link()), true);
                e.thumbnail(message.author.avatar_url().unwrap_or(message.author.default_avatar_url()))
            }).set_components(components.to_owned());

            for a in message.attachments.clone() {
                b.add_file(AttachmentType::Image(reqwest::Url::parse(a.url.clone().as_str()).unwrap()));
            }

            b
        }
    ).await.unwrap();

    let container = PendingContainer {
        original_message: message.id,
        crosspost_message: crosspost_message.id,
        user_id: message.author.id
    };

    PENDING.lock().await.insert(message.id, container.clone());
    PENDING.lock().await.insert(crosspost_message.id, container.clone());
    return Ok(());


}


fn trim_content(input: &String) -> String {
    // Accept a string, if it exceeds 2000 characters trim it to 1900 and add '..[xxx more characters]'
    if input.len() > 2000 {
        let mut output = input.clone();
        output.truncate(1900);
        output.push_str("..[");
        output.push_str(&(input.len() - 1900).to_string());
        output.push_str(" more]");
        return output;
    } else {
        return input.clone();
    }
}