use strategy_stateless::{OrderService, OrderStaticService, StrategyFactory, StrategyType};
fn main() {
    //dynamic dispatch
    println!("==========dynamic dispatch==========");
    let strategies: Vec<OrderService> = vec![
        OrderService::new(StrategyFactory::get_strategy(StrategyType::Ali)),
        OrderService::new(StrategyFactory::get_strategy(StrategyType::WeChat)),
    ];
    for strategy in strategies {
        strategy.checkout(88.88);
    }
    //static dispatch
    println!("==========static dispatch==========");
    let we_chat_pay = OrderStaticService::new(StrategyFactory::get_we_chat_pay());
    we_chat_pay.checkout(88.88);
}
