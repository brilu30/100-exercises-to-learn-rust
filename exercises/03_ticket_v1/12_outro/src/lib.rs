// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub struct Order{
    product_name: String,
    quantity: i32,
    unit_price: i32
}
impl Order{
    pub fn new(product_name: String, quantity: i32, unit_price: i32) -> Order {
        Order { product_name, quantity, unit_price}
    }
    pub fn check_name(name: String){
        if name.is_empty() || name.as_bytes().len() > 300{
            panic!("No likey")
        }
    }
    pub fn check_quantity(quantity: i32){
        if quantity < 0{
            panic!("Can't have none dude")
        }
    }
    pub fn check_unit_price(unit_price: i32){
        if unit_price < 0{
            panic!("No free item :(")
        }
    }
    pub fn total(&self) -> i32 {
        self.unit_price * self.quantity
    }

    pub fn set_product_name(&mut self, name: String){
        self.product_name = name;
    }
    pub fn set_quantity(&mut self, quantity: i32){
        self.quantity = quantity;
    }
    pub fn set_unit_price(&mut self, unit_price: i32){
        self.unit_price = unit_price;
    }

    pub fn get_product_name(&self) -> String {
        self.product_name.clone()
    }
    pub fn get_quantity(&self) -> &i32 {
        &self.quantity
    }
    pub fn get_unit_price(&self) -> &i32 {
        &self.unit_price
    }
}
