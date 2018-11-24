extern crate lzma;

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
    match lzma::decompress(include_bytes!("code2char-src.txt.xz")) {
        Ok(decompressed) => decompressed,
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