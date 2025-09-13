use crate::strategy::{AliPay, PaymentStrategy, WeChatPay};
pub enum StrategyType {
    WeChat,
    Ali,
}

pub struct StrategyFactory;

impl StrategyFactory {
    pub fn get_strategy(t: StrategyType) -> Box<dyn PaymentStrategy> {
        match t {
            StrategyType::WeChat => Box::new(WeChatPay),
            StrategyType::Ali => Box::new(AliPay),
        }
    }

    pub fn get_we_chat_pay() -> WeChatPay {
        WeChatPay
    }
}
