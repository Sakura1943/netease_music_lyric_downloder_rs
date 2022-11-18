#![allow(dead_code)]
use clap::{Parser, Command, CommandFactory};
#[derive(Parser)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    #[arg(help = "要查询的歌曲名")]
    pub search_name: String,
    #[arg(short, long, value_name = "PATH", help = "歌词存放路径", default_value = "lyrics")]
    pub save_path: String
}

impl Cli {
    pub fn build() -> Self {
        Self::parse()
    }
    pub fn cmd() -> Command {
        Self::command()
    }
}
