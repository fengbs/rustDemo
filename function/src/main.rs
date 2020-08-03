fn main() {
    println!("Hello, world!");
    let res = function_test_first(12,3);
    println!("xxx {}",res);
}

fn function_test_first(x:i32,y:i32)->i32 {
    let k = x + y;
    println!("aaa{}",k);
    k
}
