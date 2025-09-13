// Define ApplePay strategy using macro
crate::define_strategy!(ApplePay, "apple_pay", |_, amount| format!(
    "Apple Pay: {} yuan",
    amount
));
