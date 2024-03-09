use std::path::PathBuf;

use clap::Parser;
use loader::MasterLoader;
use serenity::all::GuildId;

mod loader;
mod slack_export;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    root: PathBuf,
    #[arg(short, long)]
    guild_id: GuildId,
    #[arg(short, long)]
    discord_token: String,
    #[arg(short, long)]
    use_existing_channel: bool,
}

pub struct Context {
    pub root: PathBuf,
    pub guild_id: GuildId,
    pub discord_token: String,
    pub use_existing_channel: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let loader = MasterLoader::new(Context {
        root: args.root,
        guild_id: args.guild_id,
        discord_token: args.discord_token,
        use_existing_channel: args.use_existing_channel,
    })
    .await?;
    loader.run().await?;
    println!("Done!");
    Ok(())
}
