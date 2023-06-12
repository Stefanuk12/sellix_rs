/// Represents all of the order events.
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

/// Represents all of the query events.
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

/// Represents all of the feedback events.
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

/// Represents all of the product events.
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


/// Represents all of the subscription events.
/// <https://developers.sellix.io/#subscription-handle-webhook-events>.
pub enum Subscription {
    /// This webhook has been already sent if the product has a trial period.
    /// If not, this webhook is sent right after an invoice `PRODUCT_SUBSCRIPTION` has been successfully paid.
    /// The subscription has been started for your customer and the payment succeeded,
    /// you should implement there any logic that you might deem necessary to activate your customer's subscription.
    /// We highly suggest the usage of our custom fields to act as a metadata object in order to recognize which customer purchased your subscription and activate it to them.
    Created,
    /// If the customer updates his preferred gateway for the subscription through the Customer Billing Portal
    /// you will receive a webhook event with the updated subscription object.
    Updated,
    /// When the payment for a subscription renewal happens, this webhook is sent to let you know that everything went well.
    /// You do not need to be listening to this event specifically as you can manage your subscriptions through only the `created` and `cancelled` events.
    Renewed,
    /// Should you or the customer decide to cancel the subscription, this webhook event is sent.
    Cancelled,
    /// Event sent a few days before a subscription is about to be renewed.
    Upcoming,
    /// Optional, sent only if the product has a trial period enabled.
    /// A trial for one of your product subscription has been started, we will not create any invoice in relation to this event.
    /// If this webhook is sent, it’s preceded by a `subscription:created` event with a unique status.
    TrialStarted,
    /// Optional, sent only if the product has a trial period enabled and it has come to an end.
    /// A trial for one of your product subscription has ended, we will not create any invoice in relation to this event.
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