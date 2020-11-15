pub struct ParserNode {
    node_type: ParserNodeType,
    children: Vec<Rc<ParserNode>>,
}

impl ParserNode {
    fn new(node_type: ParserNodeType, children: Vec<Rc<Self>>) -> Self {
        Self { node_type, children }
    }
}