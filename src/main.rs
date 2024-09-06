<<<<<<< Updated upstream
fn main() {
    println!("Hello, world!");
=======
//Simple DDNS utility to update Baremetal.com domains.
//Gets public ipv4 from ipify.org
//
//Usage:
//baremetal_ddns HOST key

use core::panic;
use std::{collections::HashMap, env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough args!")
    }

    let response = reqwest::get("https://api.ipify.org?format=json")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    let ip = &response["ip"];
    let host = &args[1];
    let key = &args[2];
    let url = format!(
        "https://domain-dns.com/ip.cgi?host={}&key={}&ip={}",
        host, key, ip
    );

    let response = reqwest::get(url).await?.text().await?;
    println!("{}", response);

    Ok(())
>>>>>>> Stashed changes
}
