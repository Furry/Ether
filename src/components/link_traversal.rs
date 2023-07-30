// Module: src/components/link_traversal.rs --------------------------------------------------------- //
// Contains all libarary / feature components for Ether that should be isolated from command context  //
// -------------------------------------------------------------------------------------------------- //

use reqwest::{Client, StatusCode, Url};
use std::net::IpAddr;
use anyhow::bail;

// Represents a single hop in a link traversal
#[derive(Debug)]
pub struct Hop {
    pub status: StatusCode,
    pub url: Url,
    pub ip: IpAddr,
}

// Represents a full link traversal
#[derive(Debug)]
pub struct Traversal {
    pub end_status: StatusCode,
    pub hops: Vec<Hop>,
    pub source_url: Url,
    pub destination_url: Url,
    pub destination_ip: IpAddr,
}

/// .
///
/// # Panics
///
/// Panics if .
///
/// # Errors
///
/// This function will return an error if .
pub async fn traverse(origin: String) -> Result<Traversal, anyhow::Error> {
    let client = Client::builder().redirect(reqwest::redirect::Policy::none()).build()?;

    let mut hops: Vec<Hop> = Vec::new();
    let mut current_url = reqwest::Url::parse(&origin)?;

    loop {
        let response = client.get(current_url.clone()).send().await?;
        let status = response.status();
        let url = response.url().clone();
        let ip = match response.remote_addr() {
            Some(addr) => addr.ip(),
            None => {
                bail!("No remote address");
            }
        };

        // Add the last hop to the list
        hops.push(Hop {
            status,
            url: url.clone(),
            ip,
        });

        // If the response is not a redirection, we're done
        if !status.is_redirection() {
            return Ok(Traversal {
                end_status: status,
                hops,
                source_url: reqwest::Url::parse(&origin)?,
                destination_url: url,
                destination_ip: ip,
            });
        }

        // Following the redirect URL if the response header includes it
        if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
            current_url = Url::parse(location.to_str()?).unwrap();
        } else {
            bail!("No location header");
        }
    }
}
