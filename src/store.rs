use crate::models::{Product, Transaction, TransactionType};
use crate::errors::StoreError;
use std::collections::HashMap;
use uuid::Uuid;
use serde_json::{to_writer, from_reader};
use std::fs::File;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Store {
    products: HashMap<Uuid, Product>,
    transactions: Vec<Transaction>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            products: HashMap::new(),
            transactions: Vec::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) -> Result<(), StoreError> {
        self.products.insert(product.id, product);
        Ok(())
    }

    pub fn get_product(&self, id: &Uuid) -> Option<&Product> {
        self.products.get(id)
    }

    pub fn update_product(&mut self, product: Product) -> Result<(), StoreError> {
        if !self.products.contains_key(&product.id) {
            return Err(StoreError::NotFound);
        }
        self.products.insert(product.id, product);
        Ok(())
    }

    pub fn delete_product(&mut self, id: &Uuid) -> Result<(), StoreError> {
        self.products.remove(id).ok_or(StoreError::NotFound)?;
        Ok(())
    }

    pub fn record_sale(&mut self, product_id: Uuid, quantity: i32) -> Result<Transaction, StoreError> {
        let product = self.products.get_mut(&product_id)
            .ok_or(StoreError::NotFound)?;

        if product.quantity < quantity {
            return Err(StoreError::InsufficientInventory);
        }

        product.quantity -= quantity;
        let transaction = Transaction {
            id: Uuid::new_v4(),
            product_id,
            quantity,
            price: product.price,
            transaction_type: TransactionType::Sale,
            timestamp: chrono::Utc::now(),
        };

        self.transactions.push(transaction.clone());
        Ok(transaction)
    }

    pub fn record_purchase(&mut self, product_id: Uuid, quantity: i32, price: f64) -> Result<Transaction, StoreError> {
        let product = self.products.get_mut(&product_id)
            .ok_or(StoreError::NotFound)?;

        product.quantity += quantity;
        let transaction = Transaction {
            id: Uuid::new_v4(),
            product_id,
            quantity,
            price,
            transaction_type: TransactionType::Purchase,
            timestamp: chrono::Utc::now(),
        };

        self.transactions.push(transaction.clone());
        Ok(transaction)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), StoreError> {
        let file = File::create(path).map_err(|e| StoreError::DatabaseError(e.to_string()))?;
        to_writer(file, &StoreData {
            products: self.products.clone(),
            transactions: self.transactions.clone(),
        }).map_err(|e| StoreError::DatabaseError(e.to_string()))
    }

    pub fn load_from_file(&mut self, path: &str) -> Result<(), StoreError> {
        match File::open(path) {
            Ok(file) => {
                match from_reader(file) {
                    Ok(data) => {
                        let store_data: StoreData = data;
                        self.products = store_data.products;
                        self.transactions = store_data.transactions;
                        Ok(())
                    },
                    Err(_) => {
                        // Initialize new store file if corrupted
                        self.save_to_file(path)
                    }
                }
            },
            Err(_) => {
                // Create new store file if it doesn't exist
                self.save_to_file(path)
            }
        }
    }

    pub fn generate_inventory_report(&self) -> String {
        let mut report = String::from("Inventory Report\n================\n\n");
        for product in self.products.values() {
            report.push_str(&format!("Product: {}\nQuantity: {}\nPrice: ${:.2}\n\n",
                product.name, product.quantity, product.price));
        }
        report
    }

    pub fn generate_sales_report(&self) -> String {
        let mut report = String::from("Sales Report\n============\n\n");
        let mut total_sales = 0.0;

        for transaction in &self.transactions {
            if matches!(transaction.transaction_type, TransactionType::Sale) {
                let total = transaction.price * transaction.quantity as f64;
                total_sales += total;
                report.push_str(&format!(
                    "Sale ID: {}\nProduct ID: {}\nQuantity: {}\nPrice: ${:.2}\nTotal: ${:.2}\n\n",
                    transaction.id, transaction.product_id, transaction.quantity,
                    transaction.price, total

                ));
            }
        }

        report.push_str(&format!("Total Sales: ${:.2}\n", total_sales));
        report
    }

    pub fn generate_purchase_report(&self) -> String {
        let mut report = String::from("Purchase Report\n===============\n\n");
        let mut total_cost = 0.0;

        for transaction in &self.transactions {
            if matches!(transaction.transaction_type, TransactionType::Purchase) {
                let total = transaction.price * transaction.quantity as f64;
                total_cost += total;
                report.push_str(&format!(
                    "Purchase ID: {}\nProduct ID: {}\nQuantity: {}\nCost: ${:.2}\nTotal: ${:.2}\n\n",
                    transaction.id, transaction.product_id, transaction.quantity,
                    transaction.price, total
                ));
            }
        }

        report.push_str(&format!("Total Purchases: ${:.2}\n", total_cost));
        report
    }
}

#[derive(Serialize, Deserialize)]
struct StoreData {
    products: HashMap<Uuid, Product>,
    transactions: Vec<Transaction>,
}
