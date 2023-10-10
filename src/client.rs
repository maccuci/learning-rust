pub struct Client {
    pub name: String,
    pub email: String,
    pub age: u8,
}

impl Client {
    pub fn new(name: String, email: String, age: u8) -> Self {
        Self { name, email, age }
    }

    pub fn print(&self) {
        println!("Nome: {}", self.name);
        println!("Email: {}", self.email);
        println!("Idade: {}", self.age);
    }
}
