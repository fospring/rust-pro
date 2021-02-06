#![feature(box_syntax)]
#![feature(allocator_api)]
pub mod pin_data;
pub mod pointer_owner_ship;
pub mod mock_arc;

#[feature(dyn_trait)]
use std::mem;
trait Bird {
    fn fly(&self);
}

struct Duck;
struct Swan;

impl Bird for Duck {
    fn fly(&self) {
        println!("duck duck");
    }
}

impl Bird for Swan {
    fn fly(&self) {
        println!("swan swan");
    }
}

fn print_traitobj(p: &dyn Bird) {
    let (data, vtable): (usize, *const usize) = unsafe { mem::transmute(p) };
    println!("TrainObj [data:{}, vtable:{:p}]", data, vtable);

    unsafe {
        //
        println!(
            "data in vtable [{}, {}, {}, {}]",
            *vtable,
            *vtable.offset(1),
            *vtable.offset(2),
            *vtable.offset(3)
        );
    }
}

use futures::future::{self, Future, FutureExt};
use futures::task::{Context, Poll};
use std::pin::Pin;
struct Inc<'a>(&'a mut u32);

impl Future for Inc<'_> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> {
        unimplemented!()
    }
}

impl Drop for Inc<'_> {
    fn drop(&mut self) {
        *self.0 += 1;
    }
}
struct VCContext<F> {
    future: F,
}

impl<F> Future for VCContext<F>
where
    F: std::marker::Unpin + Future<Output = ()>,
{
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("111");
        let res = F::poll(Pin::new(&mut self.future), ctx);
        println!("res={:?}", res);
        println!("222");
        res
    }
}

impl<F> VCContext<F>
where
    F: std::marker::Unpin + Future<Output = ()>,
{
    fn new_warp_future(future: F) -> impl Future<Output = ()> {
        VCContext { future }
    }
}


#[allow(missing_debug_implementations)]
#[doc(hidden)]
pub struct PendingOnce {
    is_ready: bool,
}

impl Future for PendingOnce {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if self.is_ready {
            println!("read");
            Poll::Ready(())
        } else {
            println!("pending");
            self.is_ready = true;
            Poll::Pending
        }
    }
}

use std::time::{Duration, Instant};
use std::{thread, time};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_main() {
        let duck = Duck;
        let p_duck = &duck;
        let p_bird = p_duck as &dyn Bird;

        println!(
            "Size of p_duck {}, Size of p_bird {}",
            mem::size_of_val(&p_duck),
            mem::size_of_val(&p_bird)
        );

        let duck_fly: usize = Duck::fly as usize;
        let swan_fly: usize = Swan::fly as usize;

        println!("Duck::fly {}", duck_fly);
        println!("Swan::fly {}", swan_fly);

        print_traitobj(p_bird);
        let swan = Swan;
        print_traitobj(&swan as &dyn Bird);

        fn foo() -> i32 {
            0
        }

        let pointer = foo as *const ();
        let function = unsafe { std::mem::transmute::<*const (), fn() -> i32>(pointer) };
        assert_eq!(function(), 0);

        let raw_bytes = [0x78, 0x56, 0x34, 0x12];

        let num = unsafe { std::mem::transmute::<[u8; 4], u32>(raw_bytes) };
        println!("num:{}", num);

        // use `u32::from_ne_bytes` instead
        let num = u32::from_ne_bytes(raw_bytes);
        // or use `u32::from_le_bytes` or `u32::from_be_bytes` to specify the endianness
        let num = u32::from_le_bytes(raw_bytes);
        assert_eq!(num, 0x12345678);
        let num = u32::from_be_bytes(raw_bytes);
        assert_eq!(num, 0x78563412);
        println!("end1\n");

        use tokio::runtime::Runtime;

        // Create the runtime
        let rt = Runtime::new().unwrap();

        // Spawn a future onto the runtime
        let f = async {
            println!("now running on a worker thread");

        };
        let rc_ctx = VCContext::new_warp_future(f.boxed());
        rt.spawn(Box::pin(rc_ctx));

        let f = async {
            println!("now running on a worker thread again");

        };
        let rc_ctx = VCContext::new_warp_future(f.boxed());
        rt.spawn(Box::pin(rc_ctx));

        let f = PendingOnce{is_ready:false}.boxed();
        let rc_ctx = VCContext::new_warp_future(f.boxed());
        rt.spawn(Box::pin(rc_ctx));
        println!("end2\n");

        let f = PendingOnce{is_ready:false}.boxed();
        let rc_ctx = VCContext::new_warp_future(f.boxed());
        rt.spawn(Box::pin(rc_ctx));
        println!("end3\n");

        let f = future::pending::<()>().boxed();
        let rc_ctx = VCContext::new_warp_future(f.boxed());
        rt.spawn(Box::pin(rc_ctx));
        println!("end4\n");

        let f = async {
            // Wait for a `tokio` 0.2 `Delay`...
            println!("100 ms sleep start");
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("100 ms have elapsed");

            println!("100 ms sleep start");
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("100 ms have elapsed");

            println!("100 ms sleep start");
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("100 ms have elapsed");
        };
        let rc_ctx = VCContext::new_warp_future(f.boxed());
        rt.spawn(Box::pin(rc_ctx));
        println!("end5\n");

        let one_secs = time::Duration::from_millis(1000);
        let now = time::Instant::now();
        thread::sleep(one_secs);

        println!("end last\n");
    }
}

