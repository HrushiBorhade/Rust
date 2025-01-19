fn main() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                println!("Break inner loop");
                break;
            }
            if count == 2 {
                println!("Break outer loop");
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End of counting:  {}", count);

    while count!=0 {
        println!("count = {}", count);
        count -= 1;
    }
    println!("End of counting:  {}", count);


    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
