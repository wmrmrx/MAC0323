use super::{ItemBounds, KeyBounds, SymbolTable};

extern "C" {
    fn srand(seed: u32);
    fn rand() -> u32;
}

struct Node<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    key: Key,
    val: Item,
    height: u32,
    // Número de nós que são descendentes desse nó
    // (Ou seja, filhos, filhos dos filhos, etc.)
    // incluindo ele mesmo
    count: usize,
    child: [Option<Box<Self>>; 2],
}

impl<Key, Item> Node<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    fn new(key: Key, val: Item) -> Box<Self> {
        Box::new(Self {
            key,
            val,
            height: unsafe { rand() },
            count: 1,
            child: [None, None],
        })
    }

    fn update_count(&mut self) {
        self.count = 1_usize
            + self.child[0].as_ref().map_or(0, |c| c.count)
            + self.child[1].as_ref().map_or(0, |c| c.count);
    }

    // Devolve a nova raíz da subárvore
    fn add(mut cur: Box<Self>, key: Key, val: Item) -> Box<Self> {
        if key == cur.key {
            cur.val = val;
            cur
        } else {
            let side = if key < cur.key { 0 } else { 1 };
            let mut child = if let Some(child) = cur.child[side].take() {
                Node::add(child, key, val)
            } else {
                Node::new(key, val)
            };
            if child.height <= cur.height {
                cur.child[side] = Some(child);
                cur.update_count();
                cur
            } else {
                cur.child[side] = child.child[1 - side].take();
                cur.update_count();
                child.child[1 - side] = Some(cur);
                child.update_count();
                child
            }
        }
    }

    fn value(&mut self, key: &Key) -> Option<&mut Item> {
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

    fn rank(&self, key: &Key) -> usize {
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

    fn select(&self, k: usize) -> &Key {
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

pub struct TR<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    root: Option<Box<Node<Key, Item>>>,
}

impl<Key, Item> TR<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    pub fn new() -> Self {
        // Determinismo para facilitar
        unsafe {
            srand(0);
        }
        Self { root: None }
    }
}

impl<Key, Item> SymbolTable<Key, Item> for TR<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    fn add(&mut self, key: Key, val: Item) {
        if let Some(root) = self.root.take() {
            self.root = Some(Node::add(root, key, val));
        } else {
            self.root = Some(Node::new(key, val));
        }
    }

    fn value(&mut self, key: &Key) -> Option<&mut Item> {
        self.root.as_mut()?.value(key)
    }

    fn rank(&self, key: &Key) -> usize {
        if let Some(root) = self.root.as_ref() {
            root.rank(key)
        } else {
            0
        }
    }

    fn select(&self, k: usize) -> Option<&Key> {
        let size = self.root.as_ref().map_or(0, |root| root.count);
        if k < size {
            Some(self.root.as_ref().unwrap().select(k))
        } else {
            None
        }
    }
}
