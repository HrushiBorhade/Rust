#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let untill_delimeter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(untill_delimeter)
        } else {
            self.remainder.take()
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let delimiter = " ";
    let res: Vec<&str> = StrSplit::new(haystack, delimiter).collect();
    println!("collected res: {:#?}", res);
    assert_eq!(res, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let delimiter = " ";
    let res: Vec<&str> = StrSplit::new(haystack, delimiter).collect();
    println!("collected res: {:#?}", res);
    assert_eq!(res, vec!["a", "b", "c", "d", ""]);
}
