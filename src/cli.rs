use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "clash2linux")]
#[command(about = "Linux 代理工具，基于 mihomo 核心")]
#[command(version)]
pub struct Cli {
    /// 指定配置文件路径
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// 显示详细日志
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 更新 mihomo 核心
    UpdateCore,
    /// 订阅管理
    Subscribe {
        #[command(subcommand)]
        action: SubscribeAction,
    },
    /// 启动代理
    Start,
    /// 停止代理
    Stop,
    /// 查看状态
    Status,
    /// 启动 TUI（暂不可用）
    Tui,
}

#[derive(Subcommand, Debug)]
pub enum SubscribeAction {
    /// 添加订阅链接
    Add {
        /// 订阅链接
        url: String,
        /// 订阅名称
        #[arg(short, long)]
        name: String,
    },
}
