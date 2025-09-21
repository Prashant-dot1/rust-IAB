# 📦 Rust Orders API

A simple **RESTful API** for managing customer orders, built with [Axum](https://github.com/tokio-rs/axum) and [Tokio](https://tokio.rs/).  
This project was developed as part of the Stackpop / IAB coding exercise.

---

## 🚀 Features
- Create, list, update, retrieve, and delete customer orders.
- DTO-based request/response models (no direct exposure of domain models).
- Validation with [validator](https://crates.io/crates/validator).
- JSON error responses with detailed per-field validation errors.
- Logging (requests, responses, and DB operations) behind a **`DEV_LOGGING`** flag.
- Configurable host and port via `.env`.

---

## 🛠️ Tech Stack
- **Rust 1.70+**
- [Axum 0.8](https://crates.io/crates/axum) – web framework  
- [Tokio](https://crates.io/crates/tokio) – async runtime  
- [Serde](https://crates.io/crates/serde) – JSON (de)serialization  
- [Validator](https://crates.io/crates/validator) – input validation  
- [Tower HTTP](https://crates.io/crates/tower-http) – request/response tracing  
- [Tracing](https://crates.io/crates/tracing) – structured logs  
- [dotenvy](https://crates.io/crates/dotenvy) – environment variables
- [UUID](https://crates.io/crates/uuid) – unique identifiers
- [Regex](https://crates.io/crates/regex) – pattern matching  

---

## ⚙️ Setup

### 1. Clone the repo
```bash
git clone <your-repo-url>
cd rust_assignment
```

### 2. Install Rust ( if not already )
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 3. Create a .env file
HOST=127.0.0.1
PORT=3000
DEV_LOGGING=1

## Running Locally

### 1. Debug Mode
```bash
cargo run
```

### 2. Release Mode
```bash
cargo run --release
```

## Api Endpoints

### Create Order
```
POST /orders
Content-Type: application/json

Body:
{
  "customer": "Alice",
  "items": ["item1", "item2"]
}
```

### List Orders
```
GET /orders
```

### Retrieve Order by Id
```
GET /orders/{id}
```

### Update Status by Id
```
PUT /orders/{id}/status
Content-Type: application/json

{
  "status": "shipped"
}

Allowed statuses: pending | shipped | delivered | cancelled
```

### Delete Order
```
DELETE /orders/{id}
```


## Example Curl commands
```bash 
# Create
curl -X POST http://127.0.0.1:3000/orders \
  -H "Content-Type: application/json" \
  -d '{"customer":"Alice","items":["Book","Pen"]}'

# List
curl http://127.0.0.1:3000/orders

# Update
curl -X PUT http://127.0.0.1:3000/orders/<uuid>/status \
  -H "Content-Type: application/json" \
  -d '{"status":"delivered"}'
```

## Error Handling
Validation errors return structured JSON:

```
{
  "message": "Validation failed",
  "details": {
    "customer": ["customer name must not be empty"],
    "items": ["at least one item required"]
  }
}
```

## 🧪 Testing

The project includes comprehensive unit tests covering all modules:

### Running Tests
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test models::tests
cargo test db::tests
cargo test order_dtos::tests
cargo test errors::tests
```

### Test Coverage
- **Models**: Order creation, serialization/deserialization
- **DTOs**: Validation rules, regex patterns, conversion logic
- **Database**: CRUD operations, error handling, validation
- **Errors**: HTTP status codes, error responses, validation error formatting

### Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_functionality() {
        // Test implementation
    }
}
```

### Development & Testing Best Practices
- **Test-Driven Development**: All modules include comprehensive unit tests
- **Async Testing**: Uses `#[tokio::test]` for async function testing
- **Validation Testing**: Tests both valid and invalid input scenarios
- **Error Testing**: Verifies proper error handling and HTTP status codes
- **Mock Data**: Uses test-specific data structures and UUIDs
- **Isolation**: Each test is independent and doesn't affect others

### Project Structure
```
src/
├── main.rs         # entry point
├── routes.rs       # routes + handlers
├── models.rs       # domain model + tests
├── order_dtos.rs   # DTOs + validation + tests
├── errors.rs       # error handling + tests
├── db.rs           # in-memory DB + tests
```



