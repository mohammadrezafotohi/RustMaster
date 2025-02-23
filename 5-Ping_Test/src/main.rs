use ping::{ping};
use std::net::IpAddr;
use std::str::FromStr;
use std::time::{Duration, Instant};

fn main() {
    let mut window = simple::Window::new("5-Ping_Test", 800, 400);
    let timeout_duration = Duration::from_secs(1);
    let mut globe_image = window.load_image(include_bytes!("../globe.png")).unwrap();
    let (r, g, b) = (0, 0, 0);
    while window.next_frame() {
        window.clear_to_color(r, g, b);
        window.draw_image(&mut globe_image, 0, 0);
        ping_google(&mut window, timeout_duration);
    }
}

fn ping_google(window: &mut simple::Window, timeout_duration: Duration) {

    let start = Instant::now();

    let result = ping(
        IpAddr::from_str("8.8.8.8").unwrap(),
        Some(timeout_duration),
        Some(64),
        Some(12345),
        Some(4),
        None
    );

    let elapsed = start.elapsed().as_millis();
    
    match result {
        Ok(_) => {
            let ping_in_ms = format!("Your ping is {} ms!", elapsed);
            window.print(&ping_in_ms, 800/2 - 100, 400/2);
        },
        Err(e) => {
            let str_error = format!("{}", e);
           window.print("Failed to ping google!", 800/2 - 100, 400/2);
           window.print(&str_error, 10, 400/2 + 20);
        },
    };
}