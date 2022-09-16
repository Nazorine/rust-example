use std::str::from_utf8;

fn main() {
    
    
    let c0 = "hello world";
    println!("{:?}",c0);

    let c1 = hex::encode("hello world"); // 将&str转换为String
    println!("{:?}",c1);

    let c2 = hex::decode("68656c6c6f20776f726c64").unwrap();  // 将bites转换为Vec<u8>
    println!("{:?}",c2);

    let c3 = from_utf8(&c2).unwrap(); // 将Vec<u8>转换为String
    println!("{:?}",c3);

    let c4 = "hello world".to_owned().into_bytes();
    println!("{:?}",c4);

    let c5 = String::from_utf8(c4).unwrap();
    println!("{:?}",c5);


    
    let c6 = "hello world".as_bytes();
    println!("{:?}",c6);

    let c7 = std::str::from_utf8(c6).unwrap();
    println!("{:?}",c7);



}
