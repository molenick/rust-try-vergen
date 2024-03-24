use std::env;

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
        let built_at = env!("VERGEN_BUILD_TIMESTAMP");
        println!("built at: {}", built_at);

        println!("Build Timestamp: {}", env!("VERGEN_BUILD_TIMESTAMP"));
        println!("git describe: {}", env!("VERGEN_GIT_DESCRIBE"));
        println!("VERGEN_GIT_BRANCH: {}", env!("VERGEN_GIT_BRANCH"));
        println!("VERGEN_GIT_BRANCH: {}", env!("VERGEN_GIT_BRANCH"));
    }

    let prefix = "VERGEN_";

    // Iterate over all environment variables and print only those with the specified prefix
    for (key, value) in env::vars() {
        if key.starts_with(prefix) {
            println!("{}={}", key, value);
        }
    }
}
