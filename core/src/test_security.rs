#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub fn test() {
    panic!("error");

    unsafe {
        let x = 5;
        println!("{}", x);
    }

    let v: Option<i32> = None;
    v.unwrap();
}
