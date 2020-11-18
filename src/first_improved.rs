/// some easy improvements over `first`
///
/// 1) change `Link` to simply alias `Option<Box<Node>>`
/// 2) substitute `std::mem::replace(x, None)` with `x.take()` (yay, options!)
/// 3) substitute `match option { None => None, Some(x) => Some(y) }` with `option.map(|x| y)`

pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

type Link = Option<Box<Node>>;

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.pop_node().map(|node| node.elem)
    }

    fn pop_node(&mut self) -> Link {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        while let Some(_) = self.pop_node() {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        // normal removal...
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        // some more normal removal...
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // and exhaustion...
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
