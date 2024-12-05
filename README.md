# Rusty Store

A robust inventory management system written in Rust that helps track products, sales, and purchases.

## Installation

1. Clone the repository:

```bash
git clone https://github.com/mustafaangi/rusty-store.git
cd rusty-store
```

2. Build the project:

```bash
cargo build --release
```

3. Run the application:

```bash
cargo run
```

## Quick Start Guide

### Default Admin Credentials

```
Username: admin
Password: admin123
```

### Initial Setup

1. First run will create:
   - users.json (user accounts)
   - store.json (inventory and transactions)
2. Login with default admin credentials
3. Start adding products and managing inventory

### Basic Operations

1. **Adding Products** (Manager only):

   - Login as admin
   - Select "Add Product"
   - Enter product details

2. **Recording Sales**:

   - Select "Record Sale"
   - Choose product from inventory
   - Enter quantity

3. **Recording Purchases**:

   - Select "Record Purchase"
   - Choose product
   - Enter quantity and price

4. **Viewing Reports**:
   - Select "View Reports"
   - Choose report type:
     - Inventory
     - Sales
     - Purchases

## Project Structure

```
rusty-store/
├── src/
│   ├── main.rs       # Application entry
│   ├── auth.rs       # Authentication
│   ├── store.rs      # Core business logic
│   ├── models.rs     # Data structures
│   ├── errors.rs     # Error handling
│   └── lib.rs        # Library interface
├── Cargo.toml
└── README.md
```

## Error Handling

The system handles common errors:

- Invalid login credentials
- Insufficient inventory
- Product not found
- File system errors
- Invalid input values

## Data Persistence

Data is stored in JSON files:

- `users.json`: User accounts and roles
- `store.json`: Products and transactions

## Development

### Running Tests

```bash
cargo test
```

### Building Documentation

```bash
cargo doc --no-deps --open
```

### Debug Mode

```bash
RUST_LOG=debug cargo run
```

## Troubleshooting

### Login Issues

1. Delete users.json to reset admin account
2. Verify file permissions
3. Check username/password carefully

### Data Issues

1. Ensure write permissions in directory
2. Verify JSON files are not corrupted
3. Check available disk space

## Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit changes (`git commit -m 'Add AmazingFeature'`)
4. Push branch (`git push origin feature/AmazingFeature`)
5. Open Pull Request

## Detailed Documentation

### Core Components

1. **Authentication System (`auth.rs`)**

   - User management
   - Password hashing using bcrypt
   - Role-based access (Manager/Employee)
   - File-based user persistence
   - Session management

2. **Store Operations (`store.rs`)**

   - Product CRUD operations
   - Transaction handling
   - Inventory management
   - Report generation
   - Data persistence

3. **Data Models (`models.rs`)**

   ```rust
   Product {
       id: UUID,
       name: String,
       description: String,
       price: f64,
       quantity: i32
   }

   Transaction {
       id: UUID,
       product_id: UUID,
       quantity: i32,
       price: f64,
       transaction_type: Enum(Sale, Purchase),
       timestamp: DateTime<Utc>
   }

   User {
       id: UUID,
       username: String,
       password_hash: String,
       role: Enum(Manager, Employee)
   }
   ```

### User Roles & Permissions

1. **Manager**

   - Full system access
   - Add/Edit/Delete products
   - View all reports
   - Record transactions
   - Manage inventory

2. **Employee**
   - View inventory
   - Record sales
   - Record purchases
   - View reports

### File Structure & Persistence

1. **users.json**

   ```json
   {
     "username": {
       "id": "uuid",
       "username": "string",
       "password_hash": "string",
       "role": "Manager/Employee"
     }
   }
   ```

2. **store.json**
   ```json
   {
     "products": {
       "product_id": {
         "id": "uuid",
         "name": "string",
         "description": "string",
         "price": float,
         "quantity": integer
       }
     },
     "transactions": [
       {
         "id": "uuid",
         "product_id": "uuid",
         "quantity": integer,
         "price": float,
         "transaction_type": "Sale/Purchase",
         "timestamp": "datetime"
       }
     ]
   }
   ```

### Available Reports

1. **Inventory Report**

   - Current stock levels
   - Product details
   - Pricing information

   ```
   Inventory Report
   ================
   Product: [name]
   Quantity: [number]
   Price: $[amount]
   ```

2. **Sales Report**

   - Transaction history
   - Total sales amount
   - Individual sale details

   ```
   Sales Report
   ===========
   Sale ID: [uuid]
   Product ID: [uuid]
   Quantity: [number]
   Price: $[amount]
   Total: $[amount]
   ```

3. **Purchase Report**
   - Purchase history
   - Total cost
   - Individual purchase details
   ```
   Purchase Report
   ==============
   Purchase ID: [uuid]
   Product ID: [uuid]
   Quantity: [number]
   Cost: $[amount]
   Total: $[amount]
   ```

### Error Handling

1. **Authentication Errors**

   - Invalid credentials
   - User not found
   - Password verification failed

2. **Inventory Errors**

   - Insufficient stock
   - Product not found
   - Invalid quantity

3. **File System Errors**
   - File not found
   - Permission denied
   - Corrupted data

### Development Guide

1. **Setting Up Development Environment**

   ```bash
   # Install Rust and Cargo
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Clone and Build
   git clone https://github.com/yourusername/rusty-store.git
   cd rusty-store
   cargo build
   ```

2. **Running Tests**

   ```bash
   # Run all tests
   cargo test

   # Run specific test
   cargo test test_authentication

   # Run with logging
   RUST_LOG=debug cargo test
   ```

3. **Adding New Features**
   - Create feature branch
   - Add tests first
   - Implement feature
   - Update documentation
   - Submit PR

### Performance Considerations

1. **File Operations**

   - Batch writes when possible
   - Implement caching if needed
   - Handle concurrent access

2. **Memory Usage**
   - Use appropriate data structures
   - Implement pagination for large datasets
   - Clean up resources properly

### Security Best Practices

1. **Password Management**

   - Use bcrypt for hashing
   - Never store plain passwords
   - Implement password policies

2. **Access Control**

   - Role-based permissions
   - Session management
   - Input validation

3. **Data Protection**
   - Secure file permissions
   - Data validation
   - Error message sanitization
