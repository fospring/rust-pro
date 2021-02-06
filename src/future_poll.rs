
use futures::future::{self, Future, FutureExt};
use futures::task::{Context, Poll};
use std::pin::Pin;
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

