use super::verify_file;
use crate::{CmdExecute, process_decode, process_encode};
use clap::Parser;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "编码字符串到base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "解码base64到字符串")]
    Decode(Base64DecodeOpts),
}

impl CmdExecute for Base64SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Base64SubCommand::Encode(opts) => opts.execute().await,
            Base64SubCommand::Decode(opts) => opts.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    /// 输入编码的字符串
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    /// 输出格式, 支持(standard, urlsafe)
    #[arg(short, long, value_parser = parser_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecute for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encoded = process_encode(&self.input, self.format)?;
        println!("{}", encoded);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    /// 输入解码的Base64字符串
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    /// 输出格式, 支持(standard, urlsafe)
    #[arg(short, long, value_parser = parser_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecute for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decoded = process_decode(&self.input, self.format)?;
        let decoded = String::from_utf8(decoded)?;
        println!("{}", decoded);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parser_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("无效格式!!!")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
