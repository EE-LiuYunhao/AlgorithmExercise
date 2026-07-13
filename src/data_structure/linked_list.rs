use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    ptr,
    rc::Rc,
};

use crate::debug::DebugPrinter;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct Node<T> {
    value: T,
    pub(crate) prev: Option<Rc<RefCell<Node<T>>>>,
    pub(crate) next: Option<Rc<RefCell<Node<T>>>>,
}

pub(crate) type NodeRef<T> = Rc<RefCell<Node<T>>>;

pub(crate) fn create_node_ref<T>(value: T) -> NodeRef<T> {
    Rc::new(RefCell::new(Node::<T> {
        value,
        prev: None,
        next: None,
    }))
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        // Node equality is based on node identity, not payload equality, so
        // `Node<f32>` remains `Eq` even though `f32` itself is not.
        ptr::eq(self, other)
    }
}
impl<T> Eq for Node<T> {}

pub(crate) fn display<T: Display>(f: &mut Formatter<'_>, head: &NodeRef<T>) -> std::fmt::Result {
    let mut node = Option::Some(head.clone());
    while let Some(node_ref) = node {
        write!(f, "{} ->", node_ref.borrow().value)?;
        node = node_ref.borrow().next.clone(); // why clone?
                                               // if not clone, `node` **moves** the next from the current node.
                                               // Leading to error in the next iteration that the next is gone.
    }
    write!(f, "│|·")
}

pub(crate) fn connect<T: Display>(parent: &NodeRef<T>, child: &NodeRef<T>, debug: &DebugPrinter) {
    debug.print(format!(
        "connecting {} to {}",
        parent.borrow().value,
        child.borrow().value
    ));

    if let Some(original_parent_child) = parent.borrow().next.clone() {
        original_parent_child.borrow_mut().prev = None;
        debug.print(format!(
            "the parent {0} has child {1} before, so reset {1}'s prev to None, so that {0} can has new child",
            parent.borrow().value,
            original_parent_child.borrow().value
        ))
    }
    if let Some(original_child_parent) = child.borrow().prev.clone() {
        original_child_parent.borrow_mut().next = None;
        debug.print(format!(
            "the child {0} has parent {1} before, so reset {1}'s next to None, so that {0} can has new parent",
            child.borrow().value,
            original_child_parent.borrow().value
        ))
    }
    parent.borrow_mut().next = Option::Some(child.clone());
    child.borrow_mut().prev = Option::Some(parent.clone());
}

#[cfg(test)]
mod tests {
    use super::{connect, create_node_ref, Node};
    use crate::debug::DebugPrinter;
    use std::rc::Rc;

    fn assert_eq_impl<T: Eq>() {}

    #[test]
    fn node_of_f32_implements_eq() {
        assert_eq_impl::<Node<f32>>();
    }

    #[test]
    fn create_node_ref_initializes_detached_node() {
        let node = create_node_ref(7);
        let node = node.borrow();

        assert_eq!(node.value, 7);
        assert!(node.prev.is_none());
        assert!(node.next.is_none());
    }

    #[test]
    fn displays_linked_nodes_in_order() {
        let debug = DebugPrinter::new(false);
        let first = create_node_ref(1);
        let second = create_node_ref(2);
        let third = create_node_ref(3);

        connect(&first, &second, &debug);
        connect(&second, &third, &debug);

        assert_eq!(
            format!(
                "{}",
                crate::data_structure::DataStructure::LinkedListInt(first)
            ),
            "1 ->2 ->3 ->│|·"
        );
    }

    #[test]
    fn reconnecting_child_detaches_old_parent() {
        let debug = DebugPrinter::new(false);
        let old_parent = create_node_ref("old".to_string());
        let new_parent = create_node_ref("new".to_string());
        let child = create_node_ref("child".to_string());

        connect(&old_parent, &child, &debug);
        connect(&new_parent, &child, &debug);

        assert!(old_parent.borrow().next.is_none());
        assert!(Rc::ptr_eq(
            new_parent
                .borrow()
                .next
                .as_ref()
                .expect("new parent should point to child"),
            &child
        ));
        assert!(Rc::ptr_eq(
            child
                .borrow()
                .prev
                .as_ref()
                .expect("child should point back to new parent"),
            &new_parent
        ));
    }
}
