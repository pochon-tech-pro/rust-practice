#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct OrderId(u32);

impl From<OrderId> for u32 {
    fn from(order_id: OrderId) -> Self {
        order_id.0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Order {
    id: OrderId,
}

impl Order {
    pub fn new(id: OrderId) -> Order {
        Order { id }
    }
    pub fn id(&self) -> OrderId {
        self.id // OrderId型にCloneとCopyトレイトを継承させておく必要あり（Moveするので）
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

    #[test]
    fn create_order_new() {
        let expect = Order { id: OrderId(100) };
        let actual = Order::new(OrderId(100));
        // Order構造体にDebugトレイトを継承せないとAssertできないので注意
        // assert_eqでOrder構造体とその属性OrderIdの比較を行うので、EqとPartialEqを継承する必要がある
        assert_eq!(expect, actual, "Unit Test Failed . : actual = {:?}", actual);
    }
}