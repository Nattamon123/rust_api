use anyhow::Result;
use super::{
    config_model::{AdventurersSecret, Database, DotEnvyConfig, GuildCommandersSecret, Server},
    stage::Stage,
};
pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();
    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("port not set")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("body_limit not set")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("timeout not set")
            .parse()?,
    };
    let database = Database {
        url: std::env::var("DATABASE_URL").expect("database_url not set"),
    };
    Ok(DotEnvyConfig { server, database })
}
pub fn get_stage() -> Stage{
    dotenvy::dotenv().ok();
    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(&stage_str).unwrap_or_default()
}
pub fn get_adventurer_secret() -> Result<AdventurersSecret> {
    dotenvy::dotenv().ok();
    Ok(AdventurersSecret {
        secret: std::env::var("JWT_ADVENTURER_SECRET").expect("JWT_ADVENTURER_SECRET not set"),
        refresh_secret: std::env::var("JWT_ADVENTURER_REFRESH_SECRET")
            .expect("JWT_ADVENTURER_REFRESH_SECRET not set"),
    })
}
pub fn get_guild_commanders_secret() -> Result<GuildCommandersSecret> {
    dotenvy::dotenv().ok();
    Ok(GuildCommandersSecret {
        secret: std::env::var("JWT_GUILD_COMMANDER_SECRET")
            .expect("JWT_GUILD_COMMANDER_SECRET not set"),
        refresh_secret: std::env::var("JWT_GUILD_COMMANDER_REFRESH_SECRET")
            .expect("JWT_GUILD_COMMANDER_REFRESH_SECRET not set"),
    })
}
