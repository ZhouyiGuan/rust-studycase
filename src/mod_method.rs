#[derive(Debug)]
pub struct Rectangle {
    name: String,
    width: u32,
    height: u32
}

impl Rectangle {
    pub fn new(name: String, width: u32, height: u32) -> Self {
        Rectangle { name: name, width: width, height: height }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}