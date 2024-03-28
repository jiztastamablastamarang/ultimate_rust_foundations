use std::alloc::{alloc, dealloc, Layout};

struct SmartPointer<T> {
    ptr: *mut u8,
    data: *mut T,
    layout: Layout,
}

impl<T> SmartPointer<T> {
    fn new() -> SmartPointer<T> {
        println!("Allocating memory");

        unsafe {
            let layout = Layout::new::<T>();
            let ptr = alloc(layout);

            return SmartPointer {
                ptr,
                data: ptr as *mut T,
                layout,
            };
        }
    }

    fn set(&self, val: T) {
        unsafe { *self.data = val };
    }

    fn get(&self) -> &T {
        return unsafe { &*self.data };
    }
}

impl<T> Drop for SmartPointer<T> {
    fn drop(&mut self) {
        println!("Freeing memory");
        unsafe { dealloc(self.ptr, self.layout) };
    }
}

fn main() {
    /// The hard way.
    let sp = SmartPointer::<i32>::new();
    sp.set(1);
    println!("{}", sp.get());

   /// The easy way.
   let sp = Box::new(2);
   println!("{}", *sp);

   println!("end of main");
}
