use custom_iterators::strsplit::StrSplit;

fn print_items (haystack: &str, needle: &str) {
    for s in StrSplit::new(haystack, needle) {
        println!("{}", s);
    }
}

fn main() {
    print_items("a,b,c,d", ",");
    print_items("aa,bb,cc,dd,ee", "cc,dd");
    print_items("a b c", " ");
    print_items("a b c ", " ");
}
