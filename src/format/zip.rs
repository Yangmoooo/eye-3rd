use log::info;
use std::fs::File;
use std::io;
use std::path::Path;
use zip::read::ZipFile;
use zip::{result::ZipError, ZipArchive};

use crate::decompress::{build_pwlist, find_db, update_db};
use crate::error::EyeError as Error;
use crate::{notify, NotifyType};

// FIXME: zip aes 1. 包含多文件时仅解出第一个文件
//                2. 为文件夹时报告未加密但 UnsupportedArchive("Password required to decrypt file")
// TODO: zip zipcrypto
pub fn extract_zip(file: &Path, pw: Option<&str>, db: Option<&Path>) -> Result<(), Error> {
    let mut archive = ZipArchive::new(File::open(file)?)?;
    if archive.is_empty() {
        info!("Unable to find any steganography file");
        notify!(NotifyType::Info, "未能找到隐写文件");
        return Ok(());
    }
    let current_dir = file.parent().unwrap();
    match archive.get_aes_verification_key_and_salt(0) {
        Ok(None) => {
            info!("File is not encrypted, extracting...");
            extract_plain_zip(&mut archive, current_dir)
        }
        Ok(Some(_)) => {
            info!("File is encrypted, decrypting...");
            let output = current_dir.join(archive.name_for_index(0).unwrap_or("_default.zip"));
            extract_cipher_zip(&mut archive, pw, db, &output)
        }
        Err(e) => Err(e.into()),
    }
}

fn extract_plain_zip(file: &mut ZipArchive<File>, dir: &Path) -> Result<(), Error> {
    file.extract(dir)?;
    info!("Extract file successfully!");
    notify!(NotifyType::Ok, "文件提取成功！");
    Ok(())
}

fn extract_cipher_zip(
    file: &mut ZipArchive<File>,
    pw: Option<&str>,
    db: Option<&Path>,
    output: &Path,
) -> Result<(), Error> {
    match pw {
        Some(pw) => {
            let mut zip = file.by_index_decrypt(0, pw.as_bytes())?;
            save_zip(&mut zip, output)
        }
        None => {
            let db_path = match db {
                Some(db) => db,
                None => &find_db()?,
            };
            let mut pwlist = build_pwlist(db_path)?;
            for (freq, pw) in pwlist.iter_mut() {
                match file.by_index_decrypt(0, pw.as_bytes()) {
                    Ok(mut zip) => {
                        save_zip(&mut zip, output)?;
                        *freq += 1;
                        update_db(db_path, &mut pwlist)?;
                        return Ok(());
                    }
                    Err(ZipError::InvalidPassword) => (),
                    Err(e) => return Err(e.into()),
                }
            }
            Err(Error::NoMatchedPassword)
        }
    }
}

fn save_zip(file: &mut ZipFile, output: &Path) -> Result<(), Error> {
    let mut result = File::create(output)?;
    io::copy(file, &mut result)?;
    info!("Extract file successfully, saved to: {output:?}");
    notify!(NotifyType::Ok, "文件提取成功！已保存至：{output:?}");
    Ok(())
}
