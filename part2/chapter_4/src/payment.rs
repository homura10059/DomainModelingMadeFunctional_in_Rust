type CheckNumber = i32;
type CardNumber = String;

enum CardType {
    Visa,
    MasterCard,
}

struct CreditCardInfo {
    card_type: CardType,
    card_number: CardNumber,
}

enum PaymentMethod {
    Cash,
    Check(CheckNumber),
    Card(CreditCardInfo),
}

type PaymentAmount = i64;
enum Currency {
    EUR,
    USD,
}

struct Payment {
    amount: PaymentAmount,
    currency: Currency,
    method: PaymentMethod,
}

// PayInvoice = UnpaidInvoice -> Payment -> PaidInvoice
// ConvertPaymentCurrency = Payment -> Currency -> Payment
