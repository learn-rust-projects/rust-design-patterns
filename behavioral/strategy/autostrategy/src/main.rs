use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use paste::paste;
// ==================== 1. Strategy Trait ====================
pub trait PaymentStrategy: 'static {
    fn name(&self) -> &'static str;
    fn pay(&self, amount: f64) -> String;
}

// ==================== 2. Global Strategy Storage ====================
type StrategyConstructor = fn() -> Box<dyn PaymentStrategy>;

static STRATEGIES: LazyLock<Mutex<HashMap<&'static str, StrategyConstructor>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

// ==================== 3. Macro Definition ====================
#[macro_export]
macro_rules! define_strategy {
    ($struct_name:ident, $name:expr, $pay_fn:expr) => {
        pub struct $struct_name;

        impl PaymentStrategy for $struct_name {
            fn name(&self) -> &'static str {
                $name
            }
            fn pay(&self, amount: f64) -> String {
                $pay_fn(self, amount)
            }
        }
        paste! {
            // Auto-register using ctor
            #[ctor::ctor]
            fn [<register_strategy_$struct_name:lower>]() {
                let mut map = STRATEGIES.lock().unwrap();
                map.insert($name, || Box::new($struct_name));
            }
        }
    };
}

// ==================== 4. Define Strategies Using Macro ====================
define_strategy!(ApplePay, "apple_pay", |_, amount| format!(
    "Apple Pay: {} yuan",
    amount
));

define_strategy!(WechatPay, "wechat_pay", |_, amount| format!(
    "WeChat Pay: {} yuan",
    amount
));

// ==================== 5. Main Function ====================
fn main() {
    for (name, constructor) in STRATEGIES.lock().unwrap().iter() {
        let strategy = constructor();
        println!("Using {}: {}", name, strategy.pay(100.0));
    }
}
