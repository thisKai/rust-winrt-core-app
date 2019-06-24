use std::{
    cell::UnsafeCell,
    sync::{Arc, Mutex, MutexGuard}
};

use winrt::ComPtr;

pub struct ComProp<T>(UnsafeCell<Option<ComPtr<T>>>);
impl<T> ComProp<T> {
    pub fn new(ptr: ComPtr<T>) -> Self {
        Self(UnsafeCell::new(Some(ptr)))
    }
    pub fn get(&self) -> Option<ComPtr<T>> {
        unsafe { &*self.0.get() }.as_ref().cloned()
    }
    pub fn set(&self, value: Option<ComPtr<T>>) {
        let ptr = unsafe { &mut*self.0.get() };
        *ptr = value;
    }
}
unsafe impl<T> Send for ComProp<T> {}
unsafe impl<T> Sync for ComProp<T> {}

impl<T> Default for ComProp<T> {
    fn default() -> Self {
        Self(UnsafeCell::new(None))
    }
}

pub struct ComIterMirror<T>(Mutex<Vec<ComProp<T>>>);
impl<T> ComIterMirror<T> {
    pub fn get(&self) -> MutexGuard<Vec<ComProp<T>>> {
        self.0.lock().expect("ComIterMirror: mutex lock failed")
    }
    pub fn vec(&self) -> Vec<ComPtr<T>> {
        self
            .get()
            .iter()
            .filter_map(|i| i.get().clone())
            .collect()
    }
    pub fn push(&self, item: ComPtr<T>) {
        self.get().push(ComProp::new(item));
    }
}
impl<T> Default for ComIterMirror<T> {
    fn default() -> Self {
        Self(Mutex::new(Vec::new()))
    }
}
