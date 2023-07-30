// Module: src/database/structures.rs --------------------------------------------------------//
// All database formats and structures are defined here, supporting conversion and creation.  //
// ------------------------------------------------------------------------------------------ //

// pub enum FeatureType {
//     LinkTraversal = 0,
//     AutoModeration = 1,
//     AutoRole = 2,
//     Leveling = 3,
//     CustomCommands = 4,
//     SlashCommands = 5,
//     Music = 6,
//     Moderation = 7,
//     Logging = 8
// }

use serenity::model::prelude::Guild;

#[derive(Debug, Default)]
pub enum FeatureInversionMode {
    Blacklist = 0,
    #[default]
    Whitelist = 1
}

#[derive(Debug, Default)]
pub enum ViolationOption {
    #[default]
    Delete = 0,
    Warn = 1,
    Kick = 2,
    Ban = 3
}

#[derive(Debug, Default)]
pub struct LinkTraversalModule {
    pub enabled: bool,
    pub channel_mode: FeatureInversionMode,
    pub role_mode: FeatureInversionMode,
    pub channels: Vec<u64>,
    pub roles: Vec<u64>
}


/// The main 'warnings' feature of Ether. ---------------------------
/// If enabled, exposes the following slash commands the guild
///   - /warn <user> <reason>
///   - /warnings
///     - view <user> -- view warnings for a user
///     - clear <user> <warningNumber> -- clear a warning for a user
///     - clearall <user> -- clear all warnings for a user
/// -----------------------------------------------------------------
#[derive(Debug)]
pub struct WarningsModule {
    pub enabled: bool,
    pub required_permissions: u64,
}

#[derive(Debug, Default)]
pub struct AutoModerationModule {
    pub enabled: bool,
    pub warn_threshold: u8,
    pub warn_threshold_hit_action: ViolationOption,
    pub warn_threshold_hit_exempt: Vec<u64>,
    pub warn_expiry_time: u64,

    pub regex_enabled: bool,
    pub regex_expressions: Vec<String>,
    pub regex_action: ViolationOption,
    pub regex_exempt: Vec<u64>,

    pub invite_enabled: bool,
    pub invite_action: ViolationOption,
    pub invite_exempt: Vec<u64>,

    pub mass_mention_enabled: bool,
    pub mass_mention_threshold: u8,
    pub mass_mention_action: ViolationOption,
    pub mass_mention_exempt: Vec<u64>,

    pub spam_enabled: bool,
    pub spam_threshold: u8,
    pub spam_timeframe: u8,
    pub spam_action: ViolationOption,

    pub profanity_enabled: bool,
    pub profanity_action: ViolationOption,
    pub profanity_exempt: Vec<u64>,
    pub profanity_words: Vec<String>
}

#[derive(Debug)]
#[repr(u8)]
pub enum Features {
    LinkTraversal = 1,
    AutoModeration = 2
}

#[derive(Debug, Default)]
pub struct FeatureOptions {
    pub auto_moderation: AutoModerationModule,
    pub link_traversal: LinkTraversalModule
}

#[derive(Debug)]
pub struct DBGuild {
    pub id: u64,
    pub owner_id: u64,
    pub name: String,
    pub icon: String,
    pub active_features: u64,
    pub features: FeatureOptions
}

impl DBGuild {
    pub fn to_default(guild: &Guild) -> DBGuild {
        DBGuild {
            id: guild.id.as_u64().clone(),
            owner_id: guild.owner_id.as_u64().clone(),
            name: guild.name.clone(),
            icon: guild.icon_url().unwrap_or("".to_string()),
            active_features: 0,
            features: FeatureOptions::default()
        }
    }
}