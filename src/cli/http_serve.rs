use super::verify_path;
use crate::{CmdExecute, process_http_serve};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecute)]
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

impl CmdExecute for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.dir, self.port).await
    }
}
