use serde::{Serialize, Deserialize};

/// Represents all of the order events.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString)]
pub enum Order {
    #[strum(serialize="order:created")]
    Created,
    #[strum(serialize="order:updated")]
    Updated,
    #[strum(serialize="order:partial")]
    Partial,
    #[strum(serialize="order:paid")]
    Paid,
    #[strum(serialize="order:cancelled")]
    Cancelled,
    #[strum(serialize="order:disputed")]
    Disputed,

    #[strum(serialize="order:paid:product")]
    PaidProduct,
    #[strum(serialize="order:cancelled:product")]
    CancelledProduct,
    #[strum(serialize="order:disputed:product")]
    DisputedProduct,
    #[strum(serialize="order:created:product")]
    CreatedProduct,
    #[strum(serialize="order:partial:product")]
    PartialProduct,
    #[strum(serialize="order:updated:product")]
    UpdatedProduct
}

/// Represents all of the query events.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString)]
pub enum Query {
    #[strum(serialize="query:created")]
    Created,
    #[strum(serialize="query:replied")]
    Replied,
}

/// Represents all of the feedback events.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString)]
pub enum Feedback {
    #[strum(serialize="feedback:created")]
    Created,
}

/// Represents all of the product events.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString)]
pub enum Product {
    #[strum(serialize="product:created")]
    Created,
    #[strum(serialize="product:edited")]
    Edited,
    #[strum(serialize="product:stock")]
    Stock,
    #[strum(serialize="product:dynamic")]
    Dynamic,
}

/// Represents all of the subscription events.
/// <https://developers.sellix.io/#subscription-handle-webhook-events>.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString)]
pub enum Subscription {
    /// This webhook has been already sent if the product has a trial period.
    /// If not, this webhook is sent right after an invoice `PRODUCT_SUBSCRIPTION` has been successfully paid.
    /// The subscription has been started for your customer and the payment succeeded,
    /// you should implement there any logic that you might deem necessary to activate your customer's subscription.
    /// We highly suggest the usage of our custom fields to act as a metadata object in order to recognize which customer purchased your subscription and activate it to them.
    #[strum(serialize="subscription:created")]
    Created,
    /// If the customer updates his preferred gateway for the subscription through the Customer Billing Portal
    /// you will receive a webhook event with the updated subscription object.
    #[strum(serialize="subscription:updated")]
    Updated,
    /// When the payment for a subscription renewal happens, this webhook is sent to let you know that everything went well.
    /// You do not need to be listening to this event specifically as you can manage your subscriptions through only the `created` and `cancelled` events.
    #[strum(serialize="subscription:renewed")]
    Renewed,
    /// Should you or the customer decide to cancel the subscription, this webhook event is sent.
    #[strum(serialize="subscription:cancelled")]
    Cancelled,
    /// Event sent a few days before a subscription is about to be renewed.
    #[strum(serialize="subscription:upcoming")]
    Upcoming,
    /// Optional, sent only if the product has a trial period enabled.
    /// A trial for one of your product subscription has been started, we will not create any invoice in relation to this event.
    /// If this webhook is sent, it’s preceded by a `subscription:created` event with a unique status.
    #[strum(serialize="subscription:trial:started")]
    TrialStarted,
    /// Optional, sent only if the product has a trial period enabled and it has come to an end.
    /// A trial for one of your product subscription has ended, we will not create any invoice in relation to this event.
    #[strum(serialize="subscription:trial:ended")]
    TrialEnded,

    #[strum(serialize="subscription:trial:started:product")]
    TrialStartedProduct,
    #[strum(serialize="subscription:trial:ended:product")]
    TrialEndedProduct,
    #[strum(serialize="subscription:created:product")]
    CreatedProduct,
    #[strum(serialize="subscription:updated:product")]
    UpdatedProduct,
    #[strum(serialize="subscription:renewed:product")]
    RenewedProduct,
    #[strum(serialize="subscription:cancelled:product")]
    CancelledProduct,
    #[strum(serialize="subscription:upcoming:product")]
    UpcomingProduct,
}

/// Represents all events combined.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString)]
pub enum Event {
    #[strum(serialize="order:created")]
    OrderCreated,
    #[strum(serialize="order:updated")]
    OrderUpdated,
    #[strum(serialize="order:partial")]
    OrderPartial,
    #[strum(serialize="order:paid")]
    OrderPaid,
    #[strum(serialize="order:cancelled")]
    OrderCancelled,
    #[strum(serialize="order:disputed")]
    OrderDisputed,
    #[strum(serialize="order:paid:product")]
    OrderPaidProduct,
    #[strum(serialize="order:cancelled:product")]
    OrderCancelledProduct,
    #[strum(serialize="order:disputed:product")]
    OrderDisputedProduct,
    #[strum(serialize="order:created:product")]
    OrderCreatedProduct,
    #[strum(serialize="order:partial:product")]
    OrderPartialProduct,
    #[strum(serialize="order:updated:product")]
    OrderUpdatedProduct,
    #[strum(serialize="query:created")]
    QueryCreated,
    #[strum(serialize="query:replied")]
    QueryReplied,
    #[strum(serialize="feedback:created")]
    FeedbackCreated,
    #[strum(serialize="product:created")]
    ProductCreated,
    #[strum(serialize="product:edited")]
    ProductEdited,
    #[strum(serialize="product:stock")]
    ProductStock,
    #[strum(serialize="product:dynamic")]
    ProductDynamic,
    #[strum(serialize="subscription:created")]
    SubscriptionCreated,
    #[strum(serialize="subscription:updated")]
    SubscriptionUpdated,
    #[strum(serialize="subscription:renewed")]
    SubscriptionRenewed,
    #[strum(serialize="subscription:cancelled")]
    SubscriptionCancelled,
    #[strum(serialize="subscription:upcoming")]
    SubscriptionUpcoming,
    #[strum(serialize="subscription:trial:started")]
    SubscriptionTrialStarted,
    #[strum(serialize="subscription:trial:ended")]
    SubscriptionTrialEnded,
    #[strum(serialize="subscription:trial:started:product")]
    SubscriptionTrialStartedProduct,
    #[strum(serialize="subscription:trial:ended:product")]
    SubscriptionTrialEndedProduct,
    #[strum(serialize="subscription:created:product")]
    SubscriptionCreatedProduct,
    #[strum(serialize="subscription:updated:product")]
    SubscriptionUpdatedProduct,
    #[strum(serialize="subscription:renewed:product")]
    SubscriptionRenewedProduct,
    #[strum(serialize="subscription:cancelled:product")]
    SubscriptionCancelledProduct,
    #[strum(serialize="subscription:upcoming:product")]
    SubscriptionUpcomingProduct,
}