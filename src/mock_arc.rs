use std::boxed::Box;
use std::fmt::{self, Debug, Formatter};
use std::ops::Deref;
use std::ptr::{self, NonNull};
use std::sync::atomic::{
    AtomicUsize,
    Ordering::{Relaxed, Release},
};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct MockArc<T> {
    ptr: NonNull<MockDataInner<T>>,
}

unsafe impl<T: Sync> Send for MockArc<T> {}
unsafe impl<T: Sync> Sync for MockArc<T> {}

struct MockDataInner<T> {
    rc: AtomicUsize,
    data: T,
}

impl<T> MockArc<T> {
    fn new(data: T) -> Self {
        let inner = box MockDataInner {
            rc: AtomicUsize::new(1),
            data: data,
        };
        MockArc {
            ptr: Box::leak(inner).into(),
        }
    }
    fn inner(&self) -> &MockDataInner<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Clone for MockArc<T> {
    fn clone(&self) -> MockArc<T> {
        self.inner().rc.fetch_add(1, Relaxed);
        MockArc { ptr: self.ptr }
    }
}

impl<T> Drop for MockArc<T> {
    fn drop(&mut self) {
        if self.inner().rc.fetch_sub(1, Release) != 1 {
            return;
        }
        println!("do gc");
        unsafe { ptr::drop_in_place(&mut self.ptr.as_mut().data) };
    }
}

impl<T> Deref for MockArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner().data
    }
}

impl<T: Debug> Debug for MockArc<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&**self, f)
    }
}

#[test]
fn test_main() {
    let foo = Arc::new(vec![0]);
    let bar = Arc::clone(&foo);

    let foo2 = MockArc::new(vec![0]);
    let bar2 = MockArc::clone(&foo2);
    let handler = thread::spawn(move || {
        thread::sleep(Duration::from_millis(20));
        println!("bar {:?}", *bar);
        println!("bar2 {:?}", *bar2);
    });

    println!("foo {:?}", foo);
    println!("foo2 {:?}", foo2);
    handler.join().ok();
}
