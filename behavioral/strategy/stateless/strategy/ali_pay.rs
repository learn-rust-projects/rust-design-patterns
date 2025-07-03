use super::PaymentStrategy;
pub struct AliPay;

impl PaymentStrategy for AliPay {
    fn pay(&self, amount: f64) {
        println!("Paying {amount:.2} with Alipay");
    }
}
