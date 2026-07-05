use crate::cli::{Commands, SubscribeAction};
use anyhow::Result;

pub mod start;
pub mod status;
pub mod stop;
pub mod subscribe;
pub mod tui;
pub mod update_core;

pub fn run(command: Commands) -> Result<()> {
    match command {
        Commands::UpdateCore => update_core::run(),
        Commands::Subscribe { action } => match action {
            SubscribeAction::Add { url, name } => subscribe::add(url, name),
        },
        Commands::Start => start::run(),
        Commands::Stop => stop::run(),
        Commands::Status => status::run(),
        Commands::Tui => tui::run(),
    }
}
