struct CustomerId(i64);
struct OrderId(i64);

fn process_customer_id(id: &CustomerId) -> String {
    let inner_val = id.0; // 内部的な値を取り出して使うこともできる
    format!("id:{}", inner_val)
}

fn sample() {
    let id = OrderId(42);
    // process_customer_id(&id); // ここでエラーになる
}
