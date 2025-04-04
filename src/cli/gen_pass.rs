use crate::{CmdExecute, process_genpass};
use clap::Parser;
use zxcvbn::zxcvbn;

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

impl CmdExecute for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.numbers,
            self.symbols,
        )?;
        println!("{:?}", password);

        let estimate = zxcvbn(&password, &[]);
        eprintln!("密码强度: {}", estimate.score());
        Ok(())
    }
}
