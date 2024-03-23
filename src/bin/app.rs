use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long, default_value = "false", default_missing_value = "true")]
    version: bool,
}

fn main() {
    let cli = Cli::parse();
    dbg!(&cli.version);
    if cli.version {
        // let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "Unknown".to_string());
        // println!("Version: {}", version);

        // let commit_hash =
        //     std::env::var("VERGEN_GIT_SHA_SHORT").unwrap_or_else(|_| "Unknown".to_string());
        // println!("Commit Hash: {}", commit_hash);

        // let commit_date =
        //     std::env::var("VERGEN_GIT_COMMIT_DATE").unwrap_or_else(|_| "Unknown".to_string());
        // println!("Commit Date: {}", commit_date);

        let built_at =
            std::env::var("VERGEN_BUILD_TIMESTAMP").unwrap_or_else(|_| "Unknown".to_string());
        println!("built at: {}", built_at);

        let built_at = std::env::var("VERGEN_SHA").unwrap_or_else(|_| "Unknown".to_string());
        println!("sha: {}", built_at);
    }

    println!("I'm the app binary")
}
