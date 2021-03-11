//! # History and reconfiguration
//!
//! TODO.
//!
//! Currently, this module implements some ways of mutating the schedule.
//! What needs to happen instead is somewhat different.
//!
//! * New `Portions` generated by updated `Schedule` compared against old
//!   `Portions` and an error returned if any incompatibility is encountered.
//!   (I'm thinking of making the `Pool`s always fixed size)
//! * More importantly, new `Portions` compared against history of
//!   `ClaimedPortion`s. If the update would result in reducing the balance of
//!   a claimant who has already received their funds, it is rejected.

use crate::*;

/// Log of executed claims
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct History {
    pub history: Vec<ClaimedPortion>
}
impl History {
    pub fn new () -> Self { Self { history: vec![] } }
    /// Takes list of portions, returns the ones which aren't marked as claimed
    pub fn unclaimed (
        &self,
        claimable: &Portions
    ) -> Portions {
        // TODO throw if monotonicity of time is violated in eiter collection
        let claimed_portions: Vec<_> =
            self.history.iter().map(|claimed| claimed.portion.clone()).collect();
        claimable.iter()
            .filter(|p|{!claimed_portions.contains(p)})
            .map(|p|p.clone())
        .collect()
    }
    /// Marks one or more portions as claimed. This is irreversible.
    pub fn claim (
        &mut self,
        claimed: Seconds,
        portions: Portions
    ) {
        for portion in portions.iter() {
            self.history.push(ClaimedPortion {
                claimed,
                portion: portion.clone()
            })
        }
    }
    /// Validates a proposed update to the schedule
    pub fn validate_schedule_update<'a> (
        &self,
        old: &Portions,
        new: &Portions
    ) -> UsuallyOk {
        Ok(())
    }
}

/// History entry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ClaimedPortion {
    portion: Portion,
    claimed: Seconds
}

impl Pool {
    pub fn add_account (&mut self, ch: Account) -> UsuallyOk {
        ch.validate()?;
        self.validate()?;
        let allocated = self.accounts_total()?;
        let unallocated = self.total.u128() - allocated;
        if ch.total.u128() > unallocated {
            return Self::err_too_big(
                &self.name, ch.total.u128(), unallocated, self.total.u128()
            )
        }
        self.accounts.push(ch);
        Ok(())
    }
    define_errors!{
        err_too_big (name: &str, amount: u128, unallocated: u128, total: u128) ->
            ("pool {}: tried to add account with size {}, which is more than the remaining {} of this pool's total {}",
                name, amount, unallocated, total)}
}

impl Account {
    /*/// Allocations can be changed on the fly without affecting past vestings.
    /// FIXME: Allocations are timestamped with real time
    ///        but schedule measures time from `t_launch=0`
    ///        and allocations are realized in `Periodic`s
    ///        which only start measuring time at `start_at`
    ///        For now, reallocations should be forbidden
    ///        before the launch because trying to timestamp
    ///        an allocation would cause an underflow?
    pub fn reallocate (&mut self, t: Seconds, c: AccountConfig) -> UsuallyOk {
        //c.validate()?;
        for (t2, _) in self.config.iter() {
            if t < *t2 {
                return Self::err_realloc_time_travel(&self.name, t, *t2)
            }
        }
        self.config.push((t, c));
        self.validate()
    }*/
    define_errors!{
        err_realloc_cliff (name: &str) ->
            ("account {}: reallocations for accounts with cliffs are not supported",
                name)
        err_realloc_time_travel (name: &str, t: Seconds, t_max: Seconds) ->
            ("account {}: can not reallocate in the past ({} < {})",
                name, t, t_max)}
}