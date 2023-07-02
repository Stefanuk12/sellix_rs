// Dependencies
use serde::{Serialize, Deserialize};

/// All of the dispute reasons a customer could file.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum DisputeReason {
    /// The customer did not receive the merchandise or service.
    MerchandiseOrServiceNotReached,
    /// The customer reports that the merchandise or service is not as described.
    MerchandiseOrServiceNotAsDescribed,
    /// The customer did not authorize purchase of the merchandise or service.
    Unauthorised,
    /// The refund or credit was not processed for the customer.
    CreditNotProcessed,
    /// The transaction was a duplicate.
    DuplicateTransaction,
    /// The customer was charged an incorrect amount.
    IncorrectAmount,
    /// The customer paid for the transaction through other means.
    PaymentByOtherMeans,
    /// The customer was being charged for a subscription or a recurring transaction that was canceled.
    CanceledRecurringBilling,
    /// A problem occurred with the remittance.
    ProblemWithRemittance,
    /// Other.
    Other
}

/// The status of a Dispute.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum DisputeStatus {
    /// The dispute is open.
    Open,
    /// The dispute is waiting for a response from the customer.
    WaitingForBuyerResponse,
    /// The dispute is waiting for a response from the merchant.
    WaitingForSellerResponse,
    /// The dispute is under review with PayPal.
    UnderReview,
    /// The dispute is resolved.
    Resolved,
    /// The default status if the dispute does not have one of the other statuses.
    Other
}

/// The outcome of a Dispute.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum DisputeOutcome {
    /// The dispute was resolved in the customer's favor.
    ResolvedBuyerFavour,
    /// The dispute was resolved in the merchant's favor.
    ResolvedSellerFavour,
    /// PayPal provided the merchant or customer with protection and the case is resolved.
    ResolvedWithPayout,
    /// The customer canceled the dispute.
    CanceledByBuyer,
    /// PayPal accepted the dispute.
    Accepted,
    /// PayPal denied the disute.
    Denied,
    /// A dispute was created for the same transaction ID, and the previous dispute was closed without any decision.   
    None
}

/// The current life cycle state of a Dispute.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum DisputeStage {
    /// A customer and merchant interact in an attempt to resolve a dispute without escalation to PayPal.
    /// 
    /// Occurs when the customer: 
    /// - Has not received goods or a service. 
    /// - Reports that the received goods or service are not as described. 
    /// - Needs more details, such as a copy of the transaction or a receipt.
    Inquiry,
    /// A customer or merchant escalates an inquiry to a claim, which authorizes PayPal to investigate the case and make a determination.
    /// Occurs only when the dispute channel is `INTERNAL`. 
    /// 
    /// This stage is a PayPal dispute lifecycle stage and not a credit card or debit card chargeback.
    /// All notes that the customer sends in this stage are visible to PayPal agents only.
    /// The customer must wait for PayPal's response before the customer can take further action.
    /// In this stage, PayPal shares dispute details with the merchant, who can complete one of these actions:
    /// - Accept the claim.
    /// - Submit evidence to challenge the claim.
    /// - Make an offer to the customer to resolve the claim.
    Chargeback,
    /// The first appeal stage for merchants.
    /// A merchant can appeal a chargeback if PayPal's decision is not in the merchant's favor.
    /// If the merchant does not appeal within the appeal period, PayPal considers the case resolved.
    PreArbitration,
    /// The first appeal stage for merchants.
    /// A merchant can appeal a chargeback if PayPal's decision is not in the merchant's favor.
    /// If the merchant does not appeal within the appeal period, PayPal considers the case resolved.
    Arbitration
}

/// Describes who posted a message within the dispute.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum DisputeMessage {
    /// The customer posted the message.
    Buyer,
    /// The merchant posted the message.
    Seller,
    /// The arbiter of the dispute posted the message.
    Arbiter
}