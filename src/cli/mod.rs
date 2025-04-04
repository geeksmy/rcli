pub mod b64;
pub mod csv_convert;
pub mod gen_pass;
pub mod http_serve;
pub mod text;

pub use self::{
    b64::{Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand},
    csv_convert::{CsvOpts, OutputFormat},
    gen_pass::GenPassOpts,
    http_serve::{HttpServeOpts, HttpSubCommand},
    text::{TextFormat, TextKeyGenerateOpts, TextSignOpts, TextSubCommand, TextVerifyOpts},
};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecute)]
pub enum SubCommand {
    #[command(name = "csv", about = "显示CSV或将CSV转换为其他格式")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "生成随机密码")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "使用Base64编解码")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "对文本加解密")]
    Text(TextSubCommand),
    #[command(subcommand, about = "Http 文件服务")]
    Http(HttpSubCommand),
}

fn verify_file(input: &str) -> Result<String, &'static str> {
    if input == "-" || Path::new(input).exists() {
        Ok(input.into())
    } else {
        Err("文件不存在!!!")
    }
}

fn verify_path(input: &str) -> Result<PathBuf, &'static str> {
    let path = Path::new(input);
    if path.exists() && path.is_dir() {
        Ok(path.into())
    } else {
        Err("路径不存在或目录不存在!!!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("文件不存在!!!"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("文件不存在!!!"));
    }
}
