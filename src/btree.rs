#[allow(dead_code)]
#[allow(unused_variables)]

enum Color {
    Red,
    Black
}

pub struct RedBlackBST <'a, T> {
    head: Option<Box<Node<'a, T>>>,
    size: u32
}

impl<'a, T> RedBlackBST<'a, T> {
    pub fn new() -> Self {
        RedBlackBST {
            head: None,
            size: 0
        }
    }
    pub fn size(&self) -> u32 {
        self.size
    }
    pub fn insert(&mut self, key: u32, val: &'a T) -> Result<(), &str> {
        let mut out: Result<(), &str> = Ok(());
        match self.head {
            None => self.head = Node::new(key, val, None, Color::Red),
            Some(ref mut head) => {out = head.insert(key, val);}
        }
        match out {
            Ok(_) => self.size += 1,
            Err(_) => ()
        }
        out
    }
    pub fn get(&self, key: u32) -> Result<&'a T, &str> {
        match self.head.as_ref() {
            None => Err("Tree is empty"),
            Some(head) => head.get(key)
        }
    }
}

struct Node <'a, T> {
    key: u32,
    val: &'a T,
    l: Option<Box<Node<'a, T>>>,
    r: Option<Box<Node<'a, T>>>,
    p: Option<Box<Node<'a, T>>>,
    color: Color
}

impl<'a, T> Node<'a, T> {
    fn new(key: u32, val: &'a T, parent: Option<Box<Self>>, color: Color) -> Option<Box<Self>> {
        Some(Box::new(Node {
            key,
            val,
            l: None,
            r: None,
            p: parent,
            color
        }))
    }
    fn get(&self, key: u32) -> Result<&'a T, &str> {
        if self.key == key {
            Ok(self.val)
        } else {
            let target = if key < self.key {&self.l} else {&self.r};
            match target {
                Some(ref subnode) => subnode.get(key),
                None => Err("Key: not found")
            }
        }
    }
    fn insert(&mut self, key: u32, val: &'a T) -> Result<(), &str> {
        let mut out: Result<(), &str> = Ok(());
        if self.key != key {
            unsafe {
                let target = if key < self.key {&mut self.l} else {&mut self.r};
                match target {
                    &mut Some(ref mut subnode) => out = subnode.insert(key, val),
                    &mut None => *target = Node::new(key, val, Some(Box::new(self)), Color::Red)
                }
            }
        } else {
            out = Err("Key already present");
        }
        out
    }
}
