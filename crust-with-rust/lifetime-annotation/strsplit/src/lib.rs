#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}
impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {
    type Item = &'haystack str;

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

fn until_char(s: &str, c: char) -> &str {
    let delim = format!("{}", c);
    StrSplit::new(s, &delim)
        .next()
        .expect("strsplit will always give atleast one result")
}

#[test]
fn test_untill_char() {
    assert_eq!(until_char("hello world", 'o'), "hell");
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
