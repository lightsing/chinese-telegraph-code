static N: u8 = 10;

struct CodeNode {
    child: Vec<Option<Box<CodeNode>>>,
    val: Option<Box<char>>
}

pub fn create_code_node(val:Option<char>) -> Box<CodeNode> {
    return Box::new(CodeNode{child: vec![None; N], val: Some(Box::new(val?)) });
}

impl CodeNode {
    fn insert(&mut self, index:u8, node:Box<CodeNode>) {
        assert!(index < N);
        self.child[index] = node
    }
}