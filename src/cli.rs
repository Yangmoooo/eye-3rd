use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "eye³",
    author = "Yangmoooo",
    version,
    about = "睁开第三只眼👁 轻取隐写文件"
)]
pub struct Args {
    /// 指定输入文件路径
    #[arg(index = 1, value_name = "FILE")]
    pub input: PathBuf,

    /// 指定密码
    #[arg(short, long, value_name = "PASSWORD")]
    pub pw: Option<String>,

    /// 指定密码库路径
    #[arg(short, long, value_name = "FILE")]
    pub db: Option<PathBuf>,
}
