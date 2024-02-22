#![feature(absolute_path)]
mod datatypes;
mod patcher;

use {
    datatypes::Config,
    patcher::{replace_slice, JavaBytes},
};

use clap::Parser;
use datatypes::CliArgs;
use std::{
    fs::{copy, create_dir, create_dir_all, read, remove_dir_all, File},
    io::{stderr, stdout, Cursor, Read, Write},
    path::{absolute, Path},
    process::Command,
};
use zip::ZipArchive;

fn main() {
    let args = CliArgs::parse();

    let config: Config = serde_yaml::from_reader(
        File::open(&args.config).expect(format!("Can't find {}", &args.config).as_str()),
    )
    .expect("Please use `jsbp -h` to see how to create a config file");

    let filepath: &Path;
    let patchname: String;
    if args.overlaid {
        filepath = Path::new(&args.target);
    } else {
        patchname = format!("{}.patch", args.target);
        copy(&args.target, &patchname).unwrap();
        filepath = Path::new(&patchname);
    }

    let filename = String::from(absolute(&filepath).unwrap().to_string_lossy());
    println!("{}", filename);

    let mut archive = ZipArchive::new(Cursor::new(read(&filename).unwrap())).unwrap();
    let _ = remove_dir_all("cache");
    create_dir("cache").unwrap();

    config.classes.iter().for_each(|value| {
        println!("Start patch {}", value);
        let mut class = archive.by_name(&value).unwrap();
        let mut class_byte = Vec::new();
        class.read_to_end(&mut class_byte).unwrap();
        //Patch
        config.includes.iter().for_each(|info| {
            if args.reverse {
                class_byte = replace_slice(
                    &class_byte,
                    info.to.to_jbytes().as_slice(),
                    info.from.to_jbytes().as_slice(),
                );
            } else {
                class_byte = replace_slice(
                    &class_byte,
                    info.from.to_jbytes().as_slice(),
                    info.to.to_jbytes().as_slice(),
                );
            }
        });
        //Save
        let raw_path = format!("cache/{}", value);
        let path = Path::new(&raw_path);

        create_dir_all(path.parent().unwrap()).unwrap();

        let mut temp = File::create(raw_path).unwrap();
        temp.write(&class_byte).unwrap();
        println!("Done");
    });

    println!("waiting to executing jartool...");

    let output = Command::new(args.jartool)
        .current_dir("cache")
        .args(["-uvf", filename.as_str(), "*"])
        .output()
        .expect("Failed in append");

    let _ = stdout().write_all(&output.stdout);
    let _ = stderr().write_all(&output.stderr);

    if args.log {
        File::create("jartool-stdout.log")
            .unwrap()
            .write_all(&output.stdout)
            .unwrap();
        File::create("jartool-stderr.log")
            .unwrap()
            .write_all(&output.stdout)
            .unwrap();
    }
    println!("Patch Successfully!");
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
