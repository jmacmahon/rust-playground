use std::io;

fn main() {
    let mut v: Vec<i32> = vec![21, 34, 55, 89];

    loop {
        println!("{:?}", v);
        // let index = match read_and_parse() {
        //     Ok(num) => num,
        //     Err(_) => break,
        // };
        let index: i32 = read_and_parse().expect("uh oh");

        v.push(index);
    }
}

fn read_and_parse<T>() -> Result<T, T::Err> where T: std::str::FromStr {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");

    buf.trim().parse()
}
