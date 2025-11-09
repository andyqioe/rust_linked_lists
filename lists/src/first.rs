use std::mem;

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
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    fn push(&mut self, elem: i32) {
        let boxed: Box<Node> = Box::new(
            Node { elem: elem, next: mem::replace(&mut self.head, Link::Empty) }
        );
        self.head = Link::More(boxed);
    }
    fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            },
        }
    }
    // fn find(&self, val: i32) -> bool {}
}

impl Drop for List {
    fn drop(&mut self) {
        let mut link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut node) = link {
            link = mem::replace(&mut node.next, Link::Empty);
        }
    }
}
#[cfg(test)]
mod test {
    use super::List;
    #[test] 
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2332423);
        list.push(2222);

        let popped = list.pop();
        assert_eq!(popped, Some(2222));
        let popped = list.pop();
        assert_eq!(popped, Some(2332423));
        let popped = list.pop();
        assert_eq!(popped, Some(1));
        let popped = list.pop();
        assert_eq!(popped, None);
    }
}