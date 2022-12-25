pub struct Node<'a> {
    name: &'a str,
}

impl<'a> Node<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}
