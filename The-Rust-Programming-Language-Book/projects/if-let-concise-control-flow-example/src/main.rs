fn main() {
    let some_option = Some(3u8);
//    match some_option {
//        Some(num) => println!("the num in some: {}",num),
//        _ => (),
//    };

    if let Some(num) = some_option {
        println!("the num in some: {}", num);
    }else{
        println!("the num in some is none");
    }
}
