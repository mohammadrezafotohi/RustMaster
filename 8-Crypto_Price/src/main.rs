use surf;
use crossterm::style::SetForegroundColor;
use crossterm::{ExecutableCommand, terminal, style::{self, Color}};
use crossterm::cursor::MoveTo;
use std::io::{stdout, Write};

#[derive(serde::Deserialize)]
struct DumpedInformation {
    bitcoin: SubInformation,
    ethereum: SubInformation,
}

#[derive(serde::Deserialize)]
struct SubInformation {
    usd: f64,
    usd_market_cap: f64,
    usd_24h_vol: f64,
    usd_24h_change: f64,
}

#[tokio::main]
async fn main() {
    let address = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd&include_market_cap=true&include_24hr_vol=true&include_24hr_change=true";
    loop {
        let api_response: Result<DumpedInformation, surf::Error> = surf::get(address).recv_json().await;
        clear_console();
        match api_response {
            Ok(api_response) => {
                stdout().execute(style::SetForegroundColor(Color::Green)).unwrap();
                println!("\t\tBitCoin");
                println!("\t\tCurrent Price: {}$", api_response.bitcoin.usd);
                println!("\t\tMarket Cap: {}$", api_response.bitcoin.usd_market_cap);
                println!("\t\t24 Hours Volume: {}$", api_response.bitcoin.usd_24h_vol);
                println!("\t\t24 Hours Change: {}$", api_response.bitcoin.usd_24h_change);

                println!("\n\t\tEtheteum");
                println!("\t\tCurrent Price: {}$", api_response.ethereum.usd);
                println!("\t\tMarket Cap: {}$", api_response.ethereum.usd_market_cap);
                println!("\t\t24 Hours Volume: {}$", api_response.ethereum.usd_24h_vol);
                println!("\t\t24 Hours Change: {}$", api_response.ethereum.usd_24h_change);
                stdout().execute(style::SetForegroundColor(Color::Grey)).unwrap();
                std::thread::sleep(std::time::Duration::from_secs(10));
                },
            Err(e) => {
                println!("Error! {}",e);
                std::thread::sleep(std::time::Duration::from_secs(3));
            }
        }
    }
}

fn clear_console() {
    stdout().execute(terminal::Clear(terminal::ClearType::All))
        .expect("Failed to clear console");
    stdout().execute(MoveTo(0, 0)).expect("Faild to move up!");
}