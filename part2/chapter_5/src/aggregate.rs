#[derive(Debug, PartialEq, Clone)]
struct Quantity(i64);

#[derive(Debug, PartialEq, Clone)]
struct Price(i64);

#[derive(Debug, PartialEq, Clone)]
struct OrderLineId(i64);

#[derive(Debug, PartialEq, Clone)]
struct ProductCode(String);

#[derive(Debug, PartialEq, Clone)]
struct Product {
    product_code: ProductCode,
}

#[derive(Debug, PartialEq, Clone)]
struct OrderLine {
    order_line_id: OrderLineId,
    product: Product,
    quantity: Quantity,
    price: Price,
}

#[derive(Debug, PartialEq, Clone)]
struct Order {
    order_lines: Vec<OrderLine>,
}

fn change_order_line_price(order: &Order, order_line_id: &OrderLineId, new_price: &Price) -> Order {
    let order_line = order
        .order_lines
        .iter()
        .find(|line| line.order_line_id == order_line_id.to_owned())
        .unwrap();
    let new_order_line = OrderLine {
        price: new_price.clone(),
        ..order_line.clone()
    };
    let mut new_order_lines = order
        .order_lines
        .clone()
        .iter()
        .filter(|line| line.order_line_id != order_line_id.to_owned())
        .map(|line| line.to_owned())
        .collect::<Vec<_>>();

    new_order_lines.push(new_order_line);
    new_order_lines.sort_by(|a, b| a.order_line_id.0.partial_cmp(&b.order_line_id.0).unwrap());
    let new_order = Order {
        order_lines: new_order_lines,
        ..order.clone()
    };

    new_order
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_change_order_line_price() {
        let order_line = OrderLine {
            order_line_id: OrderLineId(1),
            product: Product {
                product_code: ProductCode("W-123".to_owned()),
            },
            quantity: Quantity(1),
            price: Price(1_000),
        };
        let order = Order {
            order_lines: vec![
                OrderLine {
                    order_line_id: OrderLineId(1),
                    ..order_line.clone()
                },
                OrderLine {
                    order_line_id: OrderLineId(2),
                    ..order_line.clone()
                },
            ],
        };

        let expected = Order {
            order_lines: vec![
                OrderLine {
                    order_line_id: OrderLineId(1),
                    price: Price(2_000),
                    ..order_line.clone()
                },
                OrderLine {
                    order_line_id: OrderLineId(2),
                    ..order_line.clone()
                },
            ],
        };

        let actual = change_order_line_price(&order, &OrderLineId(1), &Price(2_000));

        assert_eq!(actual, expected)
    }
}
