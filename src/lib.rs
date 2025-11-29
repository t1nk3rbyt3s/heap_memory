use std::ptr::{self, NonNull};
use std::alloc::{alloc, dealloc, Layout, handle_alloc_error};

pub struct Box<T> {
    ptr: NonNull<T>
}

impl<T> Box<T> {
    /// initialize our 'Box' with value 'T'
    pub fn new(value: T) -> Self {
        let layout = Layout::new::<T>();
        let raw_ptr = unsafe { alloc(layout) } as *mut T;

        let raw_ptr = match NonNull::<T>::new(raw_ptr) {
            Some(ptr) => ptr,
            None => handle_alloc_error(layout),
        };

        unsafe { ptr::write(raw_ptr.as_ptr(), value) }

        Box { ptr: raw_ptr }
    }

    pub fn as_ref(&self) -> &T {
        unsafe { &*self.ptr.as_ptr() }
    }
}
