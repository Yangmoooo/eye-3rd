use thiserror::Error;

#[derive(Error, Debug)]
pub enum EyeError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("日志初始化失败")]
    Log(#[from] log::SetLoggerError),
    #[error("不支持的文件格式")]
    UnsupportedFormat,
    #[error("未找到密码库")]
    PasswordDbNotFound,
    #[error("密码库中无匹配项目")]
    NoMatchedPassword,
    #[error("未找到隐写文件")]
    NoSteganographyFile,
    #[error("无效路径")]
    InvalidPath,
}
