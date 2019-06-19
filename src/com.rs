use std::sync::{Arc, Mutex, MutexGuard};

use winrt::ComPtr;

struct ComPropInner<T>(ComPtr<T>);
unsafe impl<T> Send for ComPropInner<T> {}

pub struct ComProp<T>(Arc<Mutex<Option<ComPropInner<T>>>>);
impl<T> ComProp<T> {
    pub fn new(ptr: ComPtr<T>) -> Self {
        Self(Arc::new(Mutex::new(Some(ComPropInner(ptr)))))
    }
    pub fn get(&self) -> Option<ComPtr<T>> {
        let lock = self.0.lock().ok()?;
        lock.as_ref().map(|inner| inner.0.clone())
    }
    pub fn set(&self, value: Option<ComPtr<T>>) {
        let mut lock = self.0.lock().expect("ComProp set: mutex lock failed");
        if let Some(current) = &mut *lock {
            if let Some(new) = value {
                current.0 = new;
            } else {
                *lock = None;
            }
        } else {
            *lock = value.map(|v| ComPropInner(v));
        }
    }
}
impl<T> Default for ComProp<T> {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(None)))
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
