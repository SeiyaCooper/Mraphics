pub struct Node {
    pub children: Vec<Node>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            children: Vec::<Node>::new(),
        }
    }
}
