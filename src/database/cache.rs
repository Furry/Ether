use std::collections::HashMap;

use futures::TryStreamExt;
use mongodb::Collection;
use poise::serenity_prelude::UserId;


use crate::commands::profile::ProfileInfo;

#[derive(Debug)]
pub struct ProfileCache {
    pub users: HashMap<UserId, Vec<ProfileInfo>>
}

// pub struct ProfileInfoPartial {
//     name: String,
//     activation: String,
//     image_url: String,
//     owner: u64
// }

pub async fn populated_cache() -> Result<ProfileCache, crate::Error> {

    println!("Populating Cache...");
    let db = crate::MONGO.try_lock()
        .unwrap()
        .clone()
        .unwrap();

    dbg!("Handle acquired");
    let col: Collection<ProfileInfo> = db.database("Ether").collection::<_>("profiles");

    let profiles = col.find(mongodb::bson::doc! {}, None)
        .await.expect("Could not fetch profiles")
        .with_type::<ProfileInfo>().try_collect::<Vec<ProfileInfo>>().await?;

    dbg!("Profiles fetched");

    let mut cache = ProfileCache {
        users: HashMap::new()
    };

    dbg!("Cache created");
    for profile in profiles {
        if cache.users.contains_key(&UserId(profile.owner)) {
            cache.users.get_mut(&UserId(profile.owner)).unwrap().push(profile);
        } else {
            cache.users.insert(UserId(profile.owner), vec![profile]);
        }
    }

    dbg!("Cache populated");

    Ok(cache)
}