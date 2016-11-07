fn main() {
    let mut x = 1;
    let y = &mut x;
    let z = &x;
    println!("{:?}", y);
}

fn test(x: &mut i32) -> i32 {
    *x + 1
}

fn test2(x: &i32) -> () {

}
