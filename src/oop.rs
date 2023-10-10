struct Client {
    name: String,
    email: String,
    age: u8,
}

impl Client {
    fn new(name: String, email: String, age: u8) -> Self {
        Self {
            name,
            email,
            age,
        }
    }

    fn print(&self) {
        println!("Nome: {}", self.name);
        println!("Email: {}", self.email);
        println!("Idade: {}", self.age);
    }
}