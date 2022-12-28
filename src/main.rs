#[macro_use]
extern crate rocket;

use std::net::{IpAddr, Ipv4Addr};

use rocket::{Config, Shutdown};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use structopt_flags::GetWithDefault;
use structopt_flags::HostOpt;
use uuid::Uuid;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "c", long = "client")]
    client_id: String,

    #[structopt(short = "s", long = "secret")]
    client_secret: String,

    #[structopt(
    short = "r",
    long = "redirect"
    )]
    redirect_uri: Option<String>,

    #[structopt(
    short = "o",
    long = "scopes",
    default_value = "read_thermostat+write_thermostat"
    )]
    scopes: String,

    #[structopt(short = "p", long = "port", default_value = "9090")]
    port: u16,

    #[structopt(flatten)]
    host_ip: HostOpt,
}

const HOST_DEFAULT: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u32,
}

async fn exchange_token(
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
    scopes: &str,
) -> Option<Login> {
    let body = [
        ("grant_type", "authorization_code"),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("scope", scopes),
    ];

    let response = reqwest::Client::new()
        .post("https://api.netatmo.com/oauth2/token")
        .form(&body)
        .send()
        .await
        .unwrap();

    let mut result: Option<Login> = None;

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<Login>().await {
                Ok(parsed) => result = Option::from(parsed),
                Err(_) => println!("Error parsing Netatmo /oauth2/token API response"),
            };
        }
        other => {
            println!(
                "Unexpected error during Netatmo /oauth2/token API CALL: {:?}, {:?}",
                other, &response
            );
        }
    }

    result
}

fn auth_url(client_id: &str, redirect_uri: &str, scopes: &str) -> String {
    let state = Uuid::new_v4();

    let params = [
        format!("client_id={}", client_id),
        format!("redirect_uri={}", redirect_uri),
        format!("scope={}", scopes),
        format!("state={}", state),
    ]
        .join("&");

    format!("https://api.netatmo.com/oauth2/authorize?{}", params)
}

#[get("/?<state>&<code>")]
async fn success(state: &str, code: &str, shutdown: Shutdown) -> String {
    let opt = Opt::from_args();

    let login_result = exchange_token(
        &opt.client_id,
        &opt.client_secret,
        &code,
        &get_redirect_uri(),
        &opt.scopes.replace("+", " "),
    )
        .await;

    let mut result = format!(
        "\n2. State and code:\n  State: {}\n  Code: {}\n\n3. Access an refresh tokens:\n  None",
        state, code
    );

    if login_result.is_some() {
        let login = login_result.unwrap();

        result = format!("\n2. State and code:\n  State: {}\n  Code: {}\n\n3. Access an refresh tokens:\n  access_token: {}\n  refresh_token: {}\n  expires_in: {}",
                         state, code, login.access_token, login.refresh_token, login.expires_in);
    }

    println!("{}", &result);
    println!("\n4. Finished");

    shutdown.notify();
    result
}

fn get_redirect_uri() -> String {
    let opt = Opt::from_args();
    let config = Config {
        address: opt.host_ip.get_with_default(HOST_DEFAULT),
        port: opt.port,
        ..Config::release_default()
    };
    let result;

    if opt.redirect_uri.is_none() {
        result = format!("{}://{}:{}", "http", config.address, opt.port);
    } else {
        result = opt.redirect_uri.unwrap();
    }

    result
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    println!("Netatmo OAUTH login\n");
    let opt = Opt::from_args();
    let config = Config {
        address: opt.host_ip.get_with_default(HOST_DEFAULT),
        port: opt.port,
        ..Config::release_default()
    };

    let rocket = rocket::custom(&config)
        .mount("/", routes![success])
        .ignite()
        .await?;

    let web_browser_url = auth_url(&opt.client_id, &get_redirect_uri(), &opt.scopes);
    println!("1. Paste URL to your browser:");
    println!("  {}", web_browser_url);

    let _rocket = rocket.launch().await?;

    Ok(())
}
