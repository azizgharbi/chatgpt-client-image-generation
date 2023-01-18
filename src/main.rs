use reqwest;
use serde_json::Value;
use std::collections::HashMap;
use std::io;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("Enter a description to generate an image :");
        let mut prompt = String::new();
        io::stdin().read_line(&mut prompt)?;
        let text: Result<String, reqwest::Error> =
            get_chat_gpt_response(&prompt, "images/generations").await;

        match text {
            Ok(t) => println!("{}", t.trim_start_matches("\n")),
            Err(e) => println!("Error: {}", e),
        }

        println!("Do you want to enter another search? (y/n)");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        if choice.trim().to_lowercase() != "y" {
            println!("GoodBye!");
            break;
        }
    }
    Ok(())
}

async fn get_chat_gpt_response(prompt: &String, slug: &str) -> Result<String, reqwest::Error> {
    let api_key = "YOUR_KEY";
    let map = HashMap::from([("prompt", &prompt)]);
    let endpoint = "https://api.openai.com/v1/".to_string() + &slug;
    println!("{}", endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&endpoint)
        .bearer_auth(api_key)
        .json(&map)
        .send()
        .await?;

    let response_text = res.text().await?;
    let response: Result<Value, serde_json::Error> = serde_json::from_str(&response_text);

    match response {
        Ok(value) => {
            let text: String = value["data"][0]["url"].to_string();
            Ok(text)
        }
        Err(e) => {
            println!("Error: {}", e);
            Ok(e.to_string())
        }
    }
}
