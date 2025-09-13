// Export all payment strategies
pub mod apple_pay;
pub mod wechat_pay;
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

pub use ctor;
pub use paste;
// ==================== 1. Strategy Trait ====================
pub trait PaymentStrategy: 'static {
    fn name(&self) -> &'static str;
    fn pay(&self, amount: f64) -> String;
}

// ==================== 2. Global Strategy Storage ====================
type StrategyConstructor = fn() -> Box<dyn PaymentStrategy>;

pub static STRATEGIES: LazyLock<Mutex<HashMap<&'static str, StrategyConstructor>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

// ==================== 3. Macro Definition ====================
#[macro_export]
macro_rules! define_strategy {
    ($struct_name:ident, $name:expr, $pay_fn:expr) => {
        pub struct $struct_name;

        impl $crate::strategies::PaymentStrategy for $struct_name {
            fn name(&self) -> &'static str {
                $name
            }
            fn pay(&self, amount: f64) -> String {
                $pay_fn(self, amount)
            }
        }
        $crate::strategies::paste::paste! {
            // Auto-register using ctor
            #[$crate::strategies::ctor::ctor]
            fn [<register_strategy_$struct_name:lower>]() {
                let mut map = $crate::strategies::STRATEGIES.lock().unwrap();
                map.insert($name, || Box::new($struct_name));
            }
        }
    };
}
