pub fn demonstrate_vector_operations() {
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);

    let third: Option<&i32> = v1.get(2);
    match third {
        Some(third) => println!("the third element in v1: {}", third),
        None => println!("third element does not exist"),
    }
    println!("vector v1: {:#?}", v1);

    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    println!("vector v before mutation: {:#?}", v);
    
    for i in &mut v {
        *i += 50;
    }
    println!("vector v after mutation: {:#?}", v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row: {:#?}", row);
}
