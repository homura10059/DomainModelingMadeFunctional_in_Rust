struct Item {}

struct ActiveCartData {
    unpaid_items: Vec<Item>,
}

struct PaidCartData {
    paid_items: Vec<Item>,
    payment: f64,
}

enum ShoppingCart {
    EmptyCart,
    ActiveCart(ActiveCartData),
    PaidCart(PaidCartData),
}

fn add_cart(cart: ShoppingCart, item: Item) -> ShoppingCart {
    match cart {
        ShoppingCart::EmptyCart => {
            let mut items = Vec::new();
            items.push(item);
            ShoppingCart::ActiveCart(ActiveCartData {
                unpaid_items: items,
            })
        }
        ShoppingCart::ActiveCart(mut data) => {
            data.unpaid_items.push(item);
            ShoppingCart::ActiveCart(data)
        }
        ShoppingCart::PaidCart(data) => ShoppingCart::PaidCart(data),
    }
}

fn make_payment(cart: ShoppingCart, payment: f64) -> ShoppingCart {
    match cart {
        ShoppingCart::EmptyCart => ShoppingCart::EmptyCart,
        ShoppingCart::ActiveCart(data) => ShoppingCart::PaidCart(PaidCartData {
            paid_items: data.unpaid_items,
            payment,
        }),
        ShoppingCart::PaidCart(data) => ShoppingCart::PaidCart(data),
    }
}
