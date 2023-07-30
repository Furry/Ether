// use std::time::SystemTime;
// use poise::serenity_prelude as serenity;
// use serenity::model::prelude::InteractionResponseType;
// use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt, CpuExt};
// use serenity::builder::{CreateEmbed, CreateApplicationCommand, CreateApplicationCommandOption, CreateMessage};
// use serenity::http::Http;
// use serenity::model::prelude::application_command::ApplicationCommandInteraction;
// use serenity::model::prelude::command::Command;
// use serenity::model::prelude::interaction::application_command::CommandDataOption;
// use serenity::prelude::Context;

// pub async fn register(http: &Http) {
//     Command::create_global_application_command(http, |command: &mut serenity::builder::CreateApplicationCommand| {
//         command.name("status")
//             .description("meta & status information about me")
//     }).await.expect("Could not register command 'status'");
// }


// // pub async fn run(_options: &[CommandDataOption], ctx: &Context, command: &ApplicationCommandInteraction) {
// //     // Handle the fields
// //     // Construct our embed
// //     let mut embed = CreateEmbed::default();
// //     embed.color(serenity::utils::Colour::from_rgb(0, 255, 255));

// //     let command_created_at = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(
// //         command.id.created_at().unix_timestamp() as u64
// //     );
// //     let now = SystemTime::now();
// //     let duration = now.duration_since(command_created_at).unwrap_or_default();
// //     let elapsed = duration.as_millis();

// //     let mut sys = System::new_all();
// //     sys.refresh_all();

// //     embed.description(
// //         format!("Pong!\n\nOne way latency: ``{}``\n\nMemory Usage: {}\nCPU :{}",
// //             u128::to_string(&elapsed),
// //             format!("Used: {}kb / Free: {}kb", &sys.used_memory() / 1000, &sys.free_memory() / 1000),
// //             format!("{}% @ {}hz", &sys.global_cpu_info().cpu_usage(), &sys.global_cpu_info().frequency())
// //         )
// //     );

// //     command.create_interaction_response(&ctx.http, |m| {
// //         m.kind(InteractionResponseType::UpdateMessage)
// //         .interaction_response_data(|b| {
// //             // b.add_embed(embed)
// //             //     .ephemeral(true)
// //         })
// //     }).await.expect("Could not send follow up message.");

// // }

