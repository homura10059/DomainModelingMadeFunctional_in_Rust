#[derive(Debug, PartialEq)]
struct ProductCode(i64);

pub trait CheckProductCodeExists {
    fn check_product_code_exists(product_code: &ProductCode) -> bool;
}
