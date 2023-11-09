use std::collections::HashMap;

use poise::serenity_prelude::{GuildId, Event};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Events {
    MemberJoin,
    MemberLeave,
    MessageSent
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Comparison {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    Includes
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Actions {
    SendMessage
}

pub struct QueueEntry;
impl QueueEntry {
    pub fn step(event: Event) {
        
    }
}

pub struct CommandQueue {
    pub queues: HashMap<GuildId,
        Vec<QueueEntry>>
}