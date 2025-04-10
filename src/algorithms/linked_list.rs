// 使用Box固定节点的长度
type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    size: usize,
    head: Link<T>,
}

struct Node<T> {
    ele: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            size: 0,
            head: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        0 == self.size
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, val: T) {
        // take方法获取到对象的所有权，将原来得位置替换为None
        let node = Some(Box::new(Node {
            ele: val,
            next: self.head.take(),
        }));
        self.head = node;
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.size -= 1;
            self.head = node.next;
            node.ele
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.ele)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.ele)
    }

    pub fn into_iter(self) -> IntoIterator<T> {
        IntoIterator(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

pub struct IntoIterator<T>(List<T>);
impl<T> Iterator for IntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.ele
        })
    }
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.ele
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // 获取当头结点的所有权
        let mut link = self.head.take();
        // 遍历释放所有的节点
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_created() {
        let mut l = List::new();

        l.push(1);
        l.push(2);
        l.push(3);

        assert!(l.size() == 3);
        assert_eq!(l.pop(), Some(3));

        assert!(l.peek() == Some(&2));
        assert_eq!(l.size(), 2);
    }
}
