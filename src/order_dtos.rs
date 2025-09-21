use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use crate::models::Order;


/// Request DTO for creating an order
#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrderDto {
    #[validate(length(min = 1, message = "customer name must not be empty"))]
    pub customer: String,

    #[validate(length(min = 1, message = "at least one item required"))]
    pub items: Vec<String>,
}

/// Request DTO for updating status
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateStatusDto {
    #[validate(regex(path = "STATUS_REGEX", message = "invalid status"))]
    pub status: String,
}

/// Response DTO
#[derive(Debug, Serialize)]
pub struct OrderResponseDto {
    pub id: Uuid,
    pub customer: String,
    pub items: Vec<String>,
    pub status: String,
}

impl From<Order> for OrderResponseDto {
    fn from(o: Order) -> Self {
        Self {
            id: o.id,
            customer: o.customer,
            items: o.items,
            status: o.status,
        }
    }
}

lazy_static::lazy_static! {
    static ref STATUS_REGEX: regex::Regex =
        regex::Regex::new(r"^(pending|shipped|delivered|cancelled)$").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_create_order_dto_validation() {
        // Valid DTO
        let valid_dto = CreateOrderDto {
            customer: "John Doe".to_string(),
            items: vec!["Item 1".to_string(), "Item 2".to_string()],
        };
        assert!(valid_dto.validate().is_ok());

        // Invalid DTO - empty customer
        let invalid_dto = CreateOrderDto {
            customer: "".to_string(),
            items: vec!["Item 1".to_string()],
        };
        assert!(invalid_dto.validate().is_err());

        // Invalid DTO - empty items
        let invalid_dto = CreateOrderDto {
            customer: "John Doe".to_string(),
            items: vec![],
        };
        assert!(invalid_dto.validate().is_err());
    }

    #[test]
    fn test_update_status_dto_validation() {
        // Valid statuses
        for status in &["pending", "shipped", "delivered", "cancelled"] {
            let dto = UpdateStatusDto {
                status: status.to_string(),
            };
            assert!(dto.validate().is_ok(), "Status '{}' should be valid", status);
        }

        // Invalid status
        let invalid_dto = UpdateStatusDto {
            status: "invalid_status".to_string(),
        };
        assert!(invalid_dto.validate().is_err());
    }

    #[test]
    fn test_order_response_dto_from_order() {
        let order = Order {
            id: Uuid::new_v4(),
            customer: "Test Customer".to_string(),
            items: vec!["Item 1".to_string(), "Item 2".to_string()],
            status: "pending".to_string(),
        };

        let response_dto = OrderResponseDto::from(order.clone());

        assert_eq!(response_dto.id, order.id);
        assert_eq!(response_dto.customer, order.customer);
        assert_eq!(response_dto.items, order.items);
        assert_eq!(response_dto.status, order.status);
    }

    #[test]
    fn test_order_response_dto_serialization() {
        let response_dto = OrderResponseDto {
            id: Uuid::new_v4(),
            customer: "Test Customer".to_string(),
            items: vec!["Product A".to_string()],
            status: "shipped".to_string(),
        };

        let json = serde_json::to_string(&response_dto).unwrap();
        assert!(json.contains("Test Customer"));
        assert!(json.contains("Product A"));
        assert!(json.contains("shipped"));
    }

    #[test]
    fn test_status_regex() {
        assert!(STATUS_REGEX.is_match("pending"));
        assert!(STATUS_REGEX.is_match("shipped"));
        assert!(STATUS_REGEX.is_match("delivered"));
        assert!(STATUS_REGEX.is_match("cancelled"));
        
        assert!(!STATUS_REGEX.is_match("invalid"));
        assert!(!STATUS_REGEX.is_match("PENDING"));
        assert!(!STATUS_REGEX.is_match(""));
        assert!(!STATUS_REGEX.is_match("pending "));
    }
}
