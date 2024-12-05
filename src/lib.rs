pub mod models;
pub mod errors;
pub mod auth;
pub mod store;

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::models::{Product, UserRole, TransactionType};
    use crate::store::Store;
    use crate::auth::Auth;
    use std::fs;

    #[test]
    fn test_add_product() {
        let mut store = Store::new();
        let product = Product {
            id: Uuid::new_v4(),
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: 10.0,
            quantity: 5,
        };

        assert!(store.add_product(product).is_ok());
    }

    #[test]
    fn test_authentication() {
        let mut auth = Auth::new();

        assert!(auth.register(
            "test_user".to_string(),
            "password123".to_string(),
            UserRole::Employee
        ).is_ok());

        assert!(auth.login("test_user", "password123").is_ok());
        assert!(auth.login("test_user", "wrong_password").is_err());
    }

    #[test]
    fn test_store_operations() {
        let mut store = Store::new();

        // Test product CRUD
        let product = Product {
            id: Uuid::new_v4(),
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: 10.0,
            quantity: 5,
        };

        // Create
        assert!(store.add_product(product.clone()).is_ok());

        // Read
        let stored_product = store.get_product(&product.id);
        assert!(stored_product.is_some());
        assert_eq!(stored_product.unwrap().name, "Test Product");

        // Update
        let mut updated_product = product.clone();
        updated_product.price = 15.0;
        assert!(store.update_product(updated_product.clone()).is_ok());
        assert_eq!(store.get_product(&product.id).unwrap().price, 15.0);

        // Delete
        assert!(store.delete_product(&product.id).is_ok());
        assert!(store.get_product(&product.id).is_none());
    }

    #[test]
    fn test_transactions() {
        let mut store = Store::new();
        let product_id = Uuid::new_v4();
        let product = Product {
            id: product_id,
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: 10.0,
            quantity: 5,
        };

        store.add_product(product).unwrap();

        // Test sale
        let sale = store.record_sale(product_id, 2).unwrap();
        assert_eq!(sale.quantity, 2);
        assert_eq!(sale.transaction_type, TransactionType::Sale);
        assert_eq!(store.get_product(&product_id).unwrap().quantity, 3);

        // Test purchase
        let purchase = store.record_purchase(product_id, 3, 8.0).unwrap();
        assert_eq!(purchase.quantity, 3);
        assert_eq!(purchase.transaction_type, TransactionType::Purchase);

        assert_eq!(store.get_product(&product_id).unwrap().quantity, 6);
    }

    #[test]
    fn test_file_persistence() {
        let mut store = Store::new();
        let product_id = Uuid::new_v4();
        let product = Product {
            id: product_id,
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: 10.0,
            quantity: 5,
        };

        store.add_product(product).unwrap();

        // Test save
        assert!(store.save_to_file("test_store.json").is_ok());

        // Test load
        let mut new_store = Store::new();
        assert!(new_store.load_from_file("test_store.json").is_ok());
        assert!(new_store.get_product(&product_id).is_some());

        // Cleanup
        fs::remove_file("test_store.json").unwrap();
    }

    #[test]
    fn test_report_generation() {
        let mut store = Store::new();
        let product = Product {
            id: Uuid::new_v4(),
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: 10.0,
            quantity: 5,
        };

        store.add_product(product).unwrap();
        let report = store.generate_inventory_report();
        assert!(report.contains("Test Product"));
        assert!(report.contains("5"));
        assert!(report.contains("$10.00"));
    }
}
