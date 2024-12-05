use rusty_store::{
    models::{Product, UserRole},
    auth,
    store::Store
};
use std::io::{self, Write};
use uuid::Uuid;

fn main() {
    let mut store = Store::new();
    let mut auth = auth::Auth::new();  // This will handle admin creation internally

    // Initialize store from file
    if let Err(e) = store.load_from_file("store.json") {
        println!("Creating new store: {}", e);
    }

    loop {
        println!("\nRusty Store Management System");
        println!("1. Login");
        println!("2. Exit");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => handle_login(&mut auth),
            "2" => break,
            _ => println!("Invalid choice"),
        }
    }

    // Save store state before exit
    if let Err(e) = store.save_to_file("store.json") {
        println!("Error saving store: {}", e);
    }
}

fn handle_login(auth: &mut auth::Auth) {
    print!("Username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();

    print!("Password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();

    match auth.login(username.trim(), password.trim()) {
        Ok(_) => {
            println!("Login successful!");
            let mut store = Store::new();
            if let Err(e) = store.load_from_file("store.json") {
                println!("Error loading store: {}", e);
                return;
            }
            handle_main_menu(auth, &mut store);
            // Save store state after operations
            if let Err(e) = store.save_to_file("store.json") {
                println!("Error saving store: {}", e);
            }
        },
        Err(_) => println!("Login failed! Invalid username or password"),
    }
}

fn handle_main_menu(auth: &auth::Auth, store: &mut Store) {
    loop {
        println!("\nMain Menu");
        println!("1. View Inventory");
        println!("2. Add Product");
        println!("3. Record Sale");
        println!("4. Record Purchase");
        println!("5. View Reports");
        println!("6. Logout");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => show_inventory(store),
            "2" => {
                if auth.is_manager() {
                    add_product(store)
                } else {
                    println!("Permission denied: Manager access required");
                }
            },
            "3" => record_sale(store),
            "4" => record_purchase(store),
            "5" => show_reports(store),
            "6" => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn show_inventory(store: &Store) {
    println!("\n{}", store.generate_inventory_report());
}

fn add_product(store: &mut Store) {
    println!("Enter product name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter description: ");
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();

    println!("Enter price: ");
    let mut price = String::new();
    io::stdin().read_line(&mut price).unwrap();
    let price: f64 = price.trim().parse().unwrap_or(0.0);

    println!("Enter quantity: ");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).unwrap();
    let quantity: i32 = quantity.trim().parse().unwrap_or(0);


    let product = Product {
        id: Uuid::new_v4(),
        name: name.trim().to_string(),
        description: description.trim().to_string(),
        price,
        quantity,
    };

    match store.add_product(product) {
        Ok(_) => println!("Product added successfully"),
        Err(e) => println!("Error adding product: {}", e),
    }
}

fn record_sale(store: &mut Store) {
    println!("\nAvailable Products:");
    println!("{}", store.generate_inventory_report());

    println!("Enter product ID: ");
    let mut id_str = String::new();
    io::stdin().read_line(&mut id_str).unwrap();
    let product_id = match Uuid::parse_str(id_str.trim()) {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid product ID");
            return;
        }
    };

    println!("Enter quantity: ");
    let mut quantity = String::new();

    io::stdin().read_line(&mut quantity).unwrap();
    let quantity: i32 = quantity.trim().parse().unwrap_or(0);

    match store.record_sale(product_id, quantity) {
        Ok(_) => println!("Sale recorded successfully"),
        Err(e) => println!("Error recording sale: {}", e),
    }
}

fn record_purchase(store: &mut Store) {
    println!("\nAvailable Products:");
    println!("{}", store.generate_inventory_report());

    println!("Enter product ID: ");
    let mut id_str = String::new();
    io::stdin().read_line(&mut id_str).unwrap();
    let product_id = match Uuid::parse_str(id_str.trim()) {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid product ID");
            return;
        }
    };

    println!("Enter quantity: ");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).unwrap();
    let quantity: i32 = quantity.trim().parse().unwrap_or(0);

    println!("Enter purchase price per unit: ");
    let mut price = String::new();
    io::stdin().read_line(&mut price).unwrap();
    let price: f64 = price.trim().parse().unwrap_or(0.0);

    match store.record_purchase(product_id, quantity, price) {
        Ok(_) => println!("Purchase recorded successfully"),
        Err(e) => println!("Error recording purchase: {}", e),
    }
}

fn show_reports(store: &Store) {
    println!("\nReports Menu");
    println!("1. Inventory Report");
    println!("2. Sales Report");
    println!("3. Purchase Report");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => println!("\n{}", store.generate_inventory_report()),
        "2" => println!("\n{}", store.generate_sales_report()),
        "3" => println!("\n{}", store.generate_purchase_report()),
        _ => println!("Invalid choice"),
    }
}
