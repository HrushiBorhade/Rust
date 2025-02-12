#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let untill_delimeter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(untill_delimeter)
        } else if self.remainder.is_empty() {
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
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
