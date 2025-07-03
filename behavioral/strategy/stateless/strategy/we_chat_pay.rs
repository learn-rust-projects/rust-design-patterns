use super::PaymentStrategy;
pub struct WeChatPay;

impl PaymentStrategy for WeChatPay {
    fn pay(&self, amount: f64) {
        println!("Paying：¥{amount:.2} with WeChat Pay");
    }
}
