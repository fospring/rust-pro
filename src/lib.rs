pub mod arc_mvp;
pub mod future_poll;
pub mod pin_data;
pub mod pointer_owner_ship;
pub mod trait_obj;

use std::time::{Duration, Instant};
use std::{thread, time};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_main() {
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
    }
}
