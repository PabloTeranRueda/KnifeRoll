use std::collections::HashMap;

pub struct ProductList{
    pub products: HashMap<u64, Product>,
}

pub struct Product{
    pub name:String,
    pub weight:i128,
    pub unit_price:i128,
    pub byproduct_list:Vec<Byproduct>
}

impl Product{

    pub fn new(name:&str, weight:i128,unit_price:i128) -> Self {

        Self{
            name: name.to_string(),
            weight: weight,
            unit_price:unit_price,
            byproduct_list:Vec::<Byproduct>::new(),
        }
    }

    pub fn append_byproduct_list(&mut self,mut byproduct_list:Vec<Byproduct>){
        for item in &mut byproduct_list{
            item.price = self.get_total_price() / item.weight
        }

        self.byproduct_list.extend(byproduct_list);
    }


    pub fn append_byproduct(&mut self,mut byproduct:Byproduct){
        byproduct.price = self.get_total_price() / byproduct.weight;

        self.byproduct_list.push(byproduct);
    }

    pub fn get_total_price(&self) -> i128 {
        return self.unit_price * self.weight
    }

    pub fn get_yield_loss_kg(&self) -> i128 {

        return self.byproduct_list.iter()
            .filter(|byproduct| {
                matches!(
                    byproduct.byproduct_type,
                    ByproductType::UsefulLoss | ByproductType::UselessLoss
                )
            })
            .map(|byproduct| byproduct.weight)
            .sum();
        }

    pub fn get_yield_loss_percentage(&self) -> f64{
        return self.get_yield_loss_kg() as f64 / self.weight as f64 * 100.0;
    }
}

pub enum ByproductType{
    Ingredients,
    UsefulLoss,
    UselessLoss,
}

pub struct Byproduct{
    pub byproduct_type:ByproductType,
    pub name:String,
    pub weight:i128,
    pub price:i128,
}

impl Byproduct{
   pub fn  new(byproduct_type:ByproductType, name:&str, weight:i128) -> Self {

    Self{
        byproduct_type: byproduct_type,
        name: name.to_string(),
        weight: weight,
        price:0,
    }
   }

    
}