use thiserror::Error;

#[derive(Error, Debug)]
pub enum EyeError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("日志初始化失败啦")]
    Log(#[from] log::SetLoggerError),
    #[error("没有找到密码库呢")]
    PasswordDbNotFound,
    #[error("密码库中好像没有匹配的项目噢")]
    NoMatchedPassword,
    #[error("不支持的文件格式")]
    UnsupportedFormat,
}
