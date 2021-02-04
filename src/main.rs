use anyhow::Result;
use apex_twitter_status::{apex, env, model::Model, twitter};
use maplit::hashmap;

#[tokio::main]
async fn main() -> Result<()> {
    let env = env::Env::init()?;

    let res = apex::get_status(
        &env.trn_api_key,
        apex::PlatFrom::Origin,
        &env.apex_user_identifier,
    )
    .send()
    .await?
    .json::<Model>()
    .await?;

    let rank_status = res
        .data
        .segments
        .get(0)
        .and_then(|x| x.stats.get("rankScore"))
        .unwrap();

    let rp = rank_status.value;
    let rank = apex::eval_rank(rank_status);
    let name = env
        .twitter_user_name_template
        .replace("{}", &format!("{}{}", rank.to_emoji(), rp.to_string()));

    let api = kuon::TwitterAPI::new_using_env().await?;
    let res = twitter::update_profile(api, &hashmap! { "name" => name.as_str() }).await?;

    println!("{:?}", res);
    Ok(())
}
