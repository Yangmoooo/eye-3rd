#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod cli;
mod decompress;
mod error;
mod format;
#[macro_use]
mod notify;

use clap::Parser;
use log::{error, info, LevelFilter};
use simplelog::{ConfigBuilder, WriteLogger};
use std::fs::File;
use std::{env, fs};

use cli::Args;
use decompress::extract;
use error::EyeError as Error;
use notify::MsgType;

fn main() {
    if let Err(e) = init_logger() {
        notify!(MsgType::Err, "日志初始化失败：{e:?}");
        return;
    }
    if let Err(e) = run() {
        notify!(MsgType::Err, "文件提取失败：{e}");
        match e {
            Error::Io(e) => error!("I/O: {e:?}"),
            Error::Zip(e) => error!("Zip: {e:?}"),
            Error::Log(e) => error!("Logger: {e:?}"),
            _ => error!("{e:?}"),
        }
    }
}

fn init_logger() -> Result<(), Error> {
    let log_config = ConfigBuilder::new()
        .set_time_offset_to_local()
        .expect("Failed to set time offset")
        .build();
    let log_path = env::current_exe()?.with_file_name("eye3rd.log");
    WriteLogger::init(
        LevelFilter::Info,
        log_config,
        File::options().append(true).create(true).open(log_path)?,
    )?;
    Ok(())
}

fn run() -> Result<(), Error> {
    let args = Args::parse();
    let input = &args.input;
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));

    info!("eye³ {version} started, processing file: {input:?}");
    notify!(MsgType::Info, "开始处理文件：{input:?}，请稍候···");

    let result = extract(input.as_path().into(), &args);
    result.and_then(|dir| {
        info!("Extract all files successfully! Saved to dir: {dir:?}");
        notify!(MsgType::Ok, "文件提取成功！已保存至目录：{dir:?}");
        fs::remove_file(input)?;
        Ok(())
    })
}
