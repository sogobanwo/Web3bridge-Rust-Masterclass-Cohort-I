use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Asset {
    pub name: String,
    pub serial_number: String,
    pub value: f64,
}

pub enum MenuOption {
    Add,
    View,
    Remove,
    Edit,
    Exit,
}

pub enum Storage {
    Vec(Vec<Asset>),
    HashMap(HashMap<String, Asset>),
}

pub fn convert_to_hashmap(storage: Storage) -> Storage {
    match storage {
        Storage::Vec(vec) => {
            let mut hashmap = HashMap::new();
            for asset in vec {
                hashmap.insert(asset.serial_number.clone(), asset);
            }
            Storage::HashMap(hashmap)
        }
        Storage::HashMap(_) => storage,
    }
}

pub fn add_asset(storage: &mut Storage, name: String, serial_number: String, value: f64) {
    match storage {
        Storage::Vec(vec) => {
            if vec.iter().any(|a| a.serial_number == serial_number) {
                println!("Error: Serial number already exists!");
                return;
            }
            let asset = Asset {
                name,
                serial_number: serial_number.clone(),
                value,
            };
            vec.push(asset);
            println!("Asset added successfully!");
        }
        Storage::HashMap(hm) => {
            if hm.contains_key(&serial_number) {
                println!("Error: Serial number already exists!");
                return;
            }
            let asset = Asset {
                name,
                serial_number: serial_number.clone(),
                value,
            };
            hm.insert(serial_number, asset);
            println!("Asset added successfully!");
        }
    }
}

pub fn view_assets(storage: &Storage) {
    match storage {
        Storage::Vec(vec) => {
            if vec.is_empty() {
                println!("No assets found.");
                return;
            }
            for asset in vec {
                println!(
                    "Name: {}, Serial: {}, Value: ${:.2}",
                    asset.name, asset.serial_number, asset.value
                );
            }
        }
        Storage::HashMap(hm) => {