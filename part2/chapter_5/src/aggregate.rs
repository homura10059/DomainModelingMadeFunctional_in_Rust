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
    total_price: Price,
}

fn change_order_line_price(order: &Order, order_line_id: &OrderLineId, new_price: &Price) -> Order {
    let order_line_pos = order
        .order_lines
        .iter()
        .position(|line| line.order_line_id == *order_line_id)
        .unwrap(); // ほんとはエラー処理必要

    let order_line = order.order_lines.get(order_line_pos).unwrap().clone();

    let new_order_line = OrderLine {
        price: new_price.clone(),
        ..order_line
    };

    let mut new_order_lines = order.order_lines.clone();

    let _old_order_line = std::mem::replace(&mut new_order_lines[order_line_pos], new_order_line);

    let new_total_price = new_order_lines
        .iter()
        .map(|line| line.price.0 * line.quantity.0)
        .sum::<i64>();

    Order {
        order_lines: new_order_lines,
        total_price: Price(new_total_price),
        // ..order.clone() // 他の要素があるなら必要
    }
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
            total_price: Price(2_000),
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
            total_price: Price(3_000),
        };

        let actual = change_order_line_price(&order, &OrderLineId(1), &Price(2_000));

        assert_eq!(actual, expected)
    }
}
