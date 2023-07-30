use poise::{ self };
use crate::{ Context, Error};

#[derive(Debug, poise::ChoiceParameter)]
pub enum ProfilePrimaryOption {
    #[name = "Create"]
    Create,
    #[name = "Delete"]
    Delete,
    #[name = "Edit"]
    Edit,
    #[name = "List"]
    List
}

#[poise::command(slash_command, ephemeral)]
pub async fn profile(
    ctx: Context<'_>,
    #[description = "The option to execute"] option: ProfilePrimaryOption
) ->  Result<(), Error> {
    match option {
        ProfilePrimaryOption::Create => {
            // Create modal
            
        }

        _ => {
            panic!("Unexpected input in Profile main selection.")
        }
    };

    Ok(())
}