use std::ops::{Deref, DerefMut};

pub mod resources {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait Resource: Default {
        unsafe fn initialize(&mut self) -> bool;
        unsafe fn clear(&mut self) -> bool;
    }
}

pub struct ManagedResource<R: resources::Resource>(R);

impl<R: resources::Resource> ManagedResource<R> {
    pub fn initialize() -> Self {
        let mut r: R = Default::default();
        unsafe {
            r.initialize();
        }
        Self(r)
    }
}

impl<R: resources::Resource> Drop for ManagedResource<R> {
    fn drop(&mut self) {
        unsafe {
            self.0.clear();
        }
    }
}

impl<R: resources::Resource> DerefMut for ManagedResource<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<R: resources::Resource> Deref for ManagedResource<R> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
