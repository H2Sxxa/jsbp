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
