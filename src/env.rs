use std::env;

#[derive(Debug)]
pub struct Env {
    pub trn_api_key: String,
    pub apex_user_identifier: String,
    pub twitter_user_name_template: String,
}

impl Env {
    pub fn init() -> Result<Self, env::VarError> {
        Ok(Env {
            trn_api_key: env::var("TRN_API_KEY")?,
            apex_user_identifier: env::var("APEX_USER_IDENTIFIER")?,
            twitter_user_name_template: env::var("TWITTER_USER_NAME_TEMPLATE")?,
        })
    }
}
