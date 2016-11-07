fn main() {
    let v = vec![1, 2, 3];
    let v2 = test(v);
    println!("{:?}", v2);

    println!("{:?}", v);
}

fn test(v: Vec<i32>) -> Vec<i32> {
    let mut v2 = v;
    v2.push(1);
    v2
}
