use core::ops::Drop;
use std::alloc::{Layout, alloc, dealloc, handle_alloc_error};
use std::ptr::{self, NonNull};
use std::ops::{Deref, DerefMut};

// TODO: Implement Deref trait
// TODO: implement DerefMut trait
// TODO: implement and use custom allocator

pub struct Box<T> {
    ptr: NonNull<T>,
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

    pub fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr.as_ptr() }
    }

    // TODO: add into_raw() method
    // TODO: add from raw() method
}

impl<T> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr.as_ptr() }
    }
}

impl<T> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr.as_ptr() }
    }
}

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        let layout = Layout::new::<T>();

        unsafe {
            ptr::drop_in_place(self.ptr.as_ptr());
            dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}
