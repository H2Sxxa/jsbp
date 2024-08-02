use std::{
    borrow::BorrowMut,
    fs::{create_dir_all, File},
    io::{Cursor, Read, Write},
    path::Path,
    sync::Arc,
};

use tokio::sync::Mutex;
use zip::ZipArchive;

use crate::datatypes::ReplaceInfo;

pub trait JVMBytes {
    fn to_jbytes(&self) -> Vec<u8>;
}

impl JVMBytes for String {
    fn to_jbytes(&self) -> Vec<u8> {
        let mut bytes = self.as_bytes().to_vec();
        transform(&mut bytes);
        bytes
    }
}

impl JVMBytes for str {
    fn to_jbytes(&self) -> Vec<u8> {
        let mut bytes = self.as_bytes().to_vec();
        transform(&mut bytes);
        bytes
    }
}

pub fn transform(bytes: &mut Vec<u8>) {
    (bytes.len() as u16)
        .to_be_bytes()
        .iter()
        .rev()
        .for_each(|value| bytes.insert(0, value.clone()));
}

#[test]
fn test_transform() {
    let mut vec = vec![1, 2, 3];
    transform(&mut vec);
    println!("{:?}", vec)
}

pub fn replace_slice<T>(source: &[T], from: &[T], to: &[T]) -> Vec<T>
where
    T: Clone + PartialEq,
{
    let mut result = source.to_vec();
    let from_len = from.len();
    let to_len = to.len();
    let mut i = 0;
    while i + from_len <= result.len() {
        if result[i..].starts_with(from) {
            result.splice(i..i + from_len, to.iter().cloned());
            i += to_len;
        } else {
            i += 1;
        }
    }

    result
}

pub async fn patch(
    archive: Arc<Mutex<ZipArchive<Cursor<Vec<u8>>>>>,
    value: String,
    includes: Vec<ReplaceInfo>,
    reverse: bool,
) {
    let mut guard = archive.lock().await;
    if let Ok(mut class) = guard.borrow_mut().by_name(&value) {
        println!("Start patch `{}`", value);
        let mut class_byte = Vec::new();

        class.read_to_end(&mut class_byte).unwrap();
        //Patch

        includes.iter().for_each(|info| {
            if reverse {
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
        println!("Patch `{}` Done!", value);
    } else {
        println!("Can't find `{}`, pass", value)
    };
}
