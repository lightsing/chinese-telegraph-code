extern crate lzma_rs;

use std::io::BufReader;
use self::lzma_rs::xz_decompress;

static N: usize = 10;

#[derive(Clone)]
pub struct CodeNode {
    child: Vec<Option<Box<CodeNode>>>,
    val: Box<Option<char>>
}

pub fn create_code_node(val: Option<char>) -> Box<CodeNode> {
    Box::new(CodeNode{child: vec![None; N], val: Box::new(val) })
}

pub fn init() -> Vec<u8> {
    let mut bytes: Vec<u8> = include_bytes!("code2char-src.txt.xz").to_vec();
    // let mut input_buffer = BufReader::new(bytes);
    let mut output_buffer: Vec<u8> = Vec::new();

    match xz_decompress(&mut BufReader::new(bytes.as_slice()), &mut output_buffer) {
        Ok(_) => output_buffer,
        Err(error) => {
            panic!("There was a problem decompress the source file: {:?}", error)
        }
    }
}

impl CodeNode {
    pub fn insert(&mut self, index: usize, node:Box<CodeNode>) {
        assert!(index < N);
        self.child[index] = Some(node)
    }
}