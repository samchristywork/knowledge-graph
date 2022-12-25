pub struct Relation<'a> {
    name: &'a str,
}

impl<'a> Relation<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}
