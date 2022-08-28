use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use std::{collections::HashMap, io::{BufReader, BufRead}, fs::File};
use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;

#[tokio::main()]
async fn main() {
    let client = Client::new();
    let joiner = Joiner::new("PtfYPQqY");
    let mut futs = FuturesUnordered::new();
    let tokens: Vec<_> = BufReader::new(File::open("./tokens.txt").unwrap()).lines().collect();
    for token in tokens {
        futs.push(joiner.join(client.clone(),token.unwrap()));
    }
    while let Some(_) = futs.next().await {
        println!("ok");
    }
    
}

#[derive(Serialize, Deserialize, Debug)]
struct Retry {
    code: String,
    global: bool,
    message: String,
    retry_after: String
}
#[derive(Serialize, Deserialize, Debug)]
struct SomeTing {
    error: HashMap<String, Value>
}

struct Joiner {
    code: String,
}

impl Joiner {
    fn new(code: &str) -> Self {
        Self { code: code.to_string() }
    }
    async fn join(&self, client: Client, token: String) {
        let url = format!("https://canary.discord.com/api/v9/invites/{}", &self.code);
        let resp = client.post(url)
            .header("authorization", &token)
            .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:102.0) Gecko/20100101 Firefox/102.0")
            .header("accept", "*/*")
            .header("accept-language", "fr,fr-FR;q=0.8,en-US;q=0.5,en;q=0.3")
            .header("accept-encoding", "gzip, deflate, br")
            .header("content-type", "application/json")
            .header("x-content-properties", "eyJsb2NhdGlvbiI6IkpvaW4gR3VpbGQiLCJsb2NhdGlvbl9ndWlsZF9pZCI6Ijk4OTkxOTY0NTY4MTE4ODk1NCIsImxvY2F0aW9uX2NoYW5uZWxfaWQiOiI5OTAzMTc0ODgxNzg4NjgyMjQiLCJsb2NhdGlvbl9jaGFubmVsX3R5cGUiOjB9")
            .header("x-super-properties", "eyJvcyI6IldpbmRvd3MiLCJicm93c2VyIjoiRmlyZWZveCIsImRldmljZSI6IiIsInN5c3RlbV9sb2NhbGUiOiJmciIsImJyb3dzZXJfdXNlcl9hZ2VudCI6Ik1vemlsbGEvNS4wIChXaW5kb3dzIE5UIDEwLjA7IFdpbjY0OyB4NjQ7IHJ2OjEwMi4wKSBHZWNrby8yMDEwMDEwMSBGaXJlZm94LzEwMi4wIiwiYnJvd3Nlcl92ZXJzaW9uIjoiMTAyLjAiLCJvc192ZXJzaW9uIjoiMTAiLCJyZWZlcnJlciI6IiIsInJlZmVycmluZ19kb21haW4iOiIiLCJyZWZlcnJlcl9jdXJyZW50IjoiIiwicmVmZXJyaW5nX2RvbWFpbl9jdXJyZW50IjoiIiwicmVsZWFzZV9jaGFubmVsIjoic3RhYmxlIiwiY2xpZW50X2J1aWxkX251bWJlciI6MTM2MjQwLCJjbGllbnRfZXZlbnRfc291cmNlIjpudWxsfQ==")
            .header("x-discord-locale", "en-US")
            .header("x-debug-options", "bugReporterEnabled")
            .header("origin", "https://discord.com")
            .header("dnt", "1")
            .header("connection", "keep-alive")
            .header("referer", "https://discord.com")
            .header("cookie", "__dcfduid=21183630021f11edb7e89582009dfd5e; __sdcfduid=21183631021f11edb7e89582009dfd5ee4936758ec8c8a248427f80a1732a58e4e71502891b76ca0584dc6fafa653638; locale=en-US")
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-origin")
            .header("te", "trailers")
            .json(&json!({}))
            .send().await.unwrap();
        if resp.status().as_u16() == 200 {
            println!("Successfully joined the server")
        } else if resp.status().as_u16() == 429 {
            let j = &resp.json::<Retry>().await.unwrap();
            println!("Ratelimited... {:?}", j);
        } else if resp.status().as_u16() == 403 {
            println!("Token is locked in some way");
        } else {
            let j = &resp.text().await.unwrap();
            println!("Error: {}", j);
        }

    }
}
