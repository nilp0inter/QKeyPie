use rocket::{get, routes, State};
use rocket::tokio::task::spawn_blocking;
use xencelabs_quick_keys::{QKDevice, ConnectionMode, QKResult, ScreenOrientation, ScreenBrightness, WheelSpeed, Event, ButtonState};
use hidapi::HidApi;
use std::{thread,time};
use std::sync::{Arc, Mutex};
use enigo::*;
use rlua::{Lua, Function};


struct SharedData {
    count: i32,
}

#[get("/increment")]
async fn increment(data: &State<Arc<Mutex<SharedData>>>) -> String {
    let mut data = data.lock().unwrap();
    data.count += 1;
    format!("Count incremented to: {}", data.count)
}

#[get("/get_count")]
async fn get_count(data: &State<Arc<Mutex<SharedData>>>) -> String {
    let data = data.lock().unwrap();
    format!("Current count is: {}", data.count)
}

#[get("/")]
async fn index(_data: &rocket::State<SharedData>) -> String {
    // Handle the HTTP request, e.g., returning a simple response
    "Hello from Rocket!".to_string()
}

#[rocket::main]
async fn main() {
    let shared_data = Arc::new(Mutex::new(SharedData { count: 0 }));

    let data_for_thread = Arc::clone(&shared_data);

    // Spawn a blocking task for managing the Xencelabs Quick Keys device
    tokio::spawn(async {
        spawn_blocking(move || {
            if let Ok(api) = HidApi::new() {
                match run(api, data_for_thread) {
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
        .mount("/", routes![increment, get_count])
        .manage(shared_data)
        .launch()
        .await
        .unwrap();
}

fn run(api: HidApi, data_for_thread: Arc<Mutex<SharedData>>) -> QKResult<()> {

    let lua = Lua::new();
    let mut enigo = Enigo::new();
    let mut current_count : i32 = 0;

    match QKDevice::open(api, ConnectionMode::Auto) {
        Ok(dev) => {
            // Wrap the device in Arc and Mutex for shared ownership and thread safety
            let sdev = Arc::new(Mutex::new(dev));
            lua.context(|lua_ctx| {
                let globals = lua_ctx.globals();
                let print: Function = globals.get("print").unwrap();
                // globals.set("vec2", vec2_constructor)?;
                let _ = print.call::<_, ()>("hello from rust");
                // This doesn't work we have to put dev behind a Mutex to make it thread safe
                // Example: static ref db = Mutex::new(mysqldata::MySQLData::init_connection(&String::from("rustsite"), &String::from("root"), &String::from("toor")));
                // let say_hi = lua_ctx.create_function(|_, (s): String| Ok(dev.show_overlay_text(&s, 3).unwrap())).unwrap();

                let sdev_clone = Arc::clone(&sdev);
                let say_hi = lua_ctx.create_function(move |_, (n): i32| {
                    let mut dev = sdev_clone.lock().unwrap();
                    Ok(dev.set_key_text(6, &format!("C: {}", n)).unwrap())
                            
                    // println!("Hello from lua: {}", s);
                    // Ok(())
                }).unwrap();
                globals.set("say_hi", say_hi).unwrap();


                {
                    let mut dev = sdev.lock().unwrap();
                    // Set device settings such as orientation, brightness, etc.
                    dev.set_screen_orientation(ScreenOrientation::Rotate180)?;
                    dev.set_screen_brightness(ScreenBrightness::Medium)?;
                    dev.set_wheel_speed(WheelSpeed::Faster)?;
                    dev.set_ring_color(0, 0, 0)?;

                    thread::sleep(time::Duration::from_millis(1000));
                    // Display "Hello World" on the device
                    dev.show_overlay_text("Hello World", 1)?;
                }
                loop {
                    let mut is_pressed = false;
                    {
                        let mut dev = sdev.lock().unwrap();
                        let data = data_for_thread.lock().unwrap();
                        if data.count != current_count {
                            current_count = data.count;
                            dev.set_key_text(7, &format!("C: {}", current_count))?;
                        }
                        // println!("Count in background task: {}", data.count);
                    }
                    {
                        let mut dev = sdev.lock().unwrap();
                        match dev.read_timeout(10) {
                            Ok(ev) => match ev {
                                Event::Button { state: ButtonState { button_7: true, .. } } => Ok(enigo.key_sequence("hello world")),
                                Event::Button { state: ButtonState { button_6: true, .. } } => { is_pressed = true; Ok(()) },
                                _ => Ok(()),
                            },
                            Err(e) => Err(e),
                        }?;
                    }
                    if is_pressed {
                        lua_ctx.load(&format!(r#"say_hi({})"#, current_count)).eval::<()>().unwrap();
                        is_pressed = false;
                    }
                }
            })
        },
        Err(e) => {
            println!("Connection error: {:?}", e);
            Err(e)
        },
    }
}
