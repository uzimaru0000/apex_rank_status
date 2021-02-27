use reqwest::RequestBuilder;

use crate::model::Stat;

pub enum Platform {
    Origin,
    PS,
    XBOX,
}

impl Platform {
    fn to_string(&self) -> String {
        match self {
            Platform::Origin => String::from("origin"),
            Platform::PS => String::from("ps"),
            Platform::XBOX => String::from("xbox"),
        }
    }
}

pub enum Rank {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Master,
    Predator,
}

impl Rank {
    pub fn to_emoji(&self) -> String {
        match self {
            Rank::Bronze => String::from("🟫"),
            Rank::Silver => String::from("⬜️"),
            Rank::Gold => String::from("🟨"),
            Rank::Platinum => String::from("🟦"),
            Rank::Diamond => String::from("💎"),
            Rank::Master => String::from("⚛️"),
            Rank::Predator => String::from("👹"),
        }
    }
}

pub fn get_status(api_key: &str, platform: Platform, user_identifier: &str) -> RequestBuilder {
    let url = create_endpoint(&platform.to_string(), user_identifier);
    let client = reqwest::Client::new();

    client.get(&url).header("TRN-Api-Key", api_key)
}

pub fn eval_rank(stat: &Stat) -> Rank {
    if 0f32 <= stat.value && stat.value < 1200f32 {
        Rank::Bronze
    } else if 1200f32 <= stat.value && stat.value < 2800f32 {
        Rank::Silver
    } else if 2800f32 <= stat.value && stat.value < 4800f32 {
        Rank::Gold
    } else if 4800f32 <= stat.value && stat.value < 7200f32 {
        Rank::Platinum
    } else if 7200f32 <= stat.value && stat.value < 10000f32 {
        Rank::Diamond
    } else if 10000f32 <= stat.value {
        if let Some(rank) = stat.rank {
            if rank <= 750 {
                Rank::Predator
            } else {
                Rank::Master
            }
        } else {
            Rank::Master
        }
    } else {
        panic!("OUT OF RANGE");
    }
}

fn create_endpoint(platform: &str, user_identifier: &str) -> String {
    format!(
        "https://public-api.tracker.gg/v2/apex/standard/profile/{}/{}",
        platform, user_identifier
    )
}
