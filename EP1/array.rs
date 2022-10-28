//! Alocação de um array estático

extern "C" {
    fn calloc(num: usize, size: usize) -> usize;
}

// Box<[T]> = unique_ptr<T[]>
/// Aloca um array estático
pub fn new<T: Default>(len: usize) -> Box<[T]> {
    let mut arr = unsafe {
        Box::from_raw(std::slice::from_raw_parts_mut(
            calloc(len, std::mem::size_of::<T>()) as *mut _,
            len,
        ))
    };
    for i in arr.iter_mut() {
        *i = T::default();
    }
    arr
}
