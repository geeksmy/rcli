pub mod base64;
pub mod csv;
pub mod gen_pass;

use self::csv::CsvOpts;
use clap::Parser;
use gen_pass::GenPassOpts;
use std::path::Path;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "显示CSV或将CSV转换为其他格式")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "生成随机密码")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(input: &str) -> Result<String, &'static str> {
    if input == "-" || Path::new(input).exists() {
        Ok(input.into())
    } else {
        Err("文件不存在!!!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("文件不存在!!!"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not-exist"), Err("文件不存在!!!"));
    }
}
