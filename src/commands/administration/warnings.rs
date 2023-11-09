
use poise::serenity_prelude::{GuildId, UserId};
use serenity::model::Permissions;
use serenity::model::user::User;
use serenity::prelude::Mentionable;
use crate::{ Context, Error };
use mongodb::bson::doc;
use futures::stream::TryStreamExt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Warning {
    user: UserId,
    issuer: UserId,
    guild: GuildId,
    reason: String,
    expires_timestamp: u64
}

// Parse time in the format of 1d, 1h, 1w, 1m, 1y, 48h ect. Create a timestamp for it. If invalid, return 0.
pub fn parse_time(time: String) -> u64 {
    let time = time.chars();
    let time = time.as_str();
    let pairs = time.split_at(time.len() - 1);
    let time = pairs.0.parse::<u64>().unwrap_or(0);
    let time = match pairs.1 {
        "d" => time * 86400,
        "h" => time * 3600,
        "w" => time * 604800,
        "m" => time * 2592000,
        "y" => time * 31536000,
        _ => 0
    };

    let now = std::time::SystemTime::now();
    let now = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    let now = now.as_secs();
    now + time

}

#[poise::command(
    slash_command, 
    required_permissions = "MANAGE_MESSAGES"
)]pub async fn warn(
    ctx: Context<'_>,
    
    // Fields
    #[description = "The user to warn"] user: User,
    #[description = "The reason for the warning"] reason: String,
    #[description = "The amount of time to warn for, else infinite. (1d, 1m)"] length: Option<String>,
) -> Result<(), Error> {
    let db = crate::MONGO
        .try_lock()
        .unwrap()
        .clone()
        .unwrap();

    let expires_timestamp = if length.is_some() {
        parse_time(length.clone().unwrap())
    } else {
        0
    };

    db.database("Ether")
        .collection("warnings")
        .insert_one(Warning {
            user: user.id,
            issuer: ctx.author().id,
            guild: ctx.guild_id()
                .expect("Could not resolve guild id."),
            reason: reason.clone(),
            expires_timestamp: expires_timestamp.clone()
        }, None).await.expect("Error inserting warning into database.");

    ctx.send(|b| {
        b.embed(|e| {
            crate::components::embeds::default(e);
            e.title("Warn Applied.");
            e.description(format!(
                "Applied a warning to {} for ``{}``.{}",
                user.mention(),
                reason,
                if expires_timestamp == 0 {
                    String::new()
                } else {
                    format!("\n This warning will expire at <t:{}>.", expires_timestamp)
                }
            ))
        })
    }).await.expect("Could not send reply in 'warn'");

    Ok(())
}

#[poise::command(slash_command)]
pub async fn warnings(
    ctx: Context<'_>,
    #[description = "The user to get warnings for"] user: Option<User>
) -> Result<(), Error> {

    // If they user does not have manage_messages, they can only get their own warnings.
    if !ctx.author_member()
        .await.unwrap()
        .permissions.unwrap().contains(Permissions::MANAGE_MESSAGES) {
        if user.is_some() {
            ctx.send(|m| {
                m.ephemeral(true);
                m.content("You do not have permission to view other users warnings.")
            }).await.expect("Could not send reply in 'warnings'");
            return Ok(())
        }
    }

    // Resolve the user to an ID
    let guild = ctx.guild().unwrap().id;
    let user = if user.is_some() {
        user.unwrap().id
    } else {
        ctx.author().clone().id
    };

    let db = crate::MONGO
        .try_lock()?
        .clone()
        .unwrap();

    let warnings_col: mongodb::Collection<Warning>  = db.database("Ether").collection::<_>("warnings");

    let warnings = warnings_col.find(doc!{
            "user": user.to_string(),
            "guild": guild.to_string()
        }, None)
        .await.expect("Could not fetch warnings from database.")
        .with_type::<Warning>().try_collect::<Vec<Warning>>().await.unwrap();


    let warn_string = warnings.into_iter().enumerate().map(|(index, warning)| {
        format!("**Warning {} by <@{}>:**\n\nReason: {}{}", 
            index + 1,
            warning.issuer,
            warning.reason,
            if warning.expires_timestamp == 0 {
                String::new()
            } else {
                format!("\nExpires at <t:{}>.", warning.expires_timestamp)
            }
        )
    }).collect::<Vec<String>>();

    crate::components::paginate::paginate(ctx, warn_string).await.unwrap();
    Ok(())
}