use std::process::exit;

use reqwest;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// health-check endpoint
    #[arg(short, long)]
    url: String,

    /// expected http status codes
    #[arg(short, long, value_delimiter = ',', num_args=1..)]
    status_codes: Vec<u16>,
}

fn healthcheck(url: String, status_codes: Vec<u16>) -> i32 {
    return match reqwest::blocking::get(url) {
        Err(_) => 1,
        Ok(r) => {
            for code in status_codes {
                if r.status().as_u16() == code {
                    return 0;
                }
            }
            return 1;
        }
    }
}

fn main() {
    let args = Args::parse();
    exit(healthcheck(args.url, args.status_codes));
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn test_200() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/health");
            then.status(200)
                .header("content-type", "text/html")
                .body("");
        });
        assert_eq!(healthcheck(server.url("/health"), vec![200]), 0);
    }

    #[test]
    fn test_201() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/health");
            then.status(201)
                .header("content-type", "text/html")
                .body("");
        });
        assert_eq!(healthcheck(server.url("/health"), vec![201]), 0);
    }

    #[test]
    fn test_20x() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/health");
            then.status(299)
                .header("content-type", "text/html")
                .body("");
        });
        let codes: Vec<u16> = (200..=299).collect();
        assert_eq!(healthcheck(server.url("/health"), codes), 0);
    }
}
