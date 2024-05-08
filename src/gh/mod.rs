use crate::info;
use reqwest::{
	header::{HeaderMap, HeaderValue},
	Client,
};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Repo {
	archive_url: String,
	default_branch: String,
}

static API_URL: &str = "https://api.github.com";
static HEADER_ACCEPT: &str = "application/vnd.github+json";
static HEADER_AGENT: &str = "github-backup";

pub async fn get_personal_repositories_urls(
	access_token: &str,
	page: u32,
) -> Result<Vec<String>, String> {
	let url = format!("{}/user/repos?per_page=100&type=owner&page={}&sort=updated", API_URL, page);

	let client = Client::new();

	let response = match client.get(url).headers(get_header(access_token)).send().await {
		Ok(response) => response,
		Err(e) => return Err(format!("{}", e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => return Err(format!("{}", e)),
	};

	let result: Vec<Repo> = serde_json::from_str(&response).unwrap();

	let mut urls = Vec::new();

	for repo in result {
		let url = repo.archive_url;
		let branch = repo.default_branch;

		let url = url.replace("{archive_format}", "zipball");
		let url = url.replace("{/ref}", format!("/{}", branch).as_str());

		urls.push(url);
	}

	info!("Found {} repositories", urls.len());

	Ok(urls)
}

pub async fn download_to_backup(
	url: String,
	access_token: &str,
	output: &String,
) -> Result<(), String> {
	let partial_url = format!("{}/repos/", API_URL);
	let repo_name = url.replace(&partial_url, "");
	let repo_name = repo_name.replace("/zipball/", "@");
	info!("Downloading {}", &repo_name);

	let client = Client::new();

	let response = match client.get(url).headers(get_header(access_token)).send().await {
		Ok(response) => response,
		Err(e) => return Err(format!("{}", e)),
	};

	let url = response.url().to_string();

	let response = match client.get(url).headers(get_header(access_token)).send().await {
		Ok(response) => response,
		Err(e) => return Err(format!("{}", e)),
	};

	let response = match response.bytes().await {
		Ok(response) => response,
		Err(e) => return Err(format!("{}", e)),
	};

	let file_name = repo_name.replace(['/', '@', ':', '.', ' '], "_");
	let file_name = format!("{}.zip", file_name);

	let mut path = std::path::PathBuf::from(output);
	path.push(file_name);

	let mut file = match std::fs::File::create(path) {
		Ok(file) => file,
		Err(e) => return Err(format!("{}", e)),
	};

	match file.write_all(&response) {
		Ok(_) => (),
		Err(e) => return Err(format!("{}", e)),
	};

	Ok(())
}

fn get_header(access_token: &str) -> HeaderMap {
	let mut headers = HeaderMap::new();
	headers.insert(
		"Authorization",
		HeaderValue::from_str(&format!("Bearer {}", access_token)).expect(""),
	);
	headers.insert("Accept", HeaderValue::from_str(HEADER_ACCEPT).expect(""));
	headers.insert("User-Agent", HeaderValue::from_str(HEADER_AGENT).expect(""));

	headers
}
