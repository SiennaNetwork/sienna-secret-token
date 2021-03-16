#![allow(non_snake_case)]
#[macro_use] extern crate kukumba;
#[macro_use] extern crate sienna_schedule; use sienna_schedule::{Schedule,Vesting};
#[macro_use] mod helpers; use helpers::{harness, mock_env, tx};

use cosmwasm_std::{StdError, HumanAddr, Uint128};
use sienna_mgmt::{PRELAUNCH, NOTHING};

kukumba!(

    #[claim_minting_pool]

    given "a contract with the production schedule" {
        harness!(deps; ADMIN);
        let s: Schedule = serde_json::from_str(include_str!("../../config.json")).unwrap();
        test_tx!(deps, ADMIN, 0, 0;
            Configure { portions: s.all().unwrap() } => tx_ok!());

        let lpf  = HumanAddr::from("secret1TODO50xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        let rem1 = HumanAddr::from("secret1TODO51xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        let rem2 = HumanAddr::from("secret1TODO52xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        let rem3 = HumanAddr::from("secret1TODO53xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    }

    when "the liquidity provision fund is claimed before launch"
    then "it is not transferred" {
        test_tx!(deps, lpf, 1, 1;
            Claim {} => tx_err!(PRELAUNCH));
    }
    when "the remaining pool tokens are claimed before launch"
    then "they are not transferred" {
        for rem in [&rem1, &rem2, &rem3].iter() {
            test_tx!(deps, *rem, 1, 1;
                Claim {} => tx_err!(PRELAUNCH));
        }
    }

    when "the contract is launched" {
        let t_launch = 2*86400u64;
        test_tx!(deps, ADMIN, 2, t_launch;
            Launch {} => tx_ok_launch!(s.total));
    }
    and "the liquidity provision fund is claimed"
    then "it is received in full" {
        test_tx!(deps, lpf, 3, t_launch + 1;
            Claim {} => tx_ok_claim!(lpf, SIENNA!(300000u128)));
    }
    and "the liquidity provision fund is claimed again"
    then "nothing more is transfered" {
        test_tx!(deps, lpf, 3, t_launch + 1;
            Claim {} => tx_err!(NOTHING));
    }

    when "the remaining pool tokens are claimed"
    then "the corresponding portion of them is transferred" {
        println!("{:#?}", &s)
        test_tx!(deps, rem1, 3, t_launch + 1;
            Claim {} => tx_ok_claim!(rem1, SIENNA!(1250u128)));
        test_tx!(deps, rem2, 4, t_launch + 86400;
            Claim {} => tx_ok_claim!(rem2, SIENNA!(2*750u128)));
    }

    //when "the remaining pool tokens are claimed late"
    //then "the corresponding portion of them is transferred" {
        //test_tx!(deps, rem3, 6, t_launch + 86400 + 3*86400;
            //Claim {} => tx_ok_claim!(rem1, SIENNA!(4*1500u128)));
    //}

    //when "more remaining pool tokens are claimed"
    //then "the corresponding portion of them is transferred" {
        //test_tx!(deps, rem1, 6, t_launch + 86400 + 4*86400;
            //Claim {} => tx_ok_claim!(rem1, SIENNA!(2500u128)));
        //test_tx!(deps, rem2, 7, t_launch + 86400 + 4*86400 + 1;
            //Claim {} => tx_ok_claim!(rem2, SIENNA!(1500u128)));
        //test_tx!(deps, rem3, 8, t_launch + 86400 + 4*86400 + 2;
            //Claim {} => tx_ok_claim!(rem3, SIENNA!(500u128)));
    //}

    //when "the allocations of remaining pool tokens are changed"
    //then "subsequent claims use the new allocations" {
        //test_tx!(deps, ADMIN, 9, t_launch + 4*86400 + 3;
            //Reallocate {
                //pool_name: "Minting Pool".to_string(),
                //channel_name: "PoolRem".to_string(),
                //allocations: vec![
                    //allocation( 900, &rem1),
                    //allocation(1000, &rem2),
                    //allocation(1100, &rem3)
                //] } => tx_ok!());
        //test_tx!(deps, rem3, 6, t_launch + 86400 + 5*86400;
            //Claim {} => tx_ok_claim!(rem3, SIENNA!(1100u128)));
        //test_tx!(deps, rem2, 6, t_launch + 86400 + 5*86400 + 10;
            //Claim {} => tx_ok_claim!(rem2, SIENNA!(1000u128)));
        //test_tx!(deps, rem1, 6, t_launch + 86400 + 5*86400 + 20;
            //Claim {} => tx_ok_claim!(rem1, SIENNA!( 900u128)));
    //}

);