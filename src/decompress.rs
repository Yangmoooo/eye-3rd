use home::home_dir;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::cli::Args;
use crate::error::EyeError as Error;
use crate::format::{zip::extract_zip, Format};

pub fn extract(format: Format, args: &Args) -> Result<PathBuf, Error> {
    let (input, pw, db) = (&args.input, args.pw.as_deref(), args.db.as_deref());
    match format {
        Format::Mp4 | Format::Mkv | Format::Zip => extract_zip(input, pw, db),
        _ => Err(Error::UnsupportedFormat),
    }
}

pub fn find_db() -> Result<PathBuf, Error> {
    let exe = env::current_exe()?;
    let exe_dir = exe.parent().ok_or(Error::PasswordDbNotFound)?;
    let name = "eye3rd.db.txt";

    let dirs = vec![
        exe_dir.to_path_buf(),
        exe_dir
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf())
            .unwrap_or_default(), // 在未指定 rust target 的 cargo 项目中使用
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

pub fn build_pwlist(db: &Path) -> Result<Vec<(u32, String)>, Error> {
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

pub fn update_db(db: &Path, pwlist: &mut Vec<(u32, String)>) -> Result<(), Error> {
    pwlist.sort_by(|a, b| b.0.cmp(&a.0));
    let mut writer = BufWriter::new(File::create(db)?);
    for (freq, pw) in pwlist {
        writeln!(writer, "{freq},{pw}")?;
    }
    Ok(())
}
