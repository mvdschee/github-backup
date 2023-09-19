use clap::Parser;

mod gh;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Your personal token from GitHub
    #[arg(short, long)]
    token: String,

    /// Where to save the repositories
    #[arg(short, long, default_value = "./backup")]
    output: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.token.is_empty() {
        error!("You need to provide a token");
        return;
    }

    let repos = match gh::get_personal_repositories_urls(&args.token).await {
        Ok(repos) => repos,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    match std::fs::create_dir_all(&args.output) {
        Ok(_) => (),
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    for repo in repos {
        gh::download_to_backup(repo,&args.token, &args.output).await.unwrap();
    }

    info!("Done");
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