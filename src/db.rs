use crate::order_dtos::{CreateOrderDto, OrderResponseDto, UpdateStatusDto};
use crate::models::Order;
use crate::errors::ApiError;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;
use validator::Validate;

pub type Db = Arc<RwLock<HashMap<Uuid, Order>>>; // Using an in memory hashMap

pub async fn create_order(db: Db, data: CreateOrderDto) -> Result<OrderResponseDto, ApiError> {
    data.validate()?; // validation
    let order = Order {
        id: Uuid::new_v4(),
        customer: data.customer,
        items: data.items,
        status: "pending".into(),
    };
    {
        let mut map = db.write().await;
        map.insert(order.id, order.clone());
        info!("Inserted order into DB: {:?}", order);
    }
    Ok(order.into())
}

pub async fn get_order(db: Db, id: Uuid) -> Result<OrderResponseDto, ApiError> {
    db.read()
        .await
        .get(&id)
        .cloned()
        .map(OrderResponseDto::from)
        .ok_or(ApiError::NotFound)
}

pub async fn list_orders(db: Db) -> Vec<OrderResponseDto> {
    db.read()
        .await
        .values()
        .cloned()
        .map(OrderResponseDto::from)
        .collect()
}

pub async fn update_status(db: Db, id: Uuid, data: UpdateStatusDto) -> Result<OrderResponseDto, ApiError> {
    data.validate()?; // validation
    let mut map = db.write().await;
    if let Some(order) = map.get_mut(&id) {
        order.status = data.status;
        info!("Updated order {:?} => status {}", id, order.status);
        return Ok(order.clone().into());
    }
    Err(ApiError::NotFound)
}

pub async fn delete_order(db: Db, id: Uuid) -> Result<(), ApiError> {
    let mut map = db.write().await;
    if let Some(_) = map.remove(&id) {
        info!("Deleted order {:?}", id);
        Ok(())
    } else {
        Err(ApiError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_db() -> Db {
        Arc::new(RwLock::new(HashMap::new()))
    }

    #[tokio::test]
    async fn test_create_order() {
        let db = create_test_db();
        let dto = CreateOrderDto {
            customer: "Test Customer".to_string(),
            items: vec!["Item 1".to_string(), "Item 2".to_string()],
        };

        let result = create_order(db.clone(), dto).await;
        assert!(result.is_ok());

        let order = result.unwrap();
        assert_eq!(order.customer, "Test Customer");
        assert_eq!(order.items.len(), 2);
        assert_eq!(order.status, "pending");
    }

    #[tokio::test]
    async fn test_create_order_validation_error() {
        let db = create_test_db();
        let invalid_dto = CreateOrderDto {
            customer: "".to_string(), // Invalid: empty customer
            items: vec!["Item 1".to_string()],
        };

        let result = create_order(db, invalid_dto).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_order() {
        let db = create_test_db();
        let dto = CreateOrderDto {
            customer: "Test Customer".to_string(),
            items: vec!["Item 1".to_string()],
        };

        let created_order = create_order(db.clone(), dto).await.unwrap();
        let retrieved_order = get_order(db, created_order.id).await.unwrap();

        assert_eq!(created_order.id, retrieved_order.id);
        assert_eq!(created_order.customer, retrieved_order.customer);
    }

    #[tokio::test]
    async fn test_get_order_not_found() {
        let db = create_test_db();
        let non_existent_id = Uuid::new_v4();

        let result = get_order(db, non_existent_id).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::NotFound));
    }

    #[tokio::test]
    async fn test_list_orders() {
        let db = create_test_db();

        // Initially empty
        let orders = list_orders(db.clone()).await;
        assert_eq!(orders.len(), 0);

        // Add some orders
        let dto1 = CreateOrderDto {
            customer: "Customer 1".to_string(),
            items: vec!["Item 1".to_string()],
        };
        let dto2 = CreateOrderDto {
            customer: "Customer 2".to_string(),
            items: vec!["Item 2".to_string()],
        };

        create_order(db.clone(), dto1).await.unwrap();
        create_order(db.clone(), dto2).await.unwrap();

        let orders = list_orders(db).await;
        assert_eq!(orders.len(), 2);
    }

    #[tokio::test]
    async fn test_update_status() {
        let db = create_test_db();
        let dto = CreateOrderDto {
            customer: "Test Customer".to_string(),
            items: vec!["Item 1".to_string()],
        };

        let created_order = create_order(db.clone(), dto).await.unwrap();
        let update_dto = UpdateStatusDto {
            status: "shipped".to_string(),
        };

        let updated_order = update_status(db, created_order.id, update_dto).await.unwrap();
        assert_eq!(updated_order.status, "shipped");
        assert_eq!(updated_order.id, created_order.id);
    }

    #[tokio::test]
    async fn test_update_status_not_found() {
        let db = create_test_db();
        let non_existent_id = Uuid::new_v4();
        let update_dto = UpdateStatusDto {
            status: "shipped".to_string(),
        };

        let result = update_status(db, non_existent_id, update_dto).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::NotFound));
    }

    #[tokio::test]
    async fn test_update_status_validation_error() {
        let db = create_test_db();
        let dto = CreateOrderDto {
            customer: "Test Customer".to_string(),
            items: vec!["Item 1".to_string()],
        };

        let created_order = create_order(db.clone(), dto).await.unwrap();
        let invalid_update_dto = UpdateStatusDto {
            status: "invalid_status".to_string(),
        };

        let result = update_status(db, created_order.id, invalid_update_dto).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_order() {
        let db = create_test_db();
        let dto = CreateOrderDto {
            customer: "Test Customer".to_string(),
            items: vec!["Item 1".to_string()],
        };

        let created_order = create_order(db.clone(), dto).await.unwrap();
        let delete_result = delete_order(db.clone(), created_order.id).await;
        assert!(delete_result.is_ok());

        // Verify order is deleted
        let get_result = get_order(db, created_order.id).await;
        assert!(get_result.is_err());
        assert!(matches!(get_result.unwrap_err(), ApiError::NotFound));
    }

    #[tokio::test]
    async fn test_delete_order_not_found() {
        let db = create_test_db();
        let non_existent_id = Uuid::new_v4();

        let result = delete_order(db, non_existent_id).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::NotFound));
    }
}
