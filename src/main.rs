use std::io;
mod total_charge;

fn add_item() -> total_charge::Item{
    
    let mut user_in = String::new();
    println!("Item Name: ");
    io::stdin()
        .read_line(&mut user_in)
        .expect("Failed to read user input");
    let item_name = user_in.clone();
    user_in.clear();
    
    println!("###.##");
    println!("Item Price: ");
    io::stdin()
        .read_line(&mut user_in)
        .expect("Failed to read user input");
    let item_price = user_in.trim().parse().unwrap();
    user_in.clear();
    
    println!("Wic/EverythingElse/Clothing");
    println!("Item Type: ");
    io::stdin()
        .read_line(&mut user_in)
        .expect("Failed to read user input");
    
    let item_type; 
    match user_in.trim(){
        "Wic" => {
            item_type = total_charge::ItemType::Wic;

        }
    
        "EverythingElse" => {
            item_type = total_charge::ItemType::EverythingElse;
        }

        "Clothing" => {
            item_type = total_charge::ItemType::Clothing;
        }
    
        other => {
            println!("{} is not an option", other);
            panic!("Panicking...");
        }
    }
    user_in.clear();

    let item = total_charge::Item {
        name: Some(item_name),
        price: item_price,
        item_type: item_type,
    }; 
    return item;
}



fn start_prog() {
    let mut user_in = String::new(); 
    let mut cart = vec![];
    let get_charge;
    let state;
    
    loop{
    println!("Add Item To Cart?");
    println!("Yes/No");
    io::stdin()
        .read_line(&mut user_in)
        .expect("Failed to read user input");
       
    match user_in.trim(){
        "Yes" => {
            user_in.clear();
            let new_cart_item = add_item();
            cart.push(new_cart_item);
        }

        "No" => {
            user_in.clear();
            break;
            }
        other => {
            println!("{} is not an option", other);
            user_in.clear();
            continue;
        }
    }
    }

    println!("Add State");
    println!("DE/NJ/PA");
    io::stdin()
        .read_line(&mut user_in)
        .expect("Failed to read user input");
    match user_in.trim(){
        "DE" => {
            state = user_in; 
        }

        "NJ" => {
            state = user_in;
        }

        "PA" => {
            state = user_in;
        }
        other => {
        println!("{} is not an option", other);
        panic!("Panicking...");
        }
    } 
    get_charge = total_charge::calculate_total_charge(cart, state);
    println!("the total cost of the items is: {} ", get_charge);
}


fn main() {
    println!("Capstone Hw");
    start_prog();
}
