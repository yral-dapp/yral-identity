use ic_agent::identity::{Delegation, SignedDelegation};
use ic_types::{
    messages::{Delegation as IcDelegation, SignedDelegation as IcSignedDelegation},
    CanisterId, PrincipalId,
};

fn into_ic_delegation(del: Delegation) -> IcDelegation {
    let pubkey = del.pubkey;
    let expiration = ic_types::Time::from_nanos_since_unix_epoch(del.expiration);
    let Some(agent_targets) = del.targets else {
        return IcDelegation::new(pubkey, expiration);
    };
    let targets = agent_targets
        .into_iter()
        .map(|p| CanisterId::unchecked_from_principal(PrincipalId(p)))
        .collect();
    IcDelegation::new_with_targets(pubkey, expiration, targets)
}

pub fn into_ic_signed_delegation(sdel: SignedDelegation) -> IcSignedDelegation {
    let ic_delegation = into_ic_delegation(sdel.delegation);
    IcSignedDelegation::new(ic_delegation, sdel.signature)
}
