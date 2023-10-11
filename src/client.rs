use core::fmt;

pub struct Client {
    pub name: String,
    pub email: String,
    pub age: u8,
    pub role: Role
}

pub enum Role {
    Admin(String),
    User(String)
}

impl Client {
    pub fn new(name: String, email: String, age: u8, role: Role) -> Self {
        Self { name, email, age, role }
    }

    pub fn print(&self) {
        println!("Nome: {}", self.name);
        println!("Email: {}", self.email);
        println!("Idade: {}", self.age);
        println!("Cargo: {}", self.role);
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Admin(name) => write!(f, "{}", name),
            Role::User(name) => write!(f, "{}", name),
        }
    }
}