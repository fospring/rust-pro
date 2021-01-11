#[cfg(test)]
mod test {
    #[test]
    fn test_base_tpe() {
        let a = 1;
        let b = a.clone();
        let a = 2;
        println!("{}", b);
        
        
        let a = 1;
        let b = a;
        let a = 2;
        println!("{}", b);
        
        use std::rc::Rc;
        let mut a = Rc::new(1);
        // let mut b = Rc::clone(&a);
        // let a = 2;
        *Rc::get_mut(&mut a).unwrap() = 4;
        println!("{}", a);
    }
}
