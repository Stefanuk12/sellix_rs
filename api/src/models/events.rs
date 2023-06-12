/// Represents all of the order events
pub enum Order {
    Created,
    Updated,
    Partial,
    Paid,
    Cancelled,
    Disputed,

    PaidProduct,
    CancelledProduct,
    DisputedProduct,
    CreatedProduct,
    PartialProduct,
    UpdatedProduct
}
impl Order {
    pub fn from_str(x: &str) -> Self {
        match x {
            "order:created" => Self::Created,
            "order:partial" => Self::Partial,
            "order:updated" => Self::Updated,
            "order:paid" => Self::Paid,
            "order:cancelled" => Self::Cancelled,
            "order:disputed" => Self::Disputed,
            "order:paid:product" => Self::PaidProduct,
            "order:cancelled:product" => Self::CancelledProduct,
            "order:disputed:product" => Self::DisputedProduct,
            "order:created:product" => Self::CreatedProduct,
            "order:partial:product" => Self::PartialProduct,
            "order:updated:product" => Self::UpdatedProduct,
            _ => panic!("Invalid input"),
        }
    }
}

/// Represents all of the query events
pub enum Query {
    Created,
    Replied,
}
impl Query {
    pub fn from_str(x: &str) -> Self {
        match x {
            "query:created" => Self::Created,
            "query:replied" => Self::Replied,
            _ => panic!("Invalid input"),
        }
    }
}

/// Represents all of the feedback events
pub enum Feedback {
    Created,
}
impl Feedback {
    pub fn from_str(x: &str) -> Self {
        match x {
            "feedback:created" => Self::Created,
            _ => panic!("Invalid input"),
        }
    }
}

/// Represents all of the product events
pub enum Product {
    Created,
    Edited,
    Stock,
    Dynamic,
}
impl Product {
    pub fn from_str(x: &str) -> Self {
        match x {
            "product:created" => Self::Created,
            "product:edited" => Self::Edited,
            "product:stock" => Self::Stock,
            "product:dynamic" => Self::Dynamic,
            _ => panic!("Invalid input"),
        }
    }
}


/// Represents all of the subscription events
pub enum Subscription {
    Created,
    Updated,
    Renewed,
    Cancelled,
    Upcoming,

    TrialStarted,
    TrialEnded,
    TrialStartedProduct,
    TrialEndedProduct,
    CreatedProduct,
    UpdatedProduct,
    RenewedProduct,
    CancelledProduct,
    UpcomingProduct,
}
impl Subscription {
    pub fn from_str(x: &str) -> Self {
        match x {
            "subscription:created" => Self::Created,
            "subscription:updated" => Self::Updated,
            "subscription:renewed" => Self::Renewed,
            "subscription:cancelled" => Self::Cancelled,
            "subscription:upcoming" => Self::Upcoming,
            "subscription:trial:started" => Self::TrialStarted,
            "subscription:trial:ended" => Self::TrialEnded,
            "subscription:trial:started:product" => Self::TrialStartedProduct,
            "subscription:trial:ended:product" => Self::TrialEndedProduct,
            "subscription:created:product" => Self::CreatedProduct,
            "subscription:updated:product" => Self::UpdatedProduct,
            "subscription:renewed:product" => Self::RenewedProduct,
            "subscription:cancelled:product" => Self::CancelledProduct,
            "subscription:upcoming:product" => Self::UpcomingProduct,
            _ => panic!("Invalid input"),
        }
    }
}