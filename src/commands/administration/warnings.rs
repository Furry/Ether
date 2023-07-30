use poise::{self, CreateReply};
use serenity::builder::CreateEmbed;
use crate::{ Context, Error };

#[poise::command(slash_command, ephemeral)]
pub async fn warn(
    ctx: Context<'_>
) -> Result<(), Error> {
    let db = crate::MONGO
        .try_lock()
        .unwrap()
        .clone()
        .unwrap();

    
    return Ok(())
}