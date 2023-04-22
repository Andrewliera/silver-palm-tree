mod total_charge;

fn main() {
   let cart_items = vec![
    total_charge::Item { 
        name: Some("test_name_0".into()),
        price: 12.99,
        item_type: total_charge::ItemType::EverythingElse,
        },
    total_charge::Item { 
        name: Some("test_name_1".into()),
        price: 1.99,
        item_type: total_charge::ItemType::Clothing,
        },
    total_charge::Item { 
        name: Some("test_name_2".into()),
        price: 1.89,
        item_type: total_charge::ItemType::Wic,
        },
    total_charge::Item { 
        name: Some("test_name_3".into()),
        price: 2.00,
        item_type: total_charge::ItemType::EverythingElse,
        },
    total_charge::Item { 
        name: Some("test_name_4".into()),
        price: 15.99,
        item_type: total_charge::ItemType::Clothing,
        },
    total_charge::Item { 
        name: Some("test_name_5_fFurr".into()),
        price: 15.99,
        item_type: total_charge::ItemType::Clothing,
        },  ];

    let state = "PA";
    total_charge::calculate_total_charge(cart_items, state.to_string());
   
    let cart_items_2 = vec![
    total_charge::Item { 
        name: Some("test_name_0".into()),
        price: 12.99,
        item_type: total_charge::ItemType::EverythingElse,
        },
    total_charge::Item { 
        name: Some("test_name_1".into()),
        price: 1.99,
        item_type: total_charge::ItemType::Clothing,
        },  ];
        let state_2 = "NJ";
    total_charge::calculate_total_charge(cart_items_2, state_2.to_string());

}
