use std::collections::{BTreeMap, HashMap};

use crate::skelet;

pub fn haffcode(
    hafftree: &skelet::HaffBaseNode,
    mut el_code: HashMap<char, String>,
    code: String,
) -> HashMap<char, String> {
    match hafftree {
        skelet::HaffBaseNode::LeafNode(node) => {
            el_code.insert(*node.value(), code.clone());
        }
        skelet::HaffBaseNode::InternalNode(node) => {
            let mut code1 = code.clone();
            code1.push('0');
            let mut code2 = code.clone();
            code2.push('1');
            el_code = haffcode(&node.clone().left(), el_code.clone(), code1);
            el_code = haffcode(&node.clone().right(), el_code.clone(), code2);
        }
    };
    el_code
}

pub fn build_tree(char_counts: &BTreeMap<char, usize>) -> skelet::HaffTree {
    let mut vec_leaf = vec![];
    for (key, value) in char_counts {
        vec_leaf.push(skelet::HaffTree::new(skelet::HaffBaseNode::LeafNode(
            skelet::HaffLeafNode::new(*key, *value),
        )));
    }
    let mut tmp3: skelet::HaffInternalNode;
    let mut tmp1: skelet::HaffTree;
    let mut tmp2: skelet::HaffTree;
    while vec_leaf.len() > 1 {
        let indx1 = vec_leaf
            .iter()
            .enumerate()
            .min_by(|x, y| x.1.weight().cmp(&y.1.weight()))
            .unwrap();
        tmp1 = vec_leaf.swap_remove(indx1.0);
        let indx2 = vec_leaf
            .iter()
            .enumerate()
            .min_by(|x, y| x.1.weight().cmp(&y.1.weight()))
            .unwrap();
        tmp2 = vec_leaf.swap_remove(indx2.0);
        tmp3 = skelet::HaffInternalNode::new(
            tmp1.weight() + tmp2.weight(),
            Box::new(tmp1.root()),
            Box::new(tmp2.root()),
        );
        vec_leaf.push(skelet::HaffTree::new(skelet::HaffBaseNode::InternalNode(
            tmp3,
        )));
    }
    vec_leaf.pop().unwrap()
}

pub fn encode(input: String, el_code: &HashMap<char, String>) -> String {
    let mut output = "".to_string();
    for c in input.chars() {
        output.push_str(el_code.iter().find(|x| *x.0 == c).unwrap().1);
    }
    output
}

pub fn decode(input: String, hafftree: &skelet::HaffBaseNode) -> String {
    let mut output = "".to_string();
    let mut node = hafftree;
    for c in input.chars() {
        if let skelet::HaffBaseNode::InternalNode(inter) = node {
            node = if c == '0' { &inter.left } else { &inter.right };
        }
        if let skelet::HaffBaseNode::LeafNode(leaf) = node {
            output.push(*leaf.value());
            node = hafftree;
        }
    }
    output
}
