use std::{cell::RefCell, fmt::Formatter, rc::Rc};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]

pub(crate) struct Node {
    pub(crate) id: String,
    pub(crate) children: Vec<(Rc<RefCell<Node>>, i32)>,
}

pub(crate) type NodeRef = Rc<RefCell<Node>>;

pub(crate) fn display(f: &mut Formatter<'_>, nodes: &[NodeRef]) -> std::fmt::Result {
    write!(f, "[")?;
    for (index, value) in nodes.iter().enumerate() {
        let node = value.borrow();
        if index > 0 {
            write!(f, ",\n\t{0} -> [", node.id)?;
        } else {
            write!(f, "\n\t{0} -> [", node.id)?;
        }

        for (index, child) in node.children.iter().enumerate() {
            if index > 0 {
                write!(f, ", {0}({1})", child.0.borrow().id, child.1)?;
            } else {
                write!(f, "{0}({1})", child.0.borrow().id, child.1)?;
            }
        }
        write!(f, "]")?;
    }
    write!(f, "]")
}

pub(crate) fn create_node(id: String) -> NodeRef {
    Rc::new(RefCell::new(Node {
        id,
        children: Vec::new(),
    }))
}
