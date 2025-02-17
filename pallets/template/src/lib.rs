//! Pallet que armazena um mapa de struct no Substrate.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    frame_support::StorageMap,
};
use frame_system::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[derive(Encode, Decode, Default, Clone, PartialEq, TypeInfo, MaxEncodedLen)]
    pub struct Guest {
        pub id: u32,
        pub name: Vec<u8>,
        pub email: Vec<u8>,
        pub checkin: u64,
    }

    #[pallet::storage]
    #[pallet::getter(fn get_item)]
    pub(super) type Guests<T: Config> = StorageMap<_, Blake2_128Concat, u32, Item, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ItemStored(u32, Vec<u8>, Vec<u8>, u64),
    }

    #[pallet::error]
    pub enum Error<T> {
        GuestAlreadyExists,
        GuestNotFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn store_item(origin: OriginFor<T>, id: u32, name: Vec<u8>, email: Vec<u8>, checkin: u64) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            ensure!(!Guests::<T>::contains_key(id), Error::<T>::GuestAlreadyExists);

            let guest = Guest { id: id.clone(), name: name.clone(), email: email.clone(), checkin: checkin.clone() };
            Guests::<T>::insert(id, guest);

            Self::deposit_event(Event::ItemStored(id, name, email, checkin));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn get_item(origin: OriginFor<T>, key: u32) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            ensure!(Guests::<T>::contains_key(key), Error::<T>::GuestNotFound);

            let guest = Guests::<T>::get(key).unwrap();
            log::info!("Guest: {:?}", guest);
            Ok(())
        }
    }
}