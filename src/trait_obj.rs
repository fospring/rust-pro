
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
}