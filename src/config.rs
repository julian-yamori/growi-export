use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::path_filter::PathFilter;

/// config.toml から読み込む設定
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Config {
    /// 読み込み元の json が格納されたディレクトリのパス
    pub input: PathBuf,

    /// MarkDown の出力先ディレクトリのパス
    pub output: PathBuf,

    pub filter: Option<FilterConfig>,
}

impl Config {
    /// toml から読込
    pub fn load_toml(toml_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let toml_str = fs::read_to_string(&toml_path)?;
        let setting: Self = toml::from_str(&toml_str)?;

        Ok(setting)
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct FilterConfig {
    /// ここに指定されたページの配下のみを出力する
    pub path: Option<PathFilter>,
}
