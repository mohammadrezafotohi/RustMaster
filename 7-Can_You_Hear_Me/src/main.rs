use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream, SupportedStreamConfig};
use crossterm::cursor::MoveTo;
use crossterm::style::SetForegroundColor;
use std::thread::sleep;
use std::time::Duration;
use std::process::Command;
use crossterm::{ExecutableCommand, terminal, style::{self, Color}};
use std::io::{stdout, Write};
use std::io;

fn main() {
    let banner = 
    r#"
██▓    ▄████▄   ▄▄▄       ███▄    █     ██░ ██ ▓█████ ▄▄▄       ██▀███     ▓██   ██▓ ▒█████   █    ██  ▐██▌ 
▓██▒   ▒██▀ ▀█  ▒████▄     ██ ▀█   █    ▓██░ ██▒▓█   ▀▒████▄    ▓██ ▒ ██▒    ▒██  ██▒▒██▒  ██▒ ██  ▓██▒ ▐██▌ 
▒██▒   ▒▓█    ▄ ▒██  ▀█▄  ▓██  ▀█ ██▒   ▒██▀▀██░▒███  ▒██  ▀█▄  ▓██ ░▄█ ▒     ▒██ ██░▒██░  ██▒▓██  ▒██░ ▐██▌ 
░██░   ▒▓▓▄ ▄██▒░██▄▄▄▄██ ▓██▒  ▐▌██▒   ░▓█ ░██ ▒▓█  ▄░██▄▄▄▄██ ▒██▀▀█▄       ░ ▐██▓░▒██   ██░▓▓█  ░██░ ▓██▒ 
░██░   ▒ ▓███▀ ░ ▓█   ▓██▒▒██░   ▓██░   ░▓█▒░██▓░▒████▒▓█   ▓██▒░██▓ ▒██▒     ░ ██▒▓░░ ████▓▒░▒▒█████▓  ▒▄▄  
░▓     ░ ░▒ ▒  ░ ▒▒   ▓▒█░░ ▒░   ▒ ▒     ▒ ░░▒░▒░░ ▒░ ░▒▒   ▓▒█░░ ▒▓ ░▒▓░      ██▒▒▒ ░ ▒░▒░▒░ ░▒▓▒ ▒ ▒  ░▀▀▒ 
 ▒ ░     ░  ▒     ▒   ▒▒ ░░ ░░   ░ ▒░    ▒ ░▒░ ░ ░ ░  ░ ▒   ▒▒ ░  ░▒ ░ ▒░    ▓██ ░▒░   ░ ▒ ▒░ ░░▒░ ░ ░  ░  ░ 
 ▒ ░   ░          ░   ▒      ░   ░ ░     ░  ░░ ░   ░    ░   ▒     ░░   ░     ▒ ▒ ░░  ░ ░ ░ ▒   ░░░ ░ ░     ░ 
 ░     ░ ░            ░  ░         ░     ░  ░  ░   ░  ░     ░  ░   ░         ░ ░         ░ ░     ░      ░    
       ░                                                                     ░ ░                             
"#;

    // Input Device Settings
    let host = cpal::default_host();
    let default_microphone = host.default_input_device().expect("input device unavailable");
    let mut supported_configs_range = default_microphone.supported_input_configs().expect("!");
    let supported_config = supported_configs_range.next().expect("no supported config!").with_max_sample_rate();

    // Voice Flow
    let sample_format = supported_config.sample_format();
    let config = supported_config.into();
    let stream = default_microphone.build_input_stream(&config, 
        move |data: &[f32], _| 
        {let rms = calculate_rms(data);
            
            print_banner(banner);
            print_height(rms);
            //println!("{}", rms);
            std::thread::sleep(std::time::Duration::from_millis(15));
            clear_console();
        },
        move |err| {
            print_banner(banner);
            eprintln!("Error: {:?}", err);
            std::thread::sleep(std::time::Duration::from_millis(15));
            clear_console();
        },
        None,
    ).expect("Failed to create stream!");

    stream.play().expect("Strem Failed!");
    std::thread::sleep(std::time::Duration::from_millis(300));
}

// Root Mean Square of Listened Data
fn calculate_rms(data: &[f32]) -> f32 {
    let sum_of_squares: f32 = data.iter().map(|&sample| sample * sample).sum();
    let rms = (sum_of_squares / data.len() as f32).sqrt();
    rms
}

fn clear_console() {
    stdout().execute(terminal::Clear(terminal::ClearType::All))
        .expect("Failed to clear console");
    stdout().execute(MoveTo(0, 0)).expect("Faild to move up!");
}

fn print_banner(banner: &str) {
    stdout().execute(style::SetForegroundColor(Color::Red)).unwrap();
    println!("{}\n\n\n", banner);
    stdout().execute(style::SetForegroundColor(Color::Grey)).unwrap();
}

fn print_height(rms: f32) {
    let val = rms * 1000.0 / 2.0;
    stdout().execute(style::SetForegroundColor(Color::Blue)).unwrap();
    println!("\t\t###################################################################################");
    print!("\t\t");
    stdout().execute(style::SetForegroundColor(Color::Green)).unwrap();
    for _ in 0..val as i32 {
        print!("#");
        io::stdout().flush().unwrap();
    }
    print!("#");
    stdout().execute(style::SetForegroundColor(Color::Blue)).unwrap();
    println!("\n\t\t###################################################################################");
    stdout().execute(style::SetForegroundColor(Color::Grey)).unwrap();
}