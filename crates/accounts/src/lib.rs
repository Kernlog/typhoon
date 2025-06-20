#![no_std]

pub use {accounts::*, programs::*};
use {
    bytemuck::{AnyBitPattern, NoUninit},
    pinocchio::{
        account_info::{AccountInfo, Ref, RefMut},
        pubkey::Pubkey,
    },
    sealed::Sealed,
    typhoon_errors::Error,
};

mod accounts;
mod programs;

pub trait FromAccountInfo<'a>: Sized {
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error>;
}

pub trait ReadableAccount: AsRef<AccountInfo> {
    type Data<'a>
    where
        Self: 'a;

    fn key(&self) -> &Pubkey;
    fn is_owned_by(&self, owner: &Pubkey) -> bool;
    fn lamports(&self) -> Result<Ref<'_, u64>, Error>;
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error>;
}

pub trait WritableAccount: ReadableAccount + Sealed {
    type DataMut<'a>
    where
        Self: 'a;

    fn assign(&self, new_owner: &Pubkey);
    fn realloc(&self, new_len: usize, zero_init: bool) -> Result<(), Error>;
    fn mut_lamports(&self) -> Result<RefMut<'_, u64>, Error>;
    fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error>;
}

pub trait SignerAccount: ReadableAccount + Sealed {}

mod sealed {
    use {
        super::{Mut, ReadableAccount, Signer},
        pinocchio::account_info::AccountInfo,
    };

    pub trait Sealed {}

    impl<T> Sealed for Mut<T> where T: ReadableAccount + AsRef<AccountInfo> {}
    impl Sealed for Signer<'_> {}
}

pub trait ProgramId {
    const ID: Pubkey;
}

pub trait ProgramIds {
    const IDS: &'static [Pubkey];
}

pub trait Owner {
    const OWNER: Pubkey;
}

pub trait Owners {
    const OWNERS: &'static [Pubkey];
}

pub trait Discriminator {
    const DISCRIMINATOR: &'static [u8];
}

pub trait RefFromBytes {
    fn read(data: &[u8]) -> Option<&Self>;
    fn read_mut(data: &mut [u8]) -> Option<&mut Self>;
}

impl<T> RefFromBytes for T
where
    T: Discriminator + AnyBitPattern + NoUninit,
{
    fn read(data: &[u8]) -> Option<&Self> {
        let dis_len = T::DISCRIMINATOR.len();
        bytemuck::try_from_bytes(&data[dis_len..core::mem::size_of::<T>() + dis_len]).ok()
    }

    fn read_mut(data: &mut [u8]) -> Option<&mut Self> {
        let dis_len = T::DISCRIMINATOR.len();
        bytemuck::try_from_bytes_mut(&mut data[dis_len..core::mem::size_of::<T>() + dis_len]).ok()
    }
}
