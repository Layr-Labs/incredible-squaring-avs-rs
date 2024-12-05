///Module containing a contract's types and functions.
/**

```solidity
library IAllocationManagerTypes {
    struct SlashingParams { address operator; uint32 operatorSetId; address[] strategies; uint256 wadToSlash; string description; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IAllocationManagerTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct SlashingParams { address operator; uint32 operatorSetId; address[] strategies; uint256 wadToSlash; string description; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SlashingParams {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub wadToSlash: alloy::sol_types::private::primitives::aliases::U256,
        pub description: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            u32,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::String,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SlashingParams> for UnderlyingRustTuple<'_> {
            fn from(value: SlashingParams) -> Self {
                (
                    value.operator,
                    value.operatorSetId,
                    value.strategies,
                    value.wadToSlash,
                    value.description,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SlashingParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operator: tuple.0,
                    operatorSetId: tuple.1,
                    strategies: tuple.2,
                    wadToSlash: tuple.3,
                    description: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SlashingParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SlashingParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.wadToSlash),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.description,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SlashingParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for SlashingParams {
            const NAME: &'static str = "SlashingParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SlashingParams(address operator,uint32 operatorSetId,address[] strategies,uint256 wadToSlash,string description)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operator,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorSetId)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.wadToSlash)
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.description,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SlashingParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorSetId,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wadToSlash,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.description,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operator,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorSetId,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wadToSlash,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.description,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IAllocationManagerTypes`](self) contract instance.

See the [wrapper's documentation](`IAllocationManagerTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IAllocationManagerTypesInstance<T, P, N> {
        IAllocationManagerTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IAllocationManagerTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IAllocationManagerTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IAllocationManagerTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IAllocationManagerTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IAllocationManagerTypesInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IAllocationManagerTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IAllocationManagerTypes`](self) contract instance.

See the [wrapper's documentation](`IAllocationManagerTypesInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IAllocationManagerTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IAllocationManagerTypesInstance<T, P, N> {
            IAllocationManagerTypesInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IAllocationManagerTypesInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IAllocationManagerTypesInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library IRewardsCoordinatorTypes {
    struct RewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; uint256 amount; uint32 startTimestamp; uint32 duration; }
    struct StrategyAndMultiplier { address strategy; uint96 multiplier; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IRewardsCoordinatorTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct RewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; uint256 amount; uint32 startTimestamp; uint32 duration; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RewardsSubmission {
        pub strategiesAndMultipliers: alloy::sol_types::private::Vec<
            <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
        >,
        pub token: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub startTimestamp: u32,
        pub duration: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<StrategyAndMultiplier>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            u32,
            u32,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<RewardsSubmission> for UnderlyingRustTuple<'_> {
            fn from(value: RewardsSubmission) -> Self {
                (
                    value.strategiesAndMultipliers,
                    value.token,
                    value.amount,
                    value.startTimestamp,
                    value.duration,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RewardsSubmission {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategiesAndMultipliers: tuple.0,
                    token: tuple.1,
                    amount: tuple.2,
                    startTimestamp: tuple.3,
                    duration: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RewardsSubmission {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RewardsSubmission {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.strategiesAndMultipliers,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.duration),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for RewardsSubmission {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for RewardsSubmission {
            const NAME: &'static str = "RewardsSubmission";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RewardsSubmission(StrategyAndMultiplier[] strategiesAndMultipliers,address token,uint256 amount,uint32 startTimestamp,uint32 duration)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategiesAndMultipliers,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.duration)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RewardsSubmission {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategiesAndMultipliers,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.duration,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Array<
                    StrategyAndMultiplier,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategiesAndMultipliers,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.duration,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct StrategyAndMultiplier { address strategy; uint96 multiplier; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyAndMultiplier {
        pub strategy: alloy::sol_types::private::Address,
        pub multiplier: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U96,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StrategyAndMultiplier> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyAndMultiplier) -> Self {
                (value.strategy, value.multiplier)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyAndMultiplier {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    multiplier: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StrategyAndMultiplier {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StrategyAndMultiplier {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.multiplier),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for StrategyAndMultiplier {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for StrategyAndMultiplier {
            const NAME: &'static str = "StrategyAndMultiplier";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyAndMultiplier(address strategy,uint96 multiplier)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.multiplier)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StrategyAndMultiplier {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategy,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.multiplier,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategy,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.multiplier,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IRewardsCoordinatorTypes`](self) contract instance.

See the [wrapper's documentation](`IRewardsCoordinatorTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IRewardsCoordinatorTypesInstance<T, P, N> {
        IRewardsCoordinatorTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IRewardsCoordinatorTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IRewardsCoordinatorTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IRewardsCoordinatorTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IRewardsCoordinatorTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IRewardsCoordinatorTypesInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRewardsCoordinatorTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IRewardsCoordinatorTypes`](self) contract instance.

See the [wrapper's documentation](`IRewardsCoordinatorTypesInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IRewardsCoordinatorTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IRewardsCoordinatorTypesInstance<T, P, N> {
            IRewardsCoordinatorTypesInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRewardsCoordinatorTypesInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRewardsCoordinatorTypesInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library ISignatureUtils {
    struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod ISignatureUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureWithSaltAndExpiry {
        pub signature: alloy::sol_types::private::Bytes,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignatureWithSaltAndExpiry>
        for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithSaltAndExpiry) -> Self {
                (value.signature, value.salt, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SignatureWithSaltAndExpiry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signature: tuple.0,
                    salt: tuple.1,
                    expiry: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignatureWithSaltAndExpiry {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self>
        for SignatureWithSaltAndExpiry {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SignatureWithSaltAndExpiry {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for SignatureWithSaltAndExpiry {
            const NAME: &'static str = "SignatureWithSaltAndExpiry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignatureWithSaltAndExpiry(bytes signature,bytes32 salt,uint256 expiry)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.salt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.expiry)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SignatureWithSaltAndExpiry {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.salt)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.expiry,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.salt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.expiry,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISignatureUtilsInstance<T, P, N> {
        ISignatureUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISignatureUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ISignatureUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISignatureUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISignatureUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISignatureUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISignatureUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> ISignatureUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISignatureUtilsInstance<T, P, N> {
            ISignatureUtilsInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISignatureUtilsInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISignatureUtilsInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IAllocationManagerTypes {
    struct SlashingParams {
        address operator;
        uint32 operatorSetId;
        address[] strategies;
        uint256 wadToSlash;
        string description;
    }
}

library IRewardsCoordinatorTypes {
    struct RewardsSubmission {
        StrategyAndMultiplier[] strategiesAndMultipliers;
        address token;
        uint256 amount;
        uint32 startTimestamp;
        uint32 duration;
    }
    struct StrategyAndMultiplier {
        address strategy;
        uint96 multiplier;
    }
}

library ISignatureUtils {
    struct SignatureWithSaltAndExpiry {
        bytes signature;
        bytes32 salt;
        uint256 expiry;
    }
}

interface IncredibleSquaringServiceManager {
    event Initialized(uint8 version);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator);
    event SlasherProposed(address newSlasher, uint256 slasherProposalTimestamp);
    event SlasherUpdated(address prevSlasher, address newSlasher);

    constructor(address _avsDirectory, address _registryCoordinator, address _stakeRegistry, address rewards_coordinator, address allocationManager, address _incredibleSquaringTaskManager);

    function SLASHER_PROPOSAL_DELAY() external view returns (uint256);
    function acceptProposedSlasher() external;
    function allocationManager() external view returns (address);
    function avsDirectory() external view returns (address);
    function createAVSRewardsSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
    function createOperatorSets(uint32[] memory operatorSetIds) external;
    function deregisterOperatorFromAVS(address operator) external;
    function deregisterOperatorFromOperatorSets(address operator, uint32[] memory operatorSetIds) external;
    function finalizeMigration() external;
    function getOperatorRestakedStrategies(address operator) external view returns (address[] memory);
    function getOperatorsToMigrate() external view returns (uint32[] memory operatorSetIdsToCreate, uint32[][] memory operatorSetIds, address[] memory allOperators);
    function getRestakeableStrategies() external view returns (address[] memory);
    function incredibleSquaringTaskManager() external view returns (address);
    function migrateAndCreateOperatorSetIds(uint32[] memory operatorSetsToCreate) external;
    function migrateToOperatorSets(uint32[][] memory operatorSetIds, address[] memory operators) external;
    function migrationFinalized() external view returns (bool);
    function owner() external view returns (address);
    function proposeNewSlasher(address newSlasher) external;
    function proposedSlasher() external view returns (address);
    function registerOperatorToAVS(address operator, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
    function registerOperatorToOperatorSets(address operator, uint32[] memory operatorSetIds, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
    function renounceOwnership() external;
    function rewardsInitiator() external view returns (address);
    function setRewardsInitiator(address newRewardsInitiator) external;
    function slashOperator(IAllocationManagerTypes.SlashingParams memory params) external;
    function slasher() external view returns (address);
    function slasherProposalTimestamp() external view returns (uint256);
    function transferOwnership(address newOwner) external;
    function updateAVSMetadataURI(string memory _metadataURI) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_avsDirectory",
        "type": "address",
        "internalType": "contract IAVSDirectory"
      },
      {
        "name": "_registryCoordinator",
        "type": "address",
        "internalType": "contract IRegistryCoordinator"
      },
      {
        "name": "_stakeRegistry",
        "type": "address",
        "internalType": "contract IStakeRegistry"
      },
      {
        "name": "rewards_coordinator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "allocationManager",
        "type": "address",
        "internalType": "contract IAllocationManager"
      },
      {
        "name": "_incredibleSquaringTaskManager",
        "type": "address",
        "internalType": "contract IIncredibleSquaringTaskManager"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "SLASHER_PROPOSAL_DELAY",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "acceptProposedSlasher",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "allocationManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "avsDirectory",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "createAVSRewardsSubmission",
    "inputs": [
      {
        "name": "rewardsSubmissions",
        "type": "tuple[]",
        "internalType": "struct IRewardsCoordinatorTypes.RewardsSubmission[]",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinatorTypes.StrategyAndMultiplier[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          },
          {
            "name": "token",
            "type": "address",
            "internalType": "contract IERC20"
          },
          {
            "name": "amount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "duration",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createOperatorSets",
    "inputs": [
      {
        "name": "operatorSetIds",
        "type": "uint32[]",
        "internalType": "uint32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterOperatorFromAVS",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterOperatorFromOperatorSets",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSetIds",
        "type": "uint32[]",
        "internalType": "uint32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "finalizeMigration",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getOperatorRestakedStrategies",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorsToMigrate",
    "inputs": [],
    "outputs": [
      {
        "name": "operatorSetIdsToCreate",
        "type": "uint32[]",
        "internalType": "uint32[]"
      },
      {
        "name": "operatorSetIds",
        "type": "uint32[][]",
        "internalType": "uint32[][]"
      },
      {
        "name": "allOperators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRestakeableStrategies",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "incredibleSquaringTaskManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IIncredibleSquaringTaskManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "migrateAndCreateOperatorSetIds",
    "inputs": [
      {
        "name": "operatorSetsToCreate",
        "type": "uint32[]",
        "internalType": "uint32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "migrateToOperatorSets",
    "inputs": [
      {
        "name": "operatorSetIds",
        "type": "uint32[][]",
        "internalType": "uint32[][]"
      },
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "migrationFinalized",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proposeNewSlasher",
    "inputs": [
      {
        "name": "newSlasher",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "proposedSlasher",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "registerOperatorToAVS",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSignature",
        "type": "tuple",
        "internalType": "struct ISignatureUtils.SignatureWithSaltAndExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "salt",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "expiry",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registerOperatorToOperatorSets",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSetIds",
        "type": "uint32[]",
        "internalType": "uint32[]"
      },
      {
        "name": "operatorSignature",
        "type": "tuple",
        "internalType": "struct ISignatureUtils.SignatureWithSaltAndExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "salt",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "expiry",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "rewardsInitiator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setRewardsInitiator",
    "inputs": [
      {
        "name": "newRewardsInitiator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "slashOperator",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IAllocationManagerTypes.SlashingParams",
        "components": [
          {
            "name": "operator",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "wadToSlash",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "description",
            "type": "string",
            "internalType": "string"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "slasher",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "slasherProposalTimestamp",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateAVSMetadataURI",
    "inputs": [
      {
        "name": "_metadataURI",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RewardsInitiatorUpdated",
    "inputs": [
      {
        "name": "prevRewardsInitiator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newRewardsInitiator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SlasherProposed",
    "inputs": [
      {
        "name": "newSlasher",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "slasherProposalTimestamp",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SlasherUpdated",
    "inputs": [
      {
        "name": "prevSlasher",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newSlasher",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IncredibleSquaringServiceManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6101403461024057601f612f9638819003918201601f19168301916001600160401b038311848410176102445780849260c094604052833981010312610240578051906001600160a01b03821682036102405760208101516001600160a01b0381168103610240576040820151906001600160a01b03821682036102405760608301516001600160a01b03811690819003610240576080840151936001600160a01b03851685036102405760a00151946001600160a01b03861686036102405760805260a05260c05260e052610100525f5460ff8160081c166101eb5760ff808216106101b1575b5061012052604051612d3d9081610259823960805181818161094301528181610b8e01528181610ef40152818161110b015281816111e501528181611289015281816113290152818161146901526122ed015260a0518161177a015260c05181818161034e01528181610b5e015281816110db015281816111b5015281816112f90152818161143901528181611d870152818161234f0152612bfa015260e051818181611e22015261239b0152610100518181816114e601526120b501526101205181610f930152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f6100e7565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c80630b91d665146101e457806315b7bc9a146101df5780631e2199e2146101da57806326f017e2146101d557806333cfb7b7146101d05780633bc28c8c146101cb57806360db99a3146101c657806367940c89146101c15780636b3aa72e146101bc578063715018a6146101b757806377ef731d146101b25780638999817f146101ad5780638d68349a146101a85780638da5cb5b146101a35780639926ee7d1461019e578063a364f4da14610199578063a98fb35514610194578063afe02ed51461018f578063b13442711461018a578063b78b608714610185578063c1a8e2c514610180578063ca8aa7c71461017b578063d9f9537714610176578063e46f181614610171578063e481af9d1461016c578063f2fde38b14610167578063fc299dee14610162578063fcd1c3751461015d5763fce36c7d14610158575f80fd5b611733565b611716565b6116ee565b61161e565b611603565b6115db565b611515565b6114d1565b6113f4565b6113c0565b611398565b6112e5565b61124e565b611191565b611097565b61106f565b61104d565b610fc2565b610f7e565b610f23565b610edf565b610ec2565b610e03565b610cfe565b610cc2565b610c0c565b610afa565b61092b565b61032f565b5f9103126101f357565b5f80fd5b90602080835192838152019201905f5b8181106102145750505090565b825163ffffffff16845260209384019390920191600101610207565b9080602083519182815201916020808360051b8301019401925f915b83831061025b57505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b80821061029b575050506020806001929701930193019193929061024c565b90919260208060019263ffffffff875116815201940192019061027c565b90602080835192838152019201905f5b8181106102d65750505090565b82516001600160a01b03168452602093840193909201916001016102c9565b9161031e9061031061032c95936060865260608601906101f7565b908482036020860152610230565b9160408184039101526102b9565b90565b346101f3575f3660031901126101f357604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156105215760ff915f916107f0575b50169061039b611a1d565b6103a483611a38565b914363ffffffff16905f5b60ff8116948686101561064957604051634f4c91e160e11b815294602086600481875afa958615610521575f96610629575b50604051638902624560e01b815260ff8416600482015263ffffffff86166024820152955f90879060449082906001600160a01b03165afa958615610521575f96610605575b506104328651611a38565b975f915b875183101561054357604051632efa2ca360e11b81526020816004818a5afa90811561052157610497916020915f91610526575b50610475868c611b31565b519060405180809581946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa918215610521576104d88c600194876104e7955f926104ef575b506104c99192611b31565b6001600160a01b039091169052565b6104e18c61273a565b906127f0565b920191610436565b6104c992506105149060203d811161051a575b61050c8183610853565b810190611a8f565b916104be565b503d610502565b611a12565b61053d9150823d811161051a5761050c8183610853565b5f61046a565b96509750509490916105558551611a38565b965f945f5b87518110156105ce57898861058e6105826105758584611b31565b516001600160a01b031690565b6001600160a01b031690565b61059d575b505060010161055a565b8291986104c96105b56105756001966105c595611b31565b916105bf81611b4a565b9b611b31565b90508988610593565b509488529596939450916105fa916105f5906105ea818a611b31565b9063ffffffff169052565b611a7e565b9493949291926103af565b6106229196503d805f833e61061a8183610853565b810190611aa4565b945f610427565b61064291965060203d811161051a5761050c8183610853565b945f6103e1565b849291506106578351611b58565b915f5b84518110156107da5761069e60206106756105758489611b31565b6040516309aa152760e11b81526001600160a01b03909116600482015291829081906024820190565b0381865afa908115610521576106d8916020915f916107ad575b506040518093819263871ef04960e01b8352600483019190602083019252565b0381865afa8015610521576106fd915f9161077f575b506001600160c01b03166129a6565b916107088351611a38565b935f5b8451811015610757578061075161074761074161073b61072d6001968b611bcf565b516001600160f81b03191690565b60f81c90565b60ff1690565b6105ea838a611b31565b0161070b565b50936001929196935061076a8287611b31565b526107758186611b31565b500193909361065a565b6107a0915060203d81116107a6575b6107988183610853565b810190611bb0565b876106ee565b503d61078e565b6107cd9150823d81116107d3575b6107c58183610853565b810190611ba1565b886106b8565b503d6107bb565b50506107ec83604051938493846102f5565b0390f35b610812915060203d602011610818575b61080a8183610853565b8101906119f9565b5f610390565b503d610800565b634e487b7160e01b5f52604160045260245ffd5b60a081019081106001600160401b0382111761084e57604052565b61081f565b90601f801991011681019081106001600160401b0382111761084e57604052565b6001600160401b03811161084e5760051b60200190565b359063ffffffff821682036101f357565b9080601f830112156101f35781356108b381610874565b926108c16040519485610853565b81845260208085019260051b8201019283116101f357602001905b8282106108e95750505090565b602080916108f68461088b565b8152019101906108dc565b60206003198201126101f357600435906001600160401b0382116101f35761032c9160040161089c565b346101f35761093936610901565b610941612a45565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156101f35760405163aec205c560e01b81525f8160048183865af18015610521576109e3575b50803b156101f35760405163afe02ed560e01b8152905f9082908183816109bd886004830161221e565b03925af18015610521576109cd57005b806109db5f6109e193610853565b806101e9565b005b806109db5f6109f193610853565b5f610993565b6001600160a01b038116036101f357565b9181601f840112156101f3578235916001600160401b0383116101f3576020808501948460051b0101116101f357565b6001600160401b03811161084e57601f01601f191660200190565b929192610a5f82610a38565b91610a6d6040519384610853565b8294818452818301116101f3578281602093845f960137010152565b9190916060818403126101f35760405190606082018281106001600160401b0382111761084e57604052819381356001600160401b0381116101f35782019181601f840112156101f357610ae7604093928360208695359101610a53565b8452602081013560208501520135910152565b346101f35760603660031901126101f357600435610b17816109f7565b6024356001600160401b0381116101f357610b36903690600401610a08565b91906044356001600160401b0381116101f357610b57903690600401610a89565b92610b8c337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611be0565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156101f3575f80946109bd610bfa9760405198899788968795630f10ccf160e11b875260018060a01b03166004870152606060248701526064860191611c6d565b83810360031901604485015290611ccf565b346101f3575f3660031901126101f357610c24612a45565b60685462093a808101809111610cbd574210610c6157606754610c4f906001600160a01b0316612a9d565b606780546001600160a01b0319169055005b60405162461bcd60e51b815260206004820152602e60248201527f536572766963654d616e616765723a20536c61736865722070726f706f73616c60448201526d0819195b185e481b9bdd081b595d60921b6064820152608490fd5b611a6a565b346101f35760203660031901126101f3576107ec610cea600435610ce5816109f7565b611d68565b6040519182916020835260208301906102b9565b346101f35760203660031901126101f357600435610d1b816109f7565b610d23612a45565b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555005b9080601f830112156101f3578135610d9881610874565b92610da66040519485610853565b81845260208085019260051b8201019283116101f357602001905b828210610dce5750505090565b602080918335610ddd816109f7565b815201910190610dc1565b9080601f830112156101f35781602061032c93359101610a53565b346101f35760203660031901126101f3576004356001600160401b0381116101f35760a060031982360301126101f357604051610e3f81610833565b8160040135610e4d816109f7565b8152610e5b6024830161088b565b602082015260448201356001600160401b0381116101f357610e839060043691850101610d81565b60408201526064820135606082015260848201356001600160401b0381116101f3576109e1926004610eb89236920101610de8565b60808201526120a1565b346101f3575f3660031901126101f357602060405162093a808152f35b346101f3575f3660031901126101f3576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346101f3575f3660031901126101f357610f3b612a45565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346101f3575f3660031901126101f3576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346101f35760203660031901126101f3577f2f8afc8a78fd958f3301c0233aa326b9c4b9a2884a7483227d6b0555aaa03adb611048600435611003816109f7565b61100b612a45565b606780546001600160a01b0319166001600160a01b0392909216918217905542606881905560408051928352602083019190915290918291820190565b0390a1005b346101f3575f3660031901126101f357602060ff606954166040519015158152f35b346101f3575f3660031901126101f3576033546040516001600160a01b039091168152602090f35b346101f3575f60403660031901126101f3576004356110b5816109f7565b6024356001600160401b0381116101f3576110d4903690600401610a89565b90611109337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611be0565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b156101f357611173925f9283604051809681958294639926ee7d60e01b845260018060a01b03166004840152604060248401526044830190611ccf565b03925af1801561052157611185575080f35b6109e191505f90610853565b346101f3575f60203660031901126101f3576004356111af816109f7565b6111e3337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611be0565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156101f3576040516351b27a6d60e11b81526001600160a01b039091166004820152905f908290602490829084905af1801561052157611185575080f35b346101f3575f60203660031901126101f3576004356001600160401b0381116101f35761127f903690600401610de8565b611287612a45565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156101f35760405163a98fb35560e01b815260206004820152915f918391829084908290611173906024830190611cab565b346101f3576112f336610901565b611327337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611be0565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156101f35760405163afe02ed560e01b815260206004820152905f9082906113809060248301906101f7565b93818381819703925af1801561052157611185575080f35b346101f3575f3660031901126101f3576066546040516001600160a01b039091168152602090f35b346101f3575f3660031901126101f3576113d8612a45565b60016069546113ea60ff82161561222f565b60ff191617606955005b346101f3575f60403660031901126101f357600435611412816109f7565b6024356001600160401b0381116101f357611431903690600401610a08565b9190611467337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611be0565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156101f357611173935f80946040519687958694859363c1a8e2c560e01b855260018060a01b03166004850152604060248501526044840191611c6d565b346101f3575f3660031901126101f3576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346101f35760403660031901126101f3576004356001600160401b0381116101f357366023820112156101f357806004013561155081610874565b9161155e6040519384610853565b8183526024602084019260051b820101903682116101f35760248101925b8284106115ac57602435856001600160401b0382116101f3576115a66109e1923690600401610d81565b9061228f565b83356001600160401b0381116101f3576020916115d083926024369187010161089c565b81520193019261157c565b346101f3575f3660031901126101f3576067546040516001600160a01b039091168152602090f35b346101f3575f3660031901126101f3576107ec610cea612340565b346101f35760203660031901126101f35760043561163b816109f7565b611643612a45565b6001600160a01b0381161561169a57603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3005b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b346101f3575f3660031901126101f3576065546040516001600160a01b039091168152602090f35b346101f3575f3660031901126101f3576020606854604051908152f35b346101f35760203660031901126101f3576004356001600160401b0381116101f357611763903690600401610a08565b6065549091906001600160a01b03163303611979577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316915f5b8181106117d75750823b156101f3576109bd925f928360405180968195829463fce36c7d60e01b84526004840161261e565b5f60206118326117f5610582836117ef87898b61256d565b0161258f565b604061180286888a61256d565b6040516323b872dd60e01b8152336004820152306024820152910135604482015293849283919082906064820190565b03925af180156105215761195d575b5061185561058260206117ef84868861256d565b604051636eb1769f60e11b81523060048201526001600160a01b03861660248201529190602090839060449082905afa8015610521576118f86020915f948591611940575b506118c56118b0610582856117ef888b8d61256d565b9160406118be878a8c61256d565b0135611cf9565b60405163095ea7b360e01b81526001600160a01b038a166004820152602481019190915294859283919082906044820190565b03925af191821561052157600192611912575b50016117a5565b6119329060203d8111611939575b61192a8183610853565b810190612599565b505f61190b565b503d611920565b6119579150833d81116107d3576107c58183610853565b5f61189a565b6119749060203d81116119395761192a8183610853565b611841565b60405162461bcd60e51b815260206004820152604c60248201527f536572766963654d616e61676572426173652e6f6e6c7952657761726473496e60448201527f69746961746f723a2063616c6c6572206973206e6f742074686520726577617260648201526b32399034b734ba34b0ba37b960a11b608482015260a490fd5b908160209103126101f3575160ff811681036101f35790565b6040513d5f823e3d90fd5b60405190611a2c602083610853565b5f808352366020840137565b90611a4282610874565b611a4f6040519182610853565b8281528092611a60601f1991610874565b0190602036910137565b634e487b7160e01b5f52601160045260245ffd5b60ff1660ff8114610cbd5760010190565b908160209103126101f3575161032c816109f7565b6020818303126101f3578051906001600160401b0382116101f357019080601f830112156101f3578151611ad781610874565b92611ae56040519485610853565b81845260208085019260051b8201019283116101f357602001905b828210611b0d5750505090565b8151815260209182019101611b00565b634e487b7160e01b5f52603260045260245ffd5b8051821015611b455760209160051b010190565b611b1d565b5f198114610cbd5760010190565b90611b6282610874565b611b6f6040519182610853565b8281528092611b80601f1991610874565b01905f5b828110611b9057505050565b806060602080938501015201611b84565b908160209103126101f3575190565b908160209103126101f357516001600160c01b03811681036101f35790565b908151811015611b45570160200190565b15611be757565b60405162461bcd60e51b815260206004820152605260248201527f536572766963654d616e61676572426173652e6f6e6c7952656769737472794360448201527f6f6f7264696e61746f723a2063616c6c6572206973206e6f742074686520726560648201527133b4b9ba393c9031b7b7b93234b730ba37b960711b608482015260a490fd5b916020908281520191905f5b818110611c865750505090565b90919260208060019263ffffffff611c9d8861088b565b168152019401929101611c79565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b90604080611ce68451606085526060850190611cab565b9360208101516020850152015191015290565b91908201809211610cbd57565b6bffffffffffffffffffffffff8116036101f357565b908160409103126101f3576040519060408201908282106001600160401b0383111761084e576020916040528051611d53816109f7565b83520151611d6081611d06565b602082015290565b6040516309aa152760e11b81526001600160a01b0391821660048201527f000000000000000000000000000000000000000000000000000000000000000090911690602081602481855afa90811561052157611de7916020915f9161208457506040518093819263871ef04960e01b8352600483019190602083019252565b0381855afa908115610521575f91612065575b506001600160c01b0316908115908115612012575b5061200957611e1d906129a6565b5f91907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690835b8151851015611ed557611e8e6020611e6b61073b61072d8987611bcf565b604051633ca5a5f560e01b815260ff909116600482015291829081906024820190565b0381875afa801561052157600192611ead925f92611eb5575b50611cf9565b940193611e4d565b611ece91925060203d81116107d3576107c58183610853565b905f611ea7565b611ee0919450611a38565b925f905f5b815181101561200357611efe61073b61072d8385611bcf565b604051633ca5a5f560e01b815260ff8216600482015290602082602481895afa918215610521575f92611fe3575b50905f915b818310611f4357505050600101611ee5565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa91821561052157611fa78b6104c983611fa1610582600198611fac985f91611fb5575b50516001600160a01b031690565b92611b31565b611b4a565b95019190611f31565b611fd6915060403d8111611fdc575b611fce8183610853565b810190611d1c565b5f611f93565b503d611fc4565b611ffc91925060203d81116107d3576107c58183610853565b905f611f2c565b50505050565b5061032c611a1d565b604051639aa1653d60e01b81529150602090829060049082905afa80156105215760ff915f91612046575b5016155f611e0f565b61205f915060203d6020116108185761080a8183610853565b5f61203d565b61207e915060203d6020116107a6576107988183610853565b5f611dfa565b61209b9150823d84116107d3576107c58183610853565b5f6106b8565b6066546001600160a01b031633036121b3577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156101f357604080516360db99a360e01b815260206004820181905284516001600160a01b031660248301528085015163ffffffff1660448301529184015160a06064830152805160c483018190529194859360e4850193919201905f5b818110612191575050505f83612170829694608085606085970151608486015201516023198483030160a4850152611cab565b03925af18015610521576121815750565b806109db5f61218f93610853565b565b82516001600160a01b031685528795506020948501949092019160010161213d565b60405162461bcd60e51b815260206004820152603960248201527f536572766963654d616e61676572426173652e6f6e6c79536c61736865723a2060448201527f63616c6c6572206973206e6f742074686520736c6173686572000000000000006064820152608490fd5b90602061032c9281815201906101f7565b1561223657565b60405162461bcd60e51b815260206004820152602b60248201527f536572766963654d616e616765723a204d6967726174696f6e20416c7265616460448201526a1e48119a5b985b1a5e995960aa1b6064820152608490fd5b90612298612a45565b6122a760ff606954161561222f565b6122b48151835114612afb565b5f5b81518110156122ea57806122e36122d261057560019486611b31565b6122dc8387611b31565b5190612bdb565b50016122b6565b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b156101f357612170925f928360405180968195829463ef2dfa8d60e01b845260048401612b5b565b604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156105215760ff915f9161254e575b50168015612544577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316905f9081905b80831061250057506123db9150611a38565b925f905f5b604051639aa1653d60e01b8152602081600481895afa80156105215760ff915f916124e2575b50168110156124db57604051633ca5a5f560e01b815260ff821660048201819052602082602481895afa918215610521575f926124bb575b50905f915b818310612455575050506001016123e0565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa91821561052157611fa78b6104c983611fa16105826001986124b2985f91611fb55750516001600160a01b031690565b95019190612443565b6124d491925060203d81116107d3576107c58183610853565b905f61243e565b5092505050565b6124fa915060203d81116108185761080a8183610853565b5f612406565b604051633ca5a5f560e01b815260ff84166004820152909190602081602481885afa80156105215760019261253b925f92611eb55750611cf9565b920191906123c9565b505061032c611a1d565b612567915060203d6020116108185761080a8183610853565b5f612391565b9190811015611b455760051b81013590609e19813603018212156101f3570190565b3561032c816109f7565b908160209103126101f3575180151581036101f35790565b916020908281520191905f5b8181106125ca5750505090565b90919260408060019286356125de816109f7565b848060a01b031681526bffffffffffffffffffffffff602088013561260281611d06565b1660208201520194019291016125bd565b359061218f826109f7565b9180602084016020855252604083019060408160051b85010193835f91609e1982360301905b848410612655575050505050505090565b90919293949596603f198282030187528735838112156101f3578401908135601e19833603018112156101f357820191602083359301906001600160401b0384116101f3578360061b360382136101f35761271e836080612713816126c96020989760019a60a08b9a5260a08701916125b1565b956126e76126d8898301612613565b6001600160a01b0316868a0152565b6040810135604086015261270d6127006060830161088b565b63ffffffff166060870152565b0161088b565b63ffffffff16910152565b99019701959401929190612644565b91908203918211610cbd57565b805160018111156127ec5760011c9161275283611a38565b9161276661276185835161272d565b611a38565b915f5b8581106127c95750845b82518110156127a857806127a261278f61057560019487611b31565b6104c961279c8a8561272d565b88611b31565b01612773565b50935050906127b69061273a565b6127c0909161273a565b61032c916127f0565b806127e66127dc61057560019487611b31565b6104c98389611b31565b01612769565b5090565b9182519282516128036127618287611cf9565b935f935f925f975b80871080612975575b156128f6576128266105758888611b31565b6128366105826105758888611b31565b6001600160a01b039091161015612878576128736128606105756128598a611b4a565b9989611b31565b6104c961286c8c611b4a565b9b8b611b31565b61280b565b6128856105758888611b31565b6128956105826105758888611b31565b6001600160a01b0390911611156128bf576128736128606105756128b888611b4a565b9787611b31565b936128f090611fa76128dd6105756128d68b611b4a565b9a8a611b31565b6104c96128e98d611b4a565b9c8c611b31565b9361280b565b9795919794909293945b80831061294b575050505b80831061291a57505050815290565b61294661293361057561292c86611b4a565b9585611b31565b6104c961293f87611b4a565b9688611b31565b61290b565b61297061295d61057561292c86611b4a565b6104c96129698a611b4a565b998b611b31565b612900565b50818510612814565b9061298882610a38565b6129956040519182610853565b8281528092611a60601f1991610a38565b5f81805b612a2057506129bc9061ffff1661297e565b5f5f5b8251821080612a15575b15612a0e576001811b84166129e7575b6129e290611b4a565b6129bf565b9060016129e29160ff60f81b8460f81b165f1a612a048287611bcf565b53019190506129d9565b5050905090565b5061010081106129c9565b5f198101818111610cbd5761ffff9116911661ffff8114610cbd5760010190806129aa565b6033546001600160a01b03163303612a5957565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b606654604080516001600160a01b038084168252841660208201529192917fe0d49a54274423183dadecbdf239eaac6e06ba88320b26fe8cc5ec9d050a63959190a16001600160a01b03166001600160a01b03199190911617606655565b15612b0257565b60405162461bcd60e51b815260206004820152602b60248201527f536572766963654d616e616765723a20496e707574206172726179206c656e6760448201526a0e8d040dad2e6dac2e8c6d60ab1b6064820152608490fd5b9091612b7261032c936040845260408401906102b9565b916020818403910152610230565b15612b8757565b60405162461bcd60e51b815260206004820152602660248201527f536572766963654d616e616765723a204f70657261746f72206e6f7420696e2060448201526571756f72756d60d01b6064820152608490fd5b6040516309aa152760e11b81526001600160a01b0391821660048201527f00000000000000000000000000000000000000000000000000000000000000009091169291905f90602081602481885afa801561052157612c5f956020925f92612ce8575b50604051808098819463871ef04960e01b8352600483019190602083019252565b03915afa938415610521575f94612cc7575b505f5b8251811015612cc15780612cbb612cb6612ca0610741612c9660019689611b31565b5163ffffffff1690565b848060c01b03891660ff600192161c1660011490565b612b80565b01612c74565b50925050565b612ce191945060203d6020116107a6576107988183610853565b925f612c71565b612d00919250833d85116107d3576107c58183610853565b905f612c3e56fea2646970667358221220d61e4852e9398a0883052fb74764e0e4020406edfdaf430735f3cd5496fe9e8c64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01@4a\x02@W`\x1Fa/\x968\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x02DW\x80\x84\x92`\xC0\x94`@R\x839\x81\x01\x03\x12a\x02@W\x80Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02@W` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02@W`@\x82\x01Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02@W``\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x02@W`\x80\x84\x01Q\x93`\x01`\x01`\xA0\x1B\x03\x85\x16\x85\x03a\x02@W`\xA0\x01Q\x94`\x01`\x01`\xA0\x1B\x03\x86\x16\x86\x03a\x02@W`\x80R`\xA0R`\xC0R`\xE0Ra\x01\0R_T`\xFF\x81`\x08\x1C\x16a\x01\xEBW`\xFF\x80\x82\x16\x10a\x01\xB1W[Pa\x01 R`@Qa-=\x90\x81a\x02Y\x829`\x80Q\x81\x81\x81a\tC\x01R\x81\x81a\x0B\x8E\x01R\x81\x81a\x0E\xF4\x01R\x81\x81a\x11\x0B\x01R\x81\x81a\x11\xE5\x01R\x81\x81a\x12\x89\x01R\x81\x81a\x13)\x01R\x81\x81a\x14i\x01Ra\"\xED\x01R`\xA0Q\x81a\x17z\x01R`\xC0Q\x81\x81\x81a\x03N\x01R\x81\x81a\x0B^\x01R\x81\x81a\x10\xDB\x01R\x81\x81a\x11\xB5\x01R\x81\x81a\x12\xF9\x01R\x81\x81a\x149\x01R\x81\x81a\x1D\x87\x01R\x81\x81a#O\x01Ra+\xFA\x01R`\xE0Q\x81\x81\x81a\x1E\"\x01Ra#\x9B\x01Ra\x01\0Q\x81\x81\x81a\x14\xE6\x01Ra \xB5\x01Ra\x01 Q\x81a\x0F\x93\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\0\xE7V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x0B\x91\xD6e\x14a\x01\xE4W\x80c\x15\xB7\xBC\x9A\x14a\x01\xDFW\x80c\x1E!\x99\xE2\x14a\x01\xDAW\x80c&\xF0\x17\xE2\x14a\x01\xD5W\x80c3\xCF\xB7\xB7\x14a\x01\xD0W\x80c;\xC2\x8C\x8C\x14a\x01\xCBW\x80c`\xDB\x99\xA3\x14a\x01\xC6W\x80cg\x94\x0C\x89\x14a\x01\xC1W\x80ck:\xA7.\x14a\x01\xBCW\x80cqP\x18\xA6\x14a\x01\xB7W\x80cw\xEFs\x1D\x14a\x01\xB2W\x80c\x89\x99\x81\x7F\x14a\x01\xADW\x80c\x8Dh4\x9A\x14a\x01\xA8W\x80c\x8D\xA5\xCB[\x14a\x01\xA3W\x80c\x99&\xEE}\x14a\x01\x9EW\x80c\xA3d\xF4\xDA\x14a\x01\x99W\x80c\xA9\x8F\xB3U\x14a\x01\x94W\x80c\xAF\xE0.\xD5\x14a\x01\x8FW\x80c\xB14Bq\x14a\x01\x8AW\x80c\xB7\x8B`\x87\x14a\x01\x85W\x80c\xC1\xA8\xE2\xC5\x14a\x01\x80W\x80c\xCA\x8A\xA7\xC7\x14a\x01{W\x80c\xD9\xF9Sw\x14a\x01vW\x80c\xE4o\x18\x16\x14a\x01qW\x80c\xE4\x81\xAF\x9D\x14a\x01lW\x80c\xF2\xFD\xE3\x8B\x14a\x01gW\x80c\xFC)\x9D\xEE\x14a\x01bW\x80c\xFC\xD1\xC3u\x14a\x01]Wc\xFC\xE3l}\x14a\x01XW_\x80\xFD[a\x173V[a\x17\x16V[a\x16\xEEV[a\x16\x1EV[a\x16\x03V[a\x15\xDBV[a\x15\x15V[a\x14\xD1V[a\x13\xF4V[a\x13\xC0V[a\x13\x98V[a\x12\xE5V[a\x12NV[a\x11\x91V[a\x10\x97V[a\x10oV[a\x10MV[a\x0F\xC2V[a\x0F~V[a\x0F#V[a\x0E\xDFV[a\x0E\xC2V[a\x0E\x03V[a\x0C\xFEV[a\x0C\xC2V[a\x0C\x0CV[a\n\xFAV[a\t+V[a\x03/V[_\x91\x03\x12a\x01\xF3WV[_\x80\xFD[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x02\x14WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x02\x07V[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x02[WPPPPP\x90V[\x90\x91\x92\x93\x94`\x1F\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x02\x9BWPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x02LV[\x90\x91\x92` \x80`\x01\x92c\xFF\xFF\xFF\xFF\x87Q\x16\x81R\x01\x94\x01\x92\x01\x90a\x02|V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x02\xD6WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x02\xC9V[\x91a\x03\x1E\x90a\x03\x10a\x03,\x95\x93``\x86R``\x86\x01\x90a\x01\xF7V[\x90\x84\x82\x03` \x86\x01Ra\x020V[\x91`@\x81\x84\x03\x91\x01Ra\x02\xB9V[\x90V[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x05!W`\xFF\x91_\x91a\x07\xF0W[P\x16\x90a\x03\x9Ba\x1A\x1DV[a\x03\xA4\x83a\x1A8V[\x91Cc\xFF\xFF\xFF\xFF\x16\x90_[`\xFF\x81\x16\x94\x86\x86\x10\x15a\x06IW`@QcOL\x91\xE1`\xE1\x1B\x81R\x94` \x86`\x04\x81\x87Z\xFA\x95\x86\x15a\x05!W_\x96a\x06)W[P`@Qc\x89\x02bE`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x86\x16`$\x82\x01R\x95_\x90\x87\x90`D\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x95\x86\x15a\x05!W_\x96a\x06\x05W[Pa\x042\x86Qa\x1A8V[\x97_\x91[\x87Q\x83\x10\x15a\x05CW`@Qc.\xFA,\xA3`\xE1\x1B\x81R` \x81`\x04\x81\x8AZ\xFA\x90\x81\x15a\x05!Wa\x04\x97\x91` \x91_\x91a\x05&W[Pa\x04u\x86\x8Ca\x1B1V[Q\x90`@Q\x80\x80\x95\x81\x94c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x05!Wa\x04\xD8\x8C`\x01\x94\x87a\x04\xE7\x95_\x92a\x04\xEFW[Pa\x04\xC9\x91\x92a\x1B1V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a\x04\xE1\x8Ca':V[\x90a'\xF0V[\x92\x01\x91a\x046V[a\x04\xC9\x92Pa\x05\x14\x90` =\x81\x11a\x05\x1AW[a\x05\x0C\x81\x83a\x08SV[\x81\x01\x90a\x1A\x8FV[\x91a\x04\xBEV[P=a\x05\x02V[a\x1A\x12V[a\x05=\x91P\x82=\x81\x11a\x05\x1AWa\x05\x0C\x81\x83a\x08SV[_a\x04jV[\x96P\x97PP\x94\x90\x91a\x05U\x85Qa\x1A8V[\x96_\x94_[\x87Q\x81\x10\x15a\x05\xCEW\x89\x88a\x05\x8Ea\x05\x82a\x05u\x85\x84a\x1B1V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x05\x9DW[PP`\x01\x01a\x05ZV[\x82\x91\x98a\x04\xC9a\x05\xB5a\x05u`\x01\x96a\x05\xC5\x95a\x1B1V[\x91a\x05\xBF\x81a\x1BJV[\x9Ba\x1B1V[\x90P\x89\x88a\x05\x93V[P\x94\x88R\x95\x96\x93\x94P\x91a\x05\xFA\x91a\x05\xF5\x90a\x05\xEA\x81\x8Aa\x1B1V[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[a\x1A~V[\x94\x93\x94\x92\x91\x92a\x03\xAFV[a\x06\"\x91\x96P=\x80_\x83>a\x06\x1A\x81\x83a\x08SV[\x81\x01\x90a\x1A\xA4V[\x94_a\x04'V[a\x06B\x91\x96P` =\x81\x11a\x05\x1AWa\x05\x0C\x81\x83a\x08SV[\x94_a\x03\xE1V[\x84\x92\x91Pa\x06W\x83Qa\x1BXV[\x91_[\x84Q\x81\x10\x15a\x07\xDAWa\x06\x9E` a\x06ua\x05u\x84\x89a\x1B1V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x86Z\xFA\x90\x81\x15a\x05!Wa\x06\xD8\x91` \x91_\x91a\x07\xADW[P`@Q\x80\x93\x81\x92c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x80\x15a\x05!Wa\x06\xFD\x91_\x91a\x07\x7FW[P`\x01`\x01`\xC0\x1B\x03\x16a)\xA6V[\x91a\x07\x08\x83Qa\x1A8V[\x93_[\x84Q\x81\x10\x15a\x07WW\x80a\x07Qa\x07Ga\x07Aa\x07;a\x07-`\x01\x96\x8Ba\x1B\xCFV[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`\xFF\x16\x90V[a\x05\xEA\x83\x8Aa\x1B1V[\x01a\x07\x0BV[P\x93`\x01\x92\x91\x96\x93Pa\x07j\x82\x87a\x1B1V[Ra\x07u\x81\x86a\x1B1V[P\x01\x93\x90\x93a\x06ZV[a\x07\xA0\x91P` =\x81\x11a\x07\xA6W[a\x07\x98\x81\x83a\x08SV[\x81\x01\x90a\x1B\xB0V[\x87a\x06\xEEV[P=a\x07\x8EV[a\x07\xCD\x91P\x82=\x81\x11a\x07\xD3W[a\x07\xC5\x81\x83a\x08SV[\x81\x01\x90a\x1B\xA1V[\x88a\x06\xB8V[P=a\x07\xBBV[PPa\x07\xEC\x83`@Q\x93\x84\x93\x84a\x02\xF5V[\x03\x90\xF3[a\x08\x12\x91P` =` \x11a\x08\x18W[a\x08\n\x81\x83a\x08SV[\x81\x01\x90a\x19\xF9V[_a\x03\x90V[P=a\x08\0V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08NW`@RV[a\x08\x1FV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08NW`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\x08NW`\x05\x1B` \x01\x90V[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\xF3WV[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xF3W\x815a\x08\xB3\x81a\x08tV[\x92a\x08\xC1`@Q\x94\x85a\x08SV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x01\xF3W` \x01\x90[\x82\x82\x10a\x08\xE9WPPP\x90V[` \x80\x91a\x08\xF6\x84a\x08\x8BV[\x81R\x01\x91\x01\x90a\x08\xDCV[` `\x03\x19\x82\x01\x12a\x01\xF3W`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xF3Wa\x03,\x91`\x04\x01a\x08\x9CV[4a\x01\xF3Wa\t96a\t\x01V[a\tAa*EV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x01\xF3W`@Qc\xAE\xC2\x05\xC5`\xE0\x1B\x81R_\x81`\x04\x81\x83\x86Z\xF1\x80\x15a\x05!Wa\t\xE3W[P\x80;\x15a\x01\xF3W`@Qc\xAF\xE0.\xD5`\xE0\x1B\x81R\x90_\x90\x82\x90\x81\x83\x81a\t\xBD\x88`\x04\x83\x01a\"\x1EV[\x03\x92Z\xF1\x80\x15a\x05!Wa\t\xCDW\0[\x80a\t\xDB_a\t\xE1\x93a\x08SV[\x80a\x01\xE9V[\0[\x80a\t\xDB_a\t\xF1\x93a\x08SV[_a\t\x93V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01\xF3WV[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xF3W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xF3W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x01\xF3WV[`\x01`\x01`@\x1B\x03\x81\x11a\x08NW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\n_\x82a\n8V[\x91a\nm`@Q\x93\x84a\x08SV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xF3W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x91\x90\x91``\x81\x84\x03\x12a\x01\xF3W`@Q\x90``\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08NW`@R\x81\x93\x815`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3W\x82\x01\x91\x81`\x1F\x84\x01\x12\x15a\x01\xF3Wa\n\xE7`@\x93\x92\x83` \x86\x955\x91\x01a\nSV[\x84R` \x81\x015` \x85\x01R\x015\x91\x01RV[4a\x01\xF3W``6`\x03\x19\x01\x12a\x01\xF3W`\x045a\x0B\x17\x81a\t\xF7V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3Wa\x0B6\x906\x90`\x04\x01a\n\x08V[\x91\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3Wa\x0BW\x906\x90`\x04\x01a\n\x89V[\x92a\x0B\x8C3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B\xE0V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x01\xF3W_\x80\x94a\t\xBDa\x0B\xFA\x97`@Q\x98\x89\x97\x88\x96\x87\x95c\x0F\x10\xCC\xF1`\xE1\x1B\x87R`\x01\x80`\xA0\x1B\x03\x16`\x04\x87\x01R```$\x87\x01R`d\x86\x01\x91a\x1CmV[\x83\x81\x03`\x03\x19\x01`D\x85\x01R\x90a\x1C\xCFV[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3Wa\x0C$a*EV[`hTb\t:\x80\x81\x01\x80\x91\x11a\x0C\xBDWB\x10a\x0CaW`gTa\x0CO\x90`\x01`\x01`\xA0\x1B\x03\x16a*\x9DV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FServiceManager: Slasher proposal`D\x82\x01Rm\x08\x19\x19[\x18^H\x1B\x9B\xDD\x08\x1BY]`\x92\x1B`d\x82\x01R`\x84\x90\xFD[a\x1AjV[4a\x01\xF3W` 6`\x03\x19\x01\x12a\x01\xF3Wa\x07\xECa\x0C\xEA`\x045a\x0C\xE5\x81a\t\xF7V[a\x1DhV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x02\xB9V[4a\x01\xF3W` 6`\x03\x19\x01\x12a\x01\xF3W`\x045a\r\x1B\x81a\t\xF7V[a\r#a*EV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`eU\0[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xF3W\x815a\r\x98\x81a\x08tV[\x92a\r\xA6`@Q\x94\x85a\x08SV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x01\xF3W` \x01\x90[\x82\x82\x10a\r\xCEWPPP\x90V[` \x80\x91\x835a\r\xDD\x81a\t\xF7V[\x81R\x01\x91\x01\x90a\r\xC1V[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xF3W\x81` a\x03,\x935\x91\x01a\nSV[4a\x01\xF3W` 6`\x03\x19\x01\x12a\x01\xF3W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3W`\xA0`\x03\x19\x826\x03\x01\x12a\x01\xF3W`@Qa\x0E?\x81a\x083V[\x81`\x04\x015a\x0EM\x81a\t\xF7V[\x81Ra\x0E[`$\x83\x01a\x08\x8BV[` \x82\x01R`D\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3Wa\x0E\x83\x90`\x046\x91\x85\x01\x01a\r\x81V[`@\x82\x01R`d\x82\x015``\x82\x01R`\x84\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3Wa\t\xE1\x92`\x04a\x0E\xB8\x926\x92\x01\x01a\r\xE8V[`\x80\x82\x01Ra \xA1V[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W` `@Qb\t:\x80\x81R\xF3[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3Wa\x0F;a*EV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xF3W` 6`\x03\x19\x01\x12a\x01\xF3W\x7F/\x8A\xFC\x8Ax\xFD\x95\x8F3\x01\xC0#:\xA3&\xB9\xC4\xB9\xA2\x88Jt\x83\"}k\x05U\xAA\xA0:\xDBa\x10H`\x045a\x10\x03\x81a\t\xF7V[a\x10\x0Ba*EV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90UB`h\x81\x90U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1\0[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W` `\xFF`iT\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xF3W_`@6`\x03\x19\x01\x12a\x01\xF3W`\x045a\x10\xB5\x81a\t\xF7V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3Wa\x10\xD4\x906\x90`\x04\x01a\n\x89V[\x90a\x11\t3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B\xE0V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x01\xF3Wa\x11s\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\x99&\xEE}`\xE0\x1B\x84R`\x01\x80`\xA0\x1B\x03\x16`\x04\x84\x01R`@`$\x84\x01R`D\x83\x01\x90a\x1C\xCFV[\x03\x92Z\xF1\x80\x15a\x05!Wa\x11\x85WP\x80\xF3[a\t\xE1\x91P_\x90a\x08SV[4a\x01\xF3W_` 6`\x03\x19\x01\x12a\x01\xF3W`\x045a\x11\xAF\x81a\t\xF7V[a\x11\xE33\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B\xE0V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x01\xF3W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x05!Wa\x11\x85WP\x80\xF3[4a\x01\xF3W_` 6`\x03\x19\x01\x12a\x01\xF3W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3Wa\x12\x7F\x906\x90`\x04\x01a\r\xE8V[a\x12\x87a*EV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x01\xF3W`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R` `\x04\x82\x01R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x11s\x90`$\x83\x01\x90a\x1C\xABV[4a\x01\xF3Wa\x12\xF36a\t\x01V[a\x13'3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B\xE0V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x01\xF3W`@Qc\xAF\xE0.\xD5`\xE0\x1B\x81R` `\x04\x82\x01R\x90_\x90\x82\x90a\x13\x80\x90`$\x83\x01\x90a\x01\xF7V[\x93\x81\x83\x81\x81\x97\x03\x92Z\xF1\x80\x15a\x05!Wa\x11\x85WP\x80\xF3[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W`fT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3Wa\x13\xD8a*EV[`\x01`iTa\x13\xEA`\xFF\x82\x16\x15a\"/V[`\xFF\x19\x16\x17`iU\0[4a\x01\xF3W_`@6`\x03\x19\x01\x12a\x01\xF3W`\x045a\x14\x12\x81a\t\xF7V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3Wa\x141\x906\x90`\x04\x01a\n\x08V[\x91\x90a\x14g3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B\xE0V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x01\xF3Wa\x11s\x93_\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93c\xC1\xA8\xE2\xC5`\xE0\x1B\x85R`\x01\x80`\xA0\x1B\x03\x16`\x04\x85\x01R`@`$\x85\x01R`D\x84\x01\x91a\x1CmV[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xF3W`@6`\x03\x19\x01\x12a\x01\xF3W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3W6`#\x82\x01\x12\x15a\x01\xF3W\x80`\x04\x015a\x15P\x81a\x08tV[\x91a\x15^`@Q\x93\x84a\x08SV[\x81\x83R`$` \x84\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x01\xF3W`$\x81\x01\x92[\x82\x84\x10a\x15\xACW`$5\x85`\x01`\x01`@\x1B\x03\x82\x11a\x01\xF3Wa\x15\xA6a\t\xE1\x926\x90`\x04\x01a\r\x81V[\x90a\"\x8FV[\x835`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3W` \x91a\x15\xD0\x83\x92`$6\x91\x87\x01\x01a\x08\x9CV[\x81R\x01\x93\x01\x92a\x15|V[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W`gT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3Wa\x07\xECa\x0C\xEAa#@V[4a\x01\xF3W` 6`\x03\x19\x01\x12a\x01\xF3W`\x045a\x16;\x81a\t\xF7V[a\x16Ca*EV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x16\x9AW`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W` `hT`@Q\x90\x81R\xF3[4a\x01\xF3W` 6`\x03\x19\x01\x12a\x01\xF3W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3Wa\x17c\x906\x90`\x04\x01a\n\x08V[`eT\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x19yW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91_[\x81\x81\x10a\x17\xD7WP\x82;\x15a\x01\xF3Wa\t\xBD\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xFC\xE3l}`\xE0\x1B\x84R`\x04\x84\x01a&\x1EV[_` a\x182a\x17\xF5a\x05\x82\x83a\x17\xEF\x87\x89\x8Ba%mV[\x01a%\x8FV[`@a\x18\x02\x86\x88\x8Aa%mV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x91\x015`D\x82\x01R\x93\x84\x92\x83\x91\x90\x82\x90`d\x82\x01\x90V[\x03\x92Z\xF1\x80\x15a\x05!Wa\x19]W[Pa\x18Ua\x05\x82` a\x17\xEF\x84\x86\x88a%mV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x86\x16`$\x82\x01R\x91\x90` \x90\x83\x90`D\x90\x82\x90Z\xFA\x80\x15a\x05!Wa\x18\xF8` \x91_\x94\x85\x91a\x19@W[Pa\x18\xC5a\x18\xB0a\x05\x82\x85a\x17\xEF\x88\x8B\x8Da%mV[\x91`@a\x18\xBE\x87\x8A\x8Ca%mV[\x015a\x1C\xF9V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x94\x85\x92\x83\x91\x90\x82\x90`D\x82\x01\x90V[\x03\x92Z\xF1\x91\x82\x15a\x05!W`\x01\x92a\x19\x12W[P\x01a\x17\xA5V[a\x192\x90` =\x81\x11a\x199W[a\x19*\x81\x83a\x08SV[\x81\x01\x90a%\x99V[P_a\x19\x0BV[P=a\x19 V[a\x19W\x91P\x83=\x81\x11a\x07\xD3Wa\x07\xC5\x81\x83a\x08SV[_a\x18\x9AV[a\x19t\x90` =\x81\x11a\x199Wa\x19*\x81\x83a\x08SV[a\x18AV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FServiceManagerBase.onlyRewardsIn`D\x82\x01R\x7Fitiator: caller is not the rewar`d\x82\x01Rk29\x904\xB74\xBA4\xB0\xBA7\xB9`\xA1\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x90\x81` \x91\x03\x12a\x01\xF3WQ`\xFF\x81\x16\x81\x03a\x01\xF3W\x90V[`@Q=_\x82>=\x90\xFD[`@Q\x90a\x1A,` \x83a\x08SV[_\x80\x83R6` \x84\x017V[\x90a\x1AB\x82a\x08tV[a\x1AO`@Q\x91\x82a\x08SV[\x82\x81R\x80\x92a\x1A``\x1F\x19\x91a\x08tV[\x01\x90` 6\x91\x017V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x16`\xFF\x81\x14a\x0C\xBDW`\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x01\xF3WQa\x03,\x81a\t\xF7V[` \x81\x83\x03\x12a\x01\xF3W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xF3W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x01\xF3W\x81Qa\x1A\xD7\x81a\x08tV[\x92a\x1A\xE5`@Q\x94\x85a\x08SV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x01\xF3W` \x01\x90[\x82\x82\x10a\x1B\rWPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a\x1B\0V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q\x82\x10\x15a\x1BEW` \x91`\x05\x1B\x01\x01\x90V[a\x1B\x1DV[_\x19\x81\x14a\x0C\xBDW`\x01\x01\x90V[\x90a\x1Bb\x82a\x08tV[a\x1Bo`@Q\x91\x82a\x08SV[\x82\x81R\x80\x92a\x1B\x80`\x1F\x19\x91a\x08tV[\x01\x90_[\x82\x81\x10a\x1B\x90WPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a\x1B\x84V[\x90\x81` \x91\x03\x12a\x01\xF3WQ\x90V[\x90\x81` \x91\x03\x12a\x01\xF3WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x01\xF3W\x90V[\x90\x81Q\x81\x10\x15a\x1BEW\x01` \x01\x90V[\x15a\x1B\xE7WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FServiceManagerBase.onlyRegistryC`D\x82\x01R\x7Foordinator: caller is not the re`d\x82\x01Rq3\xB4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a\x1C\x86WPPP\x90V[\x90\x91\x92` \x80`\x01\x92c\xFF\xFF\xFF\xFFa\x1C\x9D\x88a\x08\x8BV[\x16\x81R\x01\x94\x01\x92\x91\x01a\x1CyV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90`@\x80a\x1C\xE6\x84Q``\x85R``\x85\x01\x90a\x1C\xABV[\x93` \x81\x01Q` \x85\x01R\x01Q\x91\x01R\x90V[\x91\x90\x82\x01\x80\x92\x11a\x0C\xBDWV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x01\xF3WV[\x90\x81`@\x91\x03\x12a\x01\xF3W`@Q\x90`@\x82\x01\x90\x82\x82\x10`\x01`\x01`@\x1B\x03\x83\x11\x17a\x08NW` \x91`@R\x80Qa\x1DS\x81a\t\xF7V[\x83R\x01Qa\x1D`\x81a\x1D\x06V[` \x82\x01R\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x05!Wa\x1D\xE7\x91` \x91_\x91a \x84WP`@Q\x80\x93\x81\x92c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x85Z\xFA\x90\x81\x15a\x05!W_\x91a eW[P`\x01`\x01`\xC0\x1B\x03\x16\x90\x81\x15\x90\x81\x15a \x12W[Pa \tWa\x1E\x1D\x90a)\xA6V[_\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x83[\x81Q\x85\x10\x15a\x1E\xD5Wa\x1E\x8E` a\x1Eka\x07;a\x07-\x89\x87a\x1B\xCFV[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x87Z\xFA\x80\x15a\x05!W`\x01\x92a\x1E\xAD\x92_\x92a\x1E\xB5W[Pa\x1C\xF9V[\x94\x01\x93a\x1EMV[a\x1E\xCE\x91\x92P` =\x81\x11a\x07\xD3Wa\x07\xC5\x81\x83a\x08SV[\x90_a\x1E\xA7V[a\x1E\xE0\x91\x94Pa\x1A8V[\x92_\x90_[\x81Q\x81\x10\x15a \x03Wa\x1E\xFEa\x07;a\x07-\x83\x85a\x1B\xCFV[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x90` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x05!W_\x92a\x1F\xE3W[P\x90_\x91[\x81\x83\x10a\x1FCWPPP`\x01\x01a\x1E\xE5V[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x05!Wa\x1F\xA7\x8Ba\x04\xC9\x83a\x1F\xA1a\x05\x82`\x01\x98a\x1F\xAC\x98_\x91a\x1F\xB5W[PQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92a\x1B1V[a\x1BJV[\x95\x01\x91\x90a\x1F1V[a\x1F\xD6\x91P`@=\x81\x11a\x1F\xDCW[a\x1F\xCE\x81\x83a\x08SV[\x81\x01\x90a\x1D\x1CV[_a\x1F\x93V[P=a\x1F\xC4V[a\x1F\xFC\x91\x92P` =\x81\x11a\x07\xD3Wa\x07\xC5\x81\x83a\x08SV[\x90_a\x1F,V[PPPPV[Pa\x03,a\x1A\x1DV[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x91P` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x80\x15a\x05!W`\xFF\x91_\x91a FW[P\x16\x15_a\x1E\x0FV[a _\x91P` =` \x11a\x08\x18Wa\x08\n\x81\x83a\x08SV[_a =V[a ~\x91P` =` \x11a\x07\xA6Wa\x07\x98\x81\x83a\x08SV[_a\x1D\xFAV[a \x9B\x91P\x82=\x84\x11a\x07\xD3Wa\x07\xC5\x81\x83a\x08SV[_a\x06\xB8V[`fT`\x01`\x01`\xA0\x1B\x03\x163\x03a!\xB3W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x01\xF3W`@\x80Qc`\xDB\x99\xA3`\xE0\x1B\x81R` `\x04\x82\x01\x81\x90R\x84Q`\x01`\x01`\xA0\x1B\x03\x16`$\x83\x01R\x80\x85\x01Qc\xFF\xFF\xFF\xFF\x16`D\x83\x01R\x91\x84\x01Q`\xA0`d\x83\x01R\x80Q`\xC4\x83\x01\x81\x90R\x91\x94\x85\x93`\xE4\x85\x01\x93\x91\x92\x01\x90_[\x81\x81\x10a!\x91WPPP_\x83a!p\x82\x96\x94`\x80\x85``\x85\x97\x01Q`\x84\x86\x01R\x01Q`#\x19\x84\x83\x03\x01`\xA4\x85\x01Ra\x1C\xABV[\x03\x92Z\xF1\x80\x15a\x05!Wa!\x81WPV[\x80a\t\xDB_a!\x8F\x93a\x08SV[V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x87\x95P` \x94\x85\x01\x94\x90\x92\x01\x91`\x01\x01a!=V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FServiceManagerBase.onlySlasher: `D\x82\x01R\x7Fcaller is not the slasher\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x90` a\x03,\x92\x81\x81R\x01\x90a\x01\xF7V[\x15a\"6WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FServiceManager: Migration Alread`D\x82\x01Rj\x1EH\x11\x9A[\x98[\x1A^\x99Y`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[\x90a\"\x98a*EV[a\"\xA7`\xFF`iT\x16\x15a\"/V[a\"\xB4\x81Q\x83Q\x14a*\xFBV[_[\x81Q\x81\x10\x15a\"\xEAW\x80a\"\xE3a\"\xD2a\x05u`\x01\x94\x86a\x1B1V[a\"\xDC\x83\x87a\x1B1V[Q\x90a+\xDBV[P\x01a\"\xB6V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x01\xF3Wa!p\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xEF-\xFA\x8D`\xE0\x1B\x84R`\x04\x84\x01a+[V[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x05!W`\xFF\x91_\x91a%NW[P\x16\x80\x15a%DW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90_\x90\x81\x90[\x80\x83\x10a%\0WPa#\xDB\x91Pa\x1A8V[\x92_\x90_[`@Qc\x9A\xA1e=`\xE0\x1B\x81R` \x81`\x04\x81\x89Z\xFA\x80\x15a\x05!W`\xFF\x91_\x91a$\xE2W[P\x16\x81\x10\x15a$\xDBW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01\x81\x90R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x05!W_\x92a$\xBBW[P\x90_\x91[\x81\x83\x10a$UWPPP`\x01\x01a#\xE0V[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x05!Wa\x1F\xA7\x8Ba\x04\xC9\x83a\x1F\xA1a\x05\x82`\x01\x98a$\xB2\x98_\x91a\x1F\xB5WPQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95\x01\x91\x90a$CV[a$\xD4\x91\x92P` =\x81\x11a\x07\xD3Wa\x07\xC5\x81\x83a\x08SV[\x90_a$>V[P\x92PPPV[a$\xFA\x91P` =\x81\x11a\x08\x18Wa\x08\n\x81\x83a\x08SV[_a$\x06V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x90\x91\x90` \x81`$\x81\x88Z\xFA\x80\x15a\x05!W`\x01\x92a%;\x92_\x92a\x1E\xB5WPa\x1C\xF9V[\x92\x01\x91\x90a#\xC9V[PPa\x03,a\x1A\x1DV[a%g\x91P` =` \x11a\x08\x18Wa\x08\n\x81\x83a\x08SV[_a#\x91V[\x91\x90\x81\x10\x15a\x1BEW`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x01\xF3W\x01\x90V[5a\x03,\x81a\t\xF7V[\x90\x81` \x91\x03\x12a\x01\xF3WQ\x80\x15\x15\x81\x03a\x01\xF3W\x90V[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a%\xCAWPPP\x90V[\x90\x91\x92`@\x80`\x01\x92\x865a%\xDE\x81a\t\xF7V[\x84\x80`\xA0\x1B\x03\x16\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x88\x015a&\x02\x81a\x1D\x06V[\x16` \x82\x01R\x01\x94\x01\x92\x91\x01a%\xBDV[5\x90a!\x8F\x82a\t\xF7V[\x91\x80` \x84\x01` \x85RR`@\x83\x01\x90`@\x81`\x05\x1B\x85\x01\x01\x93\x83_\x91`\x9E\x19\x826\x03\x01\x90[\x84\x84\x10a&UWPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`?\x19\x82\x82\x03\x01\x87R\x875\x83\x81\x12\x15a\x01\xF3W\x84\x01\x90\x815`\x1E\x19\x836\x03\x01\x81\x12\x15a\x01\xF3W\x82\x01\x91` \x835\x93\x01\x90`\x01`\x01`@\x1B\x03\x84\x11a\x01\xF3W\x83`\x06\x1B6\x03\x82\x13a\x01\xF3Wa'\x1E\x83`\x80a'\x13\x81a&\xC9` \x98\x97`\x01\x9A`\xA0\x8B\x9AR`\xA0\x87\x01\x91a%\xB1V[\x95a&\xE7a&\xD8\x89\x83\x01a&\x13V[`\x01`\x01`\xA0\x1B\x03\x16\x86\x8A\x01RV[`@\x81\x015`@\x86\x01Ra'\ra'\0``\x83\x01a\x08\x8BV[c\xFF\xFF\xFF\xFF\x16``\x87\x01RV[\x01a\x08\x8BV[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x99\x01\x97\x01\x95\x94\x01\x92\x91\x90a&DV[\x91\x90\x82\x03\x91\x82\x11a\x0C\xBDWV[\x80Q`\x01\x81\x11\x15a'\xECW`\x01\x1C\x91a'R\x83a\x1A8V[\x91a'fa'a\x85\x83Qa'-V[a\x1A8V[\x91_[\x85\x81\x10a'\xC9WP\x84[\x82Q\x81\x10\x15a'\xA8W\x80a'\xA2a'\x8Fa\x05u`\x01\x94\x87a\x1B1V[a\x04\xC9a'\x9C\x8A\x85a'-V[\x88a\x1B1V[\x01a'sV[P\x93PP\x90a'\xB6\x90a':V[a'\xC0\x90\x91a':V[a\x03,\x91a'\xF0V[\x80a'\xE6a'\xDCa\x05u`\x01\x94\x87a\x1B1V[a\x04\xC9\x83\x89a\x1B1V[\x01a'iV[P\x90V[\x91\x82Q\x92\x82Qa(\x03a'a\x82\x87a\x1C\xF9V[\x93_\x93_\x92_\x97[\x80\x87\x10\x80a)uW[\x15a(\xF6Wa(&a\x05u\x88\x88a\x1B1V[a(6a\x05\x82a\x05u\x88\x88a\x1B1V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x10\x15a(xWa(sa(`a\x05ua(Y\x8Aa\x1BJV[\x99\x89a\x1B1V[a\x04\xC9a(l\x8Ca\x1BJV[\x9B\x8Ba\x1B1V[a(\x0BV[a(\x85a\x05u\x88\x88a\x1B1V[a(\x95a\x05\x82a\x05u\x88\x88a\x1B1V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x11\x15a(\xBFWa(sa(`a\x05ua(\xB8\x88a\x1BJV[\x97\x87a\x1B1V[\x93a(\xF0\x90a\x1F\xA7a(\xDDa\x05ua(\xD6\x8Ba\x1BJV[\x9A\x8Aa\x1B1V[a\x04\xC9a(\xE9\x8Da\x1BJV[\x9C\x8Ca\x1B1V[\x93a(\x0BV[\x97\x95\x91\x97\x94\x90\x92\x93\x94[\x80\x83\x10a)KWPPP[\x80\x83\x10a)\x1AWPPP\x81R\x90V[a)Fa)3a\x05ua),\x86a\x1BJV[\x95\x85a\x1B1V[a\x04\xC9a)?\x87a\x1BJV[\x96\x88a\x1B1V[a)\x0BV[a)pa)]a\x05ua),\x86a\x1BJV[a\x04\xC9a)i\x8Aa\x1BJV[\x99\x8Ba\x1B1V[a)\0V[P\x81\x85\x10a(\x14V[\x90a)\x88\x82a\n8V[a)\x95`@Q\x91\x82a\x08SV[\x82\x81R\x80\x92a\x1A``\x1F\x19\x91a\n8V[_\x81\x80[a* WPa)\xBC\x90a\xFF\xFF\x16a)~V[__[\x82Q\x82\x10\x80a*\x15W[\x15a*\x0EW`\x01\x81\x1B\x84\x16a)\xE7W[a)\xE2\x90a\x1BJV[a)\xBFV[\x90`\x01a)\xE2\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa*\x04\x82\x87a\x1B\xCFV[S\x01\x91\x90Pa)\xD9V[PP\x90P\x90V[Pa\x01\0\x81\x10a)\xC9V[_\x19\x81\x01\x81\x81\x11a\x0C\xBDWa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a\x0C\xBDW`\x01\x01\x90\x80a)\xAAV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a*YWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`fT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE0\xD4\x9AT'D#\x18=\xAD\xEC\xBD\xF29\xEA\xACn\x06\xBA\x882\x0B&\xFE\x8C\xC5\xEC\x9D\x05\nc\x95\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`fUV[\x15a+\x02WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FServiceManager: Input array leng`D\x82\x01Rj\x0E\x8D\x04\r\xAD.m\xAC.\x8Cm`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x91a+ra\x03,\x93`@\x84R`@\x84\x01\x90a\x02\xB9V[\x91` \x81\x84\x03\x91\x01Ra\x020V[\x15a+\x87WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FServiceManager: Operator not in `D\x82\x01Requorum`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x92\x91\x90_\x90` \x81`$\x81\x88Z\xFA\x80\x15a\x05!Wa,_\x95` \x92_\x92a,\xE8W[P`@Q\x80\x80\x98\x81\x94c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91Z\xFA\x93\x84\x15a\x05!W_\x94a,\xC7W[P_[\x82Q\x81\x10\x15a,\xC1W\x80a,\xBBa,\xB6a,\xA0a\x07Aa,\x96`\x01\x96\x89a\x1B1V[Qc\xFF\xFF\xFF\xFF\x16\x90V[\x84\x80`\xC0\x1B\x03\x89\x16`\xFF`\x01\x92\x16\x1C\x16`\x01\x14\x90V[a+\x80V[\x01a,tV[P\x92PPV[a,\xE1\x91\x94P` =` \x11a\x07\xA6Wa\x07\x98\x81\x83a\x08SV[\x92_a,qV[a-\0\x91\x92P\x83=\x85\x11a\x07\xD3Wa\x07\xC5\x81\x83a\x08SV[\x90_a,>V\xFE\xA2dipfsX\"\x12 \xD6\x1EHR\xE99\x8A\x08\x83\x05/\xB7Gd\xE0\xE4\x02\x04\x06\xED\xFD\xAFC\x075\xF3\xCDT\x96\xFE\x9E\x8CdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c80630b91d665146101e457806315b7bc9a146101df5780631e2199e2146101da57806326f017e2146101d557806333cfb7b7146101d05780633bc28c8c146101cb57806360db99a3146101c657806367940c89146101c15780636b3aa72e146101bc578063715018a6146101b757806377ef731d146101b25780638999817f146101ad5780638d68349a146101a85780638da5cb5b146101a35780639926ee7d1461019e578063a364f4da14610199578063a98fb35514610194578063afe02ed51461018f578063b13442711461018a578063b78b608714610185578063c1a8e2c514610180578063ca8aa7c71461017b578063d9f9537714610176578063e46f181614610171578063e481af9d1461016c578063f2fde38b14610167578063fc299dee14610162578063fcd1c3751461015d5763fce36c7d14610158575f80fd5b611733565b611716565b6116ee565b61161e565b611603565b6115db565b611515565b6114d1565b6113f4565b6113c0565b611398565b6112e5565b61124e565b611191565b611097565b61106f565b61104d565b610fc2565b610f7e565b610f23565b610edf565b610ec2565b610e03565b610cfe565b610cc2565b610c0c565b610afa565b61092b565b61032f565b5f9103126101f357565b5f80fd5b90602080835192838152019201905f5b8181106102145750505090565b825163ffffffff16845260209384019390920191600101610207565b9080602083519182815201916020808360051b8301019401925f915b83831061025b57505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b80821061029b575050506020806001929701930193019193929061024c565b90919260208060019263ffffffff875116815201940192019061027c565b90602080835192838152019201905f5b8181106102d65750505090565b82516001600160a01b03168452602093840193909201916001016102c9565b9161031e9061031061032c95936060865260608601906101f7565b908482036020860152610230565b9160408184039101526102b9565b90565b346101f3575f3660031901126101f357604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156105215760ff915f916107f0575b50169061039b611a1d565b6103a483611a38565b914363ffffffff16905f5b60ff8116948686101561064957604051634f4c91e160e11b815294602086600481875afa958615610521575f96610629575b50604051638902624560e01b815260ff8416600482015263ffffffff86166024820152955f90879060449082906001600160a01b03165afa958615610521575f96610605575b506104328651611a38565b975f915b875183101561054357604051632efa2ca360e11b81526020816004818a5afa90811561052157610497916020915f91610526575b50610475868c611b31565b519060405180809581946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa918215610521576104d88c600194876104e7955f926104ef575b506104c99192611b31565b6001600160a01b039091169052565b6104e18c61273a565b906127f0565b920191610436565b6104c992506105149060203d811161051a575b61050c8183610853565b810190611a8f565b916104be565b503d610502565b611a12565b61053d9150823d811161051a5761050c8183610853565b5f61046a565b96509750509490916105558551611a38565b965f945f5b87518110156105ce57898861058e6105826105758584611b31565b516001600160a01b031690565b6001600160a01b031690565b61059d575b505060010161055a565b8291986104c96105b56105756001966105c595611b31565b916105bf81611b4a565b9b611b31565b90508988610593565b509488529596939450916105fa916105f5906105ea818a611b31565b9063ffffffff169052565b611a7e565b9493949291926103af565b6106229196503d805f833e61061a8183610853565b810190611aa4565b945f610427565b61064291965060203d811161051a5761050c8183610853565b945f6103e1565b849291506106578351611b58565b915f5b84518110156107da5761069e60206106756105758489611b31565b6040516309aa152760e11b81526001600160a01b03909116600482015291829081906024820190565b0381865afa908115610521576106d8916020915f916107ad575b506040518093819263871ef04960e01b8352600483019190602083019252565b0381865afa8015610521576106fd915f9161077f575b506001600160c01b03166129a6565b916107088351611a38565b935f5b8451811015610757578061075161074761074161073b61072d6001968b611bcf565b516001600160f81b03191690565b60f81c90565b60ff1690565b6105ea838a611b31565b0161070b565b50936001929196935061076a8287611b31565b526107758186611b31565b500193909361065a565b6107a0915060203d81116107a6575b6107988183610853565b810190611bb0565b876106ee565b503d61078e565b6107cd9150823d81116107d3575b6107c58183610853565b810190611ba1565b886106b8565b503d6107bb565b50506107ec83604051938493846102f5565b0390f35b610812915060203d602011610818575b61080a8183610853565b8101906119f9565b5f610390565b503d610800565b634e487b7160e01b5f52604160045260245ffd5b60a081019081106001600160401b0382111761084e57604052565b61081f565b90601f801991011681019081106001600160401b0382111761084e57604052565b6001600160401b03811161084e5760051b60200190565b359063ffffffff821682036101f357565b9080601f830112156101f35781356108b381610874565b926108c16040519485610853565b81845260208085019260051b8201019283116101f357602001905b8282106108e95750505090565b602080916108f68461088b565b8152019101906108dc565b60206003198201126101f357600435906001600160401b0382116101f35761032c9160040161089c565b346101f35761093936610901565b610941612a45565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156101f35760405163aec205c560e01b81525f8160048183865af18015610521576109e3575b50803b156101f35760405163afe02ed560e01b8152905f9082908183816109bd886004830161221e565b03925af18015610521576109cd57005b806109db5f6109e193610853565b806101e9565b005b806109db5f6109f193610853565b5f610993565b6001600160a01b038116036101f357565b9181601f840112156101f3578235916001600160401b0383116101f3576020808501948460051b0101116101f357565b6001600160401b03811161084e57601f01601f191660200190565b929192610a5f82610a38565b91610a6d6040519384610853565b8294818452818301116101f3578281602093845f960137010152565b9190916060818403126101f35760405190606082018281106001600160401b0382111761084e57604052819381356001600160401b0381116101f35782019181601f840112156101f357610ae7604093928360208695359101610a53565b8452602081013560208501520135910152565b346101f35760603660031901126101f357600435610b17816109f7565b6024356001600160401b0381116101f357610b36903690600401610a08565b91906044356001600160401b0381116101f357610b57903690600401610a89565b92610b8c337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611be0565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156101f3575f80946109bd610bfa9760405198899788968795630f10ccf160e11b875260018060a01b03166004870152606060248701526064860191611c6d565b83810360031901604485015290611ccf565b346101f3575f3660031901126101f357610c24612a45565b60685462093a808101809111610cbd574210610c6157606754610c4f906001600160a01b0316612a9d565b606780546001600160a01b0319169055005b60405162461bcd60e51b815260206004820152602e60248201527f536572766963654d616e616765723a20536c61736865722070726f706f73616c60448201526d0819195b185e481b9bdd081b595d60921b6064820152608490fd5b611a6a565b346101f35760203660031901126101f3576107ec610cea600435610ce5816109f7565b611d68565b6040519182916020835260208301906102b9565b346101f35760203660031901126101f357600435610d1b816109f7565b610d23612a45565b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555005b9080601f830112156101f3578135610d9881610874565b92610da66040519485610853565b81845260208085019260051b8201019283116101f357602001905b828210610dce5750505090565b602080918335610ddd816109f7565b815201910190610dc1565b9080601f830112156101f35781602061032c93359101610a53565b346101f35760203660031901126101f3576004356001600160401b0381116101f35760a060031982360301126101f357604051610e3f81610833565b8160040135610e4d816109f7565b8152610e5b6024830161088b565b602082015260448201356001600160401b0381116101f357610e839060043691850101610d81565b60408201526064820135606082015260848201356001600160401b0381116101f3576109e1926004610eb89236920101610de8565b60808201526120a1565b346101f3575f3660031901126101f357602060405162093a808152f35b346101f3575f3660031901126101f3576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346101f3575f3660031901126101f357610f3b612a45565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346101f3575f3660031901126101f3576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346101f35760203660031901126101f3577f2f8afc8a78fd958f3301c0233aa326b9c4b9a2884a7483227d6b0555aaa03adb611048600435611003816109f7565b61100b612a45565b606780546001600160a01b0319166001600160a01b0392909216918217905542606881905560408051928352602083019190915290918291820190565b0390a1005b346101f3575f3660031901126101f357602060ff606954166040519015158152f35b346101f3575f3660031901126101f3576033546040516001600160a01b039091168152602090f35b346101f3575f60403660031901126101f3576004356110b5816109f7565b6024356001600160401b0381116101f3576110d4903690600401610a89565b90611109337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611be0565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b156101f357611173925f9283604051809681958294639926ee7d60e01b845260018060a01b03166004840152604060248401526044830190611ccf565b03925af1801561052157611185575080f35b6109e191505f90610853565b346101f3575f60203660031901126101f3576004356111af816109f7565b6111e3337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611be0565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156101f3576040516351b27a6d60e11b81526001600160a01b039091166004820152905f908290602490829084905af1801561052157611185575080f35b346101f3575f60203660031901126101f3576004356001600160401b0381116101f35761127f903690600401610de8565b611287612a45565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156101f35760405163a98fb35560e01b815260206004820152915f918391829084908290611173906024830190611cab565b346101f3576112f336610901565b611327337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611be0565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156101f35760405163afe02ed560e01b815260206004820152905f9082906113809060248301906101f7565b93818381819703925af1801561052157611185575080f35b346101f3575f3660031901126101f3576066546040516001600160a01b039091168152602090f35b346101f3575f3660031901126101f3576113d8612a45565b60016069546113ea60ff82161561222f565b60ff191617606955005b346101f3575f60403660031901126101f357600435611412816109f7565b6024356001600160401b0381116101f357611431903690600401610a08565b9190611467337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611be0565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156101f357611173935f80946040519687958694859363c1a8e2c560e01b855260018060a01b03166004850152604060248501526044840191611c6d565b346101f3575f3660031901126101f3576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346101f35760403660031901126101f3576004356001600160401b0381116101f357366023820112156101f357806004013561155081610874565b9161155e6040519384610853565b8183526024602084019260051b820101903682116101f35760248101925b8284106115ac57602435856001600160401b0382116101f3576115a66109e1923690600401610d81565b9061228f565b83356001600160401b0381116101f3576020916115d083926024369187010161089c565b81520193019261157c565b346101f3575f3660031901126101f3576067546040516001600160a01b039091168152602090f35b346101f3575f3660031901126101f3576107ec610cea612340565b346101f35760203660031901126101f35760043561163b816109f7565b611643612a45565b6001600160a01b0381161561169a57603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3005b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b346101f3575f3660031901126101f3576065546040516001600160a01b039091168152602090f35b346101f3575f3660031901126101f3576020606854604051908152f35b346101f35760203660031901126101f3576004356001600160401b0381116101f357611763903690600401610a08565b6065549091906001600160a01b03163303611979577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316915f5b8181106117d75750823b156101f3576109bd925f928360405180968195829463fce36c7d60e01b84526004840161261e565b5f60206118326117f5610582836117ef87898b61256d565b0161258f565b604061180286888a61256d565b6040516323b872dd60e01b8152336004820152306024820152910135604482015293849283919082906064820190565b03925af180156105215761195d575b5061185561058260206117ef84868861256d565b604051636eb1769f60e11b81523060048201526001600160a01b03861660248201529190602090839060449082905afa8015610521576118f86020915f948591611940575b506118c56118b0610582856117ef888b8d61256d565b9160406118be878a8c61256d565b0135611cf9565b60405163095ea7b360e01b81526001600160a01b038a166004820152602481019190915294859283919082906044820190565b03925af191821561052157600192611912575b50016117a5565b6119329060203d8111611939575b61192a8183610853565b810190612599565b505f61190b565b503d611920565b6119579150833d81116107d3576107c58183610853565b5f61189a565b6119749060203d81116119395761192a8183610853565b611841565b60405162461bcd60e51b815260206004820152604c60248201527f536572766963654d616e61676572426173652e6f6e6c7952657761726473496e60448201527f69746961746f723a2063616c6c6572206973206e6f742074686520726577617260648201526b32399034b734ba34b0ba37b960a11b608482015260a490fd5b908160209103126101f3575160ff811681036101f35790565b6040513d5f823e3d90fd5b60405190611a2c602083610853565b5f808352366020840137565b90611a4282610874565b611a4f6040519182610853565b8281528092611a60601f1991610874565b0190602036910137565b634e487b7160e01b5f52601160045260245ffd5b60ff1660ff8114610cbd5760010190565b908160209103126101f3575161032c816109f7565b6020818303126101f3578051906001600160401b0382116101f357019080601f830112156101f3578151611ad781610874565b92611ae56040519485610853565b81845260208085019260051b8201019283116101f357602001905b828210611b0d5750505090565b8151815260209182019101611b00565b634e487b7160e01b5f52603260045260245ffd5b8051821015611b455760209160051b010190565b611b1d565b5f198114610cbd5760010190565b90611b6282610874565b611b6f6040519182610853565b8281528092611b80601f1991610874565b01905f5b828110611b9057505050565b806060602080938501015201611b84565b908160209103126101f3575190565b908160209103126101f357516001600160c01b03811681036101f35790565b908151811015611b45570160200190565b15611be757565b60405162461bcd60e51b815260206004820152605260248201527f536572766963654d616e61676572426173652e6f6e6c7952656769737472794360448201527f6f6f7264696e61746f723a2063616c6c6572206973206e6f742074686520726560648201527133b4b9ba393c9031b7b7b93234b730ba37b960711b608482015260a490fd5b916020908281520191905f5b818110611c865750505090565b90919260208060019263ffffffff611c9d8861088b565b168152019401929101611c79565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b90604080611ce68451606085526060850190611cab565b9360208101516020850152015191015290565b91908201809211610cbd57565b6bffffffffffffffffffffffff8116036101f357565b908160409103126101f3576040519060408201908282106001600160401b0383111761084e576020916040528051611d53816109f7565b83520151611d6081611d06565b602082015290565b6040516309aa152760e11b81526001600160a01b0391821660048201527f000000000000000000000000000000000000000000000000000000000000000090911690602081602481855afa90811561052157611de7916020915f9161208457506040518093819263871ef04960e01b8352600483019190602083019252565b0381855afa908115610521575f91612065575b506001600160c01b0316908115908115612012575b5061200957611e1d906129a6565b5f91907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690835b8151851015611ed557611e8e6020611e6b61073b61072d8987611bcf565b604051633ca5a5f560e01b815260ff909116600482015291829081906024820190565b0381875afa801561052157600192611ead925f92611eb5575b50611cf9565b940193611e4d565b611ece91925060203d81116107d3576107c58183610853565b905f611ea7565b611ee0919450611a38565b925f905f5b815181101561200357611efe61073b61072d8385611bcf565b604051633ca5a5f560e01b815260ff8216600482015290602082602481895afa918215610521575f92611fe3575b50905f915b818310611f4357505050600101611ee5565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa91821561052157611fa78b6104c983611fa1610582600198611fac985f91611fb5575b50516001600160a01b031690565b92611b31565b611b4a565b95019190611f31565b611fd6915060403d8111611fdc575b611fce8183610853565b810190611d1c565b5f611f93565b503d611fc4565b611ffc91925060203d81116107d3576107c58183610853565b905f611f2c565b50505050565b5061032c611a1d565b604051639aa1653d60e01b81529150602090829060049082905afa80156105215760ff915f91612046575b5016155f611e0f565b61205f915060203d6020116108185761080a8183610853565b5f61203d565b61207e915060203d6020116107a6576107988183610853565b5f611dfa565b61209b9150823d84116107d3576107c58183610853565b5f6106b8565b6066546001600160a01b031633036121b3577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156101f357604080516360db99a360e01b815260206004820181905284516001600160a01b031660248301528085015163ffffffff1660448301529184015160a06064830152805160c483018190529194859360e4850193919201905f5b818110612191575050505f83612170829694608085606085970151608486015201516023198483030160a4850152611cab565b03925af18015610521576121815750565b806109db5f61218f93610853565b565b82516001600160a01b031685528795506020948501949092019160010161213d565b60405162461bcd60e51b815260206004820152603960248201527f536572766963654d616e61676572426173652e6f6e6c79536c61736865723a2060448201527f63616c6c6572206973206e6f742074686520736c6173686572000000000000006064820152608490fd5b90602061032c9281815201906101f7565b1561223657565b60405162461bcd60e51b815260206004820152602b60248201527f536572766963654d616e616765723a204d6967726174696f6e20416c7265616460448201526a1e48119a5b985b1a5e995960aa1b6064820152608490fd5b90612298612a45565b6122a760ff606954161561222f565b6122b48151835114612afb565b5f5b81518110156122ea57806122e36122d261057560019486611b31565b6122dc8387611b31565b5190612bdb565b50016122b6565b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b156101f357612170925f928360405180968195829463ef2dfa8d60e01b845260048401612b5b565b604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156105215760ff915f9161254e575b50168015612544577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316905f9081905b80831061250057506123db9150611a38565b925f905f5b604051639aa1653d60e01b8152602081600481895afa80156105215760ff915f916124e2575b50168110156124db57604051633ca5a5f560e01b815260ff821660048201819052602082602481895afa918215610521575f926124bb575b50905f915b818310612455575050506001016123e0565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa91821561052157611fa78b6104c983611fa16105826001986124b2985f91611fb55750516001600160a01b031690565b95019190612443565b6124d491925060203d81116107d3576107c58183610853565b905f61243e565b5092505050565b6124fa915060203d81116108185761080a8183610853565b5f612406565b604051633ca5a5f560e01b815260ff84166004820152909190602081602481885afa80156105215760019261253b925f92611eb55750611cf9565b920191906123c9565b505061032c611a1d565b612567915060203d6020116108185761080a8183610853565b5f612391565b9190811015611b455760051b81013590609e19813603018212156101f3570190565b3561032c816109f7565b908160209103126101f3575180151581036101f35790565b916020908281520191905f5b8181106125ca5750505090565b90919260408060019286356125de816109f7565b848060a01b031681526bffffffffffffffffffffffff602088013561260281611d06565b1660208201520194019291016125bd565b359061218f826109f7565b9180602084016020855252604083019060408160051b85010193835f91609e1982360301905b848410612655575050505050505090565b90919293949596603f198282030187528735838112156101f3578401908135601e19833603018112156101f357820191602083359301906001600160401b0384116101f3578360061b360382136101f35761271e836080612713816126c96020989760019a60a08b9a5260a08701916125b1565b956126e76126d8898301612613565b6001600160a01b0316868a0152565b6040810135604086015261270d6127006060830161088b565b63ffffffff166060870152565b0161088b565b63ffffffff16910152565b99019701959401929190612644565b91908203918211610cbd57565b805160018111156127ec5760011c9161275283611a38565b9161276661276185835161272d565b611a38565b915f5b8581106127c95750845b82518110156127a857806127a261278f61057560019487611b31565b6104c961279c8a8561272d565b88611b31565b01612773565b50935050906127b69061273a565b6127c0909161273a565b61032c916127f0565b806127e66127dc61057560019487611b31565b6104c98389611b31565b01612769565b5090565b9182519282516128036127618287611cf9565b935f935f925f975b80871080612975575b156128f6576128266105758888611b31565b6128366105826105758888611b31565b6001600160a01b039091161015612878576128736128606105756128598a611b4a565b9989611b31565b6104c961286c8c611b4a565b9b8b611b31565b61280b565b6128856105758888611b31565b6128956105826105758888611b31565b6001600160a01b0390911611156128bf576128736128606105756128b888611b4a565b9787611b31565b936128f090611fa76128dd6105756128d68b611b4a565b9a8a611b31565b6104c96128e98d611b4a565b9c8c611b31565b9361280b565b9795919794909293945b80831061294b575050505b80831061291a57505050815290565b61294661293361057561292c86611b4a565b9585611b31565b6104c961293f87611b4a565b9688611b31565b61290b565b61297061295d61057561292c86611b4a565b6104c96129698a611b4a565b998b611b31565b612900565b50818510612814565b9061298882610a38565b6129956040519182610853565b8281528092611a60601f1991610a38565b5f81805b612a2057506129bc9061ffff1661297e565b5f5f5b8251821080612a15575b15612a0e576001811b84166129e7575b6129e290611b4a565b6129bf565b9060016129e29160ff60f81b8460f81b165f1a612a048287611bcf565b53019190506129d9565b5050905090565b5061010081106129c9565b5f198101818111610cbd5761ffff9116911661ffff8114610cbd5760010190806129aa565b6033546001600160a01b03163303612a5957565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b606654604080516001600160a01b038084168252841660208201529192917fe0d49a54274423183dadecbdf239eaac6e06ba88320b26fe8cc5ec9d050a63959190a16001600160a01b03166001600160a01b03199190911617606655565b15612b0257565b60405162461bcd60e51b815260206004820152602b60248201527f536572766963654d616e616765723a20496e707574206172726179206c656e6760448201526a0e8d040dad2e6dac2e8c6d60ab1b6064820152608490fd5b9091612b7261032c936040845260408401906102b9565b916020818403910152610230565b15612b8757565b60405162461bcd60e51b815260206004820152602660248201527f536572766963654d616e616765723a204f70657261746f72206e6f7420696e2060448201526571756f72756d60d01b6064820152608490fd5b6040516309aa152760e11b81526001600160a01b0391821660048201527f00000000000000000000000000000000000000000000000000000000000000009091169291905f90602081602481885afa801561052157612c5f956020925f92612ce8575b50604051808098819463871ef04960e01b8352600483019190602083019252565b03915afa938415610521575f94612cc7575b505f5b8251811015612cc15780612cbb612cb6612ca0610741612c9660019689611b31565b5163ffffffff1690565b848060c01b03891660ff600192161c1660011490565b612b80565b01612c74565b50925050565b612ce191945060203d6020116107a6576107988183610853565b925f612c71565b612d00919250833d85116107d3576107c58183610853565b905f612c3e56fea2646970667358221220d61e4852e9398a0883052fb74764e0e4020406edfdaf430735f3cd5496fe9e8c64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x0B\x91\xD6e\x14a\x01\xE4W\x80c\x15\xB7\xBC\x9A\x14a\x01\xDFW\x80c\x1E!\x99\xE2\x14a\x01\xDAW\x80c&\xF0\x17\xE2\x14a\x01\xD5W\x80c3\xCF\xB7\xB7\x14a\x01\xD0W\x80c;\xC2\x8C\x8C\x14a\x01\xCBW\x80c`\xDB\x99\xA3\x14a\x01\xC6W\x80cg\x94\x0C\x89\x14a\x01\xC1W\x80ck:\xA7.\x14a\x01\xBCW\x80cqP\x18\xA6\x14a\x01\xB7W\x80cw\xEFs\x1D\x14a\x01\xB2W\x80c\x89\x99\x81\x7F\x14a\x01\xADW\x80c\x8Dh4\x9A\x14a\x01\xA8W\x80c\x8D\xA5\xCB[\x14a\x01\xA3W\x80c\x99&\xEE}\x14a\x01\x9EW\x80c\xA3d\xF4\xDA\x14a\x01\x99W\x80c\xA9\x8F\xB3U\x14a\x01\x94W\x80c\xAF\xE0.\xD5\x14a\x01\x8FW\x80c\xB14Bq\x14a\x01\x8AW\x80c\xB7\x8B`\x87\x14a\x01\x85W\x80c\xC1\xA8\xE2\xC5\x14a\x01\x80W\x80c\xCA\x8A\xA7\xC7\x14a\x01{W\x80c\xD9\xF9Sw\x14a\x01vW\x80c\xE4o\x18\x16\x14a\x01qW\x80c\xE4\x81\xAF\x9D\x14a\x01lW\x80c\xF2\xFD\xE3\x8B\x14a\x01gW\x80c\xFC)\x9D\xEE\x14a\x01bW\x80c\xFC\xD1\xC3u\x14a\x01]Wc\xFC\xE3l}\x14a\x01XW_\x80\xFD[a\x173V[a\x17\x16V[a\x16\xEEV[a\x16\x1EV[a\x16\x03V[a\x15\xDBV[a\x15\x15V[a\x14\xD1V[a\x13\xF4V[a\x13\xC0V[a\x13\x98V[a\x12\xE5V[a\x12NV[a\x11\x91V[a\x10\x97V[a\x10oV[a\x10MV[a\x0F\xC2V[a\x0F~V[a\x0F#V[a\x0E\xDFV[a\x0E\xC2V[a\x0E\x03V[a\x0C\xFEV[a\x0C\xC2V[a\x0C\x0CV[a\n\xFAV[a\t+V[a\x03/V[_\x91\x03\x12a\x01\xF3WV[_\x80\xFD[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x02\x14WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x02\x07V[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x02[WPPPPP\x90V[\x90\x91\x92\x93\x94`\x1F\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x02\x9BWPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x02LV[\x90\x91\x92` \x80`\x01\x92c\xFF\xFF\xFF\xFF\x87Q\x16\x81R\x01\x94\x01\x92\x01\x90a\x02|V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x02\xD6WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x02\xC9V[\x91a\x03\x1E\x90a\x03\x10a\x03,\x95\x93``\x86R``\x86\x01\x90a\x01\xF7V[\x90\x84\x82\x03` \x86\x01Ra\x020V[\x91`@\x81\x84\x03\x91\x01Ra\x02\xB9V[\x90V[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x05!W`\xFF\x91_\x91a\x07\xF0W[P\x16\x90a\x03\x9Ba\x1A\x1DV[a\x03\xA4\x83a\x1A8V[\x91Cc\xFF\xFF\xFF\xFF\x16\x90_[`\xFF\x81\x16\x94\x86\x86\x10\x15a\x06IW`@QcOL\x91\xE1`\xE1\x1B\x81R\x94` \x86`\x04\x81\x87Z\xFA\x95\x86\x15a\x05!W_\x96a\x06)W[P`@Qc\x89\x02bE`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x86\x16`$\x82\x01R\x95_\x90\x87\x90`D\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x95\x86\x15a\x05!W_\x96a\x06\x05W[Pa\x042\x86Qa\x1A8V[\x97_\x91[\x87Q\x83\x10\x15a\x05CW`@Qc.\xFA,\xA3`\xE1\x1B\x81R` \x81`\x04\x81\x8AZ\xFA\x90\x81\x15a\x05!Wa\x04\x97\x91` \x91_\x91a\x05&W[Pa\x04u\x86\x8Ca\x1B1V[Q\x90`@Q\x80\x80\x95\x81\x94c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x05!Wa\x04\xD8\x8C`\x01\x94\x87a\x04\xE7\x95_\x92a\x04\xEFW[Pa\x04\xC9\x91\x92a\x1B1V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a\x04\xE1\x8Ca':V[\x90a'\xF0V[\x92\x01\x91a\x046V[a\x04\xC9\x92Pa\x05\x14\x90` =\x81\x11a\x05\x1AW[a\x05\x0C\x81\x83a\x08SV[\x81\x01\x90a\x1A\x8FV[\x91a\x04\xBEV[P=a\x05\x02V[a\x1A\x12V[a\x05=\x91P\x82=\x81\x11a\x05\x1AWa\x05\x0C\x81\x83a\x08SV[_a\x04jV[\x96P\x97PP\x94\x90\x91a\x05U\x85Qa\x1A8V[\x96_\x94_[\x87Q\x81\x10\x15a\x05\xCEW\x89\x88a\x05\x8Ea\x05\x82a\x05u\x85\x84a\x1B1V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x05\x9DW[PP`\x01\x01a\x05ZV[\x82\x91\x98a\x04\xC9a\x05\xB5a\x05u`\x01\x96a\x05\xC5\x95a\x1B1V[\x91a\x05\xBF\x81a\x1BJV[\x9Ba\x1B1V[\x90P\x89\x88a\x05\x93V[P\x94\x88R\x95\x96\x93\x94P\x91a\x05\xFA\x91a\x05\xF5\x90a\x05\xEA\x81\x8Aa\x1B1V[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[a\x1A~V[\x94\x93\x94\x92\x91\x92a\x03\xAFV[a\x06\"\x91\x96P=\x80_\x83>a\x06\x1A\x81\x83a\x08SV[\x81\x01\x90a\x1A\xA4V[\x94_a\x04'V[a\x06B\x91\x96P` =\x81\x11a\x05\x1AWa\x05\x0C\x81\x83a\x08SV[\x94_a\x03\xE1V[\x84\x92\x91Pa\x06W\x83Qa\x1BXV[\x91_[\x84Q\x81\x10\x15a\x07\xDAWa\x06\x9E` a\x06ua\x05u\x84\x89a\x1B1V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x86Z\xFA\x90\x81\x15a\x05!Wa\x06\xD8\x91` \x91_\x91a\x07\xADW[P`@Q\x80\x93\x81\x92c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x80\x15a\x05!Wa\x06\xFD\x91_\x91a\x07\x7FW[P`\x01`\x01`\xC0\x1B\x03\x16a)\xA6V[\x91a\x07\x08\x83Qa\x1A8V[\x93_[\x84Q\x81\x10\x15a\x07WW\x80a\x07Qa\x07Ga\x07Aa\x07;a\x07-`\x01\x96\x8Ba\x1B\xCFV[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`\xFF\x16\x90V[a\x05\xEA\x83\x8Aa\x1B1V[\x01a\x07\x0BV[P\x93`\x01\x92\x91\x96\x93Pa\x07j\x82\x87a\x1B1V[Ra\x07u\x81\x86a\x1B1V[P\x01\x93\x90\x93a\x06ZV[a\x07\xA0\x91P` =\x81\x11a\x07\xA6W[a\x07\x98\x81\x83a\x08SV[\x81\x01\x90a\x1B\xB0V[\x87a\x06\xEEV[P=a\x07\x8EV[a\x07\xCD\x91P\x82=\x81\x11a\x07\xD3W[a\x07\xC5\x81\x83a\x08SV[\x81\x01\x90a\x1B\xA1V[\x88a\x06\xB8V[P=a\x07\xBBV[PPa\x07\xEC\x83`@Q\x93\x84\x93\x84a\x02\xF5V[\x03\x90\xF3[a\x08\x12\x91P` =` \x11a\x08\x18W[a\x08\n\x81\x83a\x08SV[\x81\x01\x90a\x19\xF9V[_a\x03\x90V[P=a\x08\0V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08NW`@RV[a\x08\x1FV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08NW`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\x08NW`\x05\x1B` \x01\x90V[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\xF3WV[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xF3W\x815a\x08\xB3\x81a\x08tV[\x92a\x08\xC1`@Q\x94\x85a\x08SV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x01\xF3W` \x01\x90[\x82\x82\x10a\x08\xE9WPPP\x90V[` \x80\x91a\x08\xF6\x84a\x08\x8BV[\x81R\x01\x91\x01\x90a\x08\xDCV[` `\x03\x19\x82\x01\x12a\x01\xF3W`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xF3Wa\x03,\x91`\x04\x01a\x08\x9CV[4a\x01\xF3Wa\t96a\t\x01V[a\tAa*EV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x01\xF3W`@Qc\xAE\xC2\x05\xC5`\xE0\x1B\x81R_\x81`\x04\x81\x83\x86Z\xF1\x80\x15a\x05!Wa\t\xE3W[P\x80;\x15a\x01\xF3W`@Qc\xAF\xE0.\xD5`\xE0\x1B\x81R\x90_\x90\x82\x90\x81\x83\x81a\t\xBD\x88`\x04\x83\x01a\"\x1EV[\x03\x92Z\xF1\x80\x15a\x05!Wa\t\xCDW\0[\x80a\t\xDB_a\t\xE1\x93a\x08SV[\x80a\x01\xE9V[\0[\x80a\t\xDB_a\t\xF1\x93a\x08SV[_a\t\x93V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01\xF3WV[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xF3W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xF3W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x01\xF3WV[`\x01`\x01`@\x1B\x03\x81\x11a\x08NW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\n_\x82a\n8V[\x91a\nm`@Q\x93\x84a\x08SV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xF3W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x91\x90\x91``\x81\x84\x03\x12a\x01\xF3W`@Q\x90``\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08NW`@R\x81\x93\x815`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3W\x82\x01\x91\x81`\x1F\x84\x01\x12\x15a\x01\xF3Wa\n\xE7`@\x93\x92\x83` \x86\x955\x91\x01a\nSV[\x84R` \x81\x015` \x85\x01R\x015\x91\x01RV[4a\x01\xF3W``6`\x03\x19\x01\x12a\x01\xF3W`\x045a\x0B\x17\x81a\t\xF7V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3Wa\x0B6\x906\x90`\x04\x01a\n\x08V[\x91\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3Wa\x0BW\x906\x90`\x04\x01a\n\x89V[\x92a\x0B\x8C3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B\xE0V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x01\xF3W_\x80\x94a\t\xBDa\x0B\xFA\x97`@Q\x98\x89\x97\x88\x96\x87\x95c\x0F\x10\xCC\xF1`\xE1\x1B\x87R`\x01\x80`\xA0\x1B\x03\x16`\x04\x87\x01R```$\x87\x01R`d\x86\x01\x91a\x1CmV[\x83\x81\x03`\x03\x19\x01`D\x85\x01R\x90a\x1C\xCFV[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3Wa\x0C$a*EV[`hTb\t:\x80\x81\x01\x80\x91\x11a\x0C\xBDWB\x10a\x0CaW`gTa\x0CO\x90`\x01`\x01`\xA0\x1B\x03\x16a*\x9DV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FServiceManager: Slasher proposal`D\x82\x01Rm\x08\x19\x19[\x18^H\x1B\x9B\xDD\x08\x1BY]`\x92\x1B`d\x82\x01R`\x84\x90\xFD[a\x1AjV[4a\x01\xF3W` 6`\x03\x19\x01\x12a\x01\xF3Wa\x07\xECa\x0C\xEA`\x045a\x0C\xE5\x81a\t\xF7V[a\x1DhV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x02\xB9V[4a\x01\xF3W` 6`\x03\x19\x01\x12a\x01\xF3W`\x045a\r\x1B\x81a\t\xF7V[a\r#a*EV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`eU\0[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xF3W\x815a\r\x98\x81a\x08tV[\x92a\r\xA6`@Q\x94\x85a\x08SV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x01\xF3W` \x01\x90[\x82\x82\x10a\r\xCEWPPP\x90V[` \x80\x91\x835a\r\xDD\x81a\t\xF7V[\x81R\x01\x91\x01\x90a\r\xC1V[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xF3W\x81` a\x03,\x935\x91\x01a\nSV[4a\x01\xF3W` 6`\x03\x19\x01\x12a\x01\xF3W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3W`\xA0`\x03\x19\x826\x03\x01\x12a\x01\xF3W`@Qa\x0E?\x81a\x083V[\x81`\x04\x015a\x0EM\x81a\t\xF7V[\x81Ra\x0E[`$\x83\x01a\x08\x8BV[` \x82\x01R`D\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3Wa\x0E\x83\x90`\x046\x91\x85\x01\x01a\r\x81V[`@\x82\x01R`d\x82\x015``\x82\x01R`\x84\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3Wa\t\xE1\x92`\x04a\x0E\xB8\x926\x92\x01\x01a\r\xE8V[`\x80\x82\x01Ra \xA1V[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W` `@Qb\t:\x80\x81R\xF3[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3Wa\x0F;a*EV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xF3W` 6`\x03\x19\x01\x12a\x01\xF3W\x7F/\x8A\xFC\x8Ax\xFD\x95\x8F3\x01\xC0#:\xA3&\xB9\xC4\xB9\xA2\x88Jt\x83\"}k\x05U\xAA\xA0:\xDBa\x10H`\x045a\x10\x03\x81a\t\xF7V[a\x10\x0Ba*EV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90UB`h\x81\x90U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1\0[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W` `\xFF`iT\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xF3W_`@6`\x03\x19\x01\x12a\x01\xF3W`\x045a\x10\xB5\x81a\t\xF7V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3Wa\x10\xD4\x906\x90`\x04\x01a\n\x89V[\x90a\x11\t3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B\xE0V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x01\xF3Wa\x11s\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\x99&\xEE}`\xE0\x1B\x84R`\x01\x80`\xA0\x1B\x03\x16`\x04\x84\x01R`@`$\x84\x01R`D\x83\x01\x90a\x1C\xCFV[\x03\x92Z\xF1\x80\x15a\x05!Wa\x11\x85WP\x80\xF3[a\t\xE1\x91P_\x90a\x08SV[4a\x01\xF3W_` 6`\x03\x19\x01\x12a\x01\xF3W`\x045a\x11\xAF\x81a\t\xF7V[a\x11\xE33\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B\xE0V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x01\xF3W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x05!Wa\x11\x85WP\x80\xF3[4a\x01\xF3W_` 6`\x03\x19\x01\x12a\x01\xF3W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3Wa\x12\x7F\x906\x90`\x04\x01a\r\xE8V[a\x12\x87a*EV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x01\xF3W`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R` `\x04\x82\x01R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x11s\x90`$\x83\x01\x90a\x1C\xABV[4a\x01\xF3Wa\x12\xF36a\t\x01V[a\x13'3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B\xE0V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x01\xF3W`@Qc\xAF\xE0.\xD5`\xE0\x1B\x81R` `\x04\x82\x01R\x90_\x90\x82\x90a\x13\x80\x90`$\x83\x01\x90a\x01\xF7V[\x93\x81\x83\x81\x81\x97\x03\x92Z\xF1\x80\x15a\x05!Wa\x11\x85WP\x80\xF3[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W`fT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3Wa\x13\xD8a*EV[`\x01`iTa\x13\xEA`\xFF\x82\x16\x15a\"/V[`\xFF\x19\x16\x17`iU\0[4a\x01\xF3W_`@6`\x03\x19\x01\x12a\x01\xF3W`\x045a\x14\x12\x81a\t\xF7V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3Wa\x141\x906\x90`\x04\x01a\n\x08V[\x91\x90a\x14g3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B\xE0V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x01\xF3Wa\x11s\x93_\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93c\xC1\xA8\xE2\xC5`\xE0\x1B\x85R`\x01\x80`\xA0\x1B\x03\x16`\x04\x85\x01R`@`$\x85\x01R`D\x84\x01\x91a\x1CmV[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xF3W`@6`\x03\x19\x01\x12a\x01\xF3W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3W6`#\x82\x01\x12\x15a\x01\xF3W\x80`\x04\x015a\x15P\x81a\x08tV[\x91a\x15^`@Q\x93\x84a\x08SV[\x81\x83R`$` \x84\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x01\xF3W`$\x81\x01\x92[\x82\x84\x10a\x15\xACW`$5\x85`\x01`\x01`@\x1B\x03\x82\x11a\x01\xF3Wa\x15\xA6a\t\xE1\x926\x90`\x04\x01a\r\x81V[\x90a\"\x8FV[\x835`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3W` \x91a\x15\xD0\x83\x92`$6\x91\x87\x01\x01a\x08\x9CV[\x81R\x01\x93\x01\x92a\x15|V[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W`gT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3Wa\x07\xECa\x0C\xEAa#@V[4a\x01\xF3W` 6`\x03\x19\x01\x12a\x01\xF3W`\x045a\x16;\x81a\t\xF7V[a\x16Ca*EV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x16\x9AW`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xF3W_6`\x03\x19\x01\x12a\x01\xF3W` `hT`@Q\x90\x81R\xF3[4a\x01\xF3W` 6`\x03\x19\x01\x12a\x01\xF3W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF3Wa\x17c\x906\x90`\x04\x01a\n\x08V[`eT\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x19yW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91_[\x81\x81\x10a\x17\xD7WP\x82;\x15a\x01\xF3Wa\t\xBD\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xFC\xE3l}`\xE0\x1B\x84R`\x04\x84\x01a&\x1EV[_` a\x182a\x17\xF5a\x05\x82\x83a\x17\xEF\x87\x89\x8Ba%mV[\x01a%\x8FV[`@a\x18\x02\x86\x88\x8Aa%mV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x91\x015`D\x82\x01R\x93\x84\x92\x83\x91\x90\x82\x90`d\x82\x01\x90V[\x03\x92Z\xF1\x80\x15a\x05!Wa\x19]W[Pa\x18Ua\x05\x82` a\x17\xEF\x84\x86\x88a%mV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x86\x16`$\x82\x01R\x91\x90` \x90\x83\x90`D\x90\x82\x90Z\xFA\x80\x15a\x05!Wa\x18\xF8` \x91_\x94\x85\x91a\x19@W[Pa\x18\xC5a\x18\xB0a\x05\x82\x85a\x17\xEF\x88\x8B\x8Da%mV[\x91`@a\x18\xBE\x87\x8A\x8Ca%mV[\x015a\x1C\xF9V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x94\x85\x92\x83\x91\x90\x82\x90`D\x82\x01\x90V[\x03\x92Z\xF1\x91\x82\x15a\x05!W`\x01\x92a\x19\x12W[P\x01a\x17\xA5V[a\x192\x90` =\x81\x11a\x199W[a\x19*\x81\x83a\x08SV[\x81\x01\x90a%\x99V[P_a\x19\x0BV[P=a\x19 V[a\x19W\x91P\x83=\x81\x11a\x07\xD3Wa\x07\xC5\x81\x83a\x08SV[_a\x18\x9AV[a\x19t\x90` =\x81\x11a\x199Wa\x19*\x81\x83a\x08SV[a\x18AV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FServiceManagerBase.onlyRewardsIn`D\x82\x01R\x7Fitiator: caller is not the rewar`d\x82\x01Rk29\x904\xB74\xBA4\xB0\xBA7\xB9`\xA1\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x90\x81` \x91\x03\x12a\x01\xF3WQ`\xFF\x81\x16\x81\x03a\x01\xF3W\x90V[`@Q=_\x82>=\x90\xFD[`@Q\x90a\x1A,` \x83a\x08SV[_\x80\x83R6` \x84\x017V[\x90a\x1AB\x82a\x08tV[a\x1AO`@Q\x91\x82a\x08SV[\x82\x81R\x80\x92a\x1A``\x1F\x19\x91a\x08tV[\x01\x90` 6\x91\x017V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x16`\xFF\x81\x14a\x0C\xBDW`\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x01\xF3WQa\x03,\x81a\t\xF7V[` \x81\x83\x03\x12a\x01\xF3W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xF3W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x01\xF3W\x81Qa\x1A\xD7\x81a\x08tV[\x92a\x1A\xE5`@Q\x94\x85a\x08SV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x01\xF3W` \x01\x90[\x82\x82\x10a\x1B\rWPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a\x1B\0V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q\x82\x10\x15a\x1BEW` \x91`\x05\x1B\x01\x01\x90V[a\x1B\x1DV[_\x19\x81\x14a\x0C\xBDW`\x01\x01\x90V[\x90a\x1Bb\x82a\x08tV[a\x1Bo`@Q\x91\x82a\x08SV[\x82\x81R\x80\x92a\x1B\x80`\x1F\x19\x91a\x08tV[\x01\x90_[\x82\x81\x10a\x1B\x90WPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a\x1B\x84V[\x90\x81` \x91\x03\x12a\x01\xF3WQ\x90V[\x90\x81` \x91\x03\x12a\x01\xF3WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x01\xF3W\x90V[\x90\x81Q\x81\x10\x15a\x1BEW\x01` \x01\x90V[\x15a\x1B\xE7WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FServiceManagerBase.onlyRegistryC`D\x82\x01R\x7Foordinator: caller is not the re`d\x82\x01Rq3\xB4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a\x1C\x86WPPP\x90V[\x90\x91\x92` \x80`\x01\x92c\xFF\xFF\xFF\xFFa\x1C\x9D\x88a\x08\x8BV[\x16\x81R\x01\x94\x01\x92\x91\x01a\x1CyV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90`@\x80a\x1C\xE6\x84Q``\x85R``\x85\x01\x90a\x1C\xABV[\x93` \x81\x01Q` \x85\x01R\x01Q\x91\x01R\x90V[\x91\x90\x82\x01\x80\x92\x11a\x0C\xBDWV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x01\xF3WV[\x90\x81`@\x91\x03\x12a\x01\xF3W`@Q\x90`@\x82\x01\x90\x82\x82\x10`\x01`\x01`@\x1B\x03\x83\x11\x17a\x08NW` \x91`@R\x80Qa\x1DS\x81a\t\xF7V[\x83R\x01Qa\x1D`\x81a\x1D\x06V[` \x82\x01R\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x05!Wa\x1D\xE7\x91` \x91_\x91a \x84WP`@Q\x80\x93\x81\x92c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x85Z\xFA\x90\x81\x15a\x05!W_\x91a eW[P`\x01`\x01`\xC0\x1B\x03\x16\x90\x81\x15\x90\x81\x15a \x12W[Pa \tWa\x1E\x1D\x90a)\xA6V[_\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x83[\x81Q\x85\x10\x15a\x1E\xD5Wa\x1E\x8E` a\x1Eka\x07;a\x07-\x89\x87a\x1B\xCFV[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x87Z\xFA\x80\x15a\x05!W`\x01\x92a\x1E\xAD\x92_\x92a\x1E\xB5W[Pa\x1C\xF9V[\x94\x01\x93a\x1EMV[a\x1E\xCE\x91\x92P` =\x81\x11a\x07\xD3Wa\x07\xC5\x81\x83a\x08SV[\x90_a\x1E\xA7V[a\x1E\xE0\x91\x94Pa\x1A8V[\x92_\x90_[\x81Q\x81\x10\x15a \x03Wa\x1E\xFEa\x07;a\x07-\x83\x85a\x1B\xCFV[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x90` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x05!W_\x92a\x1F\xE3W[P\x90_\x91[\x81\x83\x10a\x1FCWPPP`\x01\x01a\x1E\xE5V[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x05!Wa\x1F\xA7\x8Ba\x04\xC9\x83a\x1F\xA1a\x05\x82`\x01\x98a\x1F\xAC\x98_\x91a\x1F\xB5W[PQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92a\x1B1V[a\x1BJV[\x95\x01\x91\x90a\x1F1V[a\x1F\xD6\x91P`@=\x81\x11a\x1F\xDCW[a\x1F\xCE\x81\x83a\x08SV[\x81\x01\x90a\x1D\x1CV[_a\x1F\x93V[P=a\x1F\xC4V[a\x1F\xFC\x91\x92P` =\x81\x11a\x07\xD3Wa\x07\xC5\x81\x83a\x08SV[\x90_a\x1F,V[PPPPV[Pa\x03,a\x1A\x1DV[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x91P` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x80\x15a\x05!W`\xFF\x91_\x91a FW[P\x16\x15_a\x1E\x0FV[a _\x91P` =` \x11a\x08\x18Wa\x08\n\x81\x83a\x08SV[_a =V[a ~\x91P` =` \x11a\x07\xA6Wa\x07\x98\x81\x83a\x08SV[_a\x1D\xFAV[a \x9B\x91P\x82=\x84\x11a\x07\xD3Wa\x07\xC5\x81\x83a\x08SV[_a\x06\xB8V[`fT`\x01`\x01`\xA0\x1B\x03\x163\x03a!\xB3W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x01\xF3W`@\x80Qc`\xDB\x99\xA3`\xE0\x1B\x81R` `\x04\x82\x01\x81\x90R\x84Q`\x01`\x01`\xA0\x1B\x03\x16`$\x83\x01R\x80\x85\x01Qc\xFF\xFF\xFF\xFF\x16`D\x83\x01R\x91\x84\x01Q`\xA0`d\x83\x01R\x80Q`\xC4\x83\x01\x81\x90R\x91\x94\x85\x93`\xE4\x85\x01\x93\x91\x92\x01\x90_[\x81\x81\x10a!\x91WPPP_\x83a!p\x82\x96\x94`\x80\x85``\x85\x97\x01Q`\x84\x86\x01R\x01Q`#\x19\x84\x83\x03\x01`\xA4\x85\x01Ra\x1C\xABV[\x03\x92Z\xF1\x80\x15a\x05!Wa!\x81WPV[\x80a\t\xDB_a!\x8F\x93a\x08SV[V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x87\x95P` \x94\x85\x01\x94\x90\x92\x01\x91`\x01\x01a!=V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FServiceManagerBase.onlySlasher: `D\x82\x01R\x7Fcaller is not the slasher\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x90` a\x03,\x92\x81\x81R\x01\x90a\x01\xF7V[\x15a\"6WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FServiceManager: Migration Alread`D\x82\x01Rj\x1EH\x11\x9A[\x98[\x1A^\x99Y`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[\x90a\"\x98a*EV[a\"\xA7`\xFF`iT\x16\x15a\"/V[a\"\xB4\x81Q\x83Q\x14a*\xFBV[_[\x81Q\x81\x10\x15a\"\xEAW\x80a\"\xE3a\"\xD2a\x05u`\x01\x94\x86a\x1B1V[a\"\xDC\x83\x87a\x1B1V[Q\x90a+\xDBV[P\x01a\"\xB6V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x01\xF3Wa!p\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xEF-\xFA\x8D`\xE0\x1B\x84R`\x04\x84\x01a+[V[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x05!W`\xFF\x91_\x91a%NW[P\x16\x80\x15a%DW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90_\x90\x81\x90[\x80\x83\x10a%\0WPa#\xDB\x91Pa\x1A8V[\x92_\x90_[`@Qc\x9A\xA1e=`\xE0\x1B\x81R` \x81`\x04\x81\x89Z\xFA\x80\x15a\x05!W`\xFF\x91_\x91a$\xE2W[P\x16\x81\x10\x15a$\xDBW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01\x81\x90R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x05!W_\x92a$\xBBW[P\x90_\x91[\x81\x83\x10a$UWPPP`\x01\x01a#\xE0V[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x05!Wa\x1F\xA7\x8Ba\x04\xC9\x83a\x1F\xA1a\x05\x82`\x01\x98a$\xB2\x98_\x91a\x1F\xB5WPQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95\x01\x91\x90a$CV[a$\xD4\x91\x92P` =\x81\x11a\x07\xD3Wa\x07\xC5\x81\x83a\x08SV[\x90_a$>V[P\x92PPPV[a$\xFA\x91P` =\x81\x11a\x08\x18Wa\x08\n\x81\x83a\x08SV[_a$\x06V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x90\x91\x90` \x81`$\x81\x88Z\xFA\x80\x15a\x05!W`\x01\x92a%;\x92_\x92a\x1E\xB5WPa\x1C\xF9V[\x92\x01\x91\x90a#\xC9V[PPa\x03,a\x1A\x1DV[a%g\x91P` =` \x11a\x08\x18Wa\x08\n\x81\x83a\x08SV[_a#\x91V[\x91\x90\x81\x10\x15a\x1BEW`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x01\xF3W\x01\x90V[5a\x03,\x81a\t\xF7V[\x90\x81` \x91\x03\x12a\x01\xF3WQ\x80\x15\x15\x81\x03a\x01\xF3W\x90V[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a%\xCAWPPP\x90V[\x90\x91\x92`@\x80`\x01\x92\x865a%\xDE\x81a\t\xF7V[\x84\x80`\xA0\x1B\x03\x16\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x88\x015a&\x02\x81a\x1D\x06V[\x16` \x82\x01R\x01\x94\x01\x92\x91\x01a%\xBDV[5\x90a!\x8F\x82a\t\xF7V[\x91\x80` \x84\x01` \x85RR`@\x83\x01\x90`@\x81`\x05\x1B\x85\x01\x01\x93\x83_\x91`\x9E\x19\x826\x03\x01\x90[\x84\x84\x10a&UWPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`?\x19\x82\x82\x03\x01\x87R\x875\x83\x81\x12\x15a\x01\xF3W\x84\x01\x90\x815`\x1E\x19\x836\x03\x01\x81\x12\x15a\x01\xF3W\x82\x01\x91` \x835\x93\x01\x90`\x01`\x01`@\x1B\x03\x84\x11a\x01\xF3W\x83`\x06\x1B6\x03\x82\x13a\x01\xF3Wa'\x1E\x83`\x80a'\x13\x81a&\xC9` \x98\x97`\x01\x9A`\xA0\x8B\x9AR`\xA0\x87\x01\x91a%\xB1V[\x95a&\xE7a&\xD8\x89\x83\x01a&\x13V[`\x01`\x01`\xA0\x1B\x03\x16\x86\x8A\x01RV[`@\x81\x015`@\x86\x01Ra'\ra'\0``\x83\x01a\x08\x8BV[c\xFF\xFF\xFF\xFF\x16``\x87\x01RV[\x01a\x08\x8BV[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x99\x01\x97\x01\x95\x94\x01\x92\x91\x90a&DV[\x91\x90\x82\x03\x91\x82\x11a\x0C\xBDWV[\x80Q`\x01\x81\x11\x15a'\xECW`\x01\x1C\x91a'R\x83a\x1A8V[\x91a'fa'a\x85\x83Qa'-V[a\x1A8V[\x91_[\x85\x81\x10a'\xC9WP\x84[\x82Q\x81\x10\x15a'\xA8W\x80a'\xA2a'\x8Fa\x05u`\x01\x94\x87a\x1B1V[a\x04\xC9a'\x9C\x8A\x85a'-V[\x88a\x1B1V[\x01a'sV[P\x93PP\x90a'\xB6\x90a':V[a'\xC0\x90\x91a':V[a\x03,\x91a'\xF0V[\x80a'\xE6a'\xDCa\x05u`\x01\x94\x87a\x1B1V[a\x04\xC9\x83\x89a\x1B1V[\x01a'iV[P\x90V[\x91\x82Q\x92\x82Qa(\x03a'a\x82\x87a\x1C\xF9V[\x93_\x93_\x92_\x97[\x80\x87\x10\x80a)uW[\x15a(\xF6Wa(&a\x05u\x88\x88a\x1B1V[a(6a\x05\x82a\x05u\x88\x88a\x1B1V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x10\x15a(xWa(sa(`a\x05ua(Y\x8Aa\x1BJV[\x99\x89a\x1B1V[a\x04\xC9a(l\x8Ca\x1BJV[\x9B\x8Ba\x1B1V[a(\x0BV[a(\x85a\x05u\x88\x88a\x1B1V[a(\x95a\x05\x82a\x05u\x88\x88a\x1B1V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x11\x15a(\xBFWa(sa(`a\x05ua(\xB8\x88a\x1BJV[\x97\x87a\x1B1V[\x93a(\xF0\x90a\x1F\xA7a(\xDDa\x05ua(\xD6\x8Ba\x1BJV[\x9A\x8Aa\x1B1V[a\x04\xC9a(\xE9\x8Da\x1BJV[\x9C\x8Ca\x1B1V[\x93a(\x0BV[\x97\x95\x91\x97\x94\x90\x92\x93\x94[\x80\x83\x10a)KWPPP[\x80\x83\x10a)\x1AWPPP\x81R\x90V[a)Fa)3a\x05ua),\x86a\x1BJV[\x95\x85a\x1B1V[a\x04\xC9a)?\x87a\x1BJV[\x96\x88a\x1B1V[a)\x0BV[a)pa)]a\x05ua),\x86a\x1BJV[a\x04\xC9a)i\x8Aa\x1BJV[\x99\x8Ba\x1B1V[a)\0V[P\x81\x85\x10a(\x14V[\x90a)\x88\x82a\n8V[a)\x95`@Q\x91\x82a\x08SV[\x82\x81R\x80\x92a\x1A``\x1F\x19\x91a\n8V[_\x81\x80[a* WPa)\xBC\x90a\xFF\xFF\x16a)~V[__[\x82Q\x82\x10\x80a*\x15W[\x15a*\x0EW`\x01\x81\x1B\x84\x16a)\xE7W[a)\xE2\x90a\x1BJV[a)\xBFV[\x90`\x01a)\xE2\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa*\x04\x82\x87a\x1B\xCFV[S\x01\x91\x90Pa)\xD9V[PP\x90P\x90V[Pa\x01\0\x81\x10a)\xC9V[_\x19\x81\x01\x81\x81\x11a\x0C\xBDWa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a\x0C\xBDW`\x01\x01\x90\x80a)\xAAV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a*YWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`fT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE0\xD4\x9AT'D#\x18=\xAD\xEC\xBD\xF29\xEA\xACn\x06\xBA\x882\x0B&\xFE\x8C\xC5\xEC\x9D\x05\nc\x95\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`fUV[\x15a+\x02WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FServiceManager: Input array leng`D\x82\x01Rj\x0E\x8D\x04\r\xAD.m\xAC.\x8Cm`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x91a+ra\x03,\x93`@\x84R`@\x84\x01\x90a\x02\xB9V[\x91` \x81\x84\x03\x91\x01Ra\x020V[\x15a+\x87WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FServiceManager: Operator not in `D\x82\x01Requorum`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x92\x91\x90_\x90` \x81`$\x81\x88Z\xFA\x80\x15a\x05!Wa,_\x95` \x92_\x92a,\xE8W[P`@Q\x80\x80\x98\x81\x94c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91Z\xFA\x93\x84\x15a\x05!W_\x94a,\xC7W[P_[\x82Q\x81\x10\x15a,\xC1W\x80a,\xBBa,\xB6a,\xA0a\x07Aa,\x96`\x01\x96\x89a\x1B1V[Qc\xFF\xFF\xFF\xFF\x16\x90V[\x84\x80`\xC0\x1B\x03\x89\x16`\xFF`\x01\x92\x16\x1C\x16`\x01\x14\x90V[a+\x80V[\x01a,tV[P\x92PPV[a,\xE1\x91\x94P` =` \x11a\x07\xA6Wa\x07\x98\x81\x83a\x08SV[\x92_a,qV[a-\0\x91\x92P\x83=\x85\x11a\x07\xD3Wa\x07\xC5\x81\x83a\x08SV[\x90_a,>V\xFE\xA2dipfsX\"\x12 \xD6\x1EHR\xE99\x8A\x08\x83\x05/\xB7Gd\xE0\xE4\x02\x04\x06\xED\xFD\xAFC\x075\xF3\xCDT\x96\xFE\x9E\x8CdsolcC\0\x08\x1B\x003",
    );
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
```solidity
event Initialized(uint8 version);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RewardsInitiatorUpdated(address,address)` and selector `0xe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3`.
```solidity
event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RewardsInitiatorUpdated {
        #[allow(missing_docs)]
        pub prevRewardsInitiator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newRewardsInitiator: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RewardsInitiatorUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "RewardsInitiatorUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                225u8,
                28u8,
                221u8,
                241u8,
                129u8,
                106u8,
                67u8,
                49u8,
                140u8,
                161u8,
                117u8,
                187u8,
                197u8,
                44u8,
                208u8,
                24u8,
                84u8,
                54u8,
                233u8,
                203u8,
                234u8,
                215u8,
                200u8,
                58u8,
                204u8,
                84u8,
                167u8,
                62u8,
                70u8,
                23u8,
                23u8,
                227u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevRewardsInitiator: data.0,
                    newRewardsInitiator: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.prevRewardsInitiator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newRewardsInitiator,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RewardsInitiatorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RewardsInitiatorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &RewardsInitiatorUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SlasherProposed(address,uint256)` and selector `0x2f8afc8a78fd958f3301c0233aa326b9c4b9a2884a7483227d6b0555aaa03adb`.
```solidity
event SlasherProposed(address newSlasher, uint256 slasherProposalTimestamp);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SlasherProposed {
        #[allow(missing_docs)]
        pub newSlasher: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub slasherProposalTimestamp: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SlasherProposed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SlasherProposed(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8,
                138u8,
                252u8,
                138u8,
                120u8,
                253u8,
                149u8,
                143u8,
                51u8,
                1u8,
                192u8,
                35u8,
                58u8,
                163u8,
                38u8,
                185u8,
                196u8,
                185u8,
                162u8,
                136u8,
                74u8,
                116u8,
                131u8,
                34u8,
                125u8,
                107u8,
                5u8,
                85u8,
                170u8,
                160u8,
                58u8,
                219u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    newSlasher: data.0,
                    slasherProposalTimestamp: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newSlasher,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.slasherProposalTimestamp,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SlasherProposed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SlasherProposed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SlasherProposed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SlasherUpdated(address,address)` and selector `0xe0d49a54274423183dadecbdf239eaac6e06ba88320b26fe8cc5ec9d050a6395`.
```solidity
event SlasherUpdated(address prevSlasher, address newSlasher);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SlasherUpdated {
        #[allow(missing_docs)]
        pub prevSlasher: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newSlasher: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SlasherUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SlasherUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                224u8,
                212u8,
                154u8,
                84u8,
                39u8,
                68u8,
                35u8,
                24u8,
                61u8,
                173u8,
                236u8,
                189u8,
                242u8,
                57u8,
                234u8,
                172u8,
                110u8,
                6u8,
                186u8,
                136u8,
                50u8,
                11u8,
                38u8,
                254u8,
                140u8,
                197u8,
                236u8,
                157u8,
                5u8,
                10u8,
                99u8,
                149u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevSlasher: data.0,
                    newSlasher: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.prevSlasher,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newSlasher,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SlasherUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SlasherUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SlasherUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _avsDirectory, address _registryCoordinator, address _stakeRegistry, address rewards_coordinator, address allocationManager, address _incredibleSquaringTaskManager);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _avsDirectory: alloy::sol_types::private::Address,
        pub _registryCoordinator: alloy::sol_types::private::Address,
        pub _stakeRegistry: alloy::sol_types::private::Address,
        pub rewards_coordinator: alloy::sol_types::private::Address,
        pub allocationManager: alloy::sol_types::private::Address,
        pub _incredibleSquaringTaskManager: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value._avsDirectory,
                        value._registryCoordinator,
                        value._stakeRegistry,
                        value.rewards_coordinator,
                        value.allocationManager,
                        value._incredibleSquaringTaskManager,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _avsDirectory: tuple.0,
                        _registryCoordinator: tuple.1,
                        _stakeRegistry: tuple.2,
                        rewards_coordinator: tuple.3,
                        allocationManager: tuple.4,
                        _incredibleSquaringTaskManager: tuple.5,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._avsDirectory,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._stakeRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.rewards_coordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.allocationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._incredibleSquaringTaskManager,
                    ),
                )
            }
        }
    };
    /**Function with signature `SLASHER_PROPOSAL_DELAY()` and selector `0x67940c89`.
```solidity
function SLASHER_PROPOSAL_DELAY() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SLASHER_PROPOSAL_DELAYCall {}
    ///Container type for the return parameters of the [`SLASHER_PROPOSAL_DELAY()`](SLASHER_PROPOSAL_DELAYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SLASHER_PROPOSAL_DELAYReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<SLASHER_PROPOSAL_DELAYCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: SLASHER_PROPOSAL_DELAYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SLASHER_PROPOSAL_DELAYCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<SLASHER_PROPOSAL_DELAYReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: SLASHER_PROPOSAL_DELAYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SLASHER_PROPOSAL_DELAYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SLASHER_PROPOSAL_DELAYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = SLASHER_PROPOSAL_DELAYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SLASHER_PROPOSAL_DELAY()";
            const SELECTOR: [u8; 4] = [103u8, 148u8, 12u8, 137u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `acceptProposedSlasher()` and selector `0x26f017e2`.
```solidity
function acceptProposedSlasher() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptProposedSlasherCall {}
    ///Container type for the return parameters of the [`acceptProposedSlasher()`](acceptProposedSlasherCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptProposedSlasherReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<acceptProposedSlasherCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptProposedSlasherCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptProposedSlasherCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<acceptProposedSlasherReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptProposedSlasherReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptProposedSlasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for acceptProposedSlasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = acceptProposedSlasherReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "acceptProposedSlasher()";
            const SELECTOR: [u8; 4] = [38u8, 240u8, 23u8, 226u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `allocationManager()` and selector `0xca8aa7c7`.
```solidity
function allocationManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerCall {}
    ///Container type for the return parameters of the [`allocationManager()`](allocationManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allocationManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocationManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allocationManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allocationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allocationManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allocationManager()";
            const SELECTOR: [u8; 4] = [202u8, 138u8, 167u8, 199u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `avsDirectory()` and selector `0x6b3aa72e`.
```solidity
function avsDirectory() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryCall {}
    ///Container type for the return parameters of the [`avsDirectory()`](avsDirectoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<avsDirectoryCall> for UnderlyingRustTuple<'_> {
                fn from(value: avsDirectoryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsDirectoryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<avsDirectoryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: avsDirectoryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsDirectoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for avsDirectoryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsDirectoryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "avsDirectory()";
            const SELECTOR: [u8; 4] = [107u8, 58u8, 167u8, 46u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])` and selector `0xfce36c7d`.
```solidity
function createAVSRewardsSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createAVSRewardsSubmissionCall {
        pub rewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])`](createAVSRewardsSubmissionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createAVSRewardsSubmissionReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsSubmission,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createAVSRewardsSubmissionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionCall) -> Self {
                    (value.rewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createAVSRewardsSubmissionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        rewardsSubmissions: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createAVSRewardsSubmissionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createAVSRewardsSubmissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createAVSRewardsSubmissionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsSubmission,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createAVSRewardsSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])";
            const SELECTOR: [u8; 4] = [252u8, 227u8, 108u8, 125u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        IRewardsCoordinatorTypes::RewardsSubmission,
                    > as alloy_sol_types::SolType>::tokenize(&self.rewardsSubmissions),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `createOperatorSets(uint32[])` and selector `0xafe02ed5`.
```solidity
function createOperatorSets(uint32[] memory operatorSetIds) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorSetsCall {
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
    }
    ///Container type for the return parameters of the [`createOperatorSets(uint32[])`](createOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorSetsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Vec<u32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createOperatorSetsCall) -> Self {
                    (value.operatorSetIds,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOperatorSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorSetIds: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createOperatorSetsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createOperatorSets(uint32[])";
            const SELECTOR: [u8; 4] = [175u8, 224u8, 46u8, 213u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `deregisterOperatorFromAVS(address)` and selector `0xa364f4da`.
```solidity
function deregisterOperatorFromAVS(address operator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromAVSCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`deregisterOperatorFromAVS(address)`](deregisterOperatorFromAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromAVSReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromAVSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromAVSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromAVSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorFromAVSCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperatorFromAVS(address)";
            const SELECTOR: [u8; 4] = [163u8, 100u8, 244u8, 218u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `deregisterOperatorFromOperatorSets(address,uint32[])` and selector `0xc1a8e2c5`.
```solidity
function deregisterOperatorFromOperatorSets(address operator, uint32[] memory operatorSetIds) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromOperatorSetsCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
    }
    ///Container type for the return parameters of the [`deregisterOperatorFromOperatorSets(address,uint32[])`](deregisterOperatorFromOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromOperatorSetsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<u32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromOperatorSetsCall) -> Self {
                    (value.operator, value.operatorSetIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromOperatorSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSetIds: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorFromOperatorSetsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperatorFromOperatorSets(address,uint32[])";
            const SELECTOR: [u8; 4] = [193u8, 168u8, 226u8, 197u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `finalizeMigration()` and selector `0xb78b6087`.
```solidity
function finalizeMigration() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizeMigrationCall {}
    ///Container type for the return parameters of the [`finalizeMigration()`](finalizeMigrationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizeMigrationReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<finalizeMigrationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: finalizeMigrationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for finalizeMigrationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<finalizeMigrationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: finalizeMigrationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for finalizeMigrationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for finalizeMigrationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = finalizeMigrationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "finalizeMigration()";
            const SELECTOR: [u8; 4] = [183u8, 139u8, 96u8, 135u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorRestakedStrategies(address)` and selector `0x33cfb7b7`.
```solidity
function getOperatorRestakedStrategies(address operator) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorRestakedStrategiesCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorRestakedStrategies(address)`](getOperatorRestakedStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorRestakedStrategiesReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorRestakedStrategiesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRestakedStrategiesCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorRestakedStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorRestakedStrategiesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRestakedStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorRestakedStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorRestakedStrategiesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorRestakedStrategiesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorRestakedStrategies(address)";
            const SELECTOR: [u8; 4] = [51u8, 207u8, 183u8, 183u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorsToMigrate()` and selector `0x0b91d665`.
```solidity
function getOperatorsToMigrate() external view returns (uint32[] memory operatorSetIdsToCreate, uint32[][] memory operatorSetIds, address[] memory allOperators);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorsToMigrateCall {}
    ///Container type for the return parameters of the [`getOperatorsToMigrate()`](getOperatorsToMigrateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorsToMigrateReturn {
        pub operatorSetIdsToCreate: alloy::sol_types::private::Vec<u32>,
        pub operatorSetIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<u32>,
        >,
        pub allOperators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorsToMigrateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorsToMigrateCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorsToMigrateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    >,
                >,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<u32>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorsToMigrateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorsToMigrateReturn) -> Self {
                    (
                        value.operatorSetIdsToCreate,
                        value.operatorSetIds,
                        value.allOperators,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorsToMigrateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetIdsToCreate: tuple.0,
                        operatorSetIds: tuple.1,
                        allOperators: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorsToMigrateCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorsToMigrateReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    >,
                >,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorsToMigrate()";
            const SELECTOR: [u8; 4] = [11u8, 145u8, 214u8, 101u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getRestakeableStrategies()` and selector `0xe481af9d`.
```solidity
function getRestakeableStrategies() external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRestakeableStrategiesCall {}
    ///Container type for the return parameters of the [`getRestakeableStrategies()`](getRestakeableStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRestakeableStrategiesReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRestakeableStrategiesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRestakeableStrategiesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRestakeableStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRestakeableStrategiesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRestakeableStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRestakeableStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRestakeableStrategiesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRestakeableStrategiesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRestakeableStrategies()";
            const SELECTOR: [u8; 4] = [228u8, 129u8, 175u8, 157u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `incredibleSquaringTaskManager()` and selector `0x77ef731d`.
```solidity
function incredibleSquaringTaskManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct incredibleSquaringTaskManagerCall {}
    ///Container type for the return parameters of the [`incredibleSquaringTaskManager()`](incredibleSquaringTaskManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct incredibleSquaringTaskManagerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<incredibleSquaringTaskManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: incredibleSquaringTaskManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for incredibleSquaringTaskManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<incredibleSquaringTaskManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: incredibleSquaringTaskManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for incredibleSquaringTaskManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for incredibleSquaringTaskManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = incredibleSquaringTaskManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "incredibleSquaringTaskManager()";
            const SELECTOR: [u8; 4] = [119u8, 239u8, 115u8, 29u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `migrateAndCreateOperatorSetIds(uint32[])` and selector `0x15b7bc9a`.
```solidity
function migrateAndCreateOperatorSetIds(uint32[] memory operatorSetsToCreate) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrateAndCreateOperatorSetIdsCall {
        pub operatorSetsToCreate: alloy::sol_types::private::Vec<u32>,
    }
    ///Container type for the return parameters of the [`migrateAndCreateOperatorSetIds(uint32[])`](migrateAndCreateOperatorSetIdsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrateAndCreateOperatorSetIdsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Vec<u32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<migrateAndCreateOperatorSetIdsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrateAndCreateOperatorSetIdsCall) -> Self {
                    (value.operatorSetsToCreate,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrateAndCreateOperatorSetIdsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetsToCreate: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<migrateAndCreateOperatorSetIdsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrateAndCreateOperatorSetIdsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrateAndCreateOperatorSetIdsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for migrateAndCreateOperatorSetIdsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = migrateAndCreateOperatorSetIdsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "migrateAndCreateOperatorSetIds(uint32[])";
            const SELECTOR: [u8; 4] = [21u8, 183u8, 188u8, 154u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetsToCreate),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `migrateToOperatorSets(uint32[][],address[])` and selector `0xd9f95377`.
```solidity
function migrateToOperatorSets(uint32[][] memory operatorSetIds, address[] memory operators) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrateToOperatorSetsCall {
        pub operatorSetIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<u32>,
        >,
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`migrateToOperatorSets(uint32[][],address[])`](migrateToOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrateToOperatorSetsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    >,
                >,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<migrateToOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrateToOperatorSetsCall) -> Self {
                    (value.operatorSetIds, value.operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrateToOperatorSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetIds: tuple.0,
                        operators: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<migrateToOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrateToOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrateToOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for migrateToOperatorSetsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    >,
                >,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = migrateToOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "migrateToOperatorSets(uint32[][],address[])";
            const SELECTOR: [u8; 4] = [217u8, 249u8, 83u8, 119u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `migrationFinalized()` and selector `0x8d68349a`.
```solidity
function migrationFinalized() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrationFinalizedCall {}
    ///Container type for the return parameters of the [`migrationFinalized()`](migrationFinalizedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrationFinalizedReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<migrationFinalizedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrationFinalizedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrationFinalizedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<migrationFinalizedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrationFinalizedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrationFinalizedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for migrationFinalizedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = migrationFinalizedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "migrationFinalized()";
            const SELECTOR: [u8; 4] = [141u8, 104u8, 52u8, 154u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `proposeNewSlasher(address)` and selector `0x8999817f`.
```solidity
function proposeNewSlasher(address newSlasher) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposeNewSlasherCall {
        pub newSlasher: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`proposeNewSlasher(address)`](proposeNewSlasherCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposeNewSlasherReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposeNewSlasherCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposeNewSlasherCall) -> Self {
                    (value.newSlasher,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposeNewSlasherCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newSlasher: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposeNewSlasherReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposeNewSlasherReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposeNewSlasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposeNewSlasherCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposeNewSlasherReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposeNewSlasher(address)";
            const SELECTOR: [u8; 4] = [137u8, 153u8, 129u8, 127u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newSlasher,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `proposedSlasher()` and selector `0xe46f1816`.
```solidity
function proposedSlasher() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposedSlasherCall {}
    ///Container type for the return parameters of the [`proposedSlasher()`](proposedSlasherCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposedSlasherReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposedSlasherCall> for UnderlyingRustTuple<'_> {
                fn from(value: proposedSlasherCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposedSlasherCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposedSlasherReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposedSlasherReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposedSlasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposedSlasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposedSlasherReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposedSlasher()";
            const SELECTOR: [u8; 4] = [228u8, 111u8, 24u8, 22u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `registerOperatorToAVS(address,(bytes,bytes32,uint256))` and selector `0x9926ee7d`.
```solidity
function registerOperatorToAVS(address operator, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToAVSCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`registerOperatorToAVS(address,(bytes,bytes32,uint256))`](registerOperatorToAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToAVSReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorToAVSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSCall) -> Self {
                    (value.operator, value.operatorSignature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorToAVSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSignature: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorToAVSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorToAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorToAVSCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorToAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperatorToAVS(address,(bytes,bytes32,uint256))";
            const SELECTOR: [u8; 4] = [153u8, 38u8, 238u8, 125u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <ISignatureUtils::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSignature,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `registerOperatorToOperatorSets(address,uint32[],(bytes,bytes32,uint256))` and selector `0x1e2199e2`.
```solidity
function registerOperatorToOperatorSets(address operator, uint32[] memory operatorSetIds, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToOperatorSetsCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
        pub operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`registerOperatorToOperatorSets(address,uint32[],(bytes,bytes32,uint256))`](registerOperatorToOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToOperatorSetsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<u32>,
                <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorToOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToOperatorSetsCall) -> Self {
                    (value.operator, value.operatorSetIds, value.operatorSignature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorToOperatorSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSetIds: tuple.1,
                        operatorSignature: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorToOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorToOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorToOperatorSetsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorToOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperatorToOperatorSets(address,uint32[],(bytes,bytes32,uint256))";
            const SELECTOR: [u8; 4] = [30u8, 33u8, 153u8, 226u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
                    <ISignatureUtils::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSignature,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `rewardsInitiator()` and selector `0xfc299dee`.
```solidity
function rewardsInitiator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsInitiatorCall {}
    ///Container type for the return parameters of the [`rewardsInitiator()`](rewardsInitiatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsInitiatorReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<rewardsInitiatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: rewardsInitiatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rewardsInitiatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<rewardsInitiatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: rewardsInitiatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rewardsInitiatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rewardsInitiatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = rewardsInitiatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rewardsInitiator()";
            const SELECTOR: [u8; 4] = [252u8, 41u8, 157u8, 238u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setRewardsInitiator(address)` and selector `0x3bc28c8c`.
```solidity
function setRewardsInitiator(address newRewardsInitiator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsInitiatorCall {
        pub newRewardsInitiator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setRewardsInitiator(address)`](setRewardsInitiatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsInitiatorReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setRewardsInitiatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsInitiatorCall) -> Self {
                    (value.newRewardsInitiator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setRewardsInitiatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newRewardsInitiator: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setRewardsInitiatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsInitiatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setRewardsInitiatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setRewardsInitiatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setRewardsInitiatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setRewardsInitiator(address)";
            const SELECTOR: [u8; 4] = [59u8, 194u8, 140u8, 140u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newRewardsInitiator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `slashOperator((address,uint32,address[],uint256,string))` and selector `0x60db99a3`.
```solidity
function slashOperator(IAllocationManagerTypes.SlashingParams memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorCall {
        pub params: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`slashOperator((address,uint32,address[],uint256,string))`](slashOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IAllocationManagerTypes::SlashingParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<slashOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: slashOperatorCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<slashOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: slashOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashOperatorCall {
            type Parameters<'a> = (IAllocationManagerTypes::SlashingParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashOperator((address,uint32,address[],uint256,string))";
            const SELECTOR: [u8; 4] = [96u8, 219u8, 153u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IAllocationManagerTypes::SlashingParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `slasher()` and selector `0xb1344271`.
```solidity
function slasher() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slasherCall {}
    ///Container type for the return parameters of the [`slasher()`](slasherCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slasherReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<slasherCall> for UnderlyingRustTuple<'_> {
                fn from(value: slasherCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slasherCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<slasherReturn> for UnderlyingRustTuple<'_> {
                fn from(value: slasherReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slasherReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slasher()";
            const SELECTOR: [u8; 4] = [177u8, 52u8, 66u8, 113u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `slasherProposalTimestamp()` and selector `0xfcd1c375`.
```solidity
function slasherProposalTimestamp() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slasherProposalTimestampCall {}
    ///Container type for the return parameters of the [`slasherProposalTimestamp()`](slasherProposalTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slasherProposalTimestampReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<slasherProposalTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: slasherProposalTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slasherProposalTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<slasherProposalTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: slasherProposalTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slasherProposalTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slasherProposalTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slasherProposalTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slasherProposalTimestamp()";
            const SELECTOR: [u8; 4] = [252u8, 209u8, 195u8, 117u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateAVSMetadataURI(string)` and selector `0xa98fb355`.
```solidity
function updateAVSMetadataURI(string memory _metadataURI) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURICall {
        pub _metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateAVSMetadataURI(string)`](updateAVSMetadataURICall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURIReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateAVSMetadataURICall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURICall) -> Self {
                    (value._metadataURI,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateAVSMetadataURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _metadataURI: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateAVSMetadataURIReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURIReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateAVSMetadataURIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateAVSMetadataURICall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateAVSMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateAVSMetadataURI(string)";
            const SELECTOR: [u8; 4] = [169u8, 143u8, 179u8, 85u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._metadataURI,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`IncredibleSquaringServiceManager`](self) function calls.
    pub enum IncredibleSquaringServiceManagerCalls {
        SLASHER_PROPOSAL_DELAY(SLASHER_PROPOSAL_DELAYCall),
        acceptProposedSlasher(acceptProposedSlasherCall),
        allocationManager(allocationManagerCall),
        avsDirectory(avsDirectoryCall),
        createAVSRewardsSubmission(createAVSRewardsSubmissionCall),
        createOperatorSets(createOperatorSetsCall),
        deregisterOperatorFromAVS(deregisterOperatorFromAVSCall),
        deregisterOperatorFromOperatorSets(deregisterOperatorFromOperatorSetsCall),
        finalizeMigration(finalizeMigrationCall),
        getOperatorRestakedStrategies(getOperatorRestakedStrategiesCall),
        getOperatorsToMigrate(getOperatorsToMigrateCall),
        getRestakeableStrategies(getRestakeableStrategiesCall),
        incredibleSquaringTaskManager(incredibleSquaringTaskManagerCall),
        migrateAndCreateOperatorSetIds(migrateAndCreateOperatorSetIdsCall),
        migrateToOperatorSets(migrateToOperatorSetsCall),
        migrationFinalized(migrationFinalizedCall),
        owner(ownerCall),
        proposeNewSlasher(proposeNewSlasherCall),
        proposedSlasher(proposedSlasherCall),
        registerOperatorToAVS(registerOperatorToAVSCall),
        registerOperatorToOperatorSets(registerOperatorToOperatorSetsCall),
        renounceOwnership(renounceOwnershipCall),
        rewardsInitiator(rewardsInitiatorCall),
        setRewardsInitiator(setRewardsInitiatorCall),
        slashOperator(slashOperatorCall),
        slasher(slasherCall),
        slasherProposalTimestamp(slasherProposalTimestampCall),
        transferOwnership(transferOwnershipCall),
        updateAVSMetadataURI(updateAVSMetadataURICall),
    }
    #[automatically_derived]
    impl IncredibleSquaringServiceManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [11u8, 145u8, 214u8, 101u8],
            [21u8, 183u8, 188u8, 154u8],
            [30u8, 33u8, 153u8, 226u8],
            [38u8, 240u8, 23u8, 226u8],
            [51u8, 207u8, 183u8, 183u8],
            [59u8, 194u8, 140u8, 140u8],
            [96u8, 219u8, 153u8, 163u8],
            [103u8, 148u8, 12u8, 137u8],
            [107u8, 58u8, 167u8, 46u8],
            [113u8, 80u8, 24u8, 166u8],
            [119u8, 239u8, 115u8, 29u8],
            [137u8, 153u8, 129u8, 127u8],
            [141u8, 104u8, 52u8, 154u8],
            [141u8, 165u8, 203u8, 91u8],
            [153u8, 38u8, 238u8, 125u8],
            [163u8, 100u8, 244u8, 218u8],
            [169u8, 143u8, 179u8, 85u8],
            [175u8, 224u8, 46u8, 213u8],
            [177u8, 52u8, 66u8, 113u8],
            [183u8, 139u8, 96u8, 135u8],
            [193u8, 168u8, 226u8, 197u8],
            [202u8, 138u8, 167u8, 199u8],
            [217u8, 249u8, 83u8, 119u8],
            [228u8, 111u8, 24u8, 22u8],
            [228u8, 129u8, 175u8, 157u8],
            [242u8, 253u8, 227u8, 139u8],
            [252u8, 41u8, 157u8, 238u8],
            [252u8, 209u8, 195u8, 117u8],
            [252u8, 227u8, 108u8, 125u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IncredibleSquaringServiceManagerCalls {
        const NAME: &'static str = "IncredibleSquaringServiceManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 29usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::SLASHER_PROPOSAL_DELAY(_) => {
                    <SLASHER_PROPOSAL_DELAYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::acceptProposedSlasher(_) => {
                    <acceptProposedSlasherCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avsDirectory(_) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createAVSRewardsSubmission(_) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createOperatorSets(_) => {
                    <createOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperatorFromAVS(_) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperatorFromOperatorSets(_) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::finalizeMigration(_) => {
                    <finalizeMigrationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorRestakedStrategies(_) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorsToMigrate(_) => {
                    <getOperatorsToMigrateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRestakeableStrategies(_) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::incredibleSquaringTaskManager(_) => {
                    <incredibleSquaringTaskManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::migrateAndCreateOperatorSetIds(_) => {
                    <migrateAndCreateOperatorSetIdsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::migrateToOperatorSets(_) => {
                    <migrateToOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::migrationFinalized(_) => {
                    <migrationFinalizedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proposeNewSlasher(_) => {
                    <proposeNewSlasherCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposedSlasher(_) => {
                    <proposedSlasherCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperatorToAVS(_) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperatorToOperatorSets(_) => {
                    <registerOperatorToOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rewardsInitiator(_) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setRewardsInitiator(_) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashOperator(_) => {
                    <slashOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slasher(_) => <slasherCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::slasherProposalTimestamp(_) => {
                    <slasherProposalTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateAVSMetadataURI(_) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>] = &[
                {
                    fn getOperatorsToMigrate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <getOperatorsToMigrateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::getOperatorsToMigrate,
                            )
                    }
                    getOperatorsToMigrate
                },
                {
                    fn migrateAndCreateOperatorSetIds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <migrateAndCreateOperatorSetIdsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::migrateAndCreateOperatorSetIds,
                            )
                    }
                    migrateAndCreateOperatorSetIds
                },
                {
                    fn registerOperatorToOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <registerOperatorToOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::registerOperatorToOperatorSets,
                            )
                    }
                    registerOperatorToOperatorSets
                },
                {
                    fn acceptProposedSlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <acceptProposedSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::acceptProposedSlasher,
                            )
                    }
                    acceptProposedSlasher
                },
                {
                    fn getOperatorRestakedStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::getOperatorRestakedStrategies,
                            )
                    }
                    getOperatorRestakedStrategies
                },
                {
                    fn setRewardsInitiator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::setRewardsInitiator,
                            )
                    }
                    setRewardsInitiator
                },
                {
                    fn slashOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <slashOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringServiceManagerCalls::slashOperator)
                    }
                    slashOperator
                },
                {
                    fn SLASHER_PROPOSAL_DELAY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <SLASHER_PROPOSAL_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::SLASHER_PROPOSAL_DELAY,
                            )
                    }
                    SLASHER_PROPOSAL_DELAY
                },
                {
                    fn avsDirectory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <avsDirectoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringServiceManagerCalls::avsDirectory)
                    }
                    avsDirectory
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::renounceOwnership,
                            )
                    }
                    renounceOwnership
                },
                {
                    fn incredibleSquaringTaskManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <incredibleSquaringTaskManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::incredibleSquaringTaskManager,
                            )
                    }
                    incredibleSquaringTaskManager
                },
                {
                    fn proposeNewSlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <proposeNewSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::proposeNewSlasher,
                            )
                    }
                    proposeNewSlasher
                },
                {
                    fn migrationFinalized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <migrationFinalizedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::migrationFinalized,
                            )
                    }
                    migrationFinalized
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringServiceManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn registerOperatorToAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::registerOperatorToAVS,
                            )
                    }
                    registerOperatorToAVS
                },
                {
                    fn deregisterOperatorFromAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::deregisterOperatorFromAVS,
                            )
                    }
                    deregisterOperatorFromAVS
                },
                {
                    fn updateAVSMetadataURI(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::updateAVSMetadataURI,
                            )
                    }
                    updateAVSMetadataURI
                },
                {
                    fn createOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <createOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::createOperatorSets,
                            )
                    }
                    createOperatorSets
                },
                {
                    fn slasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <slasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringServiceManagerCalls::slasher)
                    }
                    slasher
                },
                {
                    fn finalizeMigration(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <finalizeMigrationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::finalizeMigration,
                            )
                    }
                    finalizeMigration
                },
                {
                    fn deregisterOperatorFromOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::deregisterOperatorFromOperatorSets,
                            )
                    }
                    deregisterOperatorFromOperatorSets
                },
                {
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::allocationManager,
                            )
                    }
                    allocationManager
                },
                {
                    fn migrateToOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <migrateToOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::migrateToOperatorSets,
                            )
                    }
                    migrateToOperatorSets
                },
                {
                    fn proposedSlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <proposedSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringServiceManagerCalls::proposedSlasher)
                    }
                    proposedSlasher
                },
                {
                    fn getRestakeableStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::getRestakeableStrategies,
                            )
                    }
                    getRestakeableStrategies
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::transferOwnership,
                            )
                    }
                    transferOwnership
                },
                {
                    fn rewardsInitiator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringServiceManagerCalls::rewardsInitiator)
                    }
                    rewardsInitiator
                },
                {
                    fn slasherProposalTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <slasherProposalTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::slasherProposalTimestamp,
                            )
                    }
                    slasherProposalTimestamp
                },
                {
                    fn createAVSRewardsSubmission(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::createAVSRewardsSubmission,
                            )
                    }
                    createAVSRewardsSubmission
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::SLASHER_PROPOSAL_DELAY(inner) => {
                    <SLASHER_PROPOSAL_DELAYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::acceptProposedSlasher(inner) => {
                    <acceptProposedSlasherCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createAVSRewardsSubmission(inner) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createOperatorSets(inner) => {
                    <createOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterOperatorFromAVS(inner) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterOperatorFromOperatorSets(inner) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::finalizeMigration(inner) => {
                    <finalizeMigrationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorRestakedStrategies(inner) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorsToMigrate(inner) => {
                    <getOperatorsToMigrateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRestakeableStrategies(inner) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::incredibleSquaringTaskManager(inner) => {
                    <incredibleSquaringTaskManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::migrateAndCreateOperatorSetIds(inner) => {
                    <migrateAndCreateOperatorSetIdsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::migrateToOperatorSets(inner) => {
                    <migrateToOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::migrationFinalized(inner) => {
                    <migrationFinalizedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proposeNewSlasher(inner) => {
                    <proposeNewSlasherCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposedSlasher(inner) => {
                    <proposedSlasherCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerOperatorToAVS(inner) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerOperatorToOperatorSets(inner) => {
                    <registerOperatorToOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rewardsInitiator(inner) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setRewardsInitiator(inner) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slashOperator(inner) => {
                    <slashOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slasher(inner) => {
                    <slasherCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::slasherProposalTimestamp(inner) => {
                    <slasherProposalTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateAVSMetadataURI(inner) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::SLASHER_PROPOSAL_DELAY(inner) => {
                    <SLASHER_PROPOSAL_DELAYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::acceptProposedSlasher(inner) => {
                    <acceptProposedSlasherCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createAVSRewardsSubmission(inner) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createOperatorSets(inner) => {
                    <createOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterOperatorFromAVS(inner) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterOperatorFromOperatorSets(inner) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::finalizeMigration(inner) => {
                    <finalizeMigrationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorRestakedStrategies(inner) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorsToMigrate(inner) => {
                    <getOperatorsToMigrateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRestakeableStrategies(inner) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::incredibleSquaringTaskManager(inner) => {
                    <incredibleSquaringTaskManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::migrateAndCreateOperatorSetIds(inner) => {
                    <migrateAndCreateOperatorSetIdsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::migrateToOperatorSets(inner) => {
                    <migrateToOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::migrationFinalized(inner) => {
                    <migrationFinalizedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::proposeNewSlasher(inner) => {
                    <proposeNewSlasherCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposedSlasher(inner) => {
                    <proposedSlasherCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerOperatorToAVS(inner) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerOperatorToOperatorSets(inner) => {
                    <registerOperatorToOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::rewardsInitiator(inner) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setRewardsInitiator(inner) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slashOperator(inner) => {
                    <slashOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slasher(inner) => {
                    <slasherCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::slasherProposalTimestamp(inner) => {
                    <slasherProposalTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateAVSMetadataURI(inner) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`IncredibleSquaringServiceManager`](self) events.
    pub enum IncredibleSquaringServiceManagerEvents {
        Initialized(Initialized),
        OwnershipTransferred(OwnershipTransferred),
        RewardsInitiatorUpdated(RewardsInitiatorUpdated),
        SlasherProposed(SlasherProposed),
        SlasherUpdated(SlasherUpdated),
    }
    #[automatically_derived]
    impl IncredibleSquaringServiceManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                47u8,
                138u8,
                252u8,
                138u8,
                120u8,
                253u8,
                149u8,
                143u8,
                51u8,
                1u8,
                192u8,
                35u8,
                58u8,
                163u8,
                38u8,
                185u8,
                196u8,
                185u8,
                162u8,
                136u8,
                74u8,
                116u8,
                131u8,
                34u8,
                125u8,
                107u8,
                5u8,
                85u8,
                170u8,
                160u8,
                58u8,
                219u8,
            ],
            [
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
            ],
            [
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ],
            [
                224u8,
                212u8,
                154u8,
                84u8,
                39u8,
                68u8,
                35u8,
                24u8,
                61u8,
                173u8,
                236u8,
                189u8,
                242u8,
                57u8,
                234u8,
                172u8,
                110u8,
                6u8,
                186u8,
                136u8,
                50u8,
                11u8,
                38u8,
                254u8,
                140u8,
                197u8,
                236u8,
                157u8,
                5u8,
                10u8,
                99u8,
                149u8,
            ],
            [
                225u8,
                28u8,
                221u8,
                241u8,
                129u8,
                106u8,
                67u8,
                49u8,
                140u8,
                161u8,
                117u8,
                187u8,
                197u8,
                44u8,
                208u8,
                24u8,
                84u8,
                54u8,
                233u8,
                203u8,
                234u8,
                215u8,
                200u8,
                58u8,
                204u8,
                84u8,
                167u8,
                62u8,
                70u8,
                23u8,
                23u8,
                227u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for IncredibleSquaringServiceManagerEvents {
        const NAME: &'static str = "IncredibleSquaringServiceManagerEvents";
        const COUNT: usize = 5usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(
                    <RewardsInitiatorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RewardsInitiatorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RewardsInitiatorUpdated)
                }
                Some(<SlasherProposed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SlasherProposed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SlasherProposed)
                }
                Some(<SlasherUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SlasherUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SlasherUpdated)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData
    for IncredibleSquaringServiceManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RewardsInitiatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SlasherProposed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SlasherUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RewardsInitiatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SlasherProposed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SlasherUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IncredibleSquaringServiceManager`](self) contract instance.

See the [wrapper's documentation](`IncredibleSquaringServiceManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IncredibleSquaringServiceManagerInstance<T, P, N> {
        IncredibleSquaringServiceManagerInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _avsDirectory: alloy::sol_types::private::Address,
        _registryCoordinator: alloy::sol_types::private::Address,
        _stakeRegistry: alloy::sol_types::private::Address,
        rewards_coordinator: alloy::sol_types::private::Address,
        allocationManager: alloy::sol_types::private::Address,
        _incredibleSquaringTaskManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<
            IncredibleSquaringServiceManagerInstance<T, P, N>,
        >,
    > {
        IncredibleSquaringServiceManagerInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _avsDirectory,
            _registryCoordinator,
            _stakeRegistry,
            rewards_coordinator,
            allocationManager,
            _incredibleSquaringTaskManager,
        )
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _avsDirectory: alloy::sol_types::private::Address,
        _registryCoordinator: alloy::sol_types::private::Address,
        _stakeRegistry: alloy::sol_types::private::Address,
        rewards_coordinator: alloy::sol_types::private::Address,
        allocationManager: alloy::sol_types::private::Address,
        _incredibleSquaringTaskManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        IncredibleSquaringServiceManagerInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _avsDirectory,
            _registryCoordinator,
            _stakeRegistry,
            rewards_coordinator,
            allocationManager,
            _incredibleSquaringTaskManager,
        )
    }
    /**A [`IncredibleSquaringServiceManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IncredibleSquaringServiceManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IncredibleSquaringServiceManagerInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug
    for IncredibleSquaringServiceManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IncredibleSquaringServiceManagerInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IncredibleSquaringServiceManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IncredibleSquaringServiceManager`](self) contract instance.

See the [wrapper's documentation](`IncredibleSquaringServiceManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _avsDirectory: alloy::sol_types::private::Address,
            _registryCoordinator: alloy::sol_types::private::Address,
            _stakeRegistry: alloy::sol_types::private::Address,
            rewards_coordinator: alloy::sol_types::private::Address,
            allocationManager: alloy::sol_types::private::Address,
            _incredibleSquaringTaskManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<IncredibleSquaringServiceManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _avsDirectory,
                _registryCoordinator,
                _stakeRegistry,
                rewards_coordinator,
                allocationManager,
                _incredibleSquaringTaskManager,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _avsDirectory: alloy::sol_types::private::Address,
            _registryCoordinator: alloy::sol_types::private::Address,
            _stakeRegistry: alloy::sol_types::private::Address,
            rewards_coordinator: alloy::sol_types::private::Address,
            allocationManager: alloy::sol_types::private::Address,
            _incredibleSquaringTaskManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _avsDirectory,
                            _registryCoordinator,
                            _stakeRegistry,
                            rewards_coordinator,
                            allocationManager,
                            _incredibleSquaringTaskManager,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<
        T,
        P: ::core::clone::Clone,
        N,
    > IncredibleSquaringServiceManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> IncredibleSquaringServiceManagerInstance<T, P, N> {
            IncredibleSquaringServiceManagerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IncredibleSquaringServiceManagerInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`SLASHER_PROPOSAL_DELAY`] function.
        pub fn SLASHER_PROPOSAL_DELAY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, SLASHER_PROPOSAL_DELAYCall, N> {
            self.call_builder(&SLASHER_PROPOSAL_DELAYCall {})
        }
        ///Creates a new call builder for the [`acceptProposedSlasher`] function.
        pub fn acceptProposedSlasher(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, acceptProposedSlasherCall, N> {
            self.call_builder(&acceptProposedSlasherCall {})
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
        }
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall {})
        }
        ///Creates a new call builder for the [`createAVSRewardsSubmission`] function.
        pub fn createAVSRewardsSubmission(
            &self,
            rewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createAVSRewardsSubmissionCall, N> {
            self.call_builder(
                &createAVSRewardsSubmissionCall {
                    rewardsSubmissions,
                },
            )
        }
        ///Creates a new call builder for the [`createOperatorSets`] function.
        pub fn createOperatorSets(
            &self,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, createOperatorSetsCall, N> {
            self.call_builder(
                &createOperatorSetsCall {
                    operatorSetIds,
                },
            )
        }
        ///Creates a new call builder for the [`deregisterOperatorFromAVS`] function.
        pub fn deregisterOperatorFromAVS(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorFromAVSCall, N> {
            self.call_builder(
                &deregisterOperatorFromAVSCall {
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`deregisterOperatorFromOperatorSets`] function.
        pub fn deregisterOperatorFromOperatorSets(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            deregisterOperatorFromOperatorSetsCall,
            N,
        > {
            self.call_builder(
                &deregisterOperatorFromOperatorSetsCall {
                    operator,
                    operatorSetIds,
                },
            )
        }
        ///Creates a new call builder for the [`finalizeMigration`] function.
        pub fn finalizeMigration(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, finalizeMigrationCall, N> {
            self.call_builder(&finalizeMigrationCall {})
        }
        ///Creates a new call builder for the [`getOperatorRestakedStrategies`] function.
        pub fn getOperatorRestakedStrategies(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getOperatorRestakedStrategiesCall,
            N,
        > {
            self.call_builder(
                &getOperatorRestakedStrategiesCall {
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorsToMigrate`] function.
        pub fn getOperatorsToMigrate(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorsToMigrateCall, N> {
            self.call_builder(&getOperatorsToMigrateCall {})
        }
        ///Creates a new call builder for the [`getRestakeableStrategies`] function.
        pub fn getRestakeableStrategies(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRestakeableStrategiesCall, N> {
            self.call_builder(&getRestakeableStrategiesCall {})
        }
        ///Creates a new call builder for the [`incredibleSquaringTaskManager`] function.
        pub fn incredibleSquaringTaskManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            incredibleSquaringTaskManagerCall,
            N,
        > {
            self.call_builder(
                &incredibleSquaringTaskManagerCall {
                },
            )
        }
        ///Creates a new call builder for the [`migrateAndCreateOperatorSetIds`] function.
        pub fn migrateAndCreateOperatorSetIds(
            &self,
            operatorSetsToCreate: alloy::sol_types::private::Vec<u32>,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            migrateAndCreateOperatorSetIdsCall,
            N,
        > {
            self.call_builder(
                &migrateAndCreateOperatorSetIdsCall {
                    operatorSetsToCreate,
                },
            )
        }
        ///Creates a new call builder for the [`migrateToOperatorSets`] function.
        pub fn migrateToOperatorSets(
            &self,
            operatorSetIds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<u32>,
            >,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, migrateToOperatorSetsCall, N> {
            self.call_builder(
                &migrateToOperatorSetsCall {
                    operatorSetIds,
                    operators,
                },
            )
        }
        ///Creates a new call builder for the [`migrationFinalized`] function.
        pub fn migrationFinalized(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, migrationFinalizedCall, N> {
            self.call_builder(&migrationFinalizedCall {})
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`proposeNewSlasher`] function.
        pub fn proposeNewSlasher(
            &self,
            newSlasher: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, proposeNewSlasherCall, N> {
            self.call_builder(
                &proposeNewSlasherCall {
                    newSlasher,
                },
            )
        }
        ///Creates a new call builder for the [`proposedSlasher`] function.
        pub fn proposedSlasher(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, proposedSlasherCall, N> {
            self.call_builder(&proposedSlasherCall {})
        }
        ///Creates a new call builder for the [`registerOperatorToAVS`] function.
        pub fn registerOperatorToAVS(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorToAVSCall, N> {
            self.call_builder(
                &registerOperatorToAVSCall {
                    operator,
                    operatorSignature,
                },
            )
        }
        ///Creates a new call builder for the [`registerOperatorToOperatorSets`] function.
        pub fn registerOperatorToOperatorSets(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
            operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            registerOperatorToOperatorSetsCall,
            N,
        > {
            self.call_builder(
                &registerOperatorToOperatorSetsCall {
                    operator,
                    operatorSetIds,
                    operatorSignature,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`rewardsInitiator`] function.
        pub fn rewardsInitiator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, rewardsInitiatorCall, N> {
            self.call_builder(&rewardsInitiatorCall {})
        }
        ///Creates a new call builder for the [`setRewardsInitiator`] function.
        pub fn setRewardsInitiator(
            &self,
            newRewardsInitiator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setRewardsInitiatorCall, N> {
            self.call_builder(
                &setRewardsInitiatorCall {
                    newRewardsInitiator,
                },
            )
        }
        ///Creates a new call builder for the [`slashOperator`] function.
        pub fn slashOperator(
            &self,
            params: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashOperatorCall, N> {
            self.call_builder(&slashOperatorCall { params })
        }
        ///Creates a new call builder for the [`slasher`] function.
        pub fn slasher(&self) -> alloy_contract::SolCallBuilder<T, &P, slasherCall, N> {
            self.call_builder(&slasherCall {})
        }
        ///Creates a new call builder for the [`slasherProposalTimestamp`] function.
        pub fn slasherProposalTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, slasherProposalTimestampCall, N> {
            self.call_builder(&slasherProposalTimestampCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`updateAVSMetadataURI`] function.
        pub fn updateAVSMetadataURI(
            &self,
            _metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateAVSMetadataURICall, N> {
            self.call_builder(
                &updateAVSMetadataURICall {
                    _metadataURI,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IncredibleSquaringServiceManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`RewardsInitiatorUpdated`] event.
        pub fn RewardsInitiatorUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RewardsInitiatorUpdated, N> {
            self.event_filter::<RewardsInitiatorUpdated>()
        }
        ///Creates a new event filter for the [`SlasherProposed`] event.
        pub fn SlasherProposed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SlasherProposed, N> {
            self.event_filter::<SlasherProposed>()
        }
        ///Creates a new event filter for the [`SlasherUpdated`] event.
        pub fn SlasherUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SlasherUpdated, N> {
            self.event_filter::<SlasherUpdated>()
        }
    }
}
