use crate::arc::Arc;
use std::cell::UnsafeCell;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

pub struct Weak<T>(pub(super) std::sync::Weak<UnsafeCell<T>>);

#[cfg(feature = "thread-safe")]
unsafe impl<T: Send> Send for Weak<T> {}
#[cfg(feature = "thread-safe")]
unsafe impl<T: Sync> Sync for Weak<T> {}

// impl
impl<T: 'static> Weak<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn upgrade(&self) -> Option<Arc<T>> {
        match self.0.upgrade() {
            None => None,
            Some(value) => Some(Arc(value)),
        }
    }
    pub fn upgradable(&self) -> bool {
        self.0.upgrade().is_some()
    }

    pub fn get(&self) -> Option<&mut T> {
        if let Some(arc) = self.0.upgrade() {
            unsafe {
                return Some(&mut *arc.get());
            }
        }
        None
    }

    pub fn eq_arc(&self, other: &Arc<T>) -> bool {
        match self.upgrade() {
            None => false,
            Some(v) => v.eq(&other),
        }
    }

    pub fn inner_ptr(&self) -> *const UnsafeCell<T> {
        self.0.as_ptr()
    }

    pub fn from_raw(ptr: *const UnsafeCell<T>) -> Weak<T> {
        let weak = unsafe { std::sync::Weak::from_raw(ptr) };
        Weak(weak)
    }

    pub fn strong_count(&self) -> usize {
        self.0.strong_count()
    }

    pub fn weak_count(&self) -> usize {
        self.0.weak_count()
    }
}

// Debug
impl<T> Debug for Weak<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// Clone
impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        Weak(self.0.clone())
    }
}

// Default
impl<T> Default for Weak<T> {
    fn default() -> Self {
        Weak(std::sync::Weak::new())
    }
}

// PartialEq
impl<T: PartialEq> PartialEq<Self> for Weak<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Weak::ptr_eq(&self.0, &other.0)
    }
}

// Eq
impl<T: Eq> Eq for Weak<T> {}

// Hash
impl<T: Hash + 'static> Hash for Weak<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner_ptr().hash(state);
    }
}
