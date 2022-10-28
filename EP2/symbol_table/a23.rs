use super::{ItemBounds, KeyBounds, SymbolTable};

type Nptr<Key, Item> = Box<Node<Key, Item>>;

#[derive(Default, Debug)]
struct TwoNode<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    key: Key,
    val: Item,
    count: usize,
    child: Option<[Nptr<Key, Item>; 2]>,
}

impl<Key, Item> TwoNode<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    fn new(key: Key, val: Item) -> Self {
        Self {
            key,
            val,
            count: 1,
            child: None,
        }
    }

    fn new_with_child(key: Key, val: Item, child: [Nptr<Key, Item>; 2]) -> Self {
        Self {
            key,
            val,
            count: 1 + child[0].count() + child[1].count(),
            child: Some(child),
        }
    }
}

#[derive(Default, Debug)]
struct ThreeNode<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    keys: [Key; 2],
    vals: [Item; 2],
    count: usize,
    child: Option<[Nptr<Key, Item>; 3]>,
}

impl<Key, Item> ThreeNode<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    fn new(keys: [Key; 2], vals: [Item; 2]) -> Self {
        Self {
            keys,
            vals,
            count: 2,
            child: None,
        }
    }

    fn new_with_child(keys: [Key; 2], vals: [Item; 2], child: [Nptr<Key, Item>; 3]) -> Self {
        Self {
            keys,
            vals,
            count: 2 + child[0].count() + child[1].count() + child[2].count(),
            child: Some(child),
        }
    }
}

#[derive(Debug)]
enum Node<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    Two(TwoNode<Key, Item>),
    Three(ThreeNode<Key, Item>),
}

impl<Key, Item> Default for Node<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    fn default() -> Self {
        Self::Two(TwoNode::default())
    }
}

