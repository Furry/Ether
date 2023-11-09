use futures::TryStreamExt;
use mongodb::bson::doc;
use poise::{ self, Modal, serenity_prelude::{SelectMenu, InteractionResponseType, ButtonStyle} };
use serenity::{builder::CreateSelectMenu, collector::CollectComponentInteraction};
use crate::{ Context, Error};

#[derive(Debug, poise::ChoiceParameter)]
pub enum ProfilePrimaryOption {
    #[name = "Create"]
    Create,
    #[name = "Edit"] 
    Edit
}

#[derive(Debug, Clone, Modal)]
#[name = "Create Profile"]
struct CreateProfileModal {
    #[name = "Profile's Name"]
    name: String,
    #[name = "Profile's Activation"]
    #[placeholder = ">>text<<, //text, text /ether, ect."]
    activation: String,

    #[name = "Profile's Image URL"]
    #[placeholder = "Leave blank to upload an image manually"]
    image_url: Option<String>
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProfileInfo {
    pub name: String,
    pub activation: String,
    pub image_url: String,
    pub owner: u64,
    pub crated_timestamp: u64,
    pub last_used_timestamp: u64,
    pub uses: u64
}

#[poise::command(slash_command, ephemeral)]
pub async fn profile(
    ctx: Context<'_>,
    #[description = "The option to execute"] option: ProfilePrimaryOption
) ->  Result<(), Error> {
    match option {
        ProfilePrimaryOption::Create => {
            create_profile(ctx).await?;
        },

        ProfilePrimaryOption::Edit => {
            edit_profile(ctx).await?;
        }

        _ => {
            panic!("Unexpected input in Profile main selection.")
        }
    };

    Ok(())
}

pub async fn create_profile(
    ctx: Context<'_>,
) -> Result<(), Error> {
    
    // Get our data from the Modal Submission
    let data;
    if let poise::Context::Application(application_context) = ctx {
        data = CreateProfileModal::execute(application_context).await?;
        if data.is_none() {
            return Ok(())
        }
    } else {
        return Ok(());
    }

    let profile_data = data.unwrap();

    if !profile_data.activation.contains("text") {
        ctx.send(|b| {
            b.embed(|e| {
                crate::components::embeds::default(e);
                e.title("Profile Creation Failed!");
                e.description("The profile activiation must include the word 'text', that represents where your message's content is. For example, ``ether:text`` or ``//text``.")
            })
        }).await?;
    } else {
        crate::MONGO
            .try_lock()
            .unwrap()
            .clone()
            .unwrap()
            .database("Ether")
            .collection("profiles")
            .insert_one(ProfileInfo {
                name: profile_data.clone().name,
                activation: profile_data.clone().activation,
                image_url: profile_data.clone().image_url.unwrap_or(String::from("")),
                owner: ctx.author().id.0,
                crated_timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                last_used_timestamp: 0,
                uses: 0
            }, None).await?;

        
        ctx.send(|b| {
            b.embed(|e| {
                crate::components::embeds::default(e);
                e.title("Profile Created");
                e.description(format!(
                    "Created a profile named ``{}`` with activation ``{}``.",
                    profile_data.name,
                    profile_data.activation
                ))
            })
        }).await?;
    }

    Ok(())
}

async fn edit_profile(ctx: Context<'_>) -> Result<(), Error> {
    // Get all of the user's profiles

    let profiles = crate::MONGO
        .try_lock()
        .unwrap()
        .clone()
        .unwrap()
        .database("Ether")
        .collection::<ProfileInfo>("profiles")
        .find(doc! {
            "owner": ctx.author().id.0 as i64
        }, None).await?
        .with_type::<ProfileInfo>().try_collect::<Vec<ProfileInfo>>().await?;

    // // Make a dropdown menu of all of the user's profiles
    // ctx.send(|reply| {
    //     reply.components(|components| {
    //         components.create_action_row(|row| {
    //             row.add_select_menu(
    //                 CreateSelectMenu::default()
    //                     .options(|t| {
    //                         t.create_option(|option| {
    //                             option.label("Select a profile to delete.")
    //                                 .description("Select a profile to delete.")
    //                                 .value("delete")
    //                         })
    //                     })
    //                     .to_owned()
    //             )
    //         })
    //     })
    // }).await?;

    paginate(ctx, profiles.clone()).await?;

    Ok(())
}

pub async fn paginate(
    ctx: Context<'_>,
    pages: Vec<ProfileInfo>,
) -> Result<(), serenity::Error> {
    // Define some unique identifiers for the navigation buttons
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}prev", ctx.id());
    let next_button_id = format!("{}next", ctx.id());
    let edit_button_id = format!("{}edit", ctx.id());
    let delete_button_id = format!("{}delete", ctx.id());

    // Send the embed with the first page as content
    let mut current_page = 0;
    ctx.send(|b| {
        b.embed(|b| {
            b.field("Profile Name", pages.get(current_page).expect("No pages left to paginate!").name.clone(), true)
            .field("Profile Activation", pages.get(current_page).expect("No pages left to paginate!").activation.clone(), true);
            if pages.get(current_page).expect("No pages left to paginate!").image_url != "" {
                b.image(pages.get(current_page).expect("No pages left to paginate!").image_url.clone())
            } else { b }
        })

        .components(|b| {
            b.create_action_row(|b| {
                b.create_button(|b| b.custom_id(&prev_button_id).emoji('◀').style(ButtonStyle::Primary))
                .create_button(|b| b.custom_id(&delete_button_id).label("Delete").style(ButtonStyle::Danger))
                .create_button(|b| b.custom_id(&edit_button_id).label("Edit").style(ButtonStyle::Primary))
                .create_button(|b| b.custom_id(&next_button_id).emoji('▶').style(ButtonStyle::Primary))
            })
        })
    })
    .await?;

    // Loop through incoming interactions with the navigation buttons
    while let Some(press) = CollectComponentInteraction::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(std::time::Duration::from_secs(3600 * 24))
        .await
    {
        // Depending on which button was pressed, go to next or previous page
        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= pages.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
        } else {
            println!("{}", press.data.custom_id);
            if press.data.custom_id == "delete" {
                println!("Delete");
            } else if press.data.custom_id == "edit" {
                println!("Edit");
            }
            continue;
        }

        // Update the message with the new page contents
        press
            .create_interaction_response(ctx, |b| {
                b.kind(InteractionResponseType::UpdateMessage)
                    .interaction_response_data(|b| b.embed(|b| {
                        b.field("Profile Name", pages.get(current_page).expect("No pages left to paginate!").name.clone(), true)
                        .field("Profile Activation", pages.get(current_page).expect("No pages left to paginate!").activation.clone(), true);

                        if pages.get(current_page).expect("No pages left to paginate!").image_url != "" {
                            b.image(pages.get(current_page).expect("No pages left to paginate!").image_url.clone())
                        } else { b }
                    }))
            })
            .await?;
    }
    Ok(())
}