mod cli;
mod process;
mod utils;

pub use cli::{
    Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand, CsvOpts, GenPassOpts,
    HttpServeOpts, HttpSubCommand, Opts, OutputFormat, SubCommand, TextFormat, TextKeyGenerateOpts,
    TextSignOpts, TextSubCommand, TextVerifyOpts,
};
use enum_dispatch::enum_dispatch;
pub use process::{
    process_csv, process_decode, process_encode, process_genpass, process_http_serve, process_sign,
    process_text_generator, process_verify,
};
pub use utils::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecute {
    async fn execute(self) -> anyhow::Result<()>;
}
