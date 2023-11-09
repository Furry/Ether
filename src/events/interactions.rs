use poise::serenity_prelude::{InteractionCreateEvent, Interaction};
use serenity::prelude::Context;

use crate::{SessionData, Error, guildspecific::ruin::handle_ruin_registry_interaction};

pub async fn handle_interaction_create(ctx: &Context, event: &Interaction, data: &SessionData) -> Result<(), Error> {

    handle_ruin_registry_interaction(ctx, event, data)
        .await.expect("Failed to handle ruin registry interaction");

    Ok(())
}