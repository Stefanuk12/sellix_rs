use std::str::FromStr;

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
#[derive(Debug, PartialEq, Eq)]
pub struct ParseOrderError;
impl FromStr for Order {
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        match x {
            "order:created" => Ok(Self::Created),
            "order:partial" => Ok(Self::Partial),
            "order:updated" => Ok(Self::Updated),
            "order:paid" => Ok(Self::Paid),
            "order:cancelled" => Ok(Self::Cancelled),
            "order:disputed" => Ok(Self::Disputed),
            "order:paid:product" => Ok(Self::PaidProduct),
            "order:cancelled:product" => Ok(Self::CancelledProduct),
            "order:disputed:product" => Ok(Self::DisputedProduct),
            "order:created:product" => Ok(Self::CreatedProduct),
            "order:partial:product" => Ok(Self::PartialProduct),
            "order:updated:product" => Ok(Self::UpdatedProduct),
            _ => Err(Self::Err {}),
        }
    }

    type Err = ParseOrderError;
}

/// Represents all of the query events.
pub enum Query {
    Created,
    Replied,
}
#[derive(Debug, PartialEq, Eq)]
pub struct ParseQueryError;
impl FromStr for Query {
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        match x {
            "query:created" => Ok(Self::Created),
            "query:replied" => Ok(Self::Replied),
            _ => Err(Self::Err {}),
        }
    }

    type Err = ParseQueryError;
}

/// Represents all of the feedback events.
pub enum Feedback {
    Created,
}
#[derive(Debug, PartialEq, Eq)]
pub struct ParseFeedbackError;
impl FromStr for Feedback {
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        match x {
            "feedback:created" => Ok(Self::Created),
            _ => Err(Self::Err {}),
        }
    }

    type Err = ParseProductError;
}

/// Represents all of the product events.
pub enum Product {
    Created,
    Edited,
    Stock,
    Dynamic,
}
#[derive(Debug, PartialEq, Eq)]
pub struct ParseProductError;
impl FromStr for Product {
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        match x {
            "product:created" => Ok(Self::Created),
            "product:edited" => Ok(Self::Edited),
            "product:stock" => Ok(Self::Stock),
            "product:dynamic" => Ok(Self::Dynamic),
            _ => Err(Self::Err {}),
        }
    }

    type Err = ParseProductError;
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
#[derive(Debug, PartialEq, Eq)]
pub struct ParseSubscriptionError;
impl FromStr for Subscription {
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        match x {
            "subscription:created" => Ok(Self::Created),
            "subscription:updated" => Ok(Self::Updated),
            "subscription:renewed" => Ok(Self::Renewed),
            "subscription:cancelled" => Ok(Self::Cancelled),
            "subscription:upcoming" => Ok(Self::Upcoming),
            "subscription:trial:started" => Ok(Self::TrialStarted),
            "subscription:trial:ended" => Ok(Self::TrialEnded),
            "subscription:trial:started:product" => Ok(Self::TrialStartedProduct),
            "subscription:trial:ended:product" => Ok(Self::TrialEndedProduct),
            "subscription:created:product" => Ok(Self::CreatedProduct),
            "subscription:updated:product" => Ok(Self::UpdatedProduct),
            "subscription:renewed:product" => Ok(Self::RenewedProduct),
            "subscription:cancelled:product" => Ok(Self::CancelledProduct),
            "subscription:upcoming:product" => Ok(Self::UpcomingProduct),
            _ => Err(Self::Err {}),
        }
    }

    type Err = ParseSubscriptionError;
}