// This function will check all locks for a specific round for Zephyrus
// Together with the voting power
// We will store everything in our state.
// The round should be in the past, so it cannot change
fn fill_vessel_hydromancer_voting_power(
    deps: DepsMut<NeutronQuery>,
    info: MessageInfo,
    round_id: u64,
) -> Result<Response<NeutronMsg>, ContractError> {

    // Query Hydro contract (this query does not exist yet)
    // If there's a start_from / limit, we need to make sure we browse all

    // HydroQueryMsg::UserVotesWithLockPower {
    //     round_id,
    //     tranche_id,
    //     address,
    // } -> UserVotesWithLockPowerResponse

    // pub struct UserVotesWithLockPowerResponse {
    //     pub votes: Vec<VoteWithLockPower>,
    // }

    // The voting power here, should be time-weighted-shares * validator_power_ratio
    // pub struct VoteWithLockPower {
    //     pub prop_id: u64,
    //     pub lock_id: u64,
    //     pub power: Decimal,
    // }

    // Loop over all locks
    // 1. For each lock, add the voting power to the vessel / round
    // 2. Then, mark as completed so we don't do it again
    // 3. Also, add the VP of all the vessels controlled by a hydromancer into a hydromancer_vp structure per round
}

fn claim_hydro_rewards(
    deps: DepsMut<NeutronQuery>,
    info: MessageInfo,
    round_id: u64,
) -> Result<Response<NeutronMsg>, ContractError> {

    // 1. Need to check that the fill_vessel_voting_power has already been run for that round
    // If not, run it first

    // 2. List the tranches

    // 3. For each tranche, check we have something left to claim, by using
    // TributeQueryMsg::OutstandingTributeClaims {
    //     user_address,
    //     round_id,
    //     tranche_id,
    //     start_from,
    //     limit,
    // } -> OutstandingTributeClaimsResponse

    // pub struct OutstandingTributeClaimsResponse {
    //     pub claims: Vec<TributeClaim>,
    // }

    // 4. We sum up the TributeClaim.amount per proposal_id.
    // It can result in multiple "Coins" with different amounts for one same proposal_id

    // 4. Loop per proposal
    // For each proposal, we loop on these TributeClaim and call Claim
    // For each successful claim, we add it to the list of coins claimed by proposal

    // 5. Query total voting power per proposal
    // HydroQueryMsg::Proposal {
    //     round_id,
    //     tranche_id,
    //     proposal_id,
    // } -> ProposalResponse

    // pub struct ProposalResponse {
    //     pub proposal: Proposal,
    // }

    // 6. For each hydromancer, we calculate their rewards by
    // calculating their voting power per proposal / total voting power per proposal
    // We store their rewards (commission %) as streamable rewards for X months (according to proposal's bid duration)
    // Then, we browse all vessels for this hydromancer, and share streamable rewards for X months (according to proposal's bid duration)

    // 7. For each vessel that was not guided by a hydromancer, we calculate rewards by
    // calculating voting power per proposal / total voting power of that proposal
    // We first store rewards as streamable rewards for X months (according to proposal's bid duration)
    // But these users, when claiming rewards, will have the ability to select whether they want to
    // withdraw all rewards at once and block their vessel (cannot sell) for X rounds
    // Or they want to just claim the 1-month rewards and be able to sell their vessel any time.
    // Next round, they can also decide, whether they withdraw all rewards, or just 1-month worth of rewards and be able to sell
}
