use std::error::Error;

type Undefined = dyn Error;

struct AcknowledgementSent();
struct OrderPlaced();
struct BillingAddress();
struct BillingOrderPlaced();

struct PlaceOrderEvent {
    acknowledgement_sent: AcknowledgementSent,
    order_placed: OrderPlaced,
    billing_address: BillingAddress,
    billing_order_placed: BillingOrderPlaced,
}

struct EnvelopeContents(String);
struct QuoteForm();
struct OrderForm();
enum CategorisedMail {
    Quote(QuoteForm),
    Order(OrderForm),
}

// fn categorise_inbound_mail(envelope: EnvelopeContents) -> CategorisedMail {}
struct ProductCatalog();

struct CalculatePricesInput {
    order_form: OrderForm,
    product_catalog: ProductCatalog,
}
