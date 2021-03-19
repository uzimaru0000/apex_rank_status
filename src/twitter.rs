use kuon::TwitterAPI;
use std::collections::HashMap;

pub async fn update_profile(
    api: TwitterAPI,
    params: &HashMap<&str, &str>,
) -> Result<kuon::User, kuon::Error> {
    let url = "https://api.twitter.com/1.1/account/update_profile.json";
    api.raw_post::<kuon::User>(url, params).await
}
