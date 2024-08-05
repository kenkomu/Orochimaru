use core::starknet::ContractAddress;

#[starknet::interface]
pub trait ISnakeGame<TContractState> {
    fn store_score(ref self: TContractState, score: u64);
    fn get_score(self: @TContractState, address: ContractAddress) -> u64;
}

#[starknet::contract]
mod SnakeGame {
    use core::starknet::{ContractAddress, get_caller_address};
    use starknet::storage_access;

    #[storage]
    struct Storage {
        scores: LegacyMap<ContractAddress, u64>,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        ScoreStored: ScoreStored,
    }

    #[derive(Drop, starknet::Event)]
    struct ScoreStored {
        #[key]
        user: ContractAddress,
        score: u64,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {}

    #[abi(embed_v0)]
    impl SnakeGame of super::ISnakeGame<ContractState> {
        fn store_score(ref self: ContractState, score: u64) {
            let caller = get_caller_address();
            self._store_score(caller, score);
        }

        fn get_score(self: @ContractState, address: ContractAddress) -> u64 {
            let storage_entry = self.scores.read(address);
            storage_entry
        }
    }

    #[generate_trait]
    impl InternalFunctions of InternalFunctionsTrait {
        fn _store_score(ref self: ContractState, user: ContractAddress, score: u64) {
            self.scores.write(user, score);
            self.emit(ScoreStored { user: user, score: score });
        }
    }
}
