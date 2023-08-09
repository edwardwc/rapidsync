use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug)]
pub struct SnapRef<T> {
    pub data: Arc<T>
}

impl<T> Deref for SnapRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Arc::as_ptr(&self.data) }
    }
}