mod a23;
mod abb;
mod arn;
mod tr;
mod vo;

pub use self::a23::A23;
pub use self::abb::ABB;
pub use self::arn::ARN;
pub use self::tr::TR;
pub use self::vo::VO;

use std::fmt::Debug;

pub trait KeyBounds: Ord + Default + Debug {}
pub trait ItemBounds: Default + Debug {}

impl<T: Ord + Default + Debug> KeyBounds for T {}
impl<T: Default + Debug> ItemBounds for T {}

// Key: Precisa implementar Ord para comparação
pub trait SymbolTable<Key, Item>
where
    Key: KeyBounds,
    Item: ItemBounds,
{
    // Adiciona um nó com o par (key, val)
    // Caso exista muda o valor do item correspondente a key para val
    fn add(&mut self, key: Key, val: Item);
    // Devolve a referência ao item correspondente à chave key.
    fn value(&mut self, key: &Key) -> Option<&mut Item>;
    // Conta o número de elementos estritamente menores que key na tabela de símbolos
    fn rank(&self, key: &Key) -> usize;
    // Devolve a referência (que deve ser imutável) para o elemento com rank k
    fn select(&self, k: usize) -> Option<&Key>;
}
