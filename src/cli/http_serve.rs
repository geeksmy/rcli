use super::verify_path;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Http 服务端")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    /// 文件目录
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    /// 端口
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
