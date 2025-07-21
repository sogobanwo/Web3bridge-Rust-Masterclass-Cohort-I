use std::io::{self, Write};
use asset_tracker::{Asset, MenuOption, Storage, add_asset, convert_to_hashmap, edit_asset, remove_asset, view_assets};

fn main() {
    let mut storage = Storage::Vec(Vec::new());
    loop {
        match show_menu() {
            Ok(MenuOption::Add) => {
                let (name, serial_number, value) = get_asset_input();
                match value {
                    Ok(v) => add_asset(&mut storage, name, serial_number, v),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Ok(MenuOption::View) => view_assets(&storage),
            Ok(MenuOption::Remove) => {
                storage = convert_to_hashmap(storage);
                let serial_number = get_serial_number_input();
                remove_asset(&mut storage, serial_number);
            }
            Ok(MenuOption::Edit) => {
                storage = convert_to_hashmap(storage);
                let serial_number = get_serial_number_input();
                let (new_name, new_value) = get_edit_input(&storage, &serial_number);
                if new_name.is_some() || new_value.is_some() {
                    print!("Save changes? (y/n): ");
                    io::stdout().flush().unwrap();
                    let mut confirm = String::new();
                    io::stdin().read_line(&mut confirm).unwrap();
                    if confirm.trim().to_lowercase() == "y" {
                        edit_asset(&mut storage, serial_number, new_name, new_value);
                    } else {
                        println!("Changes canceled.");
                    }
                }
            }
            Ok(MenuOption::Exit) => break,
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn show_menu() -> Result<MenuOption, String> {
    println!("\nAsset Tracker");
    println!("1. Add Asset");
    println!("2. View Assets");
    println!("3. Remove Asset");
    println!("4. Edit Asset");
    println!("5. Exit");
    print!("Enter choice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;
    match input.trim() {
        "1" => Ok(MenuOption::Add),
        "2" => Ok(MenuOption::View),
        "3" => Ok(MenuOption::Remove),
        "4" => Ok(MenuOption::Edit),
        "5" => Ok(MenuOption::Exit),
        _ => Err("Invalid choice".to_string()),
    }
}

fn get_asset_input() -> (String, String, Result<f64, String>) {
    print!("Enter asset name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    print!("Enter serial number: ");
    io::stdout().flush().unwrap();
    let mut serial_number = String::new();
    io::stdin().read_line(&mut serial_number).unwrap();
    let serial_number = serial_number.trim().to_string();

    print!("Enter value: ");
    io::stdout().flush().unwrap();
    let mut value_input = String::new();
    io::stdin().read_line(&mut value_input).unwrap();
    let value = value_input
        .trim()
        .parse::<f64>()
        .map_err(|_| "Invalid value!".to_string());

    (name, serial_number, value)
}

fn get_serial_number_input() -> String {
    print!("Enter serial number: ");
    io::stdout().flush().unwrap();
    let mut serial_number = String::new();
    io::stdin().read_line(&mut serial_number).unwrap();
    serial_number.trim().to_string()
}

fn get_edit_input(storage: &Storage, serial_number: &str) -> (Option<String>, Option<f64>) {
    let asset = if let Storage::HashMap(hm) = storage {
        hm.get(serial_number).cloned()
    } else {
        None
    };

    let asset = match asset {
        Some(a) => a,
        None => {
            println!("Error: Asset not found!");
            return (None, None);
        }
    };

    print!("Enter new name (or press Enter to keep '{}'): ", asset.name);
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let new_name = if name.trim().is_empty() {
        None
    } else {
        Some(name.trim().to_string())
    };

    print!(
        "Enter new value (or press Enter to keep {:.2}): ",
        asset.value
    );
    io::stdout().flush().unwrap();
    let mut value_input = String::new();
    io::stdin().read_line(&mut value_input).unwrap();
    let new_value = if value_input.trim().is_empty() {
        None
    } else {
        match value_input.trim().parse::<f64>() {
            Ok(v) => Some(v),
            Err(_) => {
                println!("Error: Invalid value!");
                return (new_name, None);
            }
        }
    };

    (new_name, new_value)
}