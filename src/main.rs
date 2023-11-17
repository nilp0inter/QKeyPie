use rocket::{get, routes};
use rocket::tokio::task::spawn_blocking;
use xencelabs_quick_keys::{QKDevice, ConnectionMode, QKResult, ScreenOrientation, ScreenBrightness, WheelSpeed, Event, ButtonState};
use hidapi::HidApi;
use std::{thread,time};
use enigo::*;


struct SharedData {
    // Add shared data fields here if necessary
}

#[get("/")]
async fn index(_data: &rocket::State<SharedData>) -> String {
    // Handle the HTTP request, e.g., returning a simple response
    "Hello from Rocket!".to_string()
}

#[rocket::main]
async fn main() {
    let shared_data = SharedData {
        // Initialize shared data fields here
    };

    // Spawn a blocking task for managing the Xencelabs Quick Keys device
    tokio::spawn(async {
        spawn_blocking(move || {
            if let Ok(api) = HidApi::new() {
                match run(api) {
                    Ok(_) => println!("Device managed successfully"),
                    Err(e) => println!("Error managing device: {:?}", e),
                }
            } else {
                eprintln!("Error initializing HidApi");
            }
        }).await.unwrap();
    });

    // Launch the Rocket application
    rocket::build()
        .mount("/", routes![index])
        .manage(shared_data)
        .launch()
        .await
        .unwrap();
}

fn run(api: HidApi) -> QKResult<()> {
    let mut enigo = Enigo::new();
    match QKDevice::open(api, ConnectionMode::Auto) {
        Ok(dev) => {
            // Set device settings such as orientation, brightness, etc.
            dev.set_screen_orientation(ScreenOrientation::Rotate270)?;
            dev.set_screen_brightness(ScreenBrightness::Medium)?;
            dev.set_wheel_speed(WheelSpeed::Faster)?;
            dev.set_ring_color(0, 0, 0)?;

            thread::sleep(time::Duration::from_millis(1000));
            // Display "Hello World" on the device
            dev.show_overlay_text("Hello World", 1)?;
            loop {
                match dev.read() {
                    Ok(ev) => match ev {
                        Event::Button { state: ButtonState { button_7: true, .. } } => Ok(enigo.key_sequence("hello world")),
                        _ => Ok(()),
                    },
                    Err(e) => Err(e),
                }?;
            }
        },
        Err(e) => {
            println!("Connection error: {:?}", e);
            Err(e)
        },
    }
}
