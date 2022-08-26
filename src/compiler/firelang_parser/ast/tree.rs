struct Node {
    pub subs: Vec<Node>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            subs: Vec::new()
        }
    }

    pub fn is_leaf_node(&self) -> bool {
        self.subs.is_empty()
    }
}