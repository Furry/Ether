use serenity::builder::{CreateEmbed};
use rand::seq::SliceRandom;

// const vec of &str
const FOOTERS: &'static [&'static str] = &[
    "Beep Boop", "Is this thing on?", "Help im trapped in a compu-"
];

pub fn default(b: &mut CreateEmbed) -> &mut CreateEmbed {
    b
        .colour(serenity::utils::Colour::from_rgb(0, 255, 255))
        .footer(|f| {
            f.text(FOOTERS.choose(&mut rand::thread_rng()).unwrap_or(&""))
        })
}