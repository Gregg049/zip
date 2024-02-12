#[derive(Debug, Clone)]
pub enum HaffBaseNode {
    LeafNode(HaffLeafNode),
    InternalNode(HaffInternalNode),
}

#[derive(Debug, Clone)]
pub struct HaffLeafNode {
    pub element: char,
    pub weight: usize,
}

impl HaffLeafNode {
    pub fn new(element: char, weight: usize) -> Self {
        HaffLeafNode { element, weight }
    }
    pub fn value(&self) -> &char {
        &self.element
    }
    fn _isleaf(&self) -> bool {
        true
    }

    fn weight(&self) -> usize {
        self.weight
    }
}
#[derive(Debug, Clone)]
pub struct HaffInternalNode {
    pub left: Box<HaffBaseNode>,
    pub right: Box<HaffBaseNode>,
    weight: usize,
}

impl HaffInternalNode {
    pub fn new(weight: usize, left: Box<HaffBaseNode>, right: Box<HaffBaseNode>) -> Self {
        HaffInternalNode {
            left,
            right,
            weight,
        }
    }
    pub fn left(self) -> Box<HaffBaseNode> {
        self.left
    }
    pub fn right(self) -> Box<HaffBaseNode> {
        self.right
    }
    fn _isleaf(&self) -> bool {
        false
    }
    fn weight(&self) -> usize {
        self.weight
    }
}

#[derive(Debug, Clone)]
pub struct HaffTree {
    root: HaffBaseNode,
}

impl HaffTree {
    pub fn new(root: HaffBaseNode) -> Self {
        HaffTree { root }
    }
    pub fn root(self) -> HaffBaseNode {
        self.root
    }
    pub fn weight(&self) -> usize {
        match &self.root {
            HaffBaseNode::LeafNode(leaf) => leaf.weight(),
            HaffBaseNode::InternalNode(internal) => internal.weight(),
        }
    }
}
