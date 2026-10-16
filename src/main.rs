mod model;

use model::product::{Product, Byproduct, ByproductType};

fn main() {
    let byproducts = Byproduct::new(ByproductType::Ingredients,"Lomos",2);
    let mut product = Product::new("Merluza",2,10,);

    product.append_byproduct(byproducts);

    for item in &product.byproduct_list{
        print!("{}",item.name)
    }
}
