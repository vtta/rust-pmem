pub struct PmemCell<T> {
    ptr: *const T,
}

impl<T> PmemCell<T> {
    pub unsafe fn new(ptr: *const T) -> Self {
        PmemCell { ptr: ptr }
    }
}

impl<T> ::std::ops::Deref for PmemCell<T> {
    type Target = T;
    fn deref(&self) -> &T { unsafe { &*self.ptr } }
}

impl<T> ::std::fmt::Debug for PmemCell<T> where T: ::std::fmt::Debug {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", (self as &T))
    }
}

impl<T> ::std::fmt::Display for PmemCell<T> where T: ::std::fmt::Display {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", (self as &T))
    }
}

pub struct PmemMutCell<T> {
    ptr: *mut T,
}

impl<T> PmemMutCell<T> {
    pub unsafe fn new(ptr: *mut T) -> Self {
        PmemMutCell { ptr: ptr }
    }
}

impl<T> ::std::ops::Deref for PmemMutCell<T> {
    type Target = T;
    fn deref(&self) -> &T { unsafe { &*self.ptr } }
}

impl<T> ::std::ops::DerefMut for PmemMutCell<T> {
    fn deref_mut(&mut self) -> &mut T { unsafe { &mut *self.ptr } }
}

impl<T> ::std::fmt::Debug for PmemMutCell<T> where T: ::std::fmt::Debug {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", (self as &T))
    }
}

impl<T> ::std::fmt::Display for PmemMutCell<T> where T: ::std::fmt::Display {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", (self as &T))
    }
}
