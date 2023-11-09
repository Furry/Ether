use std::sync::Arc;

use poise::serenity_prelude::{Context, Message, MessageUpdateEvent};
use crate::{Error, SessionData, commands::profile::ProfileInfo, guildspecific::ruin::{handle_ruin_registry_message, handle_ruin_message_update}};
use regex::Regex;

pub async fn handle_message_update(ctx: &Context, old: &Option<Message>, new: Arc<&Option<Message>>, event: &MessageUpdateEvent, data: &SessionData) -> Result<(), Error> {

    handle_ruin_message_update(ctx, old, new, event, data)
        .await.expect("Failed to handle ruin message update");

    Ok(())
}


pub async fn handle_message_create(ctx: &Context, event: &Message, data: &SessionData) -> Result<(), Error> {

    hook_control(ctx, event, data).await
        .expect("Failed to handle hook control");

    handle_ruin_registry_message(ctx, event, data)
        .await.expect("Failed to handle ruin registry");

    Ok(())
}

pub async fn hook_control(ctx: &Context, event: &Message, data: &SessionData) -> Result<(), Error> {
    if !data.cache.users.contains_key(&event.author.id) {
        return Ok(())
    }

    let profiles = data.cache.users.get(&event.author.id).unwrap();

    // Check if their message contents matches 
    let mut found_profile: Option<ProfileInfo> = None;
    let mut content: String = String::new();
    for profile in profiles {
        let re = Regex::new(profile.activation.replace("text", "([\\s\\S]+)").as_str());
        if re.is_err() {
            continue;
        }

        let re = re.unwrap();
        let captures = re.captures(event.content.as_str());
        if captures.is_none() {
            continue;
        }

        found_profile = Some(profile.clone());
        // Set content equal to the capture group contents.
        content = captures.unwrap().get(1).unwrap().as_str().to_string();
    }

    if found_profile.is_none() {
        return Ok(())
    }

    // See if there's a webhook that the bot created within the channel.
    let webhooks = event.channel_id.webhooks(&ctx.http).await?;
    let mut found_webhook: Option<serenity::model::webhook::Webhook> = None;
    for webhook in webhooks {
        if webhook.token.is_some() && webhook.clone().name.unwrap_or("".to_string()) == "EtherHook" {
            found_webhook = Some(webhook);
            break;
        }
    }

    // If there's no hook, create one
    let webhook = if found_webhook.is_none() {
        event.channel_id.create_webhook(&ctx.http, "EtherHook").await?
    } else {
        found_webhook.unwrap()
    };

    event.delete(&ctx.http).await?;

    // Send the message
    webhook.execute(&ctx.http, false, |w| {
        w.content(content)
            .username(found_profile.clone().unwrap().name)
            .avatar_url(found_profile.unwrap().image_url)
    }).await?;
    Ok(())
}