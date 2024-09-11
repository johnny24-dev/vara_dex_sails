// Code generated by sails-client-gen. DO NOT EDIT.
#[allow(unused_imports)]
use sails_rs::collections::BTreeMap;
#[allow(unused_imports)]
use sails_rs::{
    calls::{Activation, Call, Query, Remoting, RemotingAction},
    prelude::*,
    String,
};
pub struct LpVaraDexFactory<R> {
    #[allow(dead_code)]
    remoting: R,
}
impl<R> LpVaraDexFactory<R> {
    #[allow(unused)]
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}
impl<R: Remoting + Clone> traits::LpVaraDexFactory for LpVaraDexFactory<R> {
    type Args = R::Args;
    fn new(
        &self,
        factory: ActorId,
        token_a: ActorId,
        token_b: ActorId,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> impl Activation<Args = R::Args> {
        RemotingAction::<_, lp_vara_dex_factory::io::New>::new(
            self.remoting.clone(),
            (factory, token_a, token_b, name, symbol, decimals),
        )
    }
}
pub mod lp_vara_dex_factory {
    use super::*;
    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct New(());
        impl New {
            #[allow(dead_code)]
            pub fn encode_call(
                factory: ActorId,
                token_a: ActorId,
                token_b: ActorId,
                name: String,
                symbol: String,
                decimals: u8,
            ) -> Vec<u8> {
                <New as ActionIo>::encode_call(&(factory, token_a, token_b, name, symbol, decimals))
            }
        }
        impl ActionIo for New {
            const ROUTE: &'static [u8] = &[12, 78, 101, 119];
            type Params = (ActorId, ActorId, ActorId, String, String, u8);
            type Reply = ();
        }
    }
}
pub struct LpService<R> {
    remoting: R,
}
impl<R> LpService<R> {
    pub fn new(remoting: R) -> Self {
        Self { remoting }
    }
}
impl<R: Remoting + Clone> traits::LpService for LpService<R> {
    type Args = R::Args;
    fn burn(
        &mut self,
        to: ActorId,
    ) -> impl Call<Output = Result<(U256, U256), LpError>, Args = R::Args> {
        RemotingAction::<_, lp_service::io::Burn>::new(self.remoting.clone(), to)
    }
    fn mint(&mut self, to: ActorId) -> impl Call<Output = Result<U256, LpError>, Args = R::Args> {
        RemotingAction::<_, lp_service::io::Mint>::new(self.remoting.clone(), to)
    }
    fn skim(&mut self, to: ActorId) -> impl Call<Output = Result<(), LpError>, Args = R::Args> {
        RemotingAction::<_, lp_service::io::Skim>::new(self.remoting.clone(), to)
    }
    fn swap(
        &mut self,
        amount0_out: U256,
        amount1_out: U256,
        to: ActorId,
    ) -> impl Call<Output = Result<(), LpError>, Args = R::Args> {
        RemotingAction::<_, lp_service::io::Swap>::new(
            self.remoting.clone(),
            (amount0_out, amount1_out, to),
        )
    }
    fn sync(&mut self) -> impl Call<Output = Result<(), LpError>, Args = R::Args> {
        RemotingAction::<_, lp_service::io::Sync>::new(self.remoting.clone(), ())
    }
    fn approve(
        &mut self,
        spender: ActorId,
        value: U256,
    ) -> impl Call<Output = bool, Args = R::Args> {
        RemotingAction::<_, lp_service::io::Approve>::new(self.remoting.clone(), (spender, value))
    }
    fn transfer(&mut self, to: ActorId, value: U256) -> impl Call<Output = bool, Args = R::Args> {
        RemotingAction::<_, lp_service::io::Transfer>::new(self.remoting.clone(), (to, value))
    }
    fn transfer_from(
        &mut self,
        from: ActorId,
        to: ActorId,
        value: U256,
    ) -> impl Call<Output = bool, Args = R::Args> {
        RemotingAction::<_, lp_service::io::TransferFrom>::new(
            self.remoting.clone(),
            (from, to, value),
        )
    }
    fn get_reserves(&self) -> impl Query<Output = (U256, U256, u64), Args = R::Args> {
        RemotingAction::<_, lp_service::io::GetReserves>::new(self.remoting.clone(), ())
    }
    fn allowance(
        &self,
        owner: ActorId,
        spender: ActorId,
    ) -> impl Query<Output = U256, Args = R::Args> {
        RemotingAction::<_, lp_service::io::Allowance>::new(self.remoting.clone(), (owner, spender))
    }
    fn balance_of(&self, account: ActorId) -> impl Query<Output = U256, Args = R::Args> {
        RemotingAction::<_, lp_service::io::BalanceOf>::new(self.remoting.clone(), account)
    }
    fn decimals(&self) -> impl Query<Output = u8, Args = R::Args> {
        RemotingAction::<_, lp_service::io::Decimals>::new(self.remoting.clone(), ())
    }
    fn name(&self) -> impl Query<Output = String, Args = R::Args> {
        RemotingAction::<_, lp_service::io::Name>::new(self.remoting.clone(), ())
    }
    fn symbol(&self) -> impl Query<Output = String, Args = R::Args> {
        RemotingAction::<_, lp_service::io::Symbol>::new(self.remoting.clone(), ())
    }
    fn total_supply(&self) -> impl Query<Output = U256, Args = R::Args> {
        RemotingAction::<_, lp_service::io::TotalSupply>::new(self.remoting.clone(), ())
    }
}
pub mod lp_service {
    use super::*;
    pub mod io {
        use super::*;
        use sails_rs::calls::ActionIo;
        pub struct Burn(());
        impl Burn {
            #[allow(dead_code)]
            pub fn encode_call(to: ActorId) -> Vec<u8> {
                <Burn as ActionIo>::encode_call(&to)
            }
        }
        impl ActionIo for Burn {
            const ROUTE: &'static [u8] = &[
                36, 76, 112, 83, 101, 114, 118, 105, 99, 101, 16, 66, 117, 114, 110,
            ];
            type Params = ActorId;
            type Reply = Result<(U256, U256), super::LpError>;
        }
        pub struct Mint(());
        impl Mint {
            #[allow(dead_code)]
            pub fn encode_call(to: ActorId) -> Vec<u8> {
                <Mint as ActionIo>::encode_call(&to)
            }
        }
        impl ActionIo for Mint {
            const ROUTE: &'static [u8] = &[
                36, 76, 112, 83, 101, 114, 118, 105, 99, 101, 16, 77, 105, 110, 116,
            ];
            type Params = ActorId;
            type Reply = Result<U256, super::LpError>;
        }
        pub struct Skim(());
        impl Skim {
            #[allow(dead_code)]
            pub fn encode_call(to: ActorId) -> Vec<u8> {
                <Skim as ActionIo>::encode_call(&to)
            }
        }
        impl ActionIo for Skim {
            const ROUTE: &'static [u8] = &[
                36, 76, 112, 83, 101, 114, 118, 105, 99, 101, 16, 83, 107, 105, 109,
            ];
            type Params = ActorId;
            type Reply = Result<(), super::LpError>;
        }
        pub struct Swap(());
        impl Swap {
            #[allow(dead_code)]
            pub fn encode_call(amount0_out: U256, amount1_out: U256, to: ActorId) -> Vec<u8> {
                <Swap as ActionIo>::encode_call(&(amount0_out, amount1_out, to))
            }
        }
        impl ActionIo for Swap {
            const ROUTE: &'static [u8] = &[
                36, 76, 112, 83, 101, 114, 118, 105, 99, 101, 16, 83, 119, 97, 112,
            ];
            type Params = (U256, U256, ActorId);
            type Reply = Result<(), super::LpError>;
        }
        pub struct Sync(());
        impl Sync {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <Sync as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for Sync {
            const ROUTE: &'static [u8] = &[
                36, 76, 112, 83, 101, 114, 118, 105, 99, 101, 16, 83, 121, 110, 99,
            ];
            type Params = ();
            type Reply = Result<(), super::LpError>;
        }
        pub struct Approve(());
        impl Approve {
            #[allow(dead_code)]
            pub fn encode_call(spender: ActorId, value: U256) -> Vec<u8> {
                <Approve as ActionIo>::encode_call(&(spender, value))
            }
        }
        impl ActionIo for Approve {
            const ROUTE: &'static [u8] = &[
                36, 76, 112, 83, 101, 114, 118, 105, 99, 101, 28, 65, 112, 112, 114, 111, 118, 101,
            ];
            type Params = (ActorId, U256);
            type Reply = bool;
        }
        pub struct Transfer(());
        impl Transfer {
            #[allow(dead_code)]
            pub fn encode_call(to: ActorId, value: U256) -> Vec<u8> {
                <Transfer as ActionIo>::encode_call(&(to, value))
            }
        }
        impl ActionIo for Transfer {
            const ROUTE: &'static [u8] = &[
                36, 76, 112, 83, 101, 114, 118, 105, 99, 101, 32, 84, 114, 97, 110, 115, 102, 101,
                114,
            ];
            type Params = (ActorId, U256);
            type Reply = bool;
        }
        pub struct TransferFrom(());
        impl TransferFrom {
            #[allow(dead_code)]
            pub fn encode_call(from: ActorId, to: ActorId, value: U256) -> Vec<u8> {
                <TransferFrom as ActionIo>::encode_call(&(from, to, value))
            }
        }
        impl ActionIo for TransferFrom {
            const ROUTE: &'static [u8] = &[
                36, 76, 112, 83, 101, 114, 118, 105, 99, 101, 48, 84, 114, 97, 110, 115, 102, 101,
                114, 70, 114, 111, 109,
            ];
            type Params = (ActorId, ActorId, U256);
            type Reply = bool;
        }
        pub struct GetReserves(());
        impl GetReserves {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <GetReserves as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for GetReserves {
            const ROUTE: &'static [u8] = &[
                36, 76, 112, 83, 101, 114, 118, 105, 99, 101, 44, 71, 101, 116, 82, 101, 115, 101,
                114, 118, 101, 115,
            ];
            type Params = ();
            type Reply = (U256, U256, u64);
        }
        pub struct Allowance(());
        impl Allowance {
            #[allow(dead_code)]
            pub fn encode_call(owner: ActorId, spender: ActorId) -> Vec<u8> {
                <Allowance as ActionIo>::encode_call(&(owner, spender))
            }
        }
        impl ActionIo for Allowance {
            const ROUTE: &'static [u8] = &[
                36, 76, 112, 83, 101, 114, 118, 105, 99, 101, 36, 65, 108, 108, 111, 119, 97, 110,
                99, 101,
            ];
            type Params = (ActorId, ActorId);
            type Reply = U256;
        }
        pub struct BalanceOf(());
        impl BalanceOf {
            #[allow(dead_code)]
            pub fn encode_call(account: ActorId) -> Vec<u8> {
                <BalanceOf as ActionIo>::encode_call(&account)
            }
        }
        impl ActionIo for BalanceOf {
            const ROUTE: &'static [u8] = &[
                36, 76, 112, 83, 101, 114, 118, 105, 99, 101, 36, 66, 97, 108, 97, 110, 99, 101,
                79, 102,
            ];
            type Params = ActorId;
            type Reply = U256;
        }
        pub struct Decimals(());
        impl Decimals {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <Decimals as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for Decimals {
            const ROUTE: &'static [u8] = &[
                36, 76, 112, 83, 101, 114, 118, 105, 99, 101, 32, 68, 101, 99, 105, 109, 97, 108,
                115,
            ];
            type Params = ();
            type Reply = u8;
        }
        pub struct Name(());
        impl Name {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <Name as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for Name {
            const ROUTE: &'static [u8] = &[
                36, 76, 112, 83, 101, 114, 118, 105, 99, 101, 16, 78, 97, 109, 101,
            ];
            type Params = ();
            type Reply = String;
        }
        pub struct Symbol(());
        impl Symbol {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <Symbol as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for Symbol {
            const ROUTE: &'static [u8] = &[
                36, 76, 112, 83, 101, 114, 118, 105, 99, 101, 24, 83, 121, 109, 98, 111, 108,
            ];
            type Params = ();
            type Reply = String;
        }
        pub struct TotalSupply(());
        impl TotalSupply {
            #[allow(dead_code)]
            pub fn encode_call() -> Vec<u8> {
                <TotalSupply as ActionIo>::encode_call(&())
            }
        }
        impl ActionIo for TotalSupply {
            const ROUTE: &'static [u8] = &[
                36, 76, 112, 83, 101, 114, 118, 105, 99, 101, 44, 84, 111, 116, 97, 108, 83, 117,
                112, 112, 108, 121,
            ];
            type Params = ();
            type Reply = U256;
        }
    }
    #[allow(dead_code)]
    #[cfg(not(target_arch = "wasm32"))]
    pub mod events {
        use super::*;
        use sails_rs::events::*;
        #[derive(PartialEq, Debug, Encode, Decode)]
        #[codec(crate = sails_rs::scale_codec)]
        pub enum LpServiceEvents {
            Mint {
                sender: ActorId,
                amount: (U256, U256),
            },
            Burn {
                sender: ActorId,
                amount: (U256, U256),
                to: ActorId,
            },
            Swap {
                sender: ActorId,
                amount_in: (U256, U256),
                amount_out: (U256, U256),
                to: ActorId,
            },
            GetReserves {
                reserve_a: u128,
                reserve_b: u128,
                block_timestamp_last: u64,
            },
            Sync {
                reserve_a: U256,
                reserve_b: U256,
            },
            Skim {
                amount_a: U256,
                amount_b: U256,
                to: ActorId,
            },
            Approval {
                owner: ActorId,
                spender: ActorId,
                value: U256,
            },
            Transfer {
                from: ActorId,
                to: ActorId,
                value: U256,
            },
        }
        impl EventIo for LpServiceEvents {
            const ROUTE: &'static [u8] = &[36, 76, 112, 83, 101, 114, 118, 105, 99, 101];
            const EVENT_NAMES: &'static [&'static [u8]] = &[
                &[16, 77, 105, 110, 116],
                &[16, 66, 117, 114, 110],
                &[16, 83, 119, 97, 112],
                &[44, 71, 101, 116, 82, 101, 115, 101, 114, 118, 101, 115],
                &[16, 83, 121, 110, 99],
                &[16, 83, 107, 105, 109],
                &[32, 65, 112, 112, 114, 111, 118, 97, 108],
                &[32, 84, 114, 97, 110, 115, 102, 101, 114],
            ];
            type Event = Self;
        }
        pub fn listener<R: Listener<Vec<u8>>>(remoting: R) -> impl Listener<LpServiceEvents> {
            RemotingListener::<_, LpServiceEvents>::new(remoting)
        }
    }
}
#[derive(PartialEq, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum LpError {
    InsufficientAmount,
    InsufficientFormerAmount,
    InsufficientLatterAmount,
    InsufficientLiquidity,
    InvalidRecipient,
    ZeroActorId,
    TransferFailed,
    Overflow,
    DeadlineExceeded,
    IdenticalTokens,
    FeeToGettingFailed,
    InvalidTokens,
    InvalidRouter,
    CanNotConnectToVft,
    InsufficientLiquidityMinted,
    InsufficientLiquidityBurned,
    InsufficientOutputAmount,
    InsufficientInputAmount,
    KConstant,
    InvalidTo,
    CanNotConnectToFactory,
}
pub mod traits {
    use super::*;
    #[allow(dead_code)]
    pub trait LpVaraDexFactory {
        type Args;
        #[allow(clippy::new_ret_no_self)]
        #[allow(clippy::wrong_self_convention)]
        fn new(
            &self,
            factory: ActorId,
            token_a: ActorId,
            token_b: ActorId,
            name: String,
            symbol: String,
            decimals: u8,
        ) -> impl Activation<Args = Self::Args>;
    }
    #[allow(clippy::type_complexity)]
    pub trait LpService {
        type Args;
        fn burn(
            &mut self,
            to: ActorId,
        ) -> impl Call<Output = Result<(U256, U256), LpError>, Args = Self::Args>;
        fn mint(
            &mut self,
            to: ActorId,
        ) -> impl Call<Output = Result<U256, LpError>, Args = Self::Args>;
        fn skim(
            &mut self,
            to: ActorId,
        ) -> impl Call<Output = Result<(), LpError>, Args = Self::Args>;
        fn swap(
            &mut self,
            amount0_out: U256,
            amount1_out: U256,
            to: ActorId,
        ) -> impl Call<Output = Result<(), LpError>, Args = Self::Args>;
        fn sync(&mut self) -> impl Call<Output = Result<(), LpError>, Args = Self::Args>;
        fn approve(
            &mut self,
            spender: ActorId,
            value: U256,
        ) -> impl Call<Output = bool, Args = Self::Args>;
        fn transfer(
            &mut self,
            to: ActorId,
            value: U256,
        ) -> impl Call<Output = bool, Args = Self::Args>;
        fn transfer_from(
            &mut self,
            from: ActorId,
            to: ActorId,
            value: U256,
        ) -> impl Call<Output = bool, Args = Self::Args>;
        fn get_reserves(&self) -> impl Query<Output = (U256, U256, u64), Args = Self::Args>;
        fn allowance(
            &self,
            owner: ActorId,
            spender: ActorId,
        ) -> impl Query<Output = U256, Args = Self::Args>;
        fn balance_of(&self, account: ActorId) -> impl Query<Output = U256, Args = Self::Args>;
        fn decimals(&self) -> impl Query<Output = u8, Args = Self::Args>;
        fn name(&self) -> impl Query<Output = String, Args = Self::Args>;
        fn symbol(&self) -> impl Query<Output = String, Args = Self::Args>;
        fn total_supply(&self) -> impl Query<Output = U256, Args = Self::Args>;
    }
}
#[cfg(feature = "with_mocks")]
#[cfg(not(target_arch = "wasm32"))]
extern crate std;
#[cfg(feature = "with_mocks")]
#[cfg(not(target_arch = "wasm32"))]
pub mod mockall {
    use super::*;
    use sails_rs::mockall::*;
    mock! { pub LpService<A> {} #[allow(refining_impl_trait)] #[allow(clippy::type_complexity)] impl<A> traits::LpService for LpService<A> { type Args = A; fn burn (&mut self, to: ActorId,) -> MockCall<A, Result<(U256,U256,), LpError>>;fn mint (&mut self, to: ActorId,) -> MockCall<A, Result<U256, LpError>>;fn skim (&mut self, to: ActorId,) -> MockCall<A, Result<(), LpError>>;fn swap (&mut self, amount0_out: U256,amount1_out: U256,to: ActorId,) -> MockCall<A, Result<(), LpError>>;fn sync (&mut self, ) -> MockCall<A, Result<(), LpError>>;fn approve (&mut self, spender: ActorId,value: U256,) -> MockCall<A, bool>;fn transfer (&mut self, to: ActorId,value: U256,) -> MockCall<A, bool>;fn transfer_from (&mut self, from: ActorId,to: ActorId,value: U256,) -> MockCall<A, bool>;fn get_reserves (& self, ) -> MockQuery<A, (U256,U256,u64,)>;fn allowance (& self, owner: ActorId,spender: ActorId,) -> MockQuery<A, U256>;fn balance_of (& self, account: ActorId,) -> MockQuery<A, U256>;fn decimals (& self, ) -> MockQuery<A, u8>;fn name (& self, ) -> MockQuery<A, String>;fn symbol (& self, ) -> MockQuery<A, String>;fn total_supply (& self, ) -> MockQuery<A, U256>; } }
}
