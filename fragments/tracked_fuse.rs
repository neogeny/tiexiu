use std::any::type_name;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct TrackedFuse<T>(Option<()>, PhantomData<T>);

impl<T> Default for TrackedFuse<T> {
    fn default() -> Self {
        println!(
            "[CREATE] {} (Size: {} bytes)",
            type_name::<T>(),
            std::mem::size_of::<T>()
        );
        Self(Some(()), PhantomData)
    }
}

// Automatically hooks into Clones of the parent struct
impl<T> Clone for TrackedFuse<T> {
    fn clone(&self) -> Self {
        println!("[CLONE] {}", type_name::<T>());
        Self(self.0, PhantomData)
    }
}

// Automatically hooks into Drops of the parent struct
impl<T> Drop for TrackedFuse<T> {
    fn drop(&mut self) {
        println!("[DROP] {}", type_name::<T>());
    }
}
