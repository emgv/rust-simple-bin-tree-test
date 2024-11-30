use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum BoxBinTreeNodeRef<T> {
    Empty,
    Value(Box<BoxBinTreeNode<T>>)
}

#[derive(Debug)]
struct BoxBinTreeNode<T> {
    elm: T,
    left: BoxBinTreeNodeRef<T>,
    right: BoxBinTreeNodeRef<T>
}

impl<T: Ord> BoxBinTreeNodeRef<T> {

    pub fn new(value: T) -> Self {
        BoxBinTreeNodeRef::Value(
            Box::new(BoxBinTreeNode {
                elm: value,
                left: BoxBinTreeNodeRef::Empty,
                right: BoxBinTreeNodeRef::Empty
            })
        )
    }

    pub fn is_empty(&self) -> bool {
        match self {
            BoxBinTreeNodeRef::Empty => true,
            _ => false
        }
    }
}

impl<T: Ord> BoxBinTreeNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            elm: value,
            left: BoxBinTreeNodeRef::Empty,
            right: BoxBinTreeNodeRef::Empty
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.left.is_empty() && self.right.is_empty() {
            return true
        }

        return false
    }

    pub fn get(&self) -> &T {
        &self.elm
    }

    pub fn add(&mut self, value: T) -> () {
        let mut stack = vec![self];

        loop {
            let node = stack.pop().unwrap();

            if value >= node.elm {
                match node.right {
                    BoxBinTreeNodeRef::Empty => {
                        // create new node and assign to node.right
                        node.right = BoxBinTreeNodeRef::new(value);
                        return;
                    },
                    BoxBinTreeNodeRef::Value(ref mut v) => stack.insert(0, v),
                }

                return;
            }

            match node.left {
                BoxBinTreeNodeRef::Empty => {
                    // create new node and assign to node.left
                    node.left = BoxBinTreeNodeRef::new(value);
                    return;
                },
                BoxBinTreeNodeRef::Value(ref mut v) => stack.insert(0, v),
            }
        }
    }

    pub fn remove_childs_of(&mut self, value: T) -> bool {
        let mut stack = vec![self];

        loop {
            let node = stack.pop().unwrap();

            if node.elm == value {
                node.left = BoxBinTreeNodeRef::Empty;
                node.right = BoxBinTreeNodeRef::Empty;
                return true;
            }

            if value > node.elm {
                match node.right {
                    BoxBinTreeNodeRef::Value(ref mut value_right) => stack.insert(0, value_right),
                    _ => return false
                }
            }

            match node.left {
                BoxBinTreeNodeRef::Value(ref mut value_left) => stack.insert(0, value_left),
                _ => return false
            }
        }
    }
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Debug)]
struct E {
    value: i32
}

impl E {
    pub fn new(value: i32) -> E {
        E {
            value: value
        }
    }

    pub fn set(&mut self, value: i32) {
        self.value = value;
    }
}

fn main() {
    let v1 = Rc::new(RefCell::new(E::new(4)));
    let v2 = Rc::new(RefCell::new(E::new(3)));
    let v3 = Rc::new(RefCell::new(E::new(5)));
    let mut v4 = Rc::new(RefCell::new(E::new(1)));
    let elems = vec![v1.clone(), v2.clone(), v3.clone(), v4.clone()];

    let mut bin_tree = BoxBinTreeNode::new(elems[0].clone());

    for el in &elems[1..] {
        bin_tree.add(el.clone());
    }

    println!("elems array: {:?}\nbin_tree {:?}", elems, bin_tree);
    println!("root val: {:?}", bin_tree.get());

    let v = bin_tree.get();
    v.borrow_mut().set(45);
    println!("root val: {:?} (after update)", bin_tree.get());

    println!("rc-count  v1: {:?}", Rc::strong_count(&v1));
    println!("rc-count  v2: {:?}", Rc::strong_count(&v2));
    println!("rc-count  v3: {:?}", Rc::strong_count(&v3));
    println!("rc-count  v4: {:?}", Rc::strong_count(&v4));

    println!("rc-count ev1: {:?}", Rc::strong_count(&elems[0]));
    println!("rc-count ev2: {:?}", Rc::strong_count(&elems[1]));
    println!("rc-count ev3: {:?}", Rc::strong_count(&elems[2]));
    println!("rc-count ev4: {:?}", Rc::strong_count(&elems[3]));
    
    println!("\nremoving v2 children");
    let success = bin_tree.remove_childs_of(v2.clone());
    println!("rc-count v4: {:?}", Rc::strong_count(&v4));
    if success {
        println!("Removed v2 {:?}, rc-count v4: {:?}", v2, Rc::strong_count(&v4));
    } else {
        println!("Could not find v2 {:?}, rc-count v4: {:?}", v2, Rc::strong_count(&v4));
    }

    println!("\n\nResult after removal");
    println!("elems array: {:?}\nbin_tree {:?}", elems, bin_tree);
    println!("root val: {:?}", bin_tree.get());

    println!("rc-count  v1: {:?}", Rc::strong_count(&v1));
    println!("rc-count  v2: {:?}", Rc::strong_count(&v2));
    println!("rc-count  v3: {:?}", Rc::strong_count(&v3));
    println!("rc-count  v4: {:?}", Rc::strong_count(&v4));

    println!("rc-count ev1: {:?}", Rc::strong_count(&elems[0]));
    println!("rc-count ev2: {:?}", Rc::strong_count(&elems[1]));
    println!("rc-count ev3: {:?}", Rc::strong_count(&elems[2]));
    println!("rc-count ev4: {:?}", Rc::strong_count(&elems[3]));

    v4 = Rc::new(RefCell::new(E::new(10)));

    println!("\n\nAfter re-assigning v4 a new value");
    println!("rc-count  v1: {:?}", Rc::strong_count(&v1));
    println!("rc-count  v2: {:?}", Rc::strong_count(&v2));
    println!("rc-count  v3: {:?}", Rc::strong_count(&v3));
    println!("rc-count  v4: {:?}", Rc::strong_count(&v4));

    println!("rc-count ev1: {:?}", Rc::strong_count(&elems[0]));
    println!("rc-count ev2: {:?}", Rc::strong_count(&elems[1]));
    println!("rc-count ev3: {:?}", Rc::strong_count(&elems[2]));
    println!("rc-count ev4: {:?}", Rc::strong_count(&elems[3]));
}

