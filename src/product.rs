#[derive(Clone)]
pub struct Product {
    pub id: usize,
    pub name: String,
    pub cost: usize,
}

#[derive(Clone)]
pub struct Products {
    pub products: Vec<Product>
}

impl Products {
    pub fn new() -> Self {
        Products {
            products: Vec::new()
        }
    }
    pub fn add_product(&mut self, m: Product) {
        self.products.push(m);
    }
}
