// // use xencelabs_quick_keys::{QKDevice, ConnectionMode, QKResult, ScreenOrientation, ScreenBrightness, WheelSpeed, Event, ButtonState};
// // use hidapi::HidApi;
// // use std::{thread,time};
// // use std::sync::{Arc, Mutex};
// // use enigo::*;

mod config;
mod actions;
mod model;

fn model_from_config() -> anyhow::Result<model::Model> {
    let cfg = config::read_config("config.toml")?;
    println!("Config = {:?}", cfg);
    let model = model::from_config(cfg)?;
    Ok(model)
}

fn main() {
    let model = model_from_config();
    println!("Model = {:?}", model);
}
    
