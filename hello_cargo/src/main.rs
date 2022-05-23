use regex::Regex;
use reqwest::header::USER_AGENT;
use semver::{Version, VersionReq};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,       // "v1.3.1"
    name: String,           // "1.3.1 / 2021-12-01"
    draft: bool,
    prerelease: bool,
    assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,           // "node_exporter-1.3.1.darwin-amd64.tar.gz"
    // "https://github.com/prometheus/node_exporter/releases/download/v1.3.1/node_exporter-1.3.1.darwin-amd64.tar.gz"
    browser_download_url: String,
}

// Feel like this is not really proper...
//
fn extract_semver(tag: &str) -> &str {
    if tag.len() < 1 {
        return &tag
    }
    if tag.chars().nth(0).unwrap() == 'v' {
        return &tag[1..]
    } else {
        return &tag
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // let owner = "prometheus";
    // let repo = "node_exporter";
    // let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);

    let client = reqwest::Client::new();

    let releases: Vec<Release> = client
        .get("https://api.github.com/repos/prometheus/node_exporter/releases")
        .header(USER_AGENT, "get-latest-matching-semver-assets")
        .send()
        .await?
        .json()
        .await?;
        
    println!("{:#?}", releases);

    let constraint = VersionReq::parse("1.2.*").unwrap();
    let asset_regexes = [
        Regex::new(r"^node_exporter-.*\.linux-arm64\.tar\.gz$").unwrap(),  // would be nice to inject architecture (rewritten...)
        Regex::new(r"^sha256sums\.txt$").unwrap(),
    ];

    for release in releases.iter() {
        
        println!("Release with tag '{}' and named '{}': draft: {}, prerelease: {}",
            release.tag_name, release.name, release.draft, release.prerelease);

        let version = Version::parse(extract_semver(&release.tag_name)).unwrap();

        if constraint.matches(&version) {
            println!("  Matches version constraint");
        } else {
            println!("  Does not match version constraint");
            continue;
        }

        for asset in release.assets.iter() {
            for asset_regex in asset_regexes.iter() {
                if asset_regex.is_match(&asset.name) {
                    println!("  Asset name: {}", asset.name);
                    println!("        URL: {}", asset.browser_download_url);
                    break;
                }
            }
        }
    }
    Ok(())
}

