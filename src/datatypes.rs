use clap::Parser;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub classes: Vec<String>,
    pub includes: Vec<ReplaceInfo>,
}

#[derive(Deserialize, Debug, Clone)]
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
    /// 
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

    /// The tool to update file in jar
    /// 用来更新jar中文件的工具
    #[arg(long, value_name = "jar/7zip", default_value_t = format!("jar"), verbatim_doc_comment)]
    pub tool: String,

    /// The path to `7zip` executable file
    /// `%JAVA_HOME%` 中的 `7zip` 的路径
    #[arg(long, default_value_t = format!("7z"), verbatim_doc_comment)]
    pub _7zip: String,

    /// The path to `jar` executable file
    /// 可执行文件 `jar` 的路径
    #[arg(long, default_value_t = format!("jar"), verbatim_doc_comment)]
    pub jar: String,

    /// The path to ouput file
    /// 输出的文件路径
    #[arg(short, long, default_value_t = format!("%origin%.patch"), verbatim_doc_comment)]
    pub output: String,

    /// Always used to restore the patch
    /// 通常用来恢复修补
    #[arg(short, long, verbatim_doc_comment)]
    pub reverse: bool,

    /// Save the log of patch tool
    /// 保存修补工具日志
    #[arg(short, long, verbatim_doc_comment)]
    pub log: bool,

    /// Overlaid target jar file
    /// 覆盖目标文件
    #[arg(long, verbatim_doc_comment)]
    pub overlaid: bool,

    /// Enable asynchronous patch
    /// 启用异步修补
    #[arg(short, long, verbatim_doc_comment)]
    pub asynchronous: bool,
}
