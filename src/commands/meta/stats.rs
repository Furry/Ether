use poise::{self, CreateReply};
use serenity::builder::CreateEmbed;
use crate::{ Context, Error };
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt, CpuExt};
use std::time::SystemTime;
use std::env;

#[poise::command(slash_command)]
pub async fn stats(
    ctx: Context<'_>
) -> Result<(), Error> {
    let elapsed = {
        let command_created_at = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(
            ctx.created_at().unix_timestamp() as u64
        );
        let now = SystemTime::now();
        let duration = now.duration_since(command_created_at).unwrap_or_default();
        duration.as_millis()
    };

    let mut sys = System::new_all();
    sys.refresh_all();

    ctx.send(|b| {
        b.embed(|e| {
            crate::components::embeds::default(e);
            dbg!(5);
            e.field("🌱 Environment", format!(
                "Rust Version: {}\nPlatform: {}\nSerenity: {}",
                env::var("RUST_VERSION")
                    .expect("Could not read rust version from .env"), "Debian 10 x86_64", 
                env::var("SERENITY_VERSION")
                    .expect("Could not read serenity version")
            ), true);

            e.field("💻 System", format!(
                "Memory: {}\nCPU: {}\nLatency: {}",
                    format!("Memory: {:.0}mb / {:.0}mb",  &sys.used_memory() / 1000000, (&sys.free_memory() + &sys.used_memory()) / 1000000),
                    format!("{:.2}% @ {}hz", &sys.global_cpu_info().cpu_usage(), &sys.global_cpu_info().frequency()),
                    format!("One way latency: ``{}``", u128::to_string(&elapsed))
            ), true)
        })
    }).await.expect("Could not send epherial reply in 'status'");

    Ok(())
}

