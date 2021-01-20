use cosmwasm_std::testing::{mock_dependencies_with_balances, mock_env};
use cosmwasm_std::{coins, from_binary, StdResult, StdError, HumanAddr};

use sienna_mgmt as mgmt;

#[test] fn init () {
    let mut deps = mock_dependencies_with_balances(20, &[
        (&HumanAddr::from("Alice"),   &coins(1000, "SIENNA")),
        (&HumanAddr::from("Bob"),     &coins(1000, "SIENNA")),
        (&HumanAddr::from("Mallory"), &coins(   0, "SIENNA"))
    ]);

    // As the contract owner
    // When I init the contract
    // Then I want to be able to query its state
    let res = mgmt::init(
        &mut deps,
        mock_env("Alice", &coins(1000, "SIENNA")),
        mgmt::msg::InitMsg { token: None }
    ).unwrap();
    assert_eq!(0, res.messages.len());
    let res: mgmt::msg::StatusResponse = from_binary(
        &mgmt::query(&deps, mgmt::msg::QueryMsg::StatusQuery {}).unwrap()
    ).unwrap();
    assert_eq!(res.launched, None)
}

#[test] fn launch () {
    let mut deps = mock_dependencies_with_balances(20, &[
        (&HumanAddr::from("Alice"),   &coins(1000, "SIENNA")),
        (&HumanAddr::from("Bob"),     &coins(1000, "SIENNA")),
        (&HumanAddr::from("Mallory"), &coins(   0, "SIENNA"))
    ]);
    let res = mgmt::init(
        &mut deps,
        mock_env("Alice", &coins(1000, "SIENNA")),
        mgmt::msg::InitMsg { token: None }
    ).unwrap();
    assert_eq!(0, res.messages.len());

    // As Joe Shmoe
    // When I try to launch the contract
    // Then I should fail
    let env = mock_env("Mallory", &coins(1000, "SIENNA"));
    let time = env.block.time;
    let _ = mgmt::handle(&mut deps, env, mgmt::msg::HandleMsg::Launch {});
    let value: mgmt::msg::StatusResponse = from_binary(
        &mgmt::query(&deps, mgmt::msg::QueryMsg::StatusQuery {}).unwrap()
    ).unwrap();
    assert_eq!(value.launched, None);

    // As the contract owner
    // When I launch the contract
    // Then it should remember when it was first launched
    let env = mock_env("Alice", &coins(1000, "SIENNA"));
    let time = env.block.time;
    let _ = mgmt::handle(&mut deps, env, mgmt::msg::HandleMsg::Launch {});
    let value: mgmt::msg::StatusResponse = from_binary(
        &mgmt::query(&deps, mgmt::msg::QueryMsg::StatusQuery {}).unwrap()
    ).unwrap();
    assert_eq!(value.launched, Some(time));
    let env = mock_env("Alice", &coins(1000, "SIENNA"));
    let time2 = env.block.time;
    let _ = mgmt::handle(&mut deps, env, mgmt::msg::HandleMsg::Launch {});
    let value: mgmt::msg::StatusResponse = from_binary(
        &mgmt::query(&deps, mgmt::msg::QueryMsg::StatusQuery {}).unwrap()
    ).unwrap();
    assert_eq!(value.launched, Some(time));
}
