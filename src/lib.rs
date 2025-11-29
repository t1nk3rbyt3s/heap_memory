use std::ptr::NonNull;

pub struct Box<T> {
    ptr: NonNull<T>
}
