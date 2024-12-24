use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

const BOT_TOKEN: &str = "BOT_TOKEN_BURAYA";
const GUILD_ID: &str = "GUILD_ID_BURAYA";
const TARGET_URL: &str = "hedef-url";
const API_URL: &str = "https://discord.com/api/v10";

#[tokio::main]
async fn main() {
    let client = Client::new();
    let endpoint = format!("{}/guilds/{}/vanity-url", API_URL, GUILD_ID);

    println!("Sniper çalışıyor, hedef URL: {}", TARGET_URL);

    loop {
        match check_and_claim(&client, &endpoint).await {
            Ok(_) => (),
            Err(e) => eprintln!("Hata: {}", e),
        }
        sleep(Duration::from_millis(50)).await; 
    }
}

async fn check_and_claim(client: &Client, endpoint: &str) -> Result<(), reqwest::Error> {
    let response = client
        .get(endpoint)
        .bearer_auth(BOT_TOKEN)
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        if json["code"] == TARGET_URL {
            println!("URL hâlâ kullanımda.");
        } else {
            println!("URL boşa düştü, alma denemesi yapılıyor...");
            let claim_response = client
                .patch(endpoint)
                .bearer_auth(BOT_TOKEN)
                .json(&serde_json::json!({ "code": TARGET_URL }))
                .send()
                .await?;

            if claim_response.status().is_success() {
                println!("URL başarıyla alındı!");
                std::process::exit(0);
            } else {
                println!("URL alma başarısız: {:?}", claim_response.text().await?);
            }
        }
    } else {
        eprintln!("API Hatası: {:?}", response.text().await?);
    }

    Ok(())
}
