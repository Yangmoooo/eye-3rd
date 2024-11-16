use log::info;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use zip::read::ZipFile;
use zip::{result::ZipError, ZipArchive};

use crate::decompress::{build_pwlist, find_db, update_db};
use crate::error::EyeError as Error;

pub fn extract_zip(file: &Path, pw: Option<&str>, db: Option<&Path>) -> Result<PathBuf, Error> {
    let mut archive = ZipArchive::new(File::open(file)?)?;
    info!("Zip archive opened successfully");
    if archive.is_empty() {
        info!("Unable to find any steganography file");
        return Err(Error::NoSteganographyFile);
    }

    let (len, zip0) = (archive.len(), archive.by_index_raw(0)?);
    let (zip0_encrypted, zip0_is_file, zip0_is_dir) =
        (zip0.encrypted(), zip0.is_file(), zip0.is_dir());
    drop(zip0);

    let encrypted = if len == 1 {
        zip0_encrypted
    } else {
        let zip1 = archive.by_index_raw(1)?;
        (zip0_is_file && zip0_encrypted) || (zip0_is_dir && zip1.encrypted())
    };
    if encrypted {
        info!("File is encrypted, decrypting...");
        extract_cipher_zip(&mut archive, pw, db, file)
    } else {
        info!("File is not encrypted, extracting...");
        extract_plain_zip(&mut archive, file)
    }
}

fn extract_plain_zip(archive: &mut ZipArchive<File>, file: &Path) -> Result<PathBuf, Error> {
    let dir = get_output_dir(archive, file)?;
    archive.extract(&dir)?;
    Ok(dir)
}

fn extract_cipher_zip(
    archive: &mut ZipArchive<File>,
    pw: Option<&str>,
    db: Option<&Path>,
    file: &Path,
) -> Result<PathBuf, Error> {
    let dir = get_output_dir(archive, file)?;
    let len = archive.len();
    let idx = archive.by_index_raw(0)?.is_dir() as usize;
    match pw {
        Some(pw) => {
            for i in 0..len {
                let mut file = archive.by_index_decrypt(i, pw.as_bytes())?;
                save_single(&mut file, &dir)?;
            }
            Ok(dir)
        }
        None => {
            let db_path = match db {
                Some(db) => db,
                None => &find_db()?,
            };
            let mut pwlist = build_pwlist(db_path)?;
            for (freq, pw) in pwlist.iter_mut() {
                info!("Trying password: {pw}");
                let mut file = match archive.by_index_decrypt(idx, pw.as_bytes()) {
                    Ok(file) => file,
                    Err(ZipError::InvalidPassword) => continue,
                    Err(e) => return Err(e.into()),
                };
                save_single(&mut file, &dir)?;
                drop(file);
                info!("Password matched: {pw}");
                for i in idx + 1..len {
                    let mut file = archive.by_index_decrypt(i, pw.as_bytes())?;
                    save_single(&mut file, &dir)?;
                }
                *freq += 1;
                update_db(db_path, &mut pwlist)?;
                return Ok(dir);
            }
            Err(Error::NoMatchedPassword)
        }
    }
}

fn save_single(file: &mut ZipFile, dir: &Path) -> Result<(), Error> {
    if file.is_dir() {
        return Ok(());
    }
    let output = dir.join(file.enclosed_name().ok_or(Error::InvalidPath)?);
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut result = File::create(&output)?;
    io::copy(file, &mut result)?;
    Ok(())
}

fn get_output_dir(archive: &mut ZipArchive<File>, file: &Path) -> Result<PathBuf, Error> {
    let current_dir = file.parent().ok_or(Error::InvalidPath)?;
    let archive_name = file.file_stem().ok_or(Error::InvalidPath)?;
    let new_dir = current_dir.join(archive_name);
    let zip0_name = archive.name_for_index(0).ok_or(Error::InvalidPath)?;
    let zip0_name = zip0_name.strip_suffix("/").unwrap_or(zip0_name);

    // 如果压缩包内少于 5 个文件，或者压缩包名与第一个文件名相同，则解压到当前目录
    let dir = if archive.len() <= 5 || archive_name == zip0_name {
        current_dir.to_path_buf()
    } else {
        new_dir
    };
    info!("Output directory: {dir:?}");
    Ok(dir)
}
