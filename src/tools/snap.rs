use std::ops::Deref;
use std::sync::Arc;

pub struct SnapRef<'a, T> {
    pub data: &'a Arc<T>
}

impl<'a, T> Deref for SnapRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Arc::as_ptr(self.data) }
    }
}