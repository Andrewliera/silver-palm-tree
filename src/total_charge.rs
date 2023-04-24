use regex::{RegexSet};

#[derive(Debug, PartialEq)]
pub enum ItemType{
    Wic, 
    Clothing, 
    EverythingElse,
}

#[derive(Debug, PartialEq)]
pub struct Item{
    pub name: Option<String>,
    pub price: f32,
    pub item_type: ItemType, 
}

pub fn calculate_total_charge(cart:Vec<Item>, state: String){
    let mut total_charge: f32 = 0.0; 
    for item in cart{
        total_charge += calculate_item_charge(item, state.clone());
    }
    println!("total charge: {}", total_charge)
}
fn calculate_item_charge(cart_item: Item, state: String) -> f32 {
    
    let set = RegexSet::new(&[
    r"Fur",
    r"fur",
    ]).unwrap();
 
    println!("{:?}", cart_item);
    println!("{}", state);
    
    let current_item_price = cart_item.price;
    let current_item_type = cart_item.item_type;
    let current_item_tax = calculate_tax(current_item_price, current_item_type, state);
    let current_item_total = current_item_price + current_item_tax;

    let current_item_name = cart_item.name.expect("Bad Input").to_string();
    if set.is_match(&current_item_name){
        println!("Regex match found: {}", current_item_name);
    }

    return current_item_total;
}


fn calculate_tax(item_price: f32, item_type: ItemType,state: String) -> f32{
 
    match state.as_str() {
        "NJ" => {
            let _state_tax_exemption = calculate_tax_exemption(item_type, state);
            println!("_state_tax_exemption: {}", _state_tax_exemption);
            let _state_tax = 0.066;
            let _item_tax = item_price * (_state_tax - _state_tax_exemption);
            println!("_item_tax: {}", _item_tax);
            return _item_tax;
        }

        "PA" => {
            let _state_tax_exemption = calculate_tax_exemption(item_type, state);
            println!("_state_tax_exemption: {}", _state_tax_exemption);
            let _state_tax = 0.060;
            let _item_tax = item_price * (_state_tax - _state_tax_exemption);
            println!("_item_tax: {}", _item_tax);
            return _item_tax;
        }

        "DE" => {
            let _state_tax = 0.0;
            let _item_tax = item_price * _state_tax; 
            println!("_item_tax: {}", _item_tax);
            return _item_tax;
        }
        other => {
            println!("{} is not supposed to be an option", other);
            panic!("Panicking...");
        }
    }
}

fn calculate_tax_exemption(item_type: ItemType, state: String) -> f32 {
    
    match state.as_str(){

        "NJ" => {
            let _state_tax = 0.066;
            if item_type == ItemType::Wic{
                println!("_state_tax: {}", _state_tax);
                return _state_tax
            }
            println!("_state_tax: {}", _state_tax);
            return _state_tax * 0.0;
        }
        "PA" => {
            let _state_tax = 0.060;
            if item_type == ItemType::Wic || item_type == ItemType::Clothing{
                println!("_state_tax: {}", _state_tax);
                return _state_tax
            }
            println!("_state_tax: {}", _state_tax);
            return _state_tax * 0.0;
        }
        other => {        
            println!("{} is not supposed to be an option", other);
            panic!("Panicking...");
        }
    }

}
