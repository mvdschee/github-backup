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

pub async fn get_personal_repositories_urls(
	access_token: &String,
	page: u32,
) -> Result<Vec<String>, String> {
	let url = format!(
		"https://api.github.com/user/repos?per_page=100&type=owner&page={}&sort=updated",
		page
	);

	let mut headers = HeaderMap::new();
	headers.insert(
		"Authorization",
		HeaderValue::from_str(&format!("Bearer {}", access_token)).expect(""),
	);
	headers.insert("Accept", HeaderValue::from_str("application/vnd.github+json").expect(""));
	headers.insert("User-Agent", HeaderValue::from_str("github-backup").expect(""));

	let client = Client::new();

	let response = match client.get(url).headers(headers).send().await {
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
	access_token: &String,
	output: &String,
) -> Result<(), String> {
	let repo_name = url.replace("https://api.github.com/repos/", "");
	let repo_name = repo_name.replace("/zipball/", "@");
	info!("Downloading {}", &repo_name);

	let mut headers = HeaderMap::new();
	headers.insert(
		"Authorization",
		HeaderValue::from_str(&format!("Bearer {}", access_token)).expect(""),
	);
	headers.insert("Accept", HeaderValue::from_str("application/vnd.github+json").expect(""));
	headers.insert("User-Agent", HeaderValue::from_str("github-backup").expect(""));

	let client = Client::new();

	let response = match client.get(url).headers(headers.clone()).send().await {
		Ok(response) => response,
		Err(e) => return Err(format!("{}", e)),
	};

	let url = response.url().to_string();

	let response = match client.get(url).headers(headers).send().await {
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
