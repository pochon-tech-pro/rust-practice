#[derive(Debug)]
pub struct OrderId(u32);

impl From<OrderId> for u32 {
    fn from(order_id: OrderId) -> Self {
        order_id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_into() {
        let order_id = OrderId(100);
        let id: u32 = order_id.into(); // 型を指定した変数に格納する必要がある。into()の返り値は resultの型に依存するため、推論できない。
        assert_eq!(100, id, "Unit Test Failed . : id = {}", id);
    }
}