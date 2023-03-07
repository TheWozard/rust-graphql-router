use std::collections::VecDeque;

// Tree represents a standard tree data structure
pub struct Tree<T> {
    pub value: T,
    pub children: Vec<Tree<T>>,
}

impl<'a, T: 'a> Tree<T> {
    fn iter(&'a self) -> TreeIterator<'a, '_, T> {
        TreeIterator::new(self)
    }

    fn iter_condition<'b>(
        &'a self,
        condition: impl Fn(&'b T) -> bool + 'b,
    ) -> TreeIterator<'a, 'b, T> {
        TreeIterator::new_with_condition(self, condition)
    }
}

impl<'a, T: 'a + std::cmp::PartialEq> Tree<T> {
    fn has_prefix(&self, other: &Tree<T>) -> bool {
        if self.value != other.value {
            return false;
        }
        if other.children.len() == 0 {
            return true;
        }

        for possible in self.children.iter() {
            for target in other.children.iter() {
                if possible.has_prefix(target) {
                    return true;
                }
            }
        }
        false
    }
}

// TreeIterator<T> is an iterator for a Tree<T> that provides optional conditional.
pub struct TreeIterator<'a: 'b, 'b, T> {
    queue: VecDeque<&'a Tree<T>>,
    condition: Box<dyn Fn(&'b T) -> bool + 'b>,
}

impl<'a: 'b, 'b, T> TreeIterator<'a, 'b, T> {
    // new creates a new TreeIterator<T> for a given Tree<T> that iterates all nodes in a breath first approach.
    fn new(tree: &'a Tree<T>) -> TreeIterator<'a, 'b, T> {
        TreeIterator::new_with_condition(tree, |_t| true)
    }

    // new_with_condition creates a new TreeIterator<T> for a given Tree<T> that iterates nodes in a breath first approach
    // the Iterator only iterates nodes that match the passed condition. Once a node failes
    fn new_with_condition(
        tree: &'a Tree<T>,
        condition: impl Fn(&'b T) -> bool + 'b,
    ) -> TreeIterator<'a, 'b, T> {
        let mut queue = VecDeque::new();
        if condition(&tree.value) {
            queue.push_back(tree);
        }
        TreeIterator {
            queue,
            condition: Box::new(condition),
        }
    }
}

impl<'a: 'b, 'b, T> Iterator for TreeIterator<'a, 'b, T> {
    type Item = &'a Tree<T>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.queue.pop_front() {
            // If we should iterated this node
            if (self.condition)(&node.value) {
                // Add children to the queue
                for child in node.children.iter() {
                    self.queue.push_back(child)
                }
                return Some(node);
            }
        }
        // Ended
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    enum TestValues {
        A,
        B,
        C,
        D,
    }

    fn vec_compare<T: std::cmp::PartialEq>(va: &Vec<T>, vb: &Vec<T>) -> bool {
        (va.len() == vb.len()) && va.iter().zip(vb).all(|(a, b)| a == b)
    }

    macro_rules! iteration_test {
        ($($name:ident: $value:expr,)*) => {$(
            #[test]
            fn $name() {
                let (tree, expected) = $value;
                assert!(vec_compare(&expected, &tree.iter().map(|n| n.value).collect()));
            }
        )*}
    }

    iteration_test! {
        iter_single_iteration: (Tree{value: TestValues::A, children: vec![]}, vec![TestValues::A]),
        iter_breath_check_iteration: (Tree{value: TestValues::A, children: vec![
            Tree{value: TestValues::B, children: vec![
                Tree{value: TestValues::C, children:vec![]},
            ]},
            Tree{value: TestValues::B, children: vec![
                Tree{value: TestValues::C, children:vec![]},
            ]},
        ]}, vec![TestValues::A, TestValues::B, TestValues::B, TestValues::C, TestValues::C]),
    }

    macro_rules! iteration_condition_test {
        ($($name:ident: $value:expr => $condition:expr,)*) => {$(
            #[test]
            fn $name() {
                let (tree, expected) = $value;
                assert!(vec_compare(&expected, &tree.iter_condition($condition).map(|n| n.value).collect()));
            }
        )*}
    }

    iteration_condition_test! {
        iter_condition_fail_single: (Tree{value: TestValues::A, children: vec![]}, vec![]) => |_n| false,
        iter_condition_pass_single: (Tree{value: TestValues::A, children: vec![]}, vec![TestValues::A]) => |_n| true,
        iter_condition_match_tree: (Tree{value: TestValues::A, children: vec![
            Tree{value: TestValues::B, children: vec![
                Tree{value: TestValues::C, children:vec![]},
            ]},
            Tree{value: TestValues::B, children: vec![
                Tree{value: TestValues::C, children:vec![]},
            ]},
        ]}, vec![TestValues::A, TestValues::B, TestValues::B]) => |n| n == &TestValues::A || n == &TestValues::B,
        iter_condition_barrier_tree: (Tree{value: TestValues::A, children: vec![
            Tree{value: TestValues::B, children: vec![
                Tree{value: TestValues::C, children:vec![]},
            ]},
            Tree{value: TestValues::D, children: vec![
                Tree{value: TestValues::B, children:vec![]},
            ]},
        ]}, vec![TestValues::A, TestValues::D]) => |n| n != &TestValues::B,
    }

    macro_rules! tree_prefix_test {
        ($($name:ident: $value:expr,)*) => {$(
            #[test]
            fn $name() {
                let (a, b, expected) = $value;
                assert_eq!(a.has_prefix(&b), expected);
            }
        )*}
    }

    tree_prefix_test! {
        has_prefix_single_node_match: (Tree{value: TestValues::A, children: vec![]},Tree{value: TestValues::A, children: vec![]}, true),
        has_prefix_single_node_miss: (Tree{value: TestValues::A, children: vec![]},Tree{value: TestValues::B, children: vec![]}, false),
        has_prefix_multi_node_miss: (Tree{value: TestValues::A, children: vec![
            Tree{value: TestValues::B, children: vec![
                Tree{value: TestValues::C, children:vec![]},
            ]},
        ]},Tree{value: TestValues::A, children: vec![
            Tree{value: TestValues::B, children: vec![
                Tree{value: TestValues::D, children:vec![]},
            ]},
        ]}, false),
        has_prefix_multi_node_eventual_match: (Tree{value: TestValues::A, children: vec![
            Tree{value: TestValues::B, children: vec![
                Tree{value: TestValues::C, children:vec![]},
            ]},
        ]},Tree{value: TestValues::A, children: vec![
            Tree{value: TestValues::B, children: vec![
                Tree{value: TestValues::D, children:vec![]},
            ]},
            Tree{value: TestValues::B, children: vec![
                Tree{value: TestValues::D, children:vec![]},
                Tree{value: TestValues::C, children:vec![]},
            ]},
        ]}, true),
    }
}
