use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{Vector, UnorderedMap}, AccountId, env, require,
    log,
};
use core::convert::TryInto;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Players {
    pub player_list: Vector<AccountId>,
    pub winners: UnorderedMap<AccountId, bool>,
}

impl Players {
    pub fn new() -> Self {
        Self {
            player_list: Vector::new("p".as_bytes()),
            winners: UnorderedMap::new("w".as_bytes()),
        }
    }

    pub fn add(&mut self, player_id: &AccountId) {
        self.player_list.push(player_id);
    }

    pub fn draw(&mut self, n: u64) {
        require!(n <= self.player_list.len(), "n > players.len");

        for _ in 0..n {
            self.draw_one();
        }
    }

    fn draw_one(&mut self) {
        let winner_index = get_random_number(self.player_list.len());
        let winner_id = self.player_list.swap_remove(winner_index);

        self.winners.insert(&winner_id, &true);

        log!("New winner: {}", winner_id);
    }

    pub fn claim(&mut self, player_id: &AccountId) {
        let unclaimed = self.winners.get(player_id).expect("Not a winner");
        require!(unclaimed, "Already claimed");
        self.winners.insert(player_id, &false);
    }
}

pub fn get_random_number(n: u64) -> u64 {
    let mut seed = env::random_seed();
    let seed_len = seed.len();
    let mut arr: [u8; 8] = Default::default();
    seed.rotate_left(0 as usize % seed_len);
    arr.copy_from_slice(&seed[..8]);
    let r: u64 = u64::from_le_bytes(arr).try_into().unwrap();
    return r % n;
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_rng() {
        for _ in 0..100 {
            let x = get_random_number(5);
            assert!(x < 5);
        }
    }

    #[test]
    pub fn test_add() {
        let mut players = Players::new();
        let player1 = AccountId::new_unchecked("foo".to_string());
        let player2 = AccountId::new_unchecked("bar".to_string());

        players.add(&player1);
        players.add(&player2);

        let player_list = players.player_list;
        assert!(player_list.len() == 2);
        assert!(player_list.to_vec().contains(&player1));
        assert!(player_list.to_vec().contains(&player2));
    }

    // #[test]
    // #[should_panic]
    // pub fn test_add_dup() {
    //     let mut players = Players::new();
    //     let player1 = AccountId::new_unchecked("foo".to_string());

    //     players.add(&player1);
    //     players.add(&player1);
    // }

    #[test]
    pub fn test_draw() {
        let mut players = Players::new();

        for i in 0..20 {
            let player_id = AccountId::new_unchecked(format!("player{}", i));
            players.add(&player_id);
        }

        players.draw(10);

        assert!(players.winners.len() == 10);

        let mut ids = std::collections::HashSet::<AccountId>::new();
        for winner in players.winners.keys() {
            assert!(!ids.contains(&winner));
            ids.insert(winner);
        }
    }
}
