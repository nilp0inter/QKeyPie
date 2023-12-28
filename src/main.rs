mod config;
mod actions;
mod model;
mod controller;
mod events;
mod state;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // start server::main() in a separate thread
    server::main().await;

    let controller = tokio::spawn(async {
        // start the controller in the main thread
        let cfg = config::read_config("config.toml")?;
        let model = model::from_config(cfg)?;
        controller::run(model)
    }); 

    // wait for the controller to finish
    controller.await??;

    Ok(())
}
    
