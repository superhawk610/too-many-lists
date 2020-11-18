/// this fails to compile because List doesn't have a known size at compile-time
/// (some sort of indirection is required, eg. heap allocation)
///
/// pub enum List {
///     Empty,
///     Elem(i32, List),
/// }
///
/// this compiles, but isn't effecient since the first element is always stack-allocated
/// and the `Empty` variant still has to allocate empty junk (since enums are sized
/// to hold an instance of their largest variant)
///
/// [] Stack-allocated
/// () Heap-allocated
///
/// [Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)
///
/// pub enum List {
///     Empty,
///     Elem(i32, Box<List>),
/// }

// because `List` is a struct containing a single field, it's the same size as that field!
pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

// since the `Link` enum only has 2 variants (one null, and the other containing a non-null ptr),
// Rust's compiler uses _null pointer optimization_ to avoid storing the traditional enum variant
// tag, instead zeroing out instances of `Empty` and just storing the content of `More` directly
enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    // implementation of `pop` without using `pop_node`
    // pub fn pop(&mut self) -> Option<i32> {
    //     match std::mem::replace(&mut self.head, Link::Empty) {
    //         Link::Empty => None,
    //         Link::More(node) => {
    //             self.head = node.next;
    //             Some(node.elem)
    //         }
    //     }
    // }

    pub fn pop(&mut self) -> Option<i32> {
        match self.pop_node() {
            Link::Empty => None,
            Link::More(node) => Some(node.elem),
        }
    }

    fn pop_node(&mut self) -> Link {
        let mut link = std::mem::replace(&mut self.head, Link::Empty);

        if let Link::More(ref mut node) = link {
            // we have to leave a valid value in `node.next`; think of this as
            // clipping off the pointer that's now owned by `self.head`
            self.head = std::mem::replace(&mut node.next, Link::Empty);
        }

        link
    }
}

// the compiler's default implementation for Drop isn't tail-recursive; in dropping the
// first element in the list, it will attempt to drop the box containing the link to the
// next box, which will in turn attempt to drop the next element, and so on until the
// stack blows up; this custom implementation drops links one at a time _after_ replacing
// their `next` link with an empty list to prevent further recursion
//
// impl Drop for List {
//     fn drop(&mut self) {
//         let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
//
//         while let Link::More(mut boxed_node) = cur_link {
//             cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
//             // `boxed_node` goes out of scope and is dropped here, and since we've replaced
//             // it's `next` element with an empty link, the drop won't recurse further
//         }
//     }
// }
//
// with `pop_node` implemented, this becomes much easier to express
impl Drop for List {
    fn drop(&mut self) {
        // the popped node is moved into `_` and implicitly dropped, so we don't
        // need to do anything inside the loop body
        while let Link::More(_) = self.pop_node() {}
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
