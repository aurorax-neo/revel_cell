use crate::weak::Weak;
use std::cell::UnsafeCell;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

pub struct Arc<T>(pub(super) std::sync::Arc<UnsafeCell<T>>);

// impl
impl<T: 'static> Arc<T> {
    pub fn new(value: T) -> Self {
        Arc(std::sync::Arc::new(UnsafeCell::new(value)))
    }

    pub fn downgrade(&self) -> Weak<T> {
        Weak(std::sync::Arc::downgrade(&self.0))
    }

    pub fn eq_weak(&self, other: &Weak<T>) -> bool {
        match other.upgrade() {
            None => false,
            Some(v) => self.eq(&v),
        }
    }

    pub fn inner_ptr(&self) -> *const UnsafeCell<T> {
        std::sync::Arc::as_ptr(&self.0)
    }
    pub fn from_raw(ptr: *const UnsafeCell<T>) -> Arc<T> {
        let arc = unsafe { std::sync::Arc::from_raw(ptr) };
        Arc(arc)
    }

    pub fn set_value(&self, value: T) {
        unsafe {
            let inner = &mut *self.0.get();
            *inner = value;
        }
    }

    pub fn strong_count(&self) -> usize {
        std::sync::Arc::strong_count(&self.0)
    }

    pub fn weak_count(&self) -> usize {
        std::sync::Arc::weak_count(&self.0)
    }
}

#[cfg(feature = "thread-safe")]
unsafe impl<T: Send> Send for Arc<T> {}
#[cfg(feature = "thread-safe")]
unsafe impl<T: Sync> Sync for Arc<T> {}

// Debug
impl<T> Debug for Arc<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// Clone
impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        Arc(self.0.clone())
    }
}

// Default
impl<T: Default> Default for Arc<T> {
    fn default() -> Self {
        Self(std::sync::Arc::new(UnsafeCell::new(T::default())))
    }
}

// PartialEq
impl<T> PartialEq<Self> for Arc<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.0, &other.0)
    }
}

// Eq
impl<T> Eq for Arc<T> {}

// Deref and DerefMut
impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &mut *self.0.get() }
    }
}

impl<T> DerefMut for Arc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0.get() }
    }
}

// Hash
impl<T: Hash + 'static> Hash for Arc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner_ptr().hash(state);
    }
}
