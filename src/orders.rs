use async_graphql::{
    ComplexObject, Enum, ID, InputObject, MergedObject, Object, SimpleObject, scalar,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(MergedObject, Default)]
pub struct Query(OrderQuery);

#[derive(Default)]
pub struct OrderQuery;

#[derive(Serialize, Deserialize)]
pub struct OrderId(pub Uuid);

scalar!(OrderId);

#[Object]
impl OrderQuery {
    async fn order(&self, id: OrderId) -> Order {
        tracing::info!("Fetching order with id: {:?}", id.0);
        Order {
            id,
            customer: Customer {
                id: ID::from(Uuid::new_v4().to_string()),
                name: "John Doe From Order".to_string(),
                vat: "VAT123456".to_string(),
            },
            total_amount: 99.99,
            status: OrderStatus::Confirmed,
        }
    }

    #[graphql(entity)]
    async fn find_customer_by_id(&self, #[graphql(key)] id: ID) -> Customer {
        tracing::info!("Resolving customer entity with id: {}", id.to_string());
        Customer {
            id,
            name: "John Doe From Order".to_string(),
            vat: "VAT123456".to_string(),
        }
    }

    #[graphql(entity)]
    async fn find_simple_product_by_id(&self, #[graphql(key)] id: ID) -> SimpleProduct {
        SimpleProduct {
            id,
            pippo: "PIPPO".to_string(),
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Order {
    pub id: OrderId,
    pub customer: Customer,
    pub total_amount: f64,
    pub status: OrderStatus,
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "CustomerInput")]
pub struct Customer {
    pub id: ID,
    pub name: String,
    pub vat: String,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum OrderStatus {
    Draft,
    Confirmed,
    #[graphql(name = "Cancelled")]
    Deleted,
}

#[ComplexObject]
impl Order {
    async fn lines(&self) -> Vec<OrderLine> {
        tracing::info!("Fetching order lines for order id: {:?}", self.id.0);
        vec![
            OrderLine {
                line_number: 10,
                quantity: 2,
                discount: Some(5.0),
                product: SimpleProduct {
                    id: ID::from(Uuid::new_v4().to_string()),
                    pippo: "PIPPO".to_string(),
                },
            },
            OrderLine {
                line_number: 20,
                quantity: 5,
                discount: None,
                product: SimpleProduct {
                    id: ID::from(Uuid::new_v4().to_string()),
                    pippo: "PIPPO".to_string(),
                },
            },
        ]
    }
}

#[derive(SimpleObject)]
pub struct OrderLine {
    pub line_number: i32,
    pub quantity: i32,
    pub discount: Option<f64>,
    pub product: SimpleProduct,
}

#[derive(SimpleObject)]
#[graphql(shareable)]
pub struct SimpleProduct {
    pub id: ID,
    pub pippo: String,
}

// #[derive(Interface)]
// #[graphql(field(name = "id", ty = "ID"))]
// pub enum Product {
//     DangerousProduct(DangerousProduct),
//     ExpiringProduct(ExpiringProduct),
// }

// #[derive(SimpleObject)]
// #[graphql(shareable)]
// pub struct DangerousProduct {
//     pub id: ID,
// }

// #[derive(SimpleObject)]
// #[graphql(shareable)]
// pub struct ExpiringProduct {
//     pub id: ID,
// }

#[derive(MergedObject, Default)]
pub struct Mutation(OrderMutation);

#[derive(Default)]
pub struct OrderMutation;

#[Object]
impl OrderMutation {
    async fn create_order(&self, order: CreateOrder) -> Order {
        tracing::info!("Creating order for customer vat: {}", order.customer.vat);
        Order {
            id: OrderId(Uuid::new_v4()),
            customer: Customer {
                id: ID::from(Uuid::new_v4().to_string()),
                name: "New Customer".to_string(),
                vat: "VAT123456".to_string(),
            },
            total_amount: 0.0,
            status: OrderStatus::Draft,
        }
    }
}

#[derive(InputObject)]
pub struct CreateOrder {
    pub customer: Customer,
    pub items: Vec<CreateOrderLine>,
}

#[derive(InputObject)]
pub struct CreateOrderLine {
    pub product_id: ID,
    pub quantity: i32,
}

#[derive(Default)]
pub struct Subscription;
