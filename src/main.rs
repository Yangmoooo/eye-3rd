#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod cli;
mod error;
mod reveal;
#[macro_use]
mod notify;

use clap::Parser;
use log::{error, info, LevelFilter};
use simplelog::{ConfigBuilder, WriteLogger};
use std::fs::File;
use std::{env, fs::remove_file};
use zip::ZipArchive;

use cli::Args;
use error::EyeError as Error;
use notify::NotifyType;
use reveal::{reveal_cipher, reveal_plain};

fn main() {
    if let Err(e) = init_logger() {
        notify!(NotifyType::Err, "日志初始化失败啦：{e:?}");
        return;
    }
    if let Err(e) = run() {
        notify!(NotifyType::Err, "解除隐写失败啦：{e}");
        match e {
            Error::Io(e) => error!("I/O: {e:?}"),
            Error::Zip(e) => error!("Zip: {e:?}"),
            Error::Log(e) => error!("Logger: {e:?}"),
            _ => error!("Database: {e:?}"),
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

    info!("Ready to process file: {:?}", args.input);
    notify!(
        NotifyType::Info,
        "准备处理文件：{:?}，请稍候···",
        args.input
    );

    let mut archive = ZipArchive::new(File::open(&args.input)?)?;
    if archive.is_empty() {
        info!("Unable to find any steganography file");
        notify!(NotifyType::Info, "文件中似乎没有隐写文件呢");
        return Ok(());
    }
    let dir = args.input.parent().unwrap();
    let result = match archive.get_aes_verification_key_and_salt(0) {
        Ok(None) => {
            info!("Steganography file is not encrypted, extracting...");
            reveal_plain(&mut archive, dir)
        }
        Ok(Some(_)) => {
            info!("Steganography file is encrypted, decrypting...");
            let output = dir.join(archive.name_for_index(0).unwrap_or("_default.zip"));
            reveal_cipher(
                &mut archive,
                args.pw.as_deref(),
                args.db.as_deref(),
                &output,
            )
        }
        Err(e) => Err(e.into()),
    };
    result.and_then(|_| {
        remove_file(&args.input)?;
        Ok(())
    })
}
