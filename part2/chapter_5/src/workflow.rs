use std::error::Error;

type Undefined = dyn Error;

type AcknowledgementSent = Undefined;
type OrderPlaced = Undefined;
type BillingOrderPlaced = Undefined;

struct PlaceOrderEvent {
    acknowledgement_sent: AcknowledgementSent,
    order_placed: OrderPlaced,
    billing_address: BillingAddress,
    billing_order_placed: BillingOrderPlaced,
}

struct EnvelopeContents(String);
type QuoteForm = Undefined;
type OrderForm = Undefined;
enum CategorisedMail {
    Quote(QuoteForm),
    Order(OrderForm),
}

// fn categorise_inbound_mail(envelope: EnvelopeContents) -> CategorisedMail {}
type ProductCatalog = Undefined;

struct CalculatePricesInput {
    order_form: OrderForm,
    product_catalog: ProductCatalog,
}
