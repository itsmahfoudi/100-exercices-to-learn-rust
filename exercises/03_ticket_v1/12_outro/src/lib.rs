// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 characters.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
pub struct Order {
    product_name : String,
    quantity : u32,
    unit_price : u32
}

impl Order {
    pub fn new(product_name:String, quantity:u32, unit_price:u32) -> Order {
        Order::validate_emptyness(&product_name);
        Order::validate_length(&quantity);
        Order::validate_length(&unit_price);
        Order {
            product_name,
            quantity,
            unit_price
        }
    }
    pub fn validate_emptyness(value:&String) ->bool {
        if value.is_empty() {
            panic!("product_name cannot be empty!!");
        }
        if value.len() > 300 {
            panic!("product_name cannot be longer than 300 characters!!");
        }
        true
    }
    pub fn validate_length(value:&u32) -> bool{
        let val = 0;
        if value.eq(&val) {
            panic!("value should be greater than zero!!");
        } 
        true
    }
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }
    pub fn set_product_name(&mut self, new_product_name:String) {
        if Order::validate_emptyness(&new_product_name) {
            self.product_name = new_product_name;
        }
    }
    pub fn set_quantity(&mut self, new_quantity:u32) {
        if Order::validate_length(&new_quantity) {
            self.quantity = new_quantity;
        }
    }
    pub fn set_unit_price(&mut self, new_unit_price:u32) {
        if Order::validate_length(&new_unit_price) {
            self.unit_price = new_unit_price;
        }
    }
    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
}


// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
