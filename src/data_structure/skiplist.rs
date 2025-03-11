use cargo_snippet::snippet;

#[snippet("r3yohei_skiplist")]
use rand::Rng;
#[snippet("r3yohei_skiplist")]
use std::{
    alloc::{alloc, dealloc, Layout},
    fmt::Debug,
    ops::Index,
    ptr::{null_mut, read, write},
};

#[snippet("r3yohei_skiplist")]
const MAX_LEVEL: usize = 20;

#[snippet("r3yohei_skiplist")]
struct Node<T> {
    value: *mut T,
    next: [*mut Self; MAX_LEVEL],
    skip: [usize; MAX_LEVEL],
}
#[snippet("r3yohei_skiplist")]
impl<T> Node<T> {
    const LAYOUT: Layout = Layout::new::<Self>();
    const LAYOUT_T: Layout = Layout::new::<T>();

    fn alloc(value: T) -> *mut Self {
        unsafe {
            let ptr = alloc(Self::LAYOUT) as *mut Self;
            (*ptr).value = alloc(Self::LAYOUT_T) as *mut T;
            write((*ptr).value, value);
            ptr
        }
    }

    fn dealloc(ptr: *mut Self) -> T {
        unsafe {
            let value = read(&*((*ptr).value));
            dealloc((*ptr).value as *mut u8, Self::LAYOUT_T);
            dealloc(ptr as *mut u8, Self::LAYOUT);
            value
        }
    }
}

#[snippet("r3yohei_skiplist")]
pub struct SkipList<T> {
    head: Node<T>,
    len: usize,
    rng: rand::rngs::ThreadRng,
}
#[snippet("r3yohei_skiplist")]
impl<T> SkipList<T> {
    pub fn new() -> Self {
        SkipList {
            head: Node {
                value: null_mut(),
                next: [null_mut(); MAX_LEVEL],
                skip: [1; MAX_LEVEL],
            },
            len: 0,
            rng: rand::thread_rng(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn gen_level(&mut self) -> usize {
        let mut level = 1;
        while self.rng.gen_bool(0.5) && level < MAX_LEVEL {
            level += 1;
        }
        level
    }

    pub fn insert(&mut self, mut index: usize, element: T) {
        if index > self.len {
            panic!("index out of bounds");
        }

        self.len += 1;

        let new_node = Node::alloc(element);
        let new_level = self.gen_level();

        let mut cur = &mut self.head as *mut Node<T>;

        for l in (0..MAX_LEVEL).rev() {
            unsafe {
                while (*cur).skip[l] <= index {
                    index -= (*cur).skip[l];
                    cur = (*cur).next[l];
                }
                if l < new_level {
                    (*new_node).next[l] = (*cur).next[l];
                    (*cur).next[l] = new_node;
                    (*new_node).skip[l] = (*cur).skip[l] - index;
                    (*cur).skip[l] = index + 1;
                } else {
                    (*cur).skip[l] += 1;
                }
            }
        }
    }

    pub fn remove(&mut self, mut index: usize) -> T {
        if index >= self.len {
            panic!("index out of bounds");
        }

        self.len -= 1;

        let mut cur = &mut self.head as *mut Node<T>;
        index += 1;

        for l in (0..MAX_LEVEL).rev() {
            unsafe {
                while (*cur).skip[l] < index {
                    index -= (*cur).skip[l];
                    cur = (*cur).next[l];
                }
                if (*cur).skip[l] == index {
                    let next = (*cur).next[l];
                    (*cur).next[l] = (*next).next[l];
                    (*cur).skip[l] += (*next).skip[l] - 1;
                    if l == 0 {
                        return Node::dealloc(next);
                    }
                } else {
                    (*cur).skip[l] -= 1;
                }
            }
        }

        unreachable!()
    }

    pub fn push_back(&mut self, element: T) {
        self.insert(self.len, element);
    }

    pub fn push_front(&mut self, element: T) {
        self.insert(0, element);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            Some(self.remove(self.len - 1))
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            Some(self.remove(0))
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            _target: self,
            cur: &self.head,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let cur = &mut self.head as *mut Node<T>;
        IterMut { _target: self, cur }
    }
}
#[snippet("r3yohei_skiplist")]
impl<T> Drop for SkipList<T> {
    fn drop(&mut self) {
        let mut cur = self.head.next[0];
        while !cur.is_null() {
            unsafe {
                let next = (*cur).next[0];
                Node::dealloc(cur);
                cur = next;
            }
        }
    }
}
#[snippet("r3yohei_skiplist")]
impl<T: Debug> Debug for SkipList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
#[snippet("r3yohei_skiplist")]
pub struct Iter<'a, T> {
    _target: &'a SkipList<T>,
    cur: *const Node<T>,
}
#[snippet("r3yohei_skiplist")]
pub struct IterMut<'a, T> {
    _target: &'a SkipList<T>,
    cur: *mut Node<T>,
}
#[snippet("r3yohei_skiplist")]
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if unsafe { (*self.cur).next[0].is_null() } {
            None
        } else {
            self.cur = unsafe { (*self.cur).next[0] };
            Some(unsafe { &*(*self.cur).value })
        }
    }
}
#[snippet("r3yohei_skiplist")]
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if unsafe { (*self.cur).next[0].is_null() } {
            None
        } else {
            self.cur = unsafe { (*self.cur).next[0] };
            Some(unsafe { &mut *(*self.cur).value })
        }
    }
}
#[snippet("r3yohei_skiplist")]
impl<'a, T> IntoIterator for &'a SkipList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[snippet("r3yohei_skiplist")]
impl<'a, T> IntoIterator for &'a mut SkipList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
#[snippet("r3yohei_skiplist")]
impl<T> Index<usize> for SkipList<T> {
    type Output = T;

    fn index(&self, mut index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }

        let mut cur = &self.head as *const Node<T>;
        index += 1;

        for l in (0..MAX_LEVEL).rev() {
            unsafe {
                while (*cur).skip[l] <= index {
                    index -= (*cur).skip[l];
                    cur = (*cur).next[l];
                }
            }
        }

        unsafe { &*(*cur).value }
    }
}
