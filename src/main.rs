use clap::Parser;
use rdev::{grab, Event, EventType, Key};
use std::process::Command;
use std::sync::{
    atomic::{AtomicBool, Ordering},
};

static ALLOW_LOCK: AtomicBool = AtomicBool::new(true);

fn get_status() -> &'static str {
    let activated = ALLOW_LOCK.load(Ordering::Relaxed);
    return if activated {"activated"} else {"deactivated"};
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long, default_value_t = false)]
    start_disabled: bool,

    #[arg(long, default_value_t = ("".to_string()))]
    say: String,

    #[arg(short, default_value_t = false)]
    volume_max: bool
}

fn main() {
    let args = CliArgs::parse();
    if args.start_disabled {
        ALLOW_LOCK.store(false, Ordering::Relaxed)
    }
    println!("KC started - {}", get_status());
    if let Err(error) = grab(move |e | {
        event_handler(e, &args)
    }) {
        println!("Error: {:?}", error)
    }

    println!("exiting")
}

fn say(message: &str) -> Result<(), String> {
    let output = Command::new("say")
        .arg(message)
        .output()
        .map_err(|e| format!("Failed to run 'say' command: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "'say' command failed with exit code {:?}",
            output.status.code()
        ));
    }

    Ok(())
}

fn set_system_volume(volume: i32) -> Result<(), String> {
    let script = format!("set volume output volume {}", volume);
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to run 'osascript' command: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "'osascript' command failed with exit code {:?}",
            output.status.code()
        ));
    }

    Ok(())
}



fn event_handler(event: Event, args: &CliArgs) -> Option<Event> {
    // println!("My callback {:?}", event);
    match event.event_type {
        EventType::KeyPress(Key::F9) => {
            let prev_val = ALLOW_LOCK.load(Ordering::Relaxed);
            ALLOW_LOCK.store(!prev_val, Ordering::Relaxed);
            println!("KC {}", get_status());
            return None
        }
        EventType::KeyRelease(Key::F9) |
        EventType::KeyPress(Key::Function) |
        EventType::KeyRelease(Key::Function) |
        EventType::KeyPress(Key::Unknown(179)) |
        EventType::KeyRelease(Key::Unknown(179)) => None,
        _ => {
            // println!("{:?}", other);
            if ALLOW_LOCK.load(Ordering::Relaxed)   {
                ALLOW_LOCK.store(false, Ordering::Relaxed);
                println!("Trap Triggered!!");
                if args.volume_max {
                    _ = set_system_volume(100);
                }
                if args.say != "" {
                    _ = say(&args.say);
                }
                lock_shortcut();
                println!("KC {}", get_status());
                return None;
            }
            return Some(event)
            
        },
    }
}

fn lock_shortcut() {
    println!("Locking machine");
    if let Err(error) = Command::new("pmset")
        .arg("displaysleepnow")
        .output()
    {
        println!("Error locking the screen: {:?}", error);
    }
}

