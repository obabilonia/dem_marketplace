use super::*;
use frame::prelude::*;
use frame::primitives::BlakeTwo256;
use frame::traits::Hash;

impl<T: Config> Pallet<T> {
    /* 🚧 TODO 🚧: Create a function `gen_dna` which returns a `[u8; 32]`.
        - Create a `unique_payload` which contains data from `frame_system::Pallet::<T>`:
            - `parent_hash`
            - `block_number`
            - `extrinsic_index`
            - `CountForKitties::<T>::get()`
        - Use `BlakeTwo256` to calculate the `hash_of` the unique payload.
        - Return the hash as a `[u8; 32]`.
    */
    pub fn gen_dna() -> [u8; 32] {
        // Collect our unique inputs into a single object.
        let parent_hash_ = frame_system::Pallet::<T>::parent_hash();
        let block_number_ = frame_system::Pallet::<T>::block_number();        
        let extrinsic_index_ = frame_system::Pallet::<T>::extrinsic_index();
        let kitty_number = CountForKitties::<T>::get();

        let unique_payload = (parent_hash_, block_number_, extrinsic_index_, kitty_number);
        let hash: [u8; 32] = BlakeTwo256::hash_of(&unique_payload).into();
        hash
    }


    pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {

        let kitty = Kitty { dna, owner: owner.clone() };
        ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicateKitty);
        let current_count: u32 = CountForKitties::<T>::get();
        let new_count: u32 = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
        
        Kitties::<T>::insert(dna, kitty);
        
        CountForKitties::<T>::set(new_count);
        Self::deposit_event(Event::<T>::Created { owner });
        Ok(())
    }
}

