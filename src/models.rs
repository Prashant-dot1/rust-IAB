use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Domain model (not exposed directly in requests)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub customer: String,
    pub items: Vec<String>,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_creation() {
        let id = Uuid::new_v4();
        let order = Order {
            id,
            customer: "John Doe".to_string(),
            items: vec!["Item 1".to_string(), "Item 2".to_string()],
            status: "pending".to_string(),
        };

        assert_eq!(order.customer, "John Doe");
        assert_eq!(order.items.len(), 2);
        assert_eq!(order.status, "pending");
    }

    #[test]
    fn test_order_serialization() {
        let id = Uuid::new_v4();
        let order = Order {
            id,
            customer: "Jane Smith".to_string(),
            items: vec!["Product A".to_string()],
            status: "shipped".to_string(),
        };

        let json = serde_json::to_string(&order).unwrap();
        let deserialized: Order = serde_json::from_str(&json).unwrap();

        assert_eq!(order.id, deserialized.id);
        assert_eq!(order.customer, deserialized.customer);
        assert_eq!(order.items, deserialized.items);
        assert_eq!(order.status, deserialized.status);
    }
}
