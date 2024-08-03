mod datatypes;
mod patcher;

use datatypes::Config;

use clap::Parser;
use datatypes::CliArgs;
use std::{
    fs::{copy, create_dir, read, remove_dir_all, File},
    io::{stderr, stdout, Cursor, Write},
    path::{absolute, Path},
    process::Command,
    sync::Arc,
};
use tokio::{sync::Mutex, task::JoinHandle};
use zip::ZipArchive;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let config: Config = serde_yml::from_reader(
        File::open(&args.config).expect(format!("Can't find {}", &args.config).as_str()),
    )
    .expect("Please use `jsbp -h` to see how to create a config file");

    let filepath: &Path;
    let patchname: String;
    if args.overlaid {
        filepath = Path::new(&args.target);
    } else {
        patchname = args.output.replace("%origin%", &args.target);
        copy(&args.target, &patchname).unwrap();
        filepath = Path::new(&patchname);
    }

    let filename = String::from(absolute(&filepath).unwrap().to_string_lossy());
    println!("Output to `{}`", filename);

    let archive: Arc<Mutex<ZipArchive<Cursor<Vec<u8>>>>> = Arc::new(Mutex::new(
        ZipArchive::new(Cursor::new(read(&filename).unwrap())).unwrap(),
    ));

    println!("Clean cache dir...");
    let _ = remove_dir_all("cache");
    create_dir("cache").unwrap();

    if args.asynchronous {
        println!("Enable asynchronous patch");
        let handles: Vec<JoinHandle<()>> = config
            .classes
            .into_iter()
            .map(|value| {
                tokio::spawn(patcher::patch(
                    archive.clone(),
                    value,
                    config.includes.clone(),
                    args.reverse,
                ))
            })
            .collect();
        for handle in handles {
            handle.await.unwrap();
        }
    } else {
        for value in config.classes {
            patcher::patch(
                archive.clone(),
                value,
                config.includes.clone(),
                args.reverse,
            )
            .await;
        }
    }
    println!("Waiting to executing `{}`...", args.tool);

    let output = match args.tool.as_str() {
        "jar" => Command::new(args.jar)
            .args(["-uvf", filename.as_str(), "*"])
            .current_dir("cache")
            .output()
            .expect("Failed in append"),
        "7zip" => Command::new(args._7zip)
            .args(["u", filename.as_str(), "*"])
            .current_dir("cache")
            .output()
            .expect("Failed in append"),
        _ => {
            println!("Unknown tool! default to `jar`");
            Command::new(args.jar)
                .args(["-uvf", filename.as_str(), "*"])
                .current_dir("cache")
                .output()
                .expect("Failed in append")
        }
    };

    let _ = stdout().write_all(&output.stdout);
    let _ = stderr().write_all(&output.stderr);

    if args.log {
        File::create("tool-stdout.log")
            .unwrap()
            .write_all(&output.stdout)
            .unwrap();
        File::create("tool-stderr.log")
            .unwrap()
            .write_all(&output.stderr)
            .unwrap();
    }

    println!("Patch Done!");
}

#[test]
fn get_path() {
    print!(
        "{}",
        absolute(Path::new("./Cargo.toml"))
            .unwrap()
            .to_string_lossy()
    );
}

#[test]
fn test_cp() {
    copy(
        "./retrofuturagradle-1.3.33.jar",
        "patch.retrofuturagradle-1.3.33.jar",
    )
    .unwrap();
}
