use anyhow::Result;



use super::config_model::{DotEnvyConfig,Server,Database};

pub fn load() -> Result<DotEnvyConfig>{
    dotenvy :: dotenv().ok();
    let server =  Server{
        port : std::env::var("SERVER_PORT").expect("port not set").parse()?,
        body_limit : std::env::var("SERVER_BODY_LIMIT").expect("body_limit not set").parse()?,
        timeout : std::env::var("SERVER_TIMEOUT").expect("timeout not set").parse()?
    };
    let database = Database{
    url : std::env::var("DATABASE_URL").expect("database_url not set")
};

Ok(DotEnvyConfig{server,database})
}
