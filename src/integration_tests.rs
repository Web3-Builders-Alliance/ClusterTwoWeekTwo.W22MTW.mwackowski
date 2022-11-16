#[cfg(test)]
mod tests {
    use crate::helpers::MessagesContract;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, MessagesResponse};
    use cosmwasm_std::{Addr, Coin, Empty, Uint128, to_binary, BankQuery, BankMsg, coin};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};


    pub fn contract_messages() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const USER1: &str = "juno10c3slrqx3369mfsr9670au22zvq082jaej8ve4";
    const USER2: &str = "juno1and87527ua866yqh2mpakl9zkxzj5myu6f87ll";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER1),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1000000),
                    }],
                )
                .unwrap();
        })
    }

    fn send_coins_to_user2(app: &mut App) {
        app.send_tokens(Addr::unchecked(USER1), Addr::unchecked(USER2), &vec![coin(1000, NATIVE_DENOM)]);
    }

    fn store_code() -> (App, u64) {
        let mut app = mock_app();
        let messages_id = app.store_code(contract_messages());
        (app, messages_id)
    }

    fn messages_contract(app: &mut App, deposit_id: u64) -> MessagesContract {
        let msg = InstantiateMsg {};
        let messages_contract_address = app
            .instantiate_contract(
                deposit_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "messages",
                None,
            )
            .unwrap();
        MessagesContract(messages_contract_address)
    }

    fn get_all_messages(app: &App, messages_contract: &MessagesContract) -> MessagesResponse {
        app.wrap()
            .query_wasm_smart(messages_contract.addr(), &QueryMsg::GetAllMessage {})
            .unwrap()
    }

    fn get_balance(app: &App, user:String, denom:String) -> Coin {
        app.wrap().query_balance(user, denom).unwrap()
    }

    fn add_message(app: &mut App, messages_contract: &MessagesContract, owner:Addr, topic:String, message:String) {
        //use ExecuteMsg to add a message
        //use app.execute_contract to send message to contract
    }

    #[test]
    fn add_two_messages_and_query_all_messages() {

    }

    #[test]
    fn add_messages_from_two_owners_and_query_messages_by_owner() {

    }

    #[test]
    fn add_messages_from_two_owners_and_query_messages_by_topic() {

    }

    #[test]
    fn add_two_messages_and_query_messages_by_id() {
    
    }

}
