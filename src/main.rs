use rdev::{grab, Event, EventType, Key};
use std::process::Command;
use std::sync::{
    atomic::{AtomicBool, Ordering},
};

static ALLOW_LOCK: AtomicBool = AtomicBool::new(true);

fn main() {
    println!("KC started and activated");
    if let Err(error) = grab(callback) {
        println!("Error: {:?}", error)
    }
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



fn callback(event: Event) -> Option<Event> {
    // println!("My callback {:?}", event);
    match event.event_type {
        EventType::KeyPress(Key::F9) => {
            let prev_val = ALLOW_LOCK.load(Ordering::Relaxed);
            let status = if !prev_val {"activated"} else {"deactivated"};
            println!("KC {}", status);
            ALLOW_LOCK.store(!prev_val, Ordering::Relaxed);
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
                // set_system_volume(100);
                // say("Hey you! Don't even think about it");
                lock_shortcut();
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

