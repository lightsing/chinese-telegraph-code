mod converter;

fn main() {
    let mut src: Vec<u8> = converter::code2char::init();
    let string: Vec<&str> = std::str::from_utf8(src.as_slice()).unwrap()
        .split('\n')
        .collect();
    println!("{:?}", string);
}
