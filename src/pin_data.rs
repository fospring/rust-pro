
// https://rustcc.cn/article?id=1d0a46fa-da56-40ae-bb4e-fe1b85f68751
#[derive(Debug)]
struct Test {
  a: String,
  b: *const String,
}

impl Test {
  fn new(txt: &str) -> Self {
    //栈对象
    Test {
        a: String::from(txt),
        b: std::ptr::null(),
    }
  }

  fn init(&mut self) {
    let self_ref: *const String = &self.a;
    self.b = self_ref;
  }

  fn a(&self) -> &str {
    &self.a
  }

  fn b(&self) -> &String {
    unsafe {&*(self.b)}
  }
}

#[cfg(test)]
mod test {
  use super::*;
  // cargo test --lib  pin_data::test::test_no_pin
  #[test]
  fn test_no_pin() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();
  
    println!("test1 a: {}, b: {}", test1.a(), test1.b());
    std::mem::swap(&mut test1, &mut test2); //按bit 直接复制交换。
    test1.a = "I've totally changed now!".to_string();
    println!("test2 a: {}, b: {}", test2.a(), test2.b()); //证明指针test2.b仍然指向test1.a位置，但是他应该指向test2.a才对的！！！
    println!("test1 a: {}, b: {}", test1.a(), test1.b()); //证明指针test2.b仍然指向test1.a位置，但是他应该指向test2.a才对的！！！
  }
}