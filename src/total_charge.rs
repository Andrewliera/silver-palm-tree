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

pub fn calculate_total_charge(cart:Vec<Item>, state: String) -> f32 {
    let mut total_charge: f32 = 0.0; 
    for item in cart{
        total_charge += calculate_item_charge(item, state.clone());
    }
    println!("total charge: {}", total_charge);
    return total_charge;
}
fn calculate_item_charge(cart_item: Item, state: String) -> f32 {
    
 
    println!("{:?}", cart_item);
    println!("{}", state);

    let current_item_price = cart_item.price;
    let current_item_type = cart_item.item_type;
    let current_item_name = cart_item.name.expect("Bad Input").to_string();
        
    let fur_item = check_name(current_item_name);
    check_price(current_item_price);


    let current_item_tax = calculate_tax(current_item_price, current_item_type, fur_item, state);
    let current_item_total = current_item_price + current_item_tax;

    return current_item_total;
}

fn check_name(item_name: String) -> bool{
    
    if item_name.is_empty(){
        println!("Empty Item Name");
        panic!("Panicking...");
    }

    let mut fur_item: bool = false;
    let set = RegexSet::new(&[
    r"Fur",
    r"fur",
    ]).unwrap();
    if set.is_match(&item_name){
        println!("Regex match found: {}", item_name);
        fur_item = true;
    }
    return fur_item;
}

fn check_price(item_price: f32){
    if item_price < 0.0 {
        println!("Item Price is Not Supposed be Negative");
        panic!("Panicking...");
    }
}


fn calculate_tax(item_price: f32, item_type: ItemType, fur_item: bool, state: String) -> f32{
 
    match state.as_str() {
        "NJ" => {
            let _state_tax_exemption = calculate_tax_exemption(item_type, fur_item, state);
            println!("_state_tax_exemption: {}", _state_tax_exemption);
            let _state_tax = 0.066;
            let _item_tax = item_price * (_state_tax - _state_tax_exemption);
            println!("_item_tax: {}", _item_tax);
            return _item_tax;
        }

        "PA" => {
            let _state_tax_exemption = calculate_tax_exemption(item_type, fur_item, state);
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

fn calculate_tax_exemption(item_type: ItemType, fur_item: bool, state: String) -> f32 {
    
    match state.as_str(){

        "NJ" => {
            let _state_tax = 0.066;
            if item_type == ItemType::Wic{
                println!("_state_tax: {}", _state_tax);
                return _state_tax
            }else if item_type == ItemType::Clothing && fur_item == true{
                return _state_tax * 0.0;

            }else if item_type == ItemType::Clothing && fur_item == false{
                return _state_tax;
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let cart_items = vec![
        Item { 
            name: Some("test_name_0".into()),
            price: 9.99,
            item_type: ItemType::EverythingElse,
        },
        Item { 
            name: Some("testfur_name_1".into()),
            price: 1.99,
            item_type: ItemType::Clothing,
        },
        Item { 
            name: Some("test_name_2".into()),
            price: 1.89,
            item_type: ItemType::Wic,
        },
        Item { 
            name: Some("test_name_3".into()),
            price: 2.00,
            item_type: ItemType::EverythingElse,
        },
        Item { 
            name: Some("test_name_4".into()),
            price: 15.99,
            item_type: ItemType::Clothing,
        },
        Item { 
            name: Some("test_name_5_fFurr".into()),
            price: 15.99,
            item_type: ItemType::Clothing,
        },  ];

        let state = "PA";
        let test = calculate_total_charge(cart_items, state.to_string());
        assert_eq!(test, 48.569397);
    }

    #[test]
    fn happy_path_2() {
        let cart_items = vec![
        Item { 
            name: Some("test_name_0".into()),
            price: 12.99,
            item_type: ItemType::EverythingElse,
        },
        Item { 
            name: Some("test_name_1".into()),
            price: 15.49,
            item_type: ItemType::EverythingElse,
        },
        Item { 
            name: Some("test_name_2".into()),
            price: 5.99,
            item_type: ItemType::Wic,
        },
        Item { 
            name: Some("test_name_3".into()),
            price: 19.99,
            item_type: ItemType::EverythingElse,
        },
        Item { 
            name: Some("test_name_4".into()),
            price: 13.69,
            item_type: ItemType::Wic,
        },
        Item { 
            name: Some("test_name_5".into()),
            price: 91.99,
            item_type: ItemType::Clothing,
        },  ];

        let state = "DE";
        let test = calculate_total_charge(cart_items, state.to_string());
        assert_eq!(test, 160.14);
}

    #[test]
    fn happy_path_3(){
        let cart_items = vec![
        Item { 
            name: Some("test_name_0".into()),
            price: 12.99,
            item_type: ItemType::Clothing,
        },
        Item { 
            name: Some("test_name_1".into()),
            price: 11.99,
            item_type: ItemType::EverythingElse,
        }, 
        Item { 
            name: Some("test_name_2".into()),
            price: 12.99,
            item_type: ItemType::Clothing,
        },
        Item { 
            name: Some("test_name_3".into()),
            price: 0.99,
            item_type: ItemType::Wic,
        },
        Item { 
            name: Some("test_name_4".into()),
            price: 99.99,
            item_type: ItemType::Wic,
        }, ];
        let state = "NJ";
        let test = calculate_total_charge(cart_items, state.to_string());
        assert_eq!(test, 139.74134)
    }
    
    #[test]
    #[should_panic]
    fn bad_data(){
    
        let cart_items = vec![
        Item { 
            name: Some("".into()),
            price: 0.00,
            item_type: ItemType::EverythingElse,
        },
        Item { 
            name: Some("test_name_1".into()),
            price: 11.99,
            item_type: ItemType::EverythingElse,
        }, 
        Item { 
            name: Some("test_name_2".into()),
            price: -12.99,
            item_type: ItemType::Clothing,
        },
        Item { 
            name: Some("".into()),
            price: 0.99,
            item_type: ItemType::Wic,
        },
        Item { 
            name: Some("test_name_4".into()),
            price: -99.99,
            item_type: ItemType::Wic,
        }, ];
        let state = "NJ";
        let test = calculate_total_charge(cart_items, state.to_string());
        assert_eq!(test, 123.12);
        }

    #[test]
    #[should_panic]
    fn bad_data_2(){
        let cart_items = vec![
        Item { 
            name: Some("test_name_0".into()),
            price: 0.00,
            item_type: ItemType::EverythingElse,
        },
        Item { 
            name: Some("test_name_1".into()),
            price: 11.99,
            item_type: ItemType::EverythingElse,
        }, 
        Item { 
            name: Some("test_name_2".into()),
            price: -12.99,
            item_type: ItemType::Clothing,
        },
        Item { 
            name: Some("test_name_3".into()),
            price: 0.99,
            item_type: ItemType::Wic,
        },
        Item { 
            name: Some("test_name_4".into()),
            price: -99.99,
            item_type: ItemType::Wic,
        }, ];
        let state = "NJ";
        let test = calculate_total_charge(cart_items, state.to_string());
        assert_eq!(test, 123.12);

    }

    #[test]
    fn edge_case(){
            let cart_items = vec![];
            let state = "DE";
            let test = calculate_total_charge(cart_items, state.to_string());
            assert_eq!(test, 0.00);
        }
    
    #[test]
    fn edge_cases_2(){
    let cart_items = vec![
    Item { 
        name: Some("test_Fur_name_0".into()),
        price: 12.99,
        item_type: ItemType::Clothing,
        },
    Item { 
        name: Some("test_name_1".into()),
        price: 11.99,
        item_type: ItemType::Clothing,
        }, 
    Item { 
        name: Some("teFuRst_name_2".into()),
        price: 12.99,
        item_type: ItemType::Clothing,
        },
    Item { 
        name: Some("test_namFure_3".into()),
        price: 0.99,
        item_type: ItemType::Wic,
        },
    Item { 
        name: Some("test_namFure_4".into()),
        price: 99.99,
        item_type: ItemType::Clothing,
        }, ];
        let state = "NJ";
     let test = calculate_total_charge(cart_items, state.to_string());
        assert_eq!(test, 146.40668)
        }
}
