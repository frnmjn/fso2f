use async_graphql::{ComplexObject, Enum, InputObject, MergedObject, Object, SimpleObject, scalar};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(MergedObject, Default)]
pub struct QueryRoot(OrderQuery);

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
                id: Uuid::new_v4(),
                name: "John Doe".to_string(),
                vat: "VAT123456".to_string(),
            },
            total_amount: 99.99,
            status: OrderStatus::Confirmed,
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
    #[graphql(skip)]
    pub id: Uuid,
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
                product_id: Uuid::new_v4(),
            },
            OrderLine {
                line_number: 20,
                quantity: 5,
                discount: None,
                product_id: Uuid::new_v4(),
            },
        ]
    }
}

#[derive(SimpleObject)]
pub struct OrderLine {
    pub line_number: i32,
    pub quantity: i32,
    pub discount: Option<f64>,
    pub product_id: Uuid,
}

#[derive(MergedObject, Default)]
pub struct MutationRoot(OrderMutation);

#[derive(Default)]
pub struct OrderMutation;

#[Object]
impl OrderMutation {
    async fn create_order(&self, order: CreateOrder) -> Order {
        tracing::info!("Creating order for customer vat: {}", order.customer.vat);
        Order {
            id: OrderId(Uuid::new_v4()),
            customer: Customer {
                id: Uuid::new_v4(),
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
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Default)]
pub struct SubscriptionRoot;