impl<Key, Item> Node<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    fn count(&self) -> usize {
        match self {
            Self::Two(node) => node.count,
            Self::Three(node) => node.count,
        }
    }

    /// Consome a nó cur
    /// Retorna Ok(cur) caso não precise de rebalanceamento ao adicionar a subárvore do nó
    /// Retorna Err([cur, nó2], k1, v1) caso precise de rebalanceamento
    fn add(
        mut cur: Nptr<Key, Item>,
        key: Key,
        val: Item,
    ) -> Result<Nptr<Key, Item>, ([Nptr<Key, Item>; 2], Key, Item)> {
        match *cur {
            Self::Two(mut node) => {
                *cur = if key == node.key {
                    node.val = val;
                    Self::Two(node)
                } else {
                    let side = if key < node.key { 0 } else { 1 };
                    let TwoNode {
                        key: nkey,
                        val: nval,
                        child: nchild,
                        ..
                    } = node;
                    if let Some([mut next, mut other]) = nchild {
                        if side == 1 {
                            std::mem::swap(&mut other, &mut next)
                        };
                        match Self::add(next, key, val) {
                            Ok(next) => {
                                let child = if side == 0 {
                                    [next, other]
                                } else {
                                    [other, next]
                                };
                                Self::Two(TwoNode::new_with_child(nkey, nval, child))
                            }
                            Err(([c1, c2], key, val)) => Self::Three(if side == 0 {
                                ThreeNode::new_with_child([key, nkey], [val, nval], [c1, c2, other])
                            } else {
                                ThreeNode::new_with_child([nkey, key], [nval, val], [other, c1, c2])
                            }),
                        }
                    } else {
                        Self::Three(if side == 0 {
                            ThreeNode::new([key, nkey], [val, nval])
                        } else {
                            ThreeNode::new([nkey, key], [nval, val])
                        })
                    }
                };
                Ok(cur)
            }
            Self::Three(mut node) => {
                let err: Option<(Nptr<Key, Item>, Key, Item)>;
                *cur = if key == node.keys[0] {
                    node.vals[0] = val;
                    err = None;
                    Self::Three(node)
                } else if key == node.keys[1] {
                    node.vals[1] = val;
                    err = None;
                    Self::Three(node)
                } else {
                    let side = if key < node.keys[0] {
                        0
                    } else if key < node.keys[1] {
                        1
                    } else {
                        2
                    };
                    if let Some([mut next, mut other1, mut other2]) = node.child {
                        if side == 1 {
                            std::mem::swap(&mut next, &mut other1);
                        } else if side == 2 {
                            std::mem::swap(&mut next, &mut other2);
                        }
                        match Self::add(next, key, val) {
                            Ok(next) => {
                                let ThreeNode { keys, vals, .. } = node;
                                let child = if side == 0 {
                                    [next, other1, other2]
                                } else if side == 1 {
                                    [other1, next, other2]
                                } else {
                                    [other2, other1, next]
                                };
                                err = None;
                                Self::Three(ThreeNode::new_with_child(keys, vals, child))
                            }
                            Err(([c1, c2], key, val)) => {
                                let ThreeNode {
                                    keys: [k1, k2],
                                    vals: [v1, v2],
                                    ..
                                } = node;
                                let [(k1, v1), (k2, v2), (k3, v3)] = if side == 0 {
                                    [(key, val), (k1, v1), (k2, v2)]
                                } else if side == 1 {
                                    [(k1, v1), (key, val), (k2, v2)]
                                } else {
                                    [(k1, v1), (k2, v2), (key, val)]
                                };
                                let [c1, c2, c3, c4] = if side == 0 {
                                    [c1, c2, other1, other2]
                                } else if side == 1 {
                                    [other1, c1, c2, other2]
                                } else {
                                    [other2, other1, c1, c2]
                                };
                                err = Some((
                                    Box::new(Self::Two(TwoNode::new_with_child(k3, v3, [c3, c4]))),
                                    k2,
                                    v2,
                                ));
                                Self::Two(TwoNode::new_with_child(k1, v1, [c1, c2]))
                            }
                        }
                    } else {
                        let ThreeNode {
                            keys: [k1, k2],
                            vals: [v1, v2],
                            ..
                        } = node;
                        let [(k1, v1), (k2, v2), (k3, v3)] = if side == 0 {
                            [(key, val), (k1, v1), (k2, v2)]
                        } else if side == 1 {
                            [(k1, v1), (key, val), (k2, v2)]
                        } else {
                            [(k1, v1), (k2, v2), (key, val)]
                        };
                        err = Some((Box::new(Self::Two(TwoNode::new(k3, v3))), k2, v2));
                        Self::Two(TwoNode::new(k1, v1))
                    }
                };
                if let Some((node, key, val)) = err {
                    Err(([cur, node], key, val))
                } else {
                    Ok(cur)
                }
            }
        }
    }

    fn value(&mut self, key: &Key) -> Option<&mut Item> {
        match self {
            Self::Two(node) => {
                if key == &node.key {
                    return Some(&mut node.val);
                }
                let side = if key < &node.key { 0 } else { 1 };
                if let Some(child) = node.child.as_mut() {
                    child[side].value(key)
                } else {
                    None
                }
            }
            Self::Three(node) => {
                if key == &node.keys[0] {
                    return Some(&mut node.vals[0]);
                } else if key == &node.keys[1] {
                    return Some(&mut node.vals[1]);
                }
                let side = if key < &node.keys[0] {
                    0
                } else if key < &node.keys[1] {
                    1
                } else {
                    2
                };
                if let Some(child) = node.child.as_mut() {
                    child[side].value(key)
                } else {
                    None
                }
            }
        }
    }

    fn rank(&self, key: &Key) -> usize {
        match self {
            Self::Two(node) => {
                let left_count = node.child.as_ref().map_or(0, |child| child[0].count());
                if key == &node.key {
                    return left_count;
                }
                let side = if key < &node.key { 0 } else { 1 };
                node.child.as_ref().map_or(0, |child| child[side].rank(key))
                    + if side == 0 { 0 } else { left_count + 1 }
            }
            Self::Three(node) => {
                let left_count = node.child.as_ref().map_or(0, |child| child[0].count());
                let mid_count = node.child.as_ref().map_or(0, |child| child[1].count());
                if key == &node.keys[0] {
                    return left_count;
                } else if key == &node.keys[1] {
                    return left_count + 1 + mid_count;
                }
                let side = if key < &node.keys[0] {
                    0
                } else if key < &node.keys[1] {
                    1
                } else {
                    2
                };
                node.child.as_ref().map_or(0, |child| child[side].rank(key))
                    + if side == 0 {
                        0
                    } else if side == 1 {
                        left_count + 1
                    } else {
                        left_count + mid_count + 2
                    }
            }
        }
    }

    fn select(&self, k: usize) -> &Key {
        match self {
            Self::Two(node) => {
                let left_count = node.child.as_ref().map_or(0, |child| child[0].count());
                if left_count == k {
                    return &node.key;
                }
                let side = if k < left_count { 0 } else { 1 };
                let child = node.child.as_ref().unwrap();
                child[side].select(if side == 0 { k } else { k - (left_count + 1) })
            }
            Self::Three(node) => {
                let left_count = node.child.as_ref().map_or(0, |child| child[0].count());
                let mid_count = node.child.as_ref().map_or(0, |child| child[1].count());
                if left_count == k {
                    return &node.keys[0];
                } else if left_count + mid_count + 1 == k {
                    return &node.keys[1];
                }
                let side = if k < left_count {
                    0
                } else if k < left_count + mid_count + 1 {
                    1
                } else {
                    2
                };
                let child = node.child.as_ref().unwrap();
                child[side].select(if side == 0 {
                    k
                } else if side == 1 {
                    k - (left_count + 1)
                } else {
                    k - (left_count + mid_count + 2)
                })
            }
        }
    }
}

pub struct A23<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    root: Option<Nptr<Key, Item>>,
}

impl<Key, Item> A23<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    pub fn new() -> Self {
        Self { root: None }
    }
}

impl<Key, Item> SymbolTable<Key, Item> for A23<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    fn add(&mut self, key: Key, val: Item) {
        self.root = if let Some(root) = self.root.take() {
            match Node::add(root, key, val) {
                Ok(root) => Some(root),
                Err((child, key, val)) => Some(Box::new(Node::Two(TwoNode::new_with_child(
                    key, val, child,
                )))),
            }
        } else {
            Some(Box::new(Node::Two(TwoNode::new(key, val))))
        };
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
        let size = self.root.as_ref().map_or(0, |root| root.count());
        if k < size {
            Some(self.root.as_ref().unwrap().select(k))
        } else {
            None
        }
    }
}
