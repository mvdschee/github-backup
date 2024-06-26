use clap::Parser;
use std::env;

mod gh;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	/// Your personal token from GitHub
	/// or provide a TOKEN environment variable
	#[arg(short, long, default_value = "")]
	token: String,

	/// Where to save the repositories
	#[arg(short, long, default_value = "./backup")]
	output: String,
}

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();

	let args = parse_args_env();

	create_output_dir(&args.output);

	let mut page = 1;
	let mut repos = vec![];
	while repos.len() % 100 == 0 {
		let mut new_repos = match gh::get_personal_repositories_urls(&args.token, page).await {
			Ok(repos) => repos,
			Err(e) => {
				error!("{}", e);
				return;
			}
		};

		if new_repos.len() == 100 {
			page += 1;
		}

		repos.append(&mut new_repos);
	}

	for repo in repos {
		let args = args.clone();

		gh::download_to_backup(repo, &args.token, &args.output).await.unwrap();
	}

	info!("Done");
}

fn parse_args_env() -> ParsedArgs {
	let args = Args::parse();

	let token = match env::var("TOKEN") {
		Ok(token) => token,
		Err(_) => args.token.clone(),
	};

	if token.is_empty() {
		error!("You need to provide a token");
		std::process::exit(1);
	}

	ParsedArgs {
		token,
		output: args.output,
	}
}

fn create_output_dir(output: &str) {
	match std::fs::create_dir_all(output) {
		Ok(_) => (),
		Err(e) => {
			error!("{}", e);

			std::process::exit(1);
		}
	};
}
#[derive(Debug, Clone)]
pub struct ParsedArgs {
	pub token: String,
	pub output: String,
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("\x1b[90m{} \x1b[32m{} \x1b[0m{}", chrono::Local::now().format("%H:%M:%S %d-%m-%y"), "[INFO]", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("\x1b[90m{}  \x1b[33m{} \x1b[0m{}", chrono::Local::now().format("%H:%M:%S %d-%m-%y"), "[WARN]", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!("\x1b[90m{} \x1b[31m{} \x1b[0m{}", chrono::Local::now().format("%H:%M:%S %d-%m-%y"), "[ERROR]", format_args!($($arg)*))
    };
}
