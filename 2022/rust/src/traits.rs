pub(crate) trait PrintableResult {
    fn printable_result(&self, day: &str);
}

impl PrintableResult for String{
    fn printable_result(&self, day: &str) {
        println!("{} {}", day, self);
    }
}