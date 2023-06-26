pub struct DB{
    pub url: &'static str
}

impl DB {
    pub fn url() -> Self{
        DB {url: "postgresql://dboperator:operatorpass123@localhost:5432/postgres"}
    }
}