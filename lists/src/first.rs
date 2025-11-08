pub struct List {
    head: Link
}

struct Node {
    elem: i32,
    next: Link
}

enum Link {
    Empty, 
    More(Box<Node>)
}

impl List {
    pub fn new() -> self {
        // Initializes a 0 init list data struct
        List { head: Link::Empty }
    }
    fn append(&mut self, link: Link) {}
    fn pop(&mut self) -> Link {}
    fn find(&self, val: i32) -> bool {}
}
