pub struct StrSplit<'a> {
  remainder: Option<&'a str>,
  delimeter: &'a str,
}

impl<'a> StrSplit<'a> {
  pub fn new (haystack: &'a str, delimeter: &'a str) -> Self {
    Self {
      delimeter,
      remainder: Some(haystack),
    }
  }
}

impl<'a> Iterator for StrSplit<'a> {
  type Item = &'a str;
  fn next (&mut self) -> Option<Self::Item> {
    if let Some(ref mut remainder) = self.remainder {
      if let Some(next_delimeter) = remainder.find(self.delimeter) {
        let until_delimeter = &remainder[..next_delimeter];
        *remainder = &remainder[(next_delimeter + self.delimeter.len())..];
        Some(until_delimeter)
      } else { self.remainder.take() }
    }
    else { None }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn it_creates_str_split () {
    let split = StrSplit::new("a,b,c", ",");

    assert_eq!(split.remainder, Some("a,b,c"));
    assert_eq!(split.delimeter, ",");
  }

  #[test]
  fn it_iterates () {
    let haystack = "a b c d";
    let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();

    assert_eq!(letters, vec!["a", "b", "c", "d"]);
  }

  #[test]
  fn tail () {
    let haystack = "a b c d ";
    let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();

    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
  }
}