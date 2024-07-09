pub struct Command {
    pub run: Box<dyn Fn()>,
    pub usage_line: &'static str,
    pub short: &'static str,
    pub flag: &'static str,
}
