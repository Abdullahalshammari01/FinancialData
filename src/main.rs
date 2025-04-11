use serde::Deserialize;
use std::{fs::OpenOptions, io::Write, thread, time::Duration};

// Define data structures
#[derive(Debug, Deserialize)]
struct Bitcoin {
    timestamp: String,
    price: f64,
}



// Pricing trait definition
trait Pricing {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>>;
}

// Bitcoin implementation
impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let body: String = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd")
            .call()?
            .into_string()?;

        let response: serde_json::Value = serde_json::from_str(&body)?;
        self.price = response["bitcoin"]["usd"].as_f64().unwrap();
        self.timestamp = chrono::Local::now().to_rfc3339();
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("bitcoin_prices.csv")?;

        writeln!(file, "{},{}", self.timestamp, self.price)?;
        Ok(())
    }
}

// Ethereum implementation (similar structure)
#[derive(Debug, Deserialize)]
struct Ethereum {
    timestamp: String,
    price: f64,
}


// SP500 implementation (similar structure with different API endpoint)
#[derive(Debug, Deserialize)]
struct SP500 {
    timestamp: String,
    value: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin { timestamp: String::new(), price: 0.0 }),
        //Ethereum

        //SP500
    ];

    loop {
        for asset in &mut assets {
            match asset.fetch_price() {
                Ok(_) => {
                    asset.save_to_file()?;
                    println!("Successfully updated price");
                }
                Err(e) => eprintln!("Error fetching price: {}", e),
            }
        }
        thread::sleep(Duration::from_secs(10));
    }
}