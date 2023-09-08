use std::collections::VecDeque;

// Define a TreeNode struct representing a node in the multi-branch tree.
#[derive(Debug)]
pub struct TreeNode<T> {
    value: T,
    children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: TreeNode<T>) {
        self.children.push(child);
    }
}

// Define a Tree trait with depth-first and breadth-first traversal functions.
pub trait Tree<T> {
    fn root(&self) -> Option<&TreeNode<T>>;
    fn depth_first_search(&self, target: &T) -> Option<&TreeNode<T>>;
    fn breadth_first_search(&self, target: &T) -> Option<&TreeNode<T>>;
}

impl<T> Tree<T> for TreeNode<T>
where
    T: PartialEq,
{
    fn root(&self) -> Option<&TreeNode<T>> {
        Some(self)
    }

    fn depth_first_search(&self, target: &T) -> Option<&TreeNode<T>> {
        if &self.value == target {
            return Some(self);
        }

        for child in &self.children {
            if let Some(result) = child.depth_first_search(target) {
                return Some(result);
            }
        }

        None
    }

    fn breadth_first_search(&self, target: &T) -> Option<&TreeNode<T>> {
        let mut queue = VecDeque::new();
        queue.push_back(self);

        while let Some(node) = queue.pop_front() {
            if &node.value == target {
                return Some(node);
            }

            for child in &node.children {
                queue.push_back(child);
            }
        }

        None
    }
}

fn main() {
    // Create a multi-branch tree.
    let mut root = TreeNode::new("A");
    let mut b = TreeNode::new("B");
    let mut c = TreeNode::new("C");
    let d = TreeNode::new("D");
    let e = TreeNode::new("E");
    let f = TreeNode::new("F");

    b.add_child(d);
    b.add_child(e);

    c.add_child(f);

    root.add_child(b);
    root.add_child(c);

    // Perform depth-first search.
    if let Some(node) = root.depth_first_search(&"F") {
        println!("Depth-first search found node: {:?}", node);
    } else {
        println!("Node not found in depth-first search.");
    }

    // Perform breadth-first search.
    if let Some(node) = root.breadth_first_search(&"F") {
        println!("Breadth-first search found node: {:?}", node);
    } else {
        println!("Node not found in breadth-first search.");
    }
}
