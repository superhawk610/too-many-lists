/// the lists implemented by `first` and `second` aren't _persistent_; that is,
/// they exist in isolation, and in order to combine lists, the entire list
/// must be copied
///
/// persistent lists are commonly used in functional programming languages, and
/// they allow the underlying memory from lists to be reused wherever possible
///
///     list                       A -> B -> C -> D
///     list2 = tail(list1)             B -> C -> D
///     list3 = push(list2, X)     X -> B -> C -> D
///
/// with a non-persistent list, there'd be 3 copies of B -> C -> D allocated,
/// but with a persistent list, all three lists would share a single allocation
/// for B -> C -> D
///
///     list1 -- A
///              |
///     list2    B -> C -> D
///              |
///     list3 -- X
///
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn append(&self, elem: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn tail(&self) -> Self {
        Self {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        todo!()
    }

    #[test]
    fn iter() {
        todo!()
    }
}
