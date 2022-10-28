use super::{ItemBounds, KeyBounds, SymbolTable};

pub struct VO<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    // vec[i].0: Key; vec[i].1: Item
    vec: Vec<(Key, Item)>,
}

impl<Key, Item> VO<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    // Como temos um vetor ordenado nas chaves, basta fazer
    // uma busca binária para encontrar o índice de uma chave
    // (caso exista) no vetor
    fn find(&mut self, key: &Key) -> Option<&mut Item> {
        if self.vec.is_empty() {
            return None;
        }
        let (mut l, mut r) = (0, self.vec.len() - 1);
        while l < r {
            let mid = (l + r) / 2;
            if self.vec[mid].0 < *key {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        if self.vec[l].0 == *key {
            Some(&mut self.vec[l].1)
        } else {
            None
        }
    }

    // Busca binária para achar número de elementos menores que key
    fn rank(&self, key: &Key) -> usize {
        let (mut l, mut r) = (0, self.vec.len());
        while l < r {
            let mid = (l + r) / 2;
            if self.vec[mid].0 < *key {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l
    }
}

impl<Key, Item> SymbolTable<Key, Item> for VO<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    fn add(&mut self, key: Key, val: Item) {
        if let Some(key_val) = self.find(&key) {
            *key_val = val;
        } else {
            self.vec.push((key, val));
            let mut index = self.vec.len() - 1;
            while index > 0 && self.vec[index - 1].0 > self.vec[index].0 {
                self.vec.swap(index - 1, index);
                index -= 1;
            }
        }
    }

    fn value(&mut self, key: &Key) -> Option<&mut Item> {
        self.find(key)
    }

    fn rank(&self, key: &Key) -> usize {
        self.rank(key)
    }

    fn select(&self, k: usize) -> Option<&Key> {
        if k < self.vec.len() {
            Some(&self.vec[k].0)
        } else {
            None
        }
    }
}
