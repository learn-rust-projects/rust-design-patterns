// Define WechatPay strategy using macro
crate::define_strategy!(WechatPay, "wechat_pay", |_, amount| format!(
    "WeChat Pay: {} yuan",
    amount
));
