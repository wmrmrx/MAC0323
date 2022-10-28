use super::{ItemBounds, KeyBounds, SymbolTable};
use std::ptr;

#[derive(PartialEq, Debug)]
enum Color {
    Red,
    Black,
}

use self::Color::{Black, Red};

#[derive(Debug)]
struct Node<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    key: Key,
    val: Item,
    color: Color,
    // Número de nós que são descendentes desse nó
    // (Ou seja, filhos, filhos dos filhos, etc.)
    // incluindo ele mesmo
    count: usize,
    dad: *mut Self,
    child: [*mut Self; 2],
}

impl<Key, Item> Node<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    fn new(key: Key, val: Item, color: Color) -> *mut Self {
        Box::into_raw(Box::new(Self {
            key,
            val,
            color,
            count: 1,
            dad: ptr::null_mut(),
            child: [ptr::null_mut(), ptr::null_mut()],
        }))
    }

    fn update(cur: *mut Self) {
        let mut count = 0;
        unsafe {
            for side in 0..2 {
                let child = (*cur).child[side];
                if !child.is_null() {
                    count += (*child).count;
                }
            }
            (*cur).count = 1 + count;
        }
    }

    fn update_up(mut cur: *mut Self) {
        unsafe {
            while !(*cur).dad.is_null() {
                cur = (*cur).dad;
                Node::update(cur);
            }
        }
    }

    fn assign(child: *mut Self, dad: *mut Self, side: usize) {
        unsafe {
            if !child.is_null() {
                (*child).dad = dad;
            }
            if !dad.is_null() {
                (*dad).child[side] = child;
                Node::update(dad);
            }
        }
    }

    fn rotate(a: *mut Self, b: *mut Self, side: usize) {
        unsafe {
            let dad = (*b).dad;
            Node::assign((*a).child[1 - side], b, side);
            Node::assign(b, a, 1 - side);
            if dad.is_null() {
                (*a).dad = ptr::null_mut();
            } else {
                let side = if (*dad).child[0] == b { 0 } else { 1 };
                Node::assign(a, dad, side);
            }
        }
    }

    // Devolve a nova raíz da subárvore
    fn balance(mut cur: *mut Self) -> *mut Self {
        unsafe {
            let dad = (*cur).dad;
            if dad.is_null() || (*dad).color == Black {
                return ptr::null_mut();
            }
            let grand = (*dad).dad;
            if grand.is_null() {
                (*dad).color = Black;
                return ptr::null_mut();
            }
            let side = if (*dad).child[0] == cur { 0 } else { 1 };
            let dad_side = if (*grand).child[0] == dad { 0 } else { 1 };
            let uncle = (*grand).child[1 - dad_side];
            if !uncle.is_null() && (*uncle).color == Red {
                (*dad).color = Black;
                (*uncle).color = Black;
                (*grand).color = Red;
                return Node::balance(grand);
            }
            if side == dad_side {
                Node::rotate(dad, grand, side);
                (*dad).color = Black;
                (*grand).color = Red;
                if (*dad).dad.is_null() {
                    return dad;
                }
            } else {
                Node::rotate(cur, dad, side);
                Node::rotate(cur, grand, dad_side);
                (*cur).color = Black;
                (*grand).color = Red;
                if (*cur).dad.is_null() {
                    return cur;
                }
            }
            ptr::null_mut()
        }
    }

    // Caso a raíz da subárvore mude, retorna o ponteiro para a nova raíz
    // Caso contrário, retorna o ponteiro nulo
    fn add(mut cur: *mut Self, key: Key, val: Item) -> *mut Self {
        unsafe {
            if key == (*cur).key {
                (*cur).val = val;
                return ptr::null_mut();
            }
            let side = if key < (*cur).key { 0 } else { 1 };
            if (*cur).child[side].is_null() {
                let child = Node::new(key, val, Red);
                Node::assign(child, cur, side);
                let res = Node::balance(child);
                Node::update_up(child);
                res
            } else {
                Node::add((*cur).child[side], key, val)
            }
        }
    }

    fn value(&mut self, key: &Key) -> Option<&mut Item> {
        unsafe {
            if key == &self.key {
                return Some(&mut self.val);
            }
            let side = if key < &self.key { 0 } else { 1 };
            if let Some(child) = self.child[side].as_mut() {
                child.value(key)
            } else {
                None
            }
        }
    }

    fn rank(&self, key: &Key) -> usize {
        unsafe {
            let left_count = self.child[0].as_ref().map_or(0, |child| child.count);
            if key == &self.key {
                return left_count;
            }
            let side = if key < &self.key { 0 } else { 1 };
            if let Some(child) = self.child[side].as_ref() {
                child.rank(key) + if side == 1 { left_count + 1 } else { 0 }
            } else if side == 1 {
                left_count + 1
            } else {
                0
            }
        }
    }

    fn select(&self, k: usize) -> &Key {
        unsafe {
            let left_count = self.child[0].as_ref().map_or(0, |child| child.count);
            if k == left_count {
                return &self.key;
            }
            let side = if k < left_count { 0 } else { 1 };
            return self.child[side].as_ref().unwrap().select(if side == 0 {
                k
            } else {
                k - left_count - 1
            });
        }
    }
}

#[derive(Debug)]
pub struct ARN<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    root: *mut Node<Key, Item>,
}

impl<Key, Item> ARN<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    pub fn new() -> Self {
        Self {
            root: ptr::null_mut(),
        }
    }
}

impl<Key, Item> SymbolTable<Key, Item> for ARN<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    fn add(&mut self, key: Key, val: Item) {
        if self.root.is_null() {
            self.root = Node::new(key, val, Black);
        } else {
            let res = Node::add(self.root, key, val);
            if !res.is_null() {
                self.root = res;
            }
        }
    }

    fn value(&mut self, key: &Key) -> Option<&mut Item> {
        unsafe { self.root.as_mut()?.value(key) }
    }

    fn rank(&self, key: &Key) -> usize {
        unsafe {
            if let Some(root) = self.root.as_ref() {
                root.rank(key)
            } else {
                0
            }
        }
    }

    fn select(&self, k: usize) -> Option<&Key> {
        unsafe {
            let size = self.root.as_ref().map_or(0, |root| root.count);
            if k < size {
                Some(self.root.as_ref().unwrap().select(k))
            } else {
                None
            }
        }
    }
}
