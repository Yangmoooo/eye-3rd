use home::home_dir;
use log::info;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use zip::read::ZipFile;
use zip::{result::ZipError, ZipArchive};

use crate::error::EyeError as Error;
use crate::{notify, notify::NotifyType};

pub fn reveal_plain(file: &mut ZipArchive<File>, dir: &Path) -> Result<(), Error> {
    file.extract(dir)?;
    info!("Reveal steganography successfully!");
    notify!(NotifyType::Ok, "解除隐写成功！");
    Ok(())
}

pub fn reveal_cipher(
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
                None => &find_db_path()?,
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

fn find_db_path() -> Result<PathBuf, Error> {
    let exe = env::current_exe()?;
    let exe_dir = exe.parent().ok_or(Error::PasswordDbNotFound)?;
    let name = "eye3rd.db.txt";

    let dirs = vec![
        exe_dir.to_path_buf(),
        exe_dir
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf())
            .unwrap_or_default(), // for cargo run without target
        home_dir().unwrap_or_default(),
    ];

    for dir in dirs {
        let db_path = dir.join(name);
        if db_path.exists() {
            return Ok(db_path);
        }
    }
    Err(Error::PasswordDbNotFound)
}

fn save_zip(file: &mut ZipFile, output: &Path) -> Result<(), Error> {
    let mut result = File::create(output)?;
    io::copy(file, &mut result)?;
    info!("Reveal steganography successfully, saved to: {output:?}");
    notify!(NotifyType::Ok, "解除隐写成功！文件放在：{output:?}");
    Ok(())
}

fn build_pwlist(db: &Path) -> Result<Vec<(u32, String)>, Error> {
    let pwlist = BufReader::new(File::open(db)?)
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            line.split_once(',')
                .and_then(|(freq, pw)| freq.parse::<u32>().ok().map(|f| (f, pw.to_string())))
        })
        .collect();
    Ok(pwlist)
}

fn update_db(db: &Path, pwlist: &mut Vec<(u32, String)>) -> Result<(), Error> {
    pwlist.sort_by(|a, b| b.0.cmp(&a.0));
    let mut writer = BufWriter::new(File::create(db)?);
    for (freq, pw) in pwlist {
        writeln!(writer, "{freq},{pw}")?;
    }
    Ok(())
}
