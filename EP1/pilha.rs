//! Implementação de uma pilha

use super::array;

pub struct Pilha<T: Default> {
    len: usize,
    capacity: usize,
    arr: Box<[T]>,
}

#[allow(dead_code)]
impl<T: Default> Pilha<T> {
    pub fn new() -> Pilha<T> {
        Pilha {
            len: 0,
            capacity: 0,
            arr: Box::default(),
        }
    }

    pub fn push(&mut self, x: T) {
        if self.capacity == 0 {
            self.capacity = 4;
            self.arr = array::new(4);
        } else if self.len == self.capacity {
            self.capacity *= 2;
            let mut new_arr = array::new::<T>(self.capacity);
            for (idx, i) in self.arr.iter_mut().enumerate() {
                new_arr[idx] = std::mem::take(i);
            }
            self.arr = new_arr;
        }
        self.arr[self.len] = x;
        self.len += 1;
    }

    pub fn pop(&mut self) {
        if self.len > 0 {
            self.len -= 1;
        } else {
            panic!();
        }
    }

    pub fn back(&mut self) -> &mut T {
        assert!(self.len > 0);
        &mut self.arr[self.len - 1]
    }

    pub fn size(&self) -> usize {
        self.len
    }

    pub fn empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.arr[0..self.len].iter()
    }
}
