const RANDOM_VALUE :u32 = 10;

fn main() {
    println!("Random value : {RANDOM_VALUE}");
    let mut x = 5;
    println!("The value of x : {x}");
    x = 6;
    println!("The value of x : {x}");

    //shadowing
    let a  = 5;
    let a = a + 1;

    {
        let a  = a * 2;
        println!("The value of a in the inner scope: {a}");
    }

    println!("The value of a : {a}");
}
