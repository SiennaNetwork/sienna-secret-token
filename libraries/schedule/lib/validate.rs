//! # Input validation
//!
//! The `Schedule`, `Pool`, and `Account` structs implement the `Validation` trait, which
//! provides a `validate` method on top of the implicit schema validation provided by Serde.
//!
//! Unfortunately, `rustdoc` does not allow for the `impl`s that are defined
//! in this module to be rendered on this doc page, because they implement
//! `struct`s defined in another file.
//!
//! Documentation of the methods (and errors) defined in this file
//! can be found in the documentation for those structs.

use crate::*;

/// Trait for something that undergoes validation, returning `Ok` or an error.
pub trait Validation {
    /// Default implementation is a no-op
    fn validate (&self) -> UsuallyOk { Ok(()) }
}
impl<A:Validation> Validation for Vec<A> {
    fn validate (&self) -> UsuallyOk {
        for item in self.iter() {
            item.validate()?
        }
        Ok(())
    }
}
impl<A:Clone> Validation for Schedule<A> {
    /// Schedule must contain valid pools that add up to the schedule total
    fn validate (&self) -> UsuallyOk {
        self.pools.validate()?;
        if self.subtotal() != self.total.u128() {
            return self.err_total()
        }
        Ok(())
    }
}
impl<A:Clone> Validation for Pool<A> {
    fn validate (&self) -> UsuallyOk {
        self.accounts.validate()?;
        let invalid_total = if self.partial {
            self.subtotal() > self.total.u128()
        } else {
            self.subtotal() != self.total.u128()
        };
        if invalid_total { return self.err_total() }
        Ok(())
    }
}
impl<A:Clone> Validation for Account<A> {
    fn validate (&self) -> UsuallyOk {
        if self.amount == Uint128::zero() {
            return self.err_empty()
        }
        if self.cliff > self.amount {
            return self.err_cliff_too_big()
        }
        if self.amount.u128() != (
            self.cliff.u128() +
            self.portion_size() * self.portion_count() as u128 +
            self.remainder()
        ) {
            return self.err_does_not_add_up()
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use cosmwasm_std::HumanAddr;
    use crate::{Schedule, Pool, Account, validate::Validation};
    #[test] fn test_amount_eq_zero () {
        let A = Account::periodic("A", &HumanAddr::from(""), 0, 0, 0, 0, 0);
        assert_eq!(A.validate(),
                   A.err_empty());
        assert_eq!(Schedule::new(&[Pool::full("P", &[A.clone()])]).validate(),
                   A.err_empty());
        assert_eq!(Schedule::new(&[Pool::partial("P", 0, &[A.clone()])]).validate(),
                   A.err_empty());
        assert_eq!(Schedule::new(&[Pool::partial("P", 1, &[A.clone()])]).validate(),
                   A.err_empty());
    }
    #[test] fn test_cliff_gt_amount () {
        let A = Account::periodic("A", &HumanAddr::from(""), 1, 2, 0, 0, 0);
        assert_eq!(A.validate(),
                   A.err_cliff_too_big());
        assert_eq!(Schedule::new(&[Pool::full("P", &[A.clone()])]).validate(),
                   A.err_cliff_too_big());
        assert_eq!(Schedule::new(&[Pool::partial("P", 0, &[A.clone()])]).validate(),
                   A.err_cliff_too_big());
        assert_eq!(Schedule::new(&[Pool::partial("P", 1, &[A.clone()])]).validate(),
                   A.err_cliff_too_big());
    }
    #[test] fn test_account_gt_pool () {
        let A = Account::periodic("A", &HumanAddr::from(""), 2, 0, 0, 0, 0);
        let P = Pool{
            partial:  false,
            name:     "P".to_string(),
            total:    1u128.into(),
            accounts: vec![A.clone()],
        };
        let S = Schedule::new(&[P.clone()]);
        assert_eq!(A.validate(),
                   Ok(()));
        assert_eq!(S.validate(),
                   P.err_total());
        assert_eq!(Schedule::new(&[Pool::partial("P", 1, &[A.clone()])]).validate(),
                   P.err_total());
    }
    #[test] fn test_pools_lt_schedule () {
        let S: Schedule<HumanAddr> = Schedule {
            total: 1u128.into(),
            pools: vec![]
        };
        assert_eq!(S.validate(),
                   S.err_total());
    }
    #[test] fn test_pools_gt_schedule () {
        let A = Account::periodic("A", &HumanAddr::from(""), 2, 0, 0, 0, 0);
        let S = Schedule {
            total: 1u128.into(),
            pools: vec![Pool::partial("P1", 1, &[]), Pool::full("P2", &[A])]
        };
        assert_eq!(S.validate(),
                   S.err_total());
    }
}
