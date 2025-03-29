use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    /// 密码长度
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,
    /// 大写
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    /// 小写
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    /// 数字
    #[arg(long, default_value_t = true)]
    pub numbers: bool,
    /// 特殊字符
    #[arg(long, default_value_t = true)]
    pub symbols: bool,
}
