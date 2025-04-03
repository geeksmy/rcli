use super::{verify_file, verify_path};
use clap::Parser;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "使用私钥或共享秘钥对文本签名")]
    Sign(TextSignOpts),
    #[command(about = "验证签名")]
    Verify(TextVerifyOpts),
    #[command(about = "生成Key")]
    Generate(TextKeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    /// 签名文本或者文件
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    /// 签名的秘钥
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    /// 签名格式(blank3, ed25519)
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    /// 验证文本或者文件
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    /// 验证的秘钥
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long)]
    pub sign: String,
    /// 签名格式(blank3, ed25519)
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextFormat,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    /// 签名格式(blank3, ed25519)
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextFormat,
    /// 输出文件路径
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextFormat::Blake3),
            "ed25519" => Ok(TextFormat::Ed25519),
            _ => Err(anyhow::anyhow!("无效格式!!!")),
        }
    }
}

impl From<TextFormat> for &'static str {
    fn from(format: TextFormat) -> Self {
        match format {
            TextFormat::Blake3 => "blake3",
            TextFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
