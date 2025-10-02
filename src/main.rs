use rust_api::config::config_loader;
use tracing::info;

#[tokio::main]
async  fn main() {
   tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
   let dot_env = match config_loader::load(){
       Ok(config) => config,
       Err(e) => {
           tracing::error!("Failed to load config: {}", e);
           std::process::exit(1);
       }
       
   };
   info!("env loaded");
}
