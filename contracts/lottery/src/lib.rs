use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::U128,
    near_bindgen, PanicOnDefault, AccountId, Balance,
    env, require, log, ext_contract, Gas,
};
use crate::players::*;

mod players;

const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;
const TGAS: u64 = 1_000_000_000_000;

#[ext_contract(ext_ft)]
trait ExtFT {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Lottery {
    /// Account who owns (manages) the contract
    owner_id: AccountId,

    /// Reward token id
    reward_token_id: AccountId,

    /// Ticket price
    ticket_price: Balance,

    /// Players list
    players: Players,
}

#[near_bindgen]
impl Lottery {
    #[init]
    pub fn new(
        owner_id: AccountId,
        reward_token_id: AccountId,
        ticket_price: U128,
    ) -> Self {
        Self {
            owner_id,
            reward_token_id,
            ticket_price: ticket_price.0,
            players: Players::new(),
        }
    }

    #[payable]
    pub fn buy_ticket(&mut self) {
        let buyer_id = env::predecessor_account_id();
        let deposit = env::attached_deposit();

        require!(deposit == self.ticket_price, "Bad ticket price");

        self.players.add(&buyer_id);

        log!("New player added: {}", buyer_id);
    }

    pub fn draw(&mut self, n: u64) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only owner can call draw"
        );

        self.players.draw(n);
    }

    pub fn claim(&mut self) {
        let account_id = env::predecessor_account_id();
        self.players.claim(&account_id);

        ext_ft::ft_transfer(
            account_id.clone(),
            U128(ONE_NEAR),
            None,
            self.reward_token_id.clone(),
            1,
            Gas(35 * TGAS),
        );

        log!("{} claimed rewards", account_id);
    }

    pub fn get_players(&self) -> Vec<AccountId> {
        self.players.player_list.to_vec()
    }

    pub fn get_winners(&self) -> Vec<AccountId> {
        self.players.winners.keys_as_vector().to_vec()
    }
}
