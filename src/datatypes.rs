use clap::Parser;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub classes: Vec<String>,
    pub includes: Vec<ReplaceInfo>,
}

#[derive(Deserialize, Debug)]
pub struct ReplaceInfo {
    pub from: String,
    pub to: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// The path to the config of patch
    /// 修补的配置文件路径
    /// Example(config.yml)
    /// ```yaml
    /// classes:
    ///   - path/to/the/target.class
    /// includes:
    ///   - from: Here is A string
    ///     to: target string
    /// ```
    #[arg(short, long, verbatim_doc_comment)]
    pub config: String,

    /// Patch target, always a path to `jar` file
    /// 修补的目标文件，通常是一个 `jar` 文件
    #[arg(short, long, verbatim_doc_comment)]
    pub target: String,

    /// The path to the `jar.exe` in `%JAVA_HOME%/bin`
    /// `%JAVA_HOME%` 中的 `jar.exe` 的路径
    #[arg(short, long, default_value_t = format!("jar"),verbatim_doc_comment)]
    pub jartool: String,

    /// Always used to restore the patch
    /// 通常用来恢复修补
    #[arg(short, long, verbatim_doc_comment)]
    pub reverse: bool,

    /// Save the log of `jar.exe`
    /// 保存 `jar.exe` 日志
    #[arg(short, long, verbatim_doc_comment)]
    pub log: bool,

    /// Overlaid target jar file
    /// 覆盖目标文件
    #[arg(short, long, verbatim_doc_comment)]
    pub overlaid: bool,
}
