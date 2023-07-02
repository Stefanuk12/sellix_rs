// Dependencies
use serde::{Serialize, Deserialize};

/// Represents the current status of an invoice.
/// Used in [`InvoiceRaw`].
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum InvoiceStatus {
    Pending,
    Completed,
    Voided,
    WaitingForConfirmations,
    Partial,
    CustomerDisputeOngoing,
    Reversed,
    Refunded,
    WaitingShopAction,
    Processing
}

/// Adds more information onto [`InvoiceStatus`].
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum InvoiceStatusDetails {
    CartPartialOutOfStock    
}

/// Adds more information onto [`InvoiceStatus`], if it was `VOIDED`.
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum InvoiceVoidDetails {
    CartProductsOutOfStock    
}