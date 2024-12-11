///Module containing a contract's types and functions.
/**

```solidity
library IAllocationManagerTypes {
    struct CreateSetParams { uint32 operatorSetId; address[] strategies; }
    struct SlashingParams { address operator; uint32 operatorSetId; uint256 wadToSlash; string description; }
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
struct CreateSetParams { uint32 operatorSetId; address[] strategies; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CreateSetParams {
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
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
        impl ::core::convert::From<CreateSetParams> for UnderlyingRustTuple<'_> {
            fn from(value: CreateSetParams) -> Self {
                (value.operatorSetId, value.strategies)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CreateSetParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operatorSetId: tuple.0,
                    strategies: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CreateSetParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CreateSetParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
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
        impl alloy_sol_types::SolType for CreateSetParams {
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
        impl alloy_sol_types::SolStruct for CreateSetParams {
            const NAME: &'static str = "CreateSetParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CreateSetParams(uint32 operatorSetId,address[] strategies)",
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
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorSetId)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CreateSetParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
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
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
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
struct SlashingParams { address operator; uint32 operatorSetId; uint256 wadToSlash; string description; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SlashingParams {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetId: u32,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            u32,
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
                    wadToSlash: tuple.2,
                    description: tuple.3,
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
                    "SlashingParams(address operator,uint32 operatorSetId,uint256 wadToSlash,string description)",
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
    struct CreateSetParams {
        uint32 operatorSetId;
        address[] strategies;
    }
    struct SlashingParams {
        address operator;
        uint32 operatorSetId;
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
    function createOperatorSet(IAllocationManagerTypes.CreateSetParams[] memory params) external;
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
    function setAvsRegistrar() external;
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
    "name": "createOperatorSet",
    "inputs": [
      {
        "name": "params",
        "type": "tuple[]",
        "internalType": "struct IAllocationManagerTypes.CreateSetParams[]",
        "components": [
          {
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
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
    "name": "setAvsRegistrar",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    ///0x6101403461022457601f612e1738819003918201601f19168301916001600160401b038311848410176102285780849260c094604052833981010312610224578051906001600160a01b03821682036102245760208101516001600160a01b0381168103610224576040820151906001600160a01b03821682036102245760608301516001600160a01b03811690819003610224576080840151936001600160a01b03851685036102245760a00151946001600160a01b03861686036102245760805260a05260c05260e052610100525f5460ff8160081c166101cf5760ff80821610610195575b5061012052604051612bda908161023d8239608051818181610d0801528181610fcf015281816110b80152611168015260a05181611704015260c05181818161035101528181610de801528181610f9f0152818161108801528181611d6601528181611e22015281816122620152612a97015260e051818181611ebd01526122ae015261010051818181610da40152818161129a015281816114780152611b8f01526101205181610e580152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f6100e7565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c80630b91d665146102045780630ea43e43146101ff57806315b7bc9a146101fa5780631e2199e2146101f557806326f017e2146101f057806333cfb7b7146101eb5780633bc28c8c146101e657806367940c89146101e15780636b3aa72e146101dc578063715018a6146101d757806372080075146101d257806377ef731d146101cd5780638999817f146101c85780638d68349a146101c35780638da5cb5b146101be5780639926ee7d146101b9578063a364f4da146101b4578063a98fb355146101af578063afe02ed5146101aa578063b1344271146101a5578063b78b6087146101a0578063c1a8e2c51461019b578063ca8aa7c714610196578063d9f9537714610191578063e46f18161461018c578063e481af9d14610187578063e61bbee814610182578063f2fde38b1461017d578063fc299dee14610178578063fcd1c375146101735763fce36c7d1461016e575f80fd5b6116df565b6116c2565b61169a565b6115ca565b611467565b61141e565b6113f6565b611330565b611285565b611240565b61120c565b6111e4565b6111d6565b61112d565b611064565b610f5c565b610f34565b610f12565b610e87565b610e43565b610d92565b610d37565b610cf3565b610cd6565b610c53565b610c17565b610b61565b610afc565b610a44565b610905565b610332565b5f91031261021357565b5f80fd5b90602080835192838152019201905f5b8181106102345750505090565b82516001600160a01b0316845260209384019390920191600101610227565b6060808252825190820181905260808201959492602001905f5b818110610316575050508085036020820152825180865260208601906020808260051b8901019501915f905b8282106102ba57505050506102b79394506040818403910152610217565b90565b90919295601f19898203018252865190602080835192838152019201905f905b8082106102f857505050602080600192980192019201909291610299565b90919260208060019263ffffffff87511681520194019201906102da565b825163ffffffff1688526020978801979092019160010161026d565b34610213575f36600319011261021357604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156105245760ff915f916107f3575b50169061039e6119b8565b6103a7836119d3565b914363ffffffff16905f5b60ff8116948686101561064c57604051634f4c91e160e11b815294602086600481875afa958615610524575f9661062c575b50604051638902624560e01b815260ff8416600482015263ffffffff86166024820152955f90879060449082906001600160a01b03165afa958615610524575f96610608575b5061043586516119d3565b975f915b875183101561054657604051632efa2ca360e11b81526020816004818a5afa9081156105245761049a916020915f91610529575b50610478868c611acc565b519060405180809581946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa918215610524576104db8c600194876104ea955f926104f2575b506104cc9192611acc565b6001600160a01b039091169052565b6104e48c61265c565b90612712565b920191610439565b6104cc92506105179060203d811161051d575b61050f8183610856565b810190611a2a565b916104c1565b503d610505565b6119ad565b6105409150823d811161051d5761050f8183610856565b5f61046d565b965097505094909161055885516119d3565b965f945f5b87518110156105d15789886105916105856105788584611acc565b516001600160a01b031690565b6001600160a01b031690565b6105a0575b505060010161055d565b8291986104cc6105b86105786001966105c895611acc565b916105c281611ae5565b9b611acc565b90508988610596565b509488529596939450916105fd916105f8906105ed818a611acc565b9063ffffffff169052565b611a19565b9493949291926103b2565b6106259196503d805f833e61061d8183610856565b810190611a3f565b945f61042a565b61064591965060203d811161051d5761050f8183610856565b945f6103e4565b8492915061065a8351611af3565b915f5b84518110156107dd576106a160206106786105788489611acc565b6040516309aa152760e11b81526001600160a01b03909116600482015291829081906024820190565b0381865afa908115610524576106db916020915f916107b0575b506040518093819263871ef04960e01b8352600483019190602083019252565b0381865afa801561052457610700915f91610782575b506001600160c01b03166128c8565b9161070b83516119d3565b935f5b845181101561075a578061075461074a61074461073e6107306001968b611b6a565b516001600160f81b03191690565b60f81c90565b60ff1690565b6105ed838a611acc565b0161070e565b50936001929196935061076d8287611acc565b526107788186611acc565b500193909361065d565b6107a3915060203d81116107a9575b61079b8183610856565b810190611b4b565b876106f1565b503d610791565b6107d09150823d81116107d6575b6107c88183610856565b810190611b3c565b886106bb565b503d6107be565b50506107ef8360405193849384610253565b0390f35b610815915060203d60201161081b575b61080d8183610856565b810190611994565b5f610393565b503d610803565b634e487b7160e01b5f52604160045260245ffd5b608081019081106001600160401b0382111761085157604052565b610822565b90601f801991011681019081106001600160401b0382111761085157604052565b6001600160a01b0381160361021357565b359063ffffffff8216820361021357565b6001600160401b03811161085157601f01601f191660200190565b9291926108c082610899565b916108ce6040519384610856565b829481845281830111610213578281602093845f960137010152565b9080601f83011215610213578160206102b7933591016108b4565b34610213576020366003190112610213576004356001600160401b03811161021357608060031982360301126102135760405161094181610836565b816004013561094f81610877565b815261095d60248301610888565b60208201526044820135604082015260648201356001600160401b0381116102135761099c92600461099292369201016108ea565b6060820152611b7b565b005b6001600160401b0381116108515760051b60200190565b9080601f830112156102135781356109cc8161099e565b926109da6040519485610856565b81845260208085019260051b82010192831161021357602001905b828210610a025750505090565b60208091610a0f84610888565b8152019101906109f5565b602060031982011261021357600435906001600160401b038211610213576102b7916004016109b5565b3461021357610a5236610a1a565b5061099c612967565b9181601f84011215610213578235916001600160401b038311610213576020808501948460051b01011161021357565b9190916060818403126102135760405190606082018281106001600160401b0382111761085157604052819381356001600160401b0381116102135782019181601f8401121561021357610ae96040939283602086953591016108b4565b8452602081013560208501520135910152565b3461021357606036600319011261021357610b18600435610877565b6024356001600160401b03811161021357610b37903690600401610a5b565b50506044356001600160401b03811161021357610b58903690600401610a8b565b5061099c611d60565b34610213575f36600319011261021357610b79612967565b60685462093a808101809111610c12574210610bb657606754610ba4906001600160a01b03166129bf565b606780546001600160a01b0319169055005b60405162461bcd60e51b815260206004820152602e60248201527f536572766963654d616e616765723a20536c61736865722070726f706f73616c60448201526d0819195b185e481b9bdd081b595d60921b6064820152608490fd5b611a05565b34610213576020366003190112610213576107ef610c3f600435610c3a81610877565b611e03565b604051918291602083526020830190610217565b3461021357602036600319011261021357600435610c7081610877565b610c78612967565b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555005b34610213575f36600319011261021357602060405162093a808152f35b34610213575f366003190112610213576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610213575f36600319011261021357610d4f612967565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34610213575f366003190112610213577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b15610213576040516334f65bfd60e21b81523060048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166024820152905f908290604490829084905af1801561052457610e2f57005b80610e3d5f61099c93610856565b80610209565b34610213575f366003190112610213576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610213576020366003190112610213577f2f8afc8a78fd958f3301c0233aa326b9c4b9a2884a7483227d6b0555aaa03adb610f0d600435610ec881610877565b610ed0612967565b606780546001600160a01b0319166001600160a01b0392909216918217905542606881905560408051928352602083019190915290918291820190565b0390a1005b34610213575f36600319011261021357602060ff606954166040519015158152f35b34610213575f366003190112610213576033546040516001600160a01b039091168152602090f35b3461021357604036600319011261021357600435610f7981610877565b6024356001600160401b03811161021357610f98903690600401610a8b565b90610fcd337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611cd3565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b15610213575f928392604051948580948193639926ee7d60e01b835260018060a01b0316600483015260406024830152604061104282516060604486015260a4850190611caf565b91602081015160648501520151608483015203925af1801561052457610e2f57005b34610213575f60203660031901126102135760043561108281610877565b6110b6337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611cd3565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610213576040516351b27a6d60e11b81526001600160a01b039091166004820152905f908290602490829084905af1801561052457611121575080f35b61099c91505f90610856565b34610213575f6020366003190112610213576004356001600160401b0381116102135761115e9036906004016108ea565b611166612967565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102135760405163a98fb35560e01b815260206004820152915f9183918290849082906111c4906024830190611caf565b03925af1801561052457611121575080f35b3461021357610b5836610a1a565b34610213575f366003190112610213576066546040516001600160a01b039091168152602090f35b34610213575f36600319011261021357611224612967565b600160695461123660ff82161561213c565b60ff191617606955005b346102135760403660031901126102135761125c600435610877565b6024356001600160401b0381116102135761127b903690600401610a5b565b505061099c611d60565b34610213575f366003190112610213576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b9080601f830112156102135781356112e08161099e565b926112ee6040519485610856565b81845260208085019260051b82010192831161021357602001905b8282106113165750505090565b60208091833561132581610877565b815201910190611309565b34610213576040366003190112610213576004356001600160401b038111610213573660238201121561021357806004013561136b8161099e565b916113796040519384610856565b8183526024602084019260051b820101903682116102135760248101925b8284106113c757602435856001600160401b038211610213576113c161099c9236906004016112c9565b9061219c565b83356001600160401b038111610213576020916113eb8392602436918701016109b5565b815201930192611397565b34610213575f366003190112610213576067546040516001600160a01b039091168152602090f35b34610213575f366003190112610213576107ef610c3f612253565b602060031982011261021357600435906001600160401b0382116102135761146391600401610a5b565b9091565b346102135761147536611439565b907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156102135791604051928391630130fc2760e51b835280604484013060048601526040602486015252606483019060648160051b8501019280925f91603e19813603015b84841061150557875f81808a0381838e5af1801561052457610e2f57005b9193959092949650606319898203018452863582811215610213578301604082019063ffffffff61153582610888565b1683526020810135601e1982360301811215610213570190602082359201926001600160401b038311610213578260051b36038413610213578260609260406020840152520191905f905b8082106115a257505050602080600192980194019401918896959394916114e7565b90919260208060019286356115b681610877565b848060a01b03168152019401920190611580565b34610213576020366003190112610213576004356115e781610877565b6115ef612967565b6001600160a01b0381161561164657603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3005b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b34610213575f366003190112610213576065546040516001600160a01b039091168152602090f35b34610213575f366003190112610213576020606854604051908152f35b34610213576116ed36611439565b6065549091906001600160a01b03163303611914577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316915f5b8181106117725750823b1561021357611762925f92836040518096819582946321f5223b60e11b84523060048501612531565b03925af1801561052457610e2f57005b5f60206117cd6117906105858361178a87898b612480565b016124a2565b604061179d86888a612480565b6040516323b872dd60e01b8152336004820152306024820152910135604482015293849283919082906064820190565b03925af18015610524576118f8575b506117f0610585602061178a848688612480565b604051636eb1769f60e11b81523060048201526001600160a01b03861660248201529190602090839060449082905afa8015610524576118936020915f9485916118db575b5061186061184b6105858561178a888b8d612480565b916040611859878a8c612480565b0135611d94565b60405163095ea7b360e01b81526001600160a01b038a166004820152602481019190915294859283919082906044820190565b03925af1918215610524576001926118ad575b500161172f565b6118cd9060203d81116118d4575b6118c58183610856565b8101906124ac565b505f6118a6565b503d6118bb565b6118f29150833d81116107d6576107c88183610856565b5f611835565b61190f9060203d81116118d4576118c58183610856565b6117dc565b60405162461bcd60e51b815260206004820152604c60248201527f536572766963654d616e61676572426173652e6f6e6c7952657761726473496e60448201527f69746961746f723a2063616c6c6572206973206e6f742074686520726577617260648201526b32399034b734ba34b0ba37b960a11b608482015260a490fd5b90816020910312610213575160ff811681036102135790565b6040513d5f823e3d90fd5b604051906119c7602083610856565b5f808352366020840137565b906119dd8261099e565b6119ea6040519182610856565b82815280926119fb601f199161099e565b0190602036910137565b634e487b7160e01b5f52601160045260245ffd5b60ff1660ff8114610c125760010190565b9081602091031261021357516102b781610877565b602081830312610213578051906001600160401b03821161021357019080601f83011215610213578151611a728161099e565b92611a806040519485610856565b81845260208085019260051b82010192831161021357602001905b828210611aa85750505090565b8151815260209182019101611a9b565b634e487b7160e01b5f52603260045260245ffd5b8051821015611ae05760209160051b010190565b611ab8565b5f198114610c125760010190565b90611afd8261099e565b611b0a6040519182610856565b8281528092611b1b601f199161099e565b01905f5b828110611b2b57505050565b806060602080938501015201611b1f565b90816020910312610213575190565b9081602091031261021357516001600160c01b03811681036102135790565b908151811015611ae0570160200190565b6066546001600160a01b03163303611c44577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102135760605f91611c239383604051809681958294630dcfb3f560e31b84523060048501526040602485015260018060a01b03815116604485015263ffffffff6020820151166064850152604081015160848501520151608060a484015260c4830190611caf565b03925af1801561052457611c345750565b80610e3d5f611c4293610856565b565b60405162461bcd60e51b815260206004820152603960248201527f536572766963654d616e61676572426173652e6f6e6c79536c61736865723a2060448201527f63616c6c6572206973206e6f742074686520736c6173686572000000000000006064820152608490fd5b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b15611cda57565b60405162461bcd60e51b815260206004820152605260248201527f536572766963654d616e61676572426173652e6f6e6c7952656769737472794360448201527f6f6f7264696e61746f723a2063616c6c6572206973206e6f742074686520726560648201527133b4b9ba393c9031b7b7b93234b730ba37b960711b608482015260a490fd5b611c42337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611cd3565b91908201809211610c1257565b6bffffffffffffffffffffffff81160361021357565b90816040910312610213576040519060408201908282106001600160401b03831117610851576020916040528051611dee81610877565b83520151611dfb81611da1565b602082015290565b6040516309aa152760e11b81526001600160a01b0391821660048201527f000000000000000000000000000000000000000000000000000000000000000090911690602081602481855afa90811561052457611e82916020915f9161211f57506040518093819263871ef04960e01b8352600483019190602083019252565b0381855afa908115610524575f91612100575b506001600160c01b03169081159081156120ad575b506120a457611eb8906128c8565b5f91907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690835b8151851015611f7057611f296020611f0661073e6107308987611b6a565b604051633ca5a5f560e01b815260ff909116600482015291829081906024820190565b0381875afa801561052457600192611f48925f92611f50575b50611d94565b940193611ee8565b611f6991925060203d81116107d6576107c88183610856565b905f611f42565b611f7b9194506119d3565b925f905f5b815181101561209e57611f9961073e6107308385611b6a565b604051633ca5a5f560e01b815260ff8216600482015290602082602481895afa918215610524575f9261207e575b50905f915b818310611fde57505050600101611f80565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa918215610524576120428b6104cc8361203c610585600198612047985f91612050575b50516001600160a01b031690565b92611acc565b611ae5565b95019190611fcc565b612071915060403d8111612077575b6120698183610856565b810190611db7565b5f61202e565b503d61205f565b61209791925060203d81116107d6576107c88183610856565b905f611fc7565b50505050565b506102b76119b8565b604051639aa1653d60e01b81529150602090829060049082905afa80156105245760ff915f916120e1575b5016155f611eaa565b6120fa915060203d60201161081b5761080d8183610856565b5f6120d8565b612119915060203d6020116107a95761079b8183610856565b5f611e95565b6121369150823d84116107d6576107c88183610856565b5f6106bb565b1561214357565b60405162461bcd60e51b815260206004820152602b60248201527f536572766963654d616e616765723a204d6967726174696f6e20416c7265616460448201526a1e48119a5b985b1a5e995960aa1b6064820152608490fd5b91906121a6612967565b6121b560ff606954161561213c565b80518351036121fa575f5b81518110156121f457806121ed6121dc61057860019486611acc565b6121e68388611acc565b5190612a78565b50016121c0565b50509050565b60405162461bcd60e51b815260206004820152602b60248201527f536572766963654d616e616765723a20496e707574206172726179206c656e6760448201526a0e8d040dad2e6dac2e8c6d60ab1b6064820152608490fd5b604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156105245760ff915f91612461575b50168015612457577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316905f9081905b80831061241357506122ee91506119d3565b925f905f5b604051639aa1653d60e01b8152602081600481895afa80156105245760ff915f916123f5575b50168110156123ee57604051633ca5a5f560e01b815260ff821660048201819052602082602481895afa918215610524575f926123ce575b50905f915b818310612368575050506001016122f3565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa918215610524576120428b6104cc8361203c6105856001986123c5985f916120505750516001600160a01b031690565b95019190612356565b6123e791925060203d81116107d6576107c88183610856565b905f612351565b5092505050565b61240d915060203d811161081b5761080d8183610856565b5f612319565b604051633ca5a5f560e01b815260ff84166004820152909190602081602481885afa80156105245760019261244e925f92611f505750611d94565b920191906122dc565b50506102b76119b8565b61247a915060203d60201161081b5761080d8183610856565b5f6122a4565b9190811015611ae05760051b81013590609e1981360301821215610213570190565b356102b781610877565b90816020910312610213575180151581036102135790565b916020908281520191905f5b8181106124dd5750505090565b90919260408060019286356124f181610877565b848060a01b031681526bffffffffffffffffffffffff602088013561251581611da1565b1660208201520194019291016124d0565b3590611c4282610877565b928091604085019060018060a01b031685526040602086015252606083019060608160051b85010193835f91609e1982360301905b848410612577575050505050505090565b90919293949596605f19828203018752873583811215610213578401908135601e198336030181121561021357820191602083359301906001600160401b038411610213578360061b3603821361021357612640836080612635816125eb6020989760019a60a08b9a5260a08701916124c4565b956126096125fa898301612526565b6001600160a01b0316868a0152565b6040810135604086015261262f61262260608301610888565b63ffffffff166060870152565b01610888565b63ffffffff16910152565b99019701959401929190612566565b91908203918211610c1257565b8051600181111561270e5760011c91612674836119d3565b9161268861268385835161264f565b6119d3565b915f5b8581106126eb5750845b82518110156126ca57806126c46126b161057860019487611acc565b6104cc6126be8a8561264f565b88611acc565b01612695565b50935050906126d89061265c565b6126e2909161265c565b6102b791612712565b806127086126fe61057860019487611acc565b6104cc8389611acc565b0161268b565b5090565b9182519282516127256126838287611d94565b935f935f925f975b80871080612897575b15612818576127486105788888611acc565b6127586105856105788888611acc565b6001600160a01b03909116101561279a5761279561278261057861277b8a611ae5565b9989611acc565b6104cc61278e8c611ae5565b9b8b611acc565b61272d565b6127a76105788888611acc565b6127b76105856105788888611acc565b6001600160a01b0390911611156127e1576127956127826105786127da88611ae5565b9787611acc565b93612812906120426127ff6105786127f88b611ae5565b9a8a611acc565b6104cc61280b8d611ae5565b9c8c611acc565b9361272d565b9795919794909293945b80831061286d575050505b80831061283c57505050815290565b61286861285561057861284e86611ae5565b9585611acc565b6104cc61286187611ae5565b9688611acc565b61282d565b61289261287f61057861284e86611ae5565b6104cc61288b8a611ae5565b998b611acc565b612822565b50818510612736565b906128aa82610899565b6128b76040519182610856565b82815280926119fb601f1991610899565b5f81805b61294257506128de9061ffff166128a0565b5f5f5b8251821080612937575b15612930576001811b8416612909575b61290490611ae5565b6128e1565b9060016129049160ff60f81b8460f81b165f1a6129268287611b6a565b53019190506128fb565b5050905090565b5061010081106128eb565b5f198101818111610c125761ffff9116911661ffff8114610c125760010190806128cc565b6033546001600160a01b0316330361297b57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b606654604080516001600160a01b038084168252841660208201529192917fe0d49a54274423183dadecbdf239eaac6e06ba88320b26fe8cc5ec9d050a63959190a16001600160a01b03166001600160a01b03199190911617606655565b15612a2457565b60405162461bcd60e51b815260206004820152602660248201527f536572766963654d616e616765723a204f70657261746f72206e6f7420696e2060448201526571756f72756d60d01b6064820152608490fd5b6040516309aa152760e11b81526001600160a01b0391821660048201527f00000000000000000000000000000000000000000000000000000000000000009091169291905f90602081602481885afa801561052457612afc956020925f92612b85575b50604051808098819463871ef04960e01b8352600483019190602083019252565b03915afa938415610524575f94612b64575b505f5b8251811015612b5e5780612b58612b53612b3d610744612b3360019689611acc565b5163ffffffff1690565b848060c01b03891660ff600192161c1660011490565b612a1d565b01612b11565b50925050565b612b7e91945060203d6020116107a95761079b8183610856565b925f612b0e565b612b9d919250833d85116107d6576107c88183610856565b905f612adb56fea2646970667358221220d87d354b1fa836642ce2239509574e5455550289597d3d12046800ddfc1fedc264736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01@4a\x02$W`\x1Fa.\x178\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x02(W\x80\x84\x92`\xC0\x94`@R\x839\x81\x01\x03\x12a\x02$W\x80Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02$W` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02$W`@\x82\x01Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02$W``\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x02$W`\x80\x84\x01Q\x93`\x01`\x01`\xA0\x1B\x03\x85\x16\x85\x03a\x02$W`\xA0\x01Q\x94`\x01`\x01`\xA0\x1B\x03\x86\x16\x86\x03a\x02$W`\x80R`\xA0R`\xC0R`\xE0Ra\x01\0R_T`\xFF\x81`\x08\x1C\x16a\x01\xCFW`\xFF\x80\x82\x16\x10a\x01\x95W[Pa\x01 R`@Qa+\xDA\x90\x81a\x02=\x829`\x80Q\x81\x81\x81a\r\x08\x01R\x81\x81a\x0F\xCF\x01R\x81\x81a\x10\xB8\x01Ra\x11h\x01R`\xA0Q\x81a\x17\x04\x01R`\xC0Q\x81\x81\x81a\x03Q\x01R\x81\x81a\r\xE8\x01R\x81\x81a\x0F\x9F\x01R\x81\x81a\x10\x88\x01R\x81\x81a\x1Df\x01R\x81\x81a\x1E\"\x01R\x81\x81a\"b\x01Ra*\x97\x01R`\xE0Q\x81\x81\x81a\x1E\xBD\x01Ra\"\xAE\x01Ra\x01\0Q\x81\x81\x81a\r\xA4\x01R\x81\x81a\x12\x9A\x01R\x81\x81a\x14x\x01Ra\x1B\x8F\x01Ra\x01 Q\x81a\x0EX\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\0\xE7V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x0B\x91\xD6e\x14a\x02\x04W\x80c\x0E\xA4>C\x14a\x01\xFFW\x80c\x15\xB7\xBC\x9A\x14a\x01\xFAW\x80c\x1E!\x99\xE2\x14a\x01\xF5W\x80c&\xF0\x17\xE2\x14a\x01\xF0W\x80c3\xCF\xB7\xB7\x14a\x01\xEBW\x80c;\xC2\x8C\x8C\x14a\x01\xE6W\x80cg\x94\x0C\x89\x14a\x01\xE1W\x80ck:\xA7.\x14a\x01\xDCW\x80cqP\x18\xA6\x14a\x01\xD7W\x80cr\x08\0u\x14a\x01\xD2W\x80cw\xEFs\x1D\x14a\x01\xCDW\x80c\x89\x99\x81\x7F\x14a\x01\xC8W\x80c\x8Dh4\x9A\x14a\x01\xC3W\x80c\x8D\xA5\xCB[\x14a\x01\xBEW\x80c\x99&\xEE}\x14a\x01\xB9W\x80c\xA3d\xF4\xDA\x14a\x01\xB4W\x80c\xA9\x8F\xB3U\x14a\x01\xAFW\x80c\xAF\xE0.\xD5\x14a\x01\xAAW\x80c\xB14Bq\x14a\x01\xA5W\x80c\xB7\x8B`\x87\x14a\x01\xA0W\x80c\xC1\xA8\xE2\xC5\x14a\x01\x9BW\x80c\xCA\x8A\xA7\xC7\x14a\x01\x96W\x80c\xD9\xF9Sw\x14a\x01\x91W\x80c\xE4o\x18\x16\x14a\x01\x8CW\x80c\xE4\x81\xAF\x9D\x14a\x01\x87W\x80c\xE6\x1B\xBE\xE8\x14a\x01\x82W\x80c\xF2\xFD\xE3\x8B\x14a\x01}W\x80c\xFC)\x9D\xEE\x14a\x01xW\x80c\xFC\xD1\xC3u\x14a\x01sWc\xFC\xE3l}\x14a\x01nW_\x80\xFD[a\x16\xDFV[a\x16\xC2V[a\x16\x9AV[a\x15\xCAV[a\x14gV[a\x14\x1EV[a\x13\xF6V[a\x130V[a\x12\x85V[a\x12@V[a\x12\x0CV[a\x11\xE4V[a\x11\xD6V[a\x11-V[a\x10dV[a\x0F\\V[a\x0F4V[a\x0F\x12V[a\x0E\x87V[a\x0ECV[a\r\x92V[a\r7V[a\x0C\xF3V[a\x0C\xD6V[a\x0CSV[a\x0C\x17V[a\x0BaV[a\n\xFCV[a\nDV[a\t\x05V[a\x032V[_\x91\x03\x12a\x02\x13WV[_\x80\xFD[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x024WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x02'V[``\x80\x82R\x82Q\x90\x82\x01\x81\x90R`\x80\x82\x01\x95\x94\x92` \x01\x90_[\x81\x81\x10a\x03\x16WPPP\x80\x85\x03` \x82\x01R\x82Q\x80\x86R` \x86\x01\x90` \x80\x82`\x05\x1B\x89\x01\x01\x95\x01\x91_\x90[\x82\x82\x10a\x02\xBAWPPPPa\x02\xB7\x93\x94P`@\x81\x84\x03\x91\x01Ra\x02\x17V[\x90V[\x90\x91\x92\x95`\x1F\x19\x89\x82\x03\x01\x82R\x86Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x02\xF8WPPP` \x80`\x01\x92\x98\x01\x92\x01\x92\x01\x90\x92\x91a\x02\x99V[\x90\x91\x92` \x80`\x01\x92c\xFF\xFF\xFF\xFF\x87Q\x16\x81R\x01\x94\x01\x92\x01\x90a\x02\xDAV[\x82Qc\xFF\xFF\xFF\xFF\x16\x88R` \x97\x88\x01\x97\x90\x92\x01\x91`\x01\x01a\x02mV[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x05$W`\xFF\x91_\x91a\x07\xF3W[P\x16\x90a\x03\x9Ea\x19\xB8V[a\x03\xA7\x83a\x19\xD3V[\x91Cc\xFF\xFF\xFF\xFF\x16\x90_[`\xFF\x81\x16\x94\x86\x86\x10\x15a\x06LW`@QcOL\x91\xE1`\xE1\x1B\x81R\x94` \x86`\x04\x81\x87Z\xFA\x95\x86\x15a\x05$W_\x96a\x06,W[P`@Qc\x89\x02bE`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x86\x16`$\x82\x01R\x95_\x90\x87\x90`D\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x95\x86\x15a\x05$W_\x96a\x06\x08W[Pa\x045\x86Qa\x19\xD3V[\x97_\x91[\x87Q\x83\x10\x15a\x05FW`@Qc.\xFA,\xA3`\xE1\x1B\x81R` \x81`\x04\x81\x8AZ\xFA\x90\x81\x15a\x05$Wa\x04\x9A\x91` \x91_\x91a\x05)W[Pa\x04x\x86\x8Ca\x1A\xCCV[Q\x90`@Q\x80\x80\x95\x81\x94c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x05$Wa\x04\xDB\x8C`\x01\x94\x87a\x04\xEA\x95_\x92a\x04\xF2W[Pa\x04\xCC\x91\x92a\x1A\xCCV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a\x04\xE4\x8Ca&\\V[\x90a'\x12V[\x92\x01\x91a\x049V[a\x04\xCC\x92Pa\x05\x17\x90` =\x81\x11a\x05\x1DW[a\x05\x0F\x81\x83a\x08VV[\x81\x01\x90a\x1A*V[\x91a\x04\xC1V[P=a\x05\x05V[a\x19\xADV[a\x05@\x91P\x82=\x81\x11a\x05\x1DWa\x05\x0F\x81\x83a\x08VV[_a\x04mV[\x96P\x97PP\x94\x90\x91a\x05X\x85Qa\x19\xD3V[\x96_\x94_[\x87Q\x81\x10\x15a\x05\xD1W\x89\x88a\x05\x91a\x05\x85a\x05x\x85\x84a\x1A\xCCV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x05\xA0W[PP`\x01\x01a\x05]V[\x82\x91\x98a\x04\xCCa\x05\xB8a\x05x`\x01\x96a\x05\xC8\x95a\x1A\xCCV[\x91a\x05\xC2\x81a\x1A\xE5V[\x9Ba\x1A\xCCV[\x90P\x89\x88a\x05\x96V[P\x94\x88R\x95\x96\x93\x94P\x91a\x05\xFD\x91a\x05\xF8\x90a\x05\xED\x81\x8Aa\x1A\xCCV[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[a\x1A\x19V[\x94\x93\x94\x92\x91\x92a\x03\xB2V[a\x06%\x91\x96P=\x80_\x83>a\x06\x1D\x81\x83a\x08VV[\x81\x01\x90a\x1A?V[\x94_a\x04*V[a\x06E\x91\x96P` =\x81\x11a\x05\x1DWa\x05\x0F\x81\x83a\x08VV[\x94_a\x03\xE4V[\x84\x92\x91Pa\x06Z\x83Qa\x1A\xF3V[\x91_[\x84Q\x81\x10\x15a\x07\xDDWa\x06\xA1` a\x06xa\x05x\x84\x89a\x1A\xCCV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x86Z\xFA\x90\x81\x15a\x05$Wa\x06\xDB\x91` \x91_\x91a\x07\xB0W[P`@Q\x80\x93\x81\x92c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x80\x15a\x05$Wa\x07\0\x91_\x91a\x07\x82W[P`\x01`\x01`\xC0\x1B\x03\x16a(\xC8V[\x91a\x07\x0B\x83Qa\x19\xD3V[\x93_[\x84Q\x81\x10\x15a\x07ZW\x80a\x07Ta\x07Ja\x07Da\x07>a\x070`\x01\x96\x8Ba\x1BjV[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`\xFF\x16\x90V[a\x05\xED\x83\x8Aa\x1A\xCCV[\x01a\x07\x0EV[P\x93`\x01\x92\x91\x96\x93Pa\x07m\x82\x87a\x1A\xCCV[Ra\x07x\x81\x86a\x1A\xCCV[P\x01\x93\x90\x93a\x06]V[a\x07\xA3\x91P` =\x81\x11a\x07\xA9W[a\x07\x9B\x81\x83a\x08VV[\x81\x01\x90a\x1BKV[\x87a\x06\xF1V[P=a\x07\x91V[a\x07\xD0\x91P\x82=\x81\x11a\x07\xD6W[a\x07\xC8\x81\x83a\x08VV[\x81\x01\x90a\x1B<V[\x88a\x06\xBBV[P=a\x07\xBEV[PPa\x07\xEF\x83`@Q\x93\x84\x93\x84a\x02SV[\x03\x90\xF3[a\x08\x15\x91P` =` \x11a\x08\x1BW[a\x08\r\x81\x83a\x08VV[\x81\x01\x90a\x19\x94V[_a\x03\x93V[P=a\x08\x03V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08QW`@RV[a\x08\"V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08QW`@RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02\x13WV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02\x13WV[`\x01`\x01`@\x1B\x03\x81\x11a\x08QW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x08\xC0\x82a\x08\x99V[\x91a\x08\xCE`@Q\x93\x84a\x08VV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x02\x13W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x02\x13W\x81` a\x02\xB7\x935\x91\x01a\x08\xB4V[4a\x02\x13W` 6`\x03\x19\x01\x12a\x02\x13W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13W`\x80`\x03\x19\x826\x03\x01\x12a\x02\x13W`@Qa\tA\x81a\x086V[\x81`\x04\x015a\tO\x81a\x08wV[\x81Ra\t]`$\x83\x01a\x08\x88V[` \x82\x01R`D\x82\x015`@\x82\x01R`d\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13Wa\t\x9C\x92`\x04a\t\x92\x926\x92\x01\x01a\x08\xEAV[``\x82\x01Ra\x1B{V[\0[`\x01`\x01`@\x1B\x03\x81\x11a\x08QW`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x02\x13W\x815a\t\xCC\x81a\t\x9EV[\x92a\t\xDA`@Q\x94\x85a\x08VV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\x13W` \x01\x90[\x82\x82\x10a\n\x02WPPP\x90V[` \x80\x91a\n\x0F\x84a\x08\x88V[\x81R\x01\x91\x01\x90a\t\xF5V[` `\x03\x19\x82\x01\x12a\x02\x13W`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x13Wa\x02\xB7\x91`\x04\x01a\t\xB5V[4a\x02\x13Wa\nR6a\n\x1AV[Pa\t\x9Ca)gV[\x91\x81`\x1F\x84\x01\x12\x15a\x02\x13W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\x13W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x02\x13WV[\x91\x90\x91``\x81\x84\x03\x12a\x02\x13W`@Q\x90``\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08QW`@R\x81\x93\x815`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13W\x82\x01\x91\x81`\x1F\x84\x01\x12\x15a\x02\x13Wa\n\xE9`@\x93\x92\x83` \x86\x955\x91\x01a\x08\xB4V[\x84R` \x81\x015` \x85\x01R\x015\x91\x01RV[4a\x02\x13W``6`\x03\x19\x01\x12a\x02\x13Wa\x0B\x18`\x045a\x08wV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13Wa\x0B7\x906\x90`\x04\x01a\n[V[PP`D5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13Wa\x0BX\x906\x90`\x04\x01a\n\x8BV[Pa\t\x9Ca\x1D`V[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13Wa\x0Bya)gV[`hTb\t:\x80\x81\x01\x80\x91\x11a\x0C\x12WB\x10a\x0B\xB6W`gTa\x0B\xA4\x90`\x01`\x01`\xA0\x1B\x03\x16a)\xBFV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FServiceManager: Slasher proposal`D\x82\x01Rm\x08\x19\x19[\x18^H\x1B\x9B\xDD\x08\x1BY]`\x92\x1B`d\x82\x01R`\x84\x90\xFD[a\x1A\x05V[4a\x02\x13W` 6`\x03\x19\x01\x12a\x02\x13Wa\x07\xEFa\x0C?`\x045a\x0C:\x81a\x08wV[a\x1E\x03V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x02\x17V[4a\x02\x13W` 6`\x03\x19\x01\x12a\x02\x13W`\x045a\x0Cp\x81a\x08wV[a\x0Cxa)gV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`eU\0[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W` `@Qb\t:\x80\x81R\xF3[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13Wa\rOa)gV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\x13W`@Qc4\xF6[\xFD`\xE2\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90_\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x05$Wa\x0E/W\0[\x80a\x0E=_a\t\x9C\x93a\x08VV[\x80a\x02\tV[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x13W` 6`\x03\x19\x01\x12a\x02\x13W\x7F/\x8A\xFC\x8Ax\xFD\x95\x8F3\x01\xC0#:\xA3&\xB9\xC4\xB9\xA2\x88Jt\x83\"}k\x05U\xAA\xA0:\xDBa\x0F\r`\x045a\x0E\xC8\x81a\x08wV[a\x0E\xD0a)gV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90UB`h\x81\x90U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1\0[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W` `\xFF`iT\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x13W`@6`\x03\x19\x01\x12a\x02\x13W`\x045a\x0Fy\x81a\x08wV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13Wa\x0F\x98\x906\x90`\x04\x01a\n\x8BV[\x90a\x0F\xCD3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1C\xD3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x02\x13W_\x92\x83\x92`@Q\x94\x85\x80\x94\x81\x93c\x99&\xEE}`\xE0\x1B\x83R`\x01\x80`\xA0\x1B\x03\x16`\x04\x83\x01R`@`$\x83\x01R`@a\x10B\x82Q```D\x86\x01R`\xA4\x85\x01\x90a\x1C\xAFV[\x91` \x81\x01Q`d\x85\x01R\x01Q`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\x05$Wa\x0E/W\0[4a\x02\x13W_` 6`\x03\x19\x01\x12a\x02\x13W`\x045a\x10\x82\x81a\x08wV[a\x10\xB63\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1C\xD3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x02\x13W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x05$Wa\x11!WP\x80\xF3[a\t\x9C\x91P_\x90a\x08VV[4a\x02\x13W_` 6`\x03\x19\x01\x12a\x02\x13W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13Wa\x11^\x906\x90`\x04\x01a\x08\xEAV[a\x11fa)gV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\x13W`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R` `\x04\x82\x01R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x11\xC4\x90`$\x83\x01\x90a\x1C\xAFV[\x03\x92Z\xF1\x80\x15a\x05$Wa\x11!WP\x80\xF3[4a\x02\x13Wa\x0BX6a\n\x1AV[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W`fT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13Wa\x12$a)gV[`\x01`iTa\x126`\xFF\x82\x16\x15a!<V[`\xFF\x19\x16\x17`iU\0[4a\x02\x13W`@6`\x03\x19\x01\x12a\x02\x13Wa\x12\\`\x045a\x08wV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13Wa\x12{\x906\x90`\x04\x01a\n[V[PPa\t\x9Ca\x1D`V[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90\x80`\x1F\x83\x01\x12\x15a\x02\x13W\x815a\x12\xE0\x81a\t\x9EV[\x92a\x12\xEE`@Q\x94\x85a\x08VV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\x13W` \x01\x90[\x82\x82\x10a\x13\x16WPPP\x90V[` \x80\x91\x835a\x13%\x81a\x08wV[\x81R\x01\x91\x01\x90a\x13\tV[4a\x02\x13W`@6`\x03\x19\x01\x12a\x02\x13W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13W6`#\x82\x01\x12\x15a\x02\x13W\x80`\x04\x015a\x13k\x81a\t\x9EV[\x91a\x13y`@Q\x93\x84a\x08VV[\x81\x83R`$` \x84\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x02\x13W`$\x81\x01\x92[\x82\x84\x10a\x13\xC7W`$5\x85`\x01`\x01`@\x1B\x03\x82\x11a\x02\x13Wa\x13\xC1a\t\x9C\x926\x90`\x04\x01a\x12\xC9V[\x90a!\x9CV[\x835`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13W` \x91a\x13\xEB\x83\x92`$6\x91\x87\x01\x01a\t\xB5V[\x81R\x01\x93\x01\x92a\x13\x97V[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W`gT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13Wa\x07\xEFa\x0C?a\"SV[` `\x03\x19\x82\x01\x12a\x02\x13W`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x13Wa\x14c\x91`\x04\x01a\n[V[\x90\x91V[4a\x02\x13Wa\x14u6a\x149V[\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x02\x13W\x91`@Q\x92\x83\x91c\x010\xFC'`\xE5\x1B\x83R\x80`D\x84\x010`\x04\x86\x01R`@`$\x86\x01RR`d\x83\x01\x90`d\x81`\x05\x1B\x85\x01\x01\x92\x80\x92_\x91`>\x19\x816\x03\x01[\x84\x84\x10a\x15\x05W\x87_\x81\x80\x8A\x03\x81\x83\x8EZ\xF1\x80\x15a\x05$Wa\x0E/W\0[\x91\x93\x95\x90\x92\x94\x96P`c\x19\x89\x82\x03\x01\x84R\x865\x82\x81\x12\x15a\x02\x13W\x83\x01`@\x82\x01\x90c\xFF\xFF\xFF\xFFa\x155\x82a\x08\x88V[\x16\x83R` \x81\x015`\x1E\x19\x826\x03\x01\x81\x12\x15a\x02\x13W\x01\x90` \x825\x92\x01\x92`\x01`\x01`@\x1B\x03\x83\x11a\x02\x13W\x82`\x05\x1B6\x03\x84\x13a\x02\x13W\x82``\x92`@` \x84\x01RR\x01\x91\x90_\x90[\x80\x82\x10a\x15\xA2WPPP` \x80`\x01\x92\x98\x01\x94\x01\x94\x01\x91\x88\x96\x95\x93\x94\x91a\x14\xE7V[\x90\x91\x92` \x80`\x01\x92\x865a\x15\xB6\x81a\x08wV[\x84\x80`\xA0\x1B\x03\x16\x81R\x01\x94\x01\x92\x01\x90a\x15\x80V[4a\x02\x13W` 6`\x03\x19\x01\x12a\x02\x13W`\x045a\x15\xE7\x81a\x08wV[a\x15\xEFa)gV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x16FW`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W` `hT`@Q\x90\x81R\xF3[4a\x02\x13Wa\x16\xED6a\x149V[`eT\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x19\x14W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91_[\x81\x81\x10a\x17rWP\x82;\x15a\x02\x13Wa\x17b\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c!\xF5\";`\xE1\x1B\x84R0`\x04\x85\x01a%1V[\x03\x92Z\xF1\x80\x15a\x05$Wa\x0E/W\0[_` a\x17\xCDa\x17\x90a\x05\x85\x83a\x17\x8A\x87\x89\x8Ba$\x80V[\x01a$\xA2V[`@a\x17\x9D\x86\x88\x8Aa$\x80V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x91\x015`D\x82\x01R\x93\x84\x92\x83\x91\x90\x82\x90`d\x82\x01\x90V[\x03\x92Z\xF1\x80\x15a\x05$Wa\x18\xF8W[Pa\x17\xF0a\x05\x85` a\x17\x8A\x84\x86\x88a$\x80V[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x86\x16`$\x82\x01R\x91\x90` \x90\x83\x90`D\x90\x82\x90Z\xFA\x80\x15a\x05$Wa\x18\x93` \x91_\x94\x85\x91a\x18\xDBW[Pa\x18`a\x18Ka\x05\x85\x85a\x17\x8A\x88\x8B\x8Da$\x80V[\x91`@a\x18Y\x87\x8A\x8Ca$\x80V[\x015a\x1D\x94V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x94\x85\x92\x83\x91\x90\x82\x90`D\x82\x01\x90V[\x03\x92Z\xF1\x91\x82\x15a\x05$W`\x01\x92a\x18\xADW[P\x01a\x17/V[a\x18\xCD\x90` =\x81\x11a\x18\xD4W[a\x18\xC5\x81\x83a\x08VV[\x81\x01\x90a$\xACV[P_a\x18\xA6V[P=a\x18\xBBV[a\x18\xF2\x91P\x83=\x81\x11a\x07\xD6Wa\x07\xC8\x81\x83a\x08VV[_a\x185V[a\x19\x0F\x90` =\x81\x11a\x18\xD4Wa\x18\xC5\x81\x83a\x08VV[a\x17\xDCV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FServiceManagerBase.onlyRewardsIn`D\x82\x01R\x7Fitiator: caller is not the rewar`d\x82\x01Rk29\x904\xB74\xBA4\xB0\xBA7\xB9`\xA1\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x90\x81` \x91\x03\x12a\x02\x13WQ`\xFF\x81\x16\x81\x03a\x02\x13W\x90V[`@Q=_\x82>=\x90\xFD[`@Q\x90a\x19\xC7` \x83a\x08VV[_\x80\x83R6` \x84\x017V[\x90a\x19\xDD\x82a\t\x9EV[a\x19\xEA`@Q\x91\x82a\x08VV[\x82\x81R\x80\x92a\x19\xFB`\x1F\x19\x91a\t\x9EV[\x01\x90` 6\x91\x017V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x16`\xFF\x81\x14a\x0C\x12W`\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x02\x13WQa\x02\xB7\x81a\x08wV[` \x81\x83\x03\x12a\x02\x13W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x13W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x02\x13W\x81Qa\x1Ar\x81a\t\x9EV[\x92a\x1A\x80`@Q\x94\x85a\x08VV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\x13W` \x01\x90[\x82\x82\x10a\x1A\xA8WPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a\x1A\x9BV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q\x82\x10\x15a\x1A\xE0W` \x91`\x05\x1B\x01\x01\x90V[a\x1A\xB8V[_\x19\x81\x14a\x0C\x12W`\x01\x01\x90V[\x90a\x1A\xFD\x82a\t\x9EV[a\x1B\n`@Q\x91\x82a\x08VV[\x82\x81R\x80\x92a\x1B\x1B`\x1F\x19\x91a\t\x9EV[\x01\x90_[\x82\x81\x10a\x1B+WPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a\x1B\x1FV[\x90\x81` \x91\x03\x12a\x02\x13WQ\x90V[\x90\x81` \x91\x03\x12a\x02\x13WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x02\x13W\x90V[\x90\x81Q\x81\x10\x15a\x1A\xE0W\x01` \x01\x90V[`fT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1CDW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\x13W``_\x91a\x1C#\x93\x83`@Q\x80\x96\x81\x95\x82\x94c\r\xCF\xB3\xF5`\xE3\x1B\x84R0`\x04\x85\x01R`@`$\x85\x01R`\x01\x80`\xA0\x1B\x03\x81Q\x16`D\x85\x01Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16`d\x85\x01R`@\x81\x01Q`\x84\x85\x01R\x01Q`\x80`\xA4\x84\x01R`\xC4\x83\x01\x90a\x1C\xAFV[\x03\x92Z\xF1\x80\x15a\x05$Wa\x1C4WPV[\x80a\x0E=_a\x1CB\x93a\x08VV[V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FServiceManagerBase.onlySlasher: `D\x82\x01R\x7Fcaller is not the slasher\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x15a\x1C\xDAWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FServiceManagerBase.onlyRegistryC`D\x82\x01R\x7Foordinator: caller is not the re`d\x82\x01Rq3\xB4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B`\x84\x82\x01R`\xA4\x90\xFD[a\x1CB3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1C\xD3V[\x91\x90\x82\x01\x80\x92\x11a\x0C\x12WV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x02\x13WV[\x90\x81`@\x91\x03\x12a\x02\x13W`@Q\x90`@\x82\x01\x90\x82\x82\x10`\x01`\x01`@\x1B\x03\x83\x11\x17a\x08QW` \x91`@R\x80Qa\x1D\xEE\x81a\x08wV[\x83R\x01Qa\x1D\xFB\x81a\x1D\xA1V[` \x82\x01R\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x05$Wa\x1E\x82\x91` \x91_\x91a!\x1FWP`@Q\x80\x93\x81\x92c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x85Z\xFA\x90\x81\x15a\x05$W_\x91a!\0W[P`\x01`\x01`\xC0\x1B\x03\x16\x90\x81\x15\x90\x81\x15a \xADW[Pa \xA4Wa\x1E\xB8\x90a(\xC8V[_\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x83[\x81Q\x85\x10\x15a\x1FpWa\x1F)` a\x1F\x06a\x07>a\x070\x89\x87a\x1BjV[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x87Z\xFA\x80\x15a\x05$W`\x01\x92a\x1FH\x92_\x92a\x1FPW[Pa\x1D\x94V[\x94\x01\x93a\x1E\xE8V[a\x1Fi\x91\x92P` =\x81\x11a\x07\xD6Wa\x07\xC8\x81\x83a\x08VV[\x90_a\x1FBV[a\x1F{\x91\x94Pa\x19\xD3V[\x92_\x90_[\x81Q\x81\x10\x15a \x9EWa\x1F\x99a\x07>a\x070\x83\x85a\x1BjV[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x90` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x05$W_\x92a ~W[P\x90_\x91[\x81\x83\x10a\x1F\xDEWPPP`\x01\x01a\x1F\x80V[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x05$Wa B\x8Ba\x04\xCC\x83a <a\x05\x85`\x01\x98a G\x98_\x91a PW[PQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92a\x1A\xCCV[a\x1A\xE5V[\x95\x01\x91\x90a\x1F\xCCV[a q\x91P`@=\x81\x11a wW[a i\x81\x83a\x08VV[\x81\x01\x90a\x1D\xB7V[_a .V[P=a _V[a \x97\x91\x92P` =\x81\x11a\x07\xD6Wa\x07\xC8\x81\x83a\x08VV[\x90_a\x1F\xC7V[PPPPV[Pa\x02\xB7a\x19\xB8V[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x91P` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x80\x15a\x05$W`\xFF\x91_\x91a \xE1W[P\x16\x15_a\x1E\xAAV[a \xFA\x91P` =` \x11a\x08\x1BWa\x08\r\x81\x83a\x08VV[_a \xD8V[a!\x19\x91P` =` \x11a\x07\xA9Wa\x07\x9B\x81\x83a\x08VV[_a\x1E\x95V[a!6\x91P\x82=\x84\x11a\x07\xD6Wa\x07\xC8\x81\x83a\x08VV[_a\x06\xBBV[\x15a!CWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FServiceManager: Migration Alread`D\x82\x01Rj\x1EH\x11\x9A[\x98[\x1A^\x99Y`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90a!\xA6a)gV[a!\xB5`\xFF`iT\x16\x15a!<V[\x80Q\x83Q\x03a!\xFAW_[\x81Q\x81\x10\x15a!\xF4W\x80a!\xEDa!\xDCa\x05x`\x01\x94\x86a\x1A\xCCV[a!\xE6\x83\x88a\x1A\xCCV[Q\x90a*xV[P\x01a!\xC0V[PP\x90PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FServiceManager: Input array leng`D\x82\x01Rj\x0E\x8D\x04\r\xAD.m\xAC.\x8Cm`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x05$W`\xFF\x91_\x91a$aW[P\x16\x80\x15a$WW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90_\x90\x81\x90[\x80\x83\x10a$\x13WPa\"\xEE\x91Pa\x19\xD3V[\x92_\x90_[`@Qc\x9A\xA1e=`\xE0\x1B\x81R` \x81`\x04\x81\x89Z\xFA\x80\x15a\x05$W`\xFF\x91_\x91a#\xF5W[P\x16\x81\x10\x15a#\xEEW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01\x81\x90R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x05$W_\x92a#\xCEW[P\x90_\x91[\x81\x83\x10a#hWPPP`\x01\x01a\"\xF3V[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x05$Wa B\x8Ba\x04\xCC\x83a <a\x05\x85`\x01\x98a#\xC5\x98_\x91a PWPQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95\x01\x91\x90a#VV[a#\xE7\x91\x92P` =\x81\x11a\x07\xD6Wa\x07\xC8\x81\x83a\x08VV[\x90_a#QV[P\x92PPPV[a$\r\x91P` =\x81\x11a\x08\x1BWa\x08\r\x81\x83a\x08VV[_a#\x19V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x90\x91\x90` \x81`$\x81\x88Z\xFA\x80\x15a\x05$W`\x01\x92a$N\x92_\x92a\x1FPWPa\x1D\x94V[\x92\x01\x91\x90a\"\xDCV[PPa\x02\xB7a\x19\xB8V[a$z\x91P` =` \x11a\x08\x1BWa\x08\r\x81\x83a\x08VV[_a\"\xA4V[\x91\x90\x81\x10\x15a\x1A\xE0W`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x02\x13W\x01\x90V[5a\x02\xB7\x81a\x08wV[\x90\x81` \x91\x03\x12a\x02\x13WQ\x80\x15\x15\x81\x03a\x02\x13W\x90V[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a$\xDDWPPP\x90V[\x90\x91\x92`@\x80`\x01\x92\x865a$\xF1\x81a\x08wV[\x84\x80`\xA0\x1B\x03\x16\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x88\x015a%\x15\x81a\x1D\xA1V[\x16` \x82\x01R\x01\x94\x01\x92\x91\x01a$\xD0V[5\x90a\x1CB\x82a\x08wV[\x92\x80\x91`@\x85\x01\x90`\x01\x80`\xA0\x1B\x03\x16\x85R`@` \x86\x01RR``\x83\x01\x90``\x81`\x05\x1B\x85\x01\x01\x93\x83_\x91`\x9E\x19\x826\x03\x01\x90[\x84\x84\x10a%wWPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`_\x19\x82\x82\x03\x01\x87R\x875\x83\x81\x12\x15a\x02\x13W\x84\x01\x90\x815`\x1E\x19\x836\x03\x01\x81\x12\x15a\x02\x13W\x82\x01\x91` \x835\x93\x01\x90`\x01`\x01`@\x1B\x03\x84\x11a\x02\x13W\x83`\x06\x1B6\x03\x82\x13a\x02\x13Wa&@\x83`\x80a&5\x81a%\xEB` \x98\x97`\x01\x9A`\xA0\x8B\x9AR`\xA0\x87\x01\x91a$\xC4V[\x95a&\ta%\xFA\x89\x83\x01a%&V[`\x01`\x01`\xA0\x1B\x03\x16\x86\x8A\x01RV[`@\x81\x015`@\x86\x01Ra&/a&\"``\x83\x01a\x08\x88V[c\xFF\xFF\xFF\xFF\x16``\x87\x01RV[\x01a\x08\x88V[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x99\x01\x97\x01\x95\x94\x01\x92\x91\x90a%fV[\x91\x90\x82\x03\x91\x82\x11a\x0C\x12WV[\x80Q`\x01\x81\x11\x15a'\x0EW`\x01\x1C\x91a&t\x83a\x19\xD3V[\x91a&\x88a&\x83\x85\x83Qa&OV[a\x19\xD3V[\x91_[\x85\x81\x10a&\xEBWP\x84[\x82Q\x81\x10\x15a&\xCAW\x80a&\xC4a&\xB1a\x05x`\x01\x94\x87a\x1A\xCCV[a\x04\xCCa&\xBE\x8A\x85a&OV[\x88a\x1A\xCCV[\x01a&\x95V[P\x93PP\x90a&\xD8\x90a&\\V[a&\xE2\x90\x91a&\\V[a\x02\xB7\x91a'\x12V[\x80a'\x08a&\xFEa\x05x`\x01\x94\x87a\x1A\xCCV[a\x04\xCC\x83\x89a\x1A\xCCV[\x01a&\x8BV[P\x90V[\x91\x82Q\x92\x82Qa'%a&\x83\x82\x87a\x1D\x94V[\x93_\x93_\x92_\x97[\x80\x87\x10\x80a(\x97W[\x15a(\x18Wa'Ha\x05x\x88\x88a\x1A\xCCV[a'Xa\x05\x85a\x05x\x88\x88a\x1A\xCCV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x10\x15a'\x9AWa'\x95a'\x82a\x05xa'{\x8Aa\x1A\xE5V[\x99\x89a\x1A\xCCV[a\x04\xCCa'\x8E\x8Ca\x1A\xE5V[\x9B\x8Ba\x1A\xCCV[a'-V[a'\xA7a\x05x\x88\x88a\x1A\xCCV[a'\xB7a\x05\x85a\x05x\x88\x88a\x1A\xCCV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x11\x15a'\xE1Wa'\x95a'\x82a\x05xa'\xDA\x88a\x1A\xE5V[\x97\x87a\x1A\xCCV[\x93a(\x12\x90a Ba'\xFFa\x05xa'\xF8\x8Ba\x1A\xE5V[\x9A\x8Aa\x1A\xCCV[a\x04\xCCa(\x0B\x8Da\x1A\xE5V[\x9C\x8Ca\x1A\xCCV[\x93a'-V[\x97\x95\x91\x97\x94\x90\x92\x93\x94[\x80\x83\x10a(mWPPP[\x80\x83\x10a(<WPPP\x81R\x90V[a(ha(Ua\x05xa(N\x86a\x1A\xE5V[\x95\x85a\x1A\xCCV[a\x04\xCCa(a\x87a\x1A\xE5V[\x96\x88a\x1A\xCCV[a(-V[a(\x92a(\x7Fa\x05xa(N\x86a\x1A\xE5V[a\x04\xCCa(\x8B\x8Aa\x1A\xE5V[\x99\x8Ba\x1A\xCCV[a(\"V[P\x81\x85\x10a'6V[\x90a(\xAA\x82a\x08\x99V[a(\xB7`@Q\x91\x82a\x08VV[\x82\x81R\x80\x92a\x19\xFB`\x1F\x19\x91a\x08\x99V[_\x81\x80[a)BWPa(\xDE\x90a\xFF\xFF\x16a(\xA0V[__[\x82Q\x82\x10\x80a)7W[\x15a)0W`\x01\x81\x1B\x84\x16a)\tW[a)\x04\x90a\x1A\xE5V[a(\xE1V[\x90`\x01a)\x04\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa)&\x82\x87a\x1BjV[S\x01\x91\x90Pa(\xFBV[PP\x90P\x90V[Pa\x01\0\x81\x10a(\xEBV[_\x19\x81\x01\x81\x81\x11a\x0C\x12Wa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a\x0C\x12W`\x01\x01\x90\x80a(\xCCV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a){WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`fT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE0\xD4\x9AT'D#\x18=\xAD\xEC\xBD\xF29\xEA\xACn\x06\xBA\x882\x0B&\xFE\x8C\xC5\xEC\x9D\x05\nc\x95\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`fUV[\x15a*$WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FServiceManager: Operator not in `D\x82\x01Requorum`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x92\x91\x90_\x90` \x81`$\x81\x88Z\xFA\x80\x15a\x05$Wa*\xFC\x95` \x92_\x92a+\x85W[P`@Q\x80\x80\x98\x81\x94c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91Z\xFA\x93\x84\x15a\x05$W_\x94a+dW[P_[\x82Q\x81\x10\x15a+^W\x80a+Xa+Sa+=a\x07Da+3`\x01\x96\x89a\x1A\xCCV[Qc\xFF\xFF\xFF\xFF\x16\x90V[\x84\x80`\xC0\x1B\x03\x89\x16`\xFF`\x01\x92\x16\x1C\x16`\x01\x14\x90V[a*\x1DV[\x01a+\x11V[P\x92PPV[a+~\x91\x94P` =` \x11a\x07\xA9Wa\x07\x9B\x81\x83a\x08VV[\x92_a+\x0EV[a+\x9D\x91\x92P\x83=\x85\x11a\x07\xD6Wa\x07\xC8\x81\x83a\x08VV[\x90_a*\xDBV\xFE\xA2dipfsX\"\x12 \xD8}5K\x1F\xA86d,\xE2#\x95\tWNTUU\x02\x89Y}=\x12\x04h\0\xDD\xFC\x1F\xED\xC2dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c80630b91d665146102045780630ea43e43146101ff57806315b7bc9a146101fa5780631e2199e2146101f557806326f017e2146101f057806333cfb7b7146101eb5780633bc28c8c146101e657806367940c89146101e15780636b3aa72e146101dc578063715018a6146101d757806372080075146101d257806377ef731d146101cd5780638999817f146101c85780638d68349a146101c35780638da5cb5b146101be5780639926ee7d146101b9578063a364f4da146101b4578063a98fb355146101af578063afe02ed5146101aa578063b1344271146101a5578063b78b6087146101a0578063c1a8e2c51461019b578063ca8aa7c714610196578063d9f9537714610191578063e46f18161461018c578063e481af9d14610187578063e61bbee814610182578063f2fde38b1461017d578063fc299dee14610178578063fcd1c375146101735763fce36c7d1461016e575f80fd5b6116df565b6116c2565b61169a565b6115ca565b611467565b61141e565b6113f6565b611330565b611285565b611240565b61120c565b6111e4565b6111d6565b61112d565b611064565b610f5c565b610f34565b610f12565b610e87565b610e43565b610d92565b610d37565b610cf3565b610cd6565b610c53565b610c17565b610b61565b610afc565b610a44565b610905565b610332565b5f91031261021357565b5f80fd5b90602080835192838152019201905f5b8181106102345750505090565b82516001600160a01b0316845260209384019390920191600101610227565b6060808252825190820181905260808201959492602001905f5b818110610316575050508085036020820152825180865260208601906020808260051b8901019501915f905b8282106102ba57505050506102b79394506040818403910152610217565b90565b90919295601f19898203018252865190602080835192838152019201905f905b8082106102f857505050602080600192980192019201909291610299565b90919260208060019263ffffffff87511681520194019201906102da565b825163ffffffff1688526020978801979092019160010161026d565b34610213575f36600319011261021357604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156105245760ff915f916107f3575b50169061039e6119b8565b6103a7836119d3565b914363ffffffff16905f5b60ff8116948686101561064c57604051634f4c91e160e11b815294602086600481875afa958615610524575f9661062c575b50604051638902624560e01b815260ff8416600482015263ffffffff86166024820152955f90879060449082906001600160a01b03165afa958615610524575f96610608575b5061043586516119d3565b975f915b875183101561054657604051632efa2ca360e11b81526020816004818a5afa9081156105245761049a916020915f91610529575b50610478868c611acc565b519060405180809581946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa918215610524576104db8c600194876104ea955f926104f2575b506104cc9192611acc565b6001600160a01b039091169052565b6104e48c61265c565b90612712565b920191610439565b6104cc92506105179060203d811161051d575b61050f8183610856565b810190611a2a565b916104c1565b503d610505565b6119ad565b6105409150823d811161051d5761050f8183610856565b5f61046d565b965097505094909161055885516119d3565b965f945f5b87518110156105d15789886105916105856105788584611acc565b516001600160a01b031690565b6001600160a01b031690565b6105a0575b505060010161055d565b8291986104cc6105b86105786001966105c895611acc565b916105c281611ae5565b9b611acc565b90508988610596565b509488529596939450916105fd916105f8906105ed818a611acc565b9063ffffffff169052565b611a19565b9493949291926103b2565b6106259196503d805f833e61061d8183610856565b810190611a3f565b945f61042a565b61064591965060203d811161051d5761050f8183610856565b945f6103e4565b8492915061065a8351611af3565b915f5b84518110156107dd576106a160206106786105788489611acc565b6040516309aa152760e11b81526001600160a01b03909116600482015291829081906024820190565b0381865afa908115610524576106db916020915f916107b0575b506040518093819263871ef04960e01b8352600483019190602083019252565b0381865afa801561052457610700915f91610782575b506001600160c01b03166128c8565b9161070b83516119d3565b935f5b845181101561075a578061075461074a61074461073e6107306001968b611b6a565b516001600160f81b03191690565b60f81c90565b60ff1690565b6105ed838a611acc565b0161070e565b50936001929196935061076d8287611acc565b526107788186611acc565b500193909361065d565b6107a3915060203d81116107a9575b61079b8183610856565b810190611b4b565b876106f1565b503d610791565b6107d09150823d81116107d6575b6107c88183610856565b810190611b3c565b886106bb565b503d6107be565b50506107ef8360405193849384610253565b0390f35b610815915060203d60201161081b575b61080d8183610856565b810190611994565b5f610393565b503d610803565b634e487b7160e01b5f52604160045260245ffd5b608081019081106001600160401b0382111761085157604052565b610822565b90601f801991011681019081106001600160401b0382111761085157604052565b6001600160a01b0381160361021357565b359063ffffffff8216820361021357565b6001600160401b03811161085157601f01601f191660200190565b9291926108c082610899565b916108ce6040519384610856565b829481845281830111610213578281602093845f960137010152565b9080601f83011215610213578160206102b7933591016108b4565b34610213576020366003190112610213576004356001600160401b03811161021357608060031982360301126102135760405161094181610836565b816004013561094f81610877565b815261095d60248301610888565b60208201526044820135604082015260648201356001600160401b0381116102135761099c92600461099292369201016108ea565b6060820152611b7b565b005b6001600160401b0381116108515760051b60200190565b9080601f830112156102135781356109cc8161099e565b926109da6040519485610856565b81845260208085019260051b82010192831161021357602001905b828210610a025750505090565b60208091610a0f84610888565b8152019101906109f5565b602060031982011261021357600435906001600160401b038211610213576102b7916004016109b5565b3461021357610a5236610a1a565b5061099c612967565b9181601f84011215610213578235916001600160401b038311610213576020808501948460051b01011161021357565b9190916060818403126102135760405190606082018281106001600160401b0382111761085157604052819381356001600160401b0381116102135782019181601f8401121561021357610ae96040939283602086953591016108b4565b8452602081013560208501520135910152565b3461021357606036600319011261021357610b18600435610877565b6024356001600160401b03811161021357610b37903690600401610a5b565b50506044356001600160401b03811161021357610b58903690600401610a8b565b5061099c611d60565b34610213575f36600319011261021357610b79612967565b60685462093a808101809111610c12574210610bb657606754610ba4906001600160a01b03166129bf565b606780546001600160a01b0319169055005b60405162461bcd60e51b815260206004820152602e60248201527f536572766963654d616e616765723a20536c61736865722070726f706f73616c60448201526d0819195b185e481b9bdd081b595d60921b6064820152608490fd5b611a05565b34610213576020366003190112610213576107ef610c3f600435610c3a81610877565b611e03565b604051918291602083526020830190610217565b3461021357602036600319011261021357600435610c7081610877565b610c78612967565b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555005b34610213575f36600319011261021357602060405162093a808152f35b34610213575f366003190112610213576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610213575f36600319011261021357610d4f612967565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34610213575f366003190112610213577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b15610213576040516334f65bfd60e21b81523060048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166024820152905f908290604490829084905af1801561052457610e2f57005b80610e3d5f61099c93610856565b80610209565b34610213575f366003190112610213576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610213576020366003190112610213577f2f8afc8a78fd958f3301c0233aa326b9c4b9a2884a7483227d6b0555aaa03adb610f0d600435610ec881610877565b610ed0612967565b606780546001600160a01b0319166001600160a01b0392909216918217905542606881905560408051928352602083019190915290918291820190565b0390a1005b34610213575f36600319011261021357602060ff606954166040519015158152f35b34610213575f366003190112610213576033546040516001600160a01b039091168152602090f35b3461021357604036600319011261021357600435610f7981610877565b6024356001600160401b03811161021357610f98903690600401610a8b565b90610fcd337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611cd3565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b15610213575f928392604051948580948193639926ee7d60e01b835260018060a01b0316600483015260406024830152604061104282516060604486015260a4850190611caf565b91602081015160648501520151608483015203925af1801561052457610e2f57005b34610213575f60203660031901126102135760043561108281610877565b6110b6337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611cd3565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610213576040516351b27a6d60e11b81526001600160a01b039091166004820152905f908290602490829084905af1801561052457611121575080f35b61099c91505f90610856565b34610213575f6020366003190112610213576004356001600160401b0381116102135761115e9036906004016108ea565b611166612967565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102135760405163a98fb35560e01b815260206004820152915f9183918290849082906111c4906024830190611caf565b03925af1801561052457611121575080f35b3461021357610b5836610a1a565b34610213575f366003190112610213576066546040516001600160a01b039091168152602090f35b34610213575f36600319011261021357611224612967565b600160695461123660ff82161561213c565b60ff191617606955005b346102135760403660031901126102135761125c600435610877565b6024356001600160401b0381116102135761127b903690600401610a5b565b505061099c611d60565b34610213575f366003190112610213576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b9080601f830112156102135781356112e08161099e565b926112ee6040519485610856565b81845260208085019260051b82010192831161021357602001905b8282106113165750505090565b60208091833561132581610877565b815201910190611309565b34610213576040366003190112610213576004356001600160401b038111610213573660238201121561021357806004013561136b8161099e565b916113796040519384610856565b8183526024602084019260051b820101903682116102135760248101925b8284106113c757602435856001600160401b038211610213576113c161099c9236906004016112c9565b9061219c565b83356001600160401b038111610213576020916113eb8392602436918701016109b5565b815201930192611397565b34610213575f366003190112610213576067546040516001600160a01b039091168152602090f35b34610213575f366003190112610213576107ef610c3f612253565b602060031982011261021357600435906001600160401b0382116102135761146391600401610a5b565b9091565b346102135761147536611439565b907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156102135791604051928391630130fc2760e51b835280604484013060048601526040602486015252606483019060648160051b8501019280925f91603e19813603015b84841061150557875f81808a0381838e5af1801561052457610e2f57005b9193959092949650606319898203018452863582811215610213578301604082019063ffffffff61153582610888565b1683526020810135601e1982360301811215610213570190602082359201926001600160401b038311610213578260051b36038413610213578260609260406020840152520191905f905b8082106115a257505050602080600192980194019401918896959394916114e7565b90919260208060019286356115b681610877565b848060a01b03168152019401920190611580565b34610213576020366003190112610213576004356115e781610877565b6115ef612967565b6001600160a01b0381161561164657603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3005b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b34610213575f366003190112610213576065546040516001600160a01b039091168152602090f35b34610213575f366003190112610213576020606854604051908152f35b34610213576116ed36611439565b6065549091906001600160a01b03163303611914577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316915f5b8181106117725750823b1561021357611762925f92836040518096819582946321f5223b60e11b84523060048501612531565b03925af1801561052457610e2f57005b5f60206117cd6117906105858361178a87898b612480565b016124a2565b604061179d86888a612480565b6040516323b872dd60e01b8152336004820152306024820152910135604482015293849283919082906064820190565b03925af18015610524576118f8575b506117f0610585602061178a848688612480565b604051636eb1769f60e11b81523060048201526001600160a01b03861660248201529190602090839060449082905afa8015610524576118936020915f9485916118db575b5061186061184b6105858561178a888b8d612480565b916040611859878a8c612480565b0135611d94565b60405163095ea7b360e01b81526001600160a01b038a166004820152602481019190915294859283919082906044820190565b03925af1918215610524576001926118ad575b500161172f565b6118cd9060203d81116118d4575b6118c58183610856565b8101906124ac565b505f6118a6565b503d6118bb565b6118f29150833d81116107d6576107c88183610856565b5f611835565b61190f9060203d81116118d4576118c58183610856565b6117dc565b60405162461bcd60e51b815260206004820152604c60248201527f536572766963654d616e61676572426173652e6f6e6c7952657761726473496e60448201527f69746961746f723a2063616c6c6572206973206e6f742074686520726577617260648201526b32399034b734ba34b0ba37b960a11b608482015260a490fd5b90816020910312610213575160ff811681036102135790565b6040513d5f823e3d90fd5b604051906119c7602083610856565b5f808352366020840137565b906119dd8261099e565b6119ea6040519182610856565b82815280926119fb601f199161099e565b0190602036910137565b634e487b7160e01b5f52601160045260245ffd5b60ff1660ff8114610c125760010190565b9081602091031261021357516102b781610877565b602081830312610213578051906001600160401b03821161021357019080601f83011215610213578151611a728161099e565b92611a806040519485610856565b81845260208085019260051b82010192831161021357602001905b828210611aa85750505090565b8151815260209182019101611a9b565b634e487b7160e01b5f52603260045260245ffd5b8051821015611ae05760209160051b010190565b611ab8565b5f198114610c125760010190565b90611afd8261099e565b611b0a6040519182610856565b8281528092611b1b601f199161099e565b01905f5b828110611b2b57505050565b806060602080938501015201611b1f565b90816020910312610213575190565b9081602091031261021357516001600160c01b03811681036102135790565b908151811015611ae0570160200190565b6066546001600160a01b03163303611c44577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102135760605f91611c239383604051809681958294630dcfb3f560e31b84523060048501526040602485015260018060a01b03815116604485015263ffffffff6020820151166064850152604081015160848501520151608060a484015260c4830190611caf565b03925af1801561052457611c345750565b80610e3d5f611c4293610856565b565b60405162461bcd60e51b815260206004820152603960248201527f536572766963654d616e61676572426173652e6f6e6c79536c61736865723a2060448201527f63616c6c6572206973206e6f742074686520736c6173686572000000000000006064820152608490fd5b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b15611cda57565b60405162461bcd60e51b815260206004820152605260248201527f536572766963654d616e61676572426173652e6f6e6c7952656769737472794360448201527f6f6f7264696e61746f723a2063616c6c6572206973206e6f742074686520726560648201527133b4b9ba393c9031b7b7b93234b730ba37b960711b608482015260a490fd5b611c42337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611cd3565b91908201809211610c1257565b6bffffffffffffffffffffffff81160361021357565b90816040910312610213576040519060408201908282106001600160401b03831117610851576020916040528051611dee81610877565b83520151611dfb81611da1565b602082015290565b6040516309aa152760e11b81526001600160a01b0391821660048201527f000000000000000000000000000000000000000000000000000000000000000090911690602081602481855afa90811561052457611e82916020915f9161211f57506040518093819263871ef04960e01b8352600483019190602083019252565b0381855afa908115610524575f91612100575b506001600160c01b03169081159081156120ad575b506120a457611eb8906128c8565b5f91907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690835b8151851015611f7057611f296020611f0661073e6107308987611b6a565b604051633ca5a5f560e01b815260ff909116600482015291829081906024820190565b0381875afa801561052457600192611f48925f92611f50575b50611d94565b940193611ee8565b611f6991925060203d81116107d6576107c88183610856565b905f611f42565b611f7b9194506119d3565b925f905f5b815181101561209e57611f9961073e6107308385611b6a565b604051633ca5a5f560e01b815260ff8216600482015290602082602481895afa918215610524575f9261207e575b50905f915b818310611fde57505050600101611f80565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa918215610524576120428b6104cc8361203c610585600198612047985f91612050575b50516001600160a01b031690565b92611acc565b611ae5565b95019190611fcc565b612071915060403d8111612077575b6120698183610856565b810190611db7565b5f61202e565b503d61205f565b61209791925060203d81116107d6576107c88183610856565b905f611fc7565b50505050565b506102b76119b8565b604051639aa1653d60e01b81529150602090829060049082905afa80156105245760ff915f916120e1575b5016155f611eaa565b6120fa915060203d60201161081b5761080d8183610856565b5f6120d8565b612119915060203d6020116107a95761079b8183610856565b5f611e95565b6121369150823d84116107d6576107c88183610856565b5f6106bb565b1561214357565b60405162461bcd60e51b815260206004820152602b60248201527f536572766963654d616e616765723a204d6967726174696f6e20416c7265616460448201526a1e48119a5b985b1a5e995960aa1b6064820152608490fd5b91906121a6612967565b6121b560ff606954161561213c565b80518351036121fa575f5b81518110156121f457806121ed6121dc61057860019486611acc565b6121e68388611acc565b5190612a78565b50016121c0565b50509050565b60405162461bcd60e51b815260206004820152602b60248201527f536572766963654d616e616765723a20496e707574206172726179206c656e6760448201526a0e8d040dad2e6dac2e8c6d60ab1b6064820152608490fd5b604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156105245760ff915f91612461575b50168015612457577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316905f9081905b80831061241357506122ee91506119d3565b925f905f5b604051639aa1653d60e01b8152602081600481895afa80156105245760ff915f916123f5575b50168110156123ee57604051633ca5a5f560e01b815260ff821660048201819052602082602481895afa918215610524575f926123ce575b50905f915b818310612368575050506001016122f3565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa918215610524576120428b6104cc8361203c6105856001986123c5985f916120505750516001600160a01b031690565b95019190612356565b6123e791925060203d81116107d6576107c88183610856565b905f612351565b5092505050565b61240d915060203d811161081b5761080d8183610856565b5f612319565b604051633ca5a5f560e01b815260ff84166004820152909190602081602481885afa80156105245760019261244e925f92611f505750611d94565b920191906122dc565b50506102b76119b8565b61247a915060203d60201161081b5761080d8183610856565b5f6122a4565b9190811015611ae05760051b81013590609e1981360301821215610213570190565b356102b781610877565b90816020910312610213575180151581036102135790565b916020908281520191905f5b8181106124dd5750505090565b90919260408060019286356124f181610877565b848060a01b031681526bffffffffffffffffffffffff602088013561251581611da1565b1660208201520194019291016124d0565b3590611c4282610877565b928091604085019060018060a01b031685526040602086015252606083019060608160051b85010193835f91609e1982360301905b848410612577575050505050505090565b90919293949596605f19828203018752873583811215610213578401908135601e198336030181121561021357820191602083359301906001600160401b038411610213578360061b3603821361021357612640836080612635816125eb6020989760019a60a08b9a5260a08701916124c4565b956126096125fa898301612526565b6001600160a01b0316868a0152565b6040810135604086015261262f61262260608301610888565b63ffffffff166060870152565b01610888565b63ffffffff16910152565b99019701959401929190612566565b91908203918211610c1257565b8051600181111561270e5760011c91612674836119d3565b9161268861268385835161264f565b6119d3565b915f5b8581106126eb5750845b82518110156126ca57806126c46126b161057860019487611acc565b6104cc6126be8a8561264f565b88611acc565b01612695565b50935050906126d89061265c565b6126e2909161265c565b6102b791612712565b806127086126fe61057860019487611acc565b6104cc8389611acc565b0161268b565b5090565b9182519282516127256126838287611d94565b935f935f925f975b80871080612897575b15612818576127486105788888611acc565b6127586105856105788888611acc565b6001600160a01b03909116101561279a5761279561278261057861277b8a611ae5565b9989611acc565b6104cc61278e8c611ae5565b9b8b611acc565b61272d565b6127a76105788888611acc565b6127b76105856105788888611acc565b6001600160a01b0390911611156127e1576127956127826105786127da88611ae5565b9787611acc565b93612812906120426127ff6105786127f88b611ae5565b9a8a611acc565b6104cc61280b8d611ae5565b9c8c611acc565b9361272d565b9795919794909293945b80831061286d575050505b80831061283c57505050815290565b61286861285561057861284e86611ae5565b9585611acc565b6104cc61286187611ae5565b9688611acc565b61282d565b61289261287f61057861284e86611ae5565b6104cc61288b8a611ae5565b998b611acc565b612822565b50818510612736565b906128aa82610899565b6128b76040519182610856565b82815280926119fb601f1991610899565b5f81805b61294257506128de9061ffff166128a0565b5f5f5b8251821080612937575b15612930576001811b8416612909575b61290490611ae5565b6128e1565b9060016129049160ff60f81b8460f81b165f1a6129268287611b6a565b53019190506128fb565b5050905090565b5061010081106128eb565b5f198101818111610c125761ffff9116911661ffff8114610c125760010190806128cc565b6033546001600160a01b0316330361297b57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b606654604080516001600160a01b038084168252841660208201529192917fe0d49a54274423183dadecbdf239eaac6e06ba88320b26fe8cc5ec9d050a63959190a16001600160a01b03166001600160a01b03199190911617606655565b15612a2457565b60405162461bcd60e51b815260206004820152602660248201527f536572766963654d616e616765723a204f70657261746f72206e6f7420696e2060448201526571756f72756d60d01b6064820152608490fd5b6040516309aa152760e11b81526001600160a01b0391821660048201527f00000000000000000000000000000000000000000000000000000000000000009091169291905f90602081602481885afa801561052457612afc956020925f92612b85575b50604051808098819463871ef04960e01b8352600483019190602083019252565b03915afa938415610524575f94612b64575b505f5b8251811015612b5e5780612b58612b53612b3d610744612b3360019689611acc565b5163ffffffff1690565b848060c01b03891660ff600192161c1660011490565b612a1d565b01612b11565b50925050565b612b7e91945060203d6020116107a95761079b8183610856565b925f612b0e565b612b9d919250833d85116107d6576107c88183610856565b905f612adb56fea2646970667358221220d87d354b1fa836642ce2239509574e5455550289597d3d12046800ddfc1fedc264736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x0B\x91\xD6e\x14a\x02\x04W\x80c\x0E\xA4>C\x14a\x01\xFFW\x80c\x15\xB7\xBC\x9A\x14a\x01\xFAW\x80c\x1E!\x99\xE2\x14a\x01\xF5W\x80c&\xF0\x17\xE2\x14a\x01\xF0W\x80c3\xCF\xB7\xB7\x14a\x01\xEBW\x80c;\xC2\x8C\x8C\x14a\x01\xE6W\x80cg\x94\x0C\x89\x14a\x01\xE1W\x80ck:\xA7.\x14a\x01\xDCW\x80cqP\x18\xA6\x14a\x01\xD7W\x80cr\x08\0u\x14a\x01\xD2W\x80cw\xEFs\x1D\x14a\x01\xCDW\x80c\x89\x99\x81\x7F\x14a\x01\xC8W\x80c\x8Dh4\x9A\x14a\x01\xC3W\x80c\x8D\xA5\xCB[\x14a\x01\xBEW\x80c\x99&\xEE}\x14a\x01\xB9W\x80c\xA3d\xF4\xDA\x14a\x01\xB4W\x80c\xA9\x8F\xB3U\x14a\x01\xAFW\x80c\xAF\xE0.\xD5\x14a\x01\xAAW\x80c\xB14Bq\x14a\x01\xA5W\x80c\xB7\x8B`\x87\x14a\x01\xA0W\x80c\xC1\xA8\xE2\xC5\x14a\x01\x9BW\x80c\xCA\x8A\xA7\xC7\x14a\x01\x96W\x80c\xD9\xF9Sw\x14a\x01\x91W\x80c\xE4o\x18\x16\x14a\x01\x8CW\x80c\xE4\x81\xAF\x9D\x14a\x01\x87W\x80c\xE6\x1B\xBE\xE8\x14a\x01\x82W\x80c\xF2\xFD\xE3\x8B\x14a\x01}W\x80c\xFC)\x9D\xEE\x14a\x01xW\x80c\xFC\xD1\xC3u\x14a\x01sWc\xFC\xE3l}\x14a\x01nW_\x80\xFD[a\x16\xDFV[a\x16\xC2V[a\x16\x9AV[a\x15\xCAV[a\x14gV[a\x14\x1EV[a\x13\xF6V[a\x130V[a\x12\x85V[a\x12@V[a\x12\x0CV[a\x11\xE4V[a\x11\xD6V[a\x11-V[a\x10dV[a\x0F\\V[a\x0F4V[a\x0F\x12V[a\x0E\x87V[a\x0ECV[a\r\x92V[a\r7V[a\x0C\xF3V[a\x0C\xD6V[a\x0CSV[a\x0C\x17V[a\x0BaV[a\n\xFCV[a\nDV[a\t\x05V[a\x032V[_\x91\x03\x12a\x02\x13WV[_\x80\xFD[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x024WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x02'V[``\x80\x82R\x82Q\x90\x82\x01\x81\x90R`\x80\x82\x01\x95\x94\x92` \x01\x90_[\x81\x81\x10a\x03\x16WPPP\x80\x85\x03` \x82\x01R\x82Q\x80\x86R` \x86\x01\x90` \x80\x82`\x05\x1B\x89\x01\x01\x95\x01\x91_\x90[\x82\x82\x10a\x02\xBAWPPPPa\x02\xB7\x93\x94P`@\x81\x84\x03\x91\x01Ra\x02\x17V[\x90V[\x90\x91\x92\x95`\x1F\x19\x89\x82\x03\x01\x82R\x86Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x02\xF8WPPP` \x80`\x01\x92\x98\x01\x92\x01\x92\x01\x90\x92\x91a\x02\x99V[\x90\x91\x92` \x80`\x01\x92c\xFF\xFF\xFF\xFF\x87Q\x16\x81R\x01\x94\x01\x92\x01\x90a\x02\xDAV[\x82Qc\xFF\xFF\xFF\xFF\x16\x88R` \x97\x88\x01\x97\x90\x92\x01\x91`\x01\x01a\x02mV[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x05$W`\xFF\x91_\x91a\x07\xF3W[P\x16\x90a\x03\x9Ea\x19\xB8V[a\x03\xA7\x83a\x19\xD3V[\x91Cc\xFF\xFF\xFF\xFF\x16\x90_[`\xFF\x81\x16\x94\x86\x86\x10\x15a\x06LW`@QcOL\x91\xE1`\xE1\x1B\x81R\x94` \x86`\x04\x81\x87Z\xFA\x95\x86\x15a\x05$W_\x96a\x06,W[P`@Qc\x89\x02bE`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x86\x16`$\x82\x01R\x95_\x90\x87\x90`D\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x95\x86\x15a\x05$W_\x96a\x06\x08W[Pa\x045\x86Qa\x19\xD3V[\x97_\x91[\x87Q\x83\x10\x15a\x05FW`@Qc.\xFA,\xA3`\xE1\x1B\x81R` \x81`\x04\x81\x8AZ\xFA\x90\x81\x15a\x05$Wa\x04\x9A\x91` \x91_\x91a\x05)W[Pa\x04x\x86\x8Ca\x1A\xCCV[Q\x90`@Q\x80\x80\x95\x81\x94c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x05$Wa\x04\xDB\x8C`\x01\x94\x87a\x04\xEA\x95_\x92a\x04\xF2W[Pa\x04\xCC\x91\x92a\x1A\xCCV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a\x04\xE4\x8Ca&\\V[\x90a'\x12V[\x92\x01\x91a\x049V[a\x04\xCC\x92Pa\x05\x17\x90` =\x81\x11a\x05\x1DW[a\x05\x0F\x81\x83a\x08VV[\x81\x01\x90a\x1A*V[\x91a\x04\xC1V[P=a\x05\x05V[a\x19\xADV[a\x05@\x91P\x82=\x81\x11a\x05\x1DWa\x05\x0F\x81\x83a\x08VV[_a\x04mV[\x96P\x97PP\x94\x90\x91a\x05X\x85Qa\x19\xD3V[\x96_\x94_[\x87Q\x81\x10\x15a\x05\xD1W\x89\x88a\x05\x91a\x05\x85a\x05x\x85\x84a\x1A\xCCV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x05\xA0W[PP`\x01\x01a\x05]V[\x82\x91\x98a\x04\xCCa\x05\xB8a\x05x`\x01\x96a\x05\xC8\x95a\x1A\xCCV[\x91a\x05\xC2\x81a\x1A\xE5V[\x9Ba\x1A\xCCV[\x90P\x89\x88a\x05\x96V[P\x94\x88R\x95\x96\x93\x94P\x91a\x05\xFD\x91a\x05\xF8\x90a\x05\xED\x81\x8Aa\x1A\xCCV[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[a\x1A\x19V[\x94\x93\x94\x92\x91\x92a\x03\xB2V[a\x06%\x91\x96P=\x80_\x83>a\x06\x1D\x81\x83a\x08VV[\x81\x01\x90a\x1A?V[\x94_a\x04*V[a\x06E\x91\x96P` =\x81\x11a\x05\x1DWa\x05\x0F\x81\x83a\x08VV[\x94_a\x03\xE4V[\x84\x92\x91Pa\x06Z\x83Qa\x1A\xF3V[\x91_[\x84Q\x81\x10\x15a\x07\xDDWa\x06\xA1` a\x06xa\x05x\x84\x89a\x1A\xCCV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x86Z\xFA\x90\x81\x15a\x05$Wa\x06\xDB\x91` \x91_\x91a\x07\xB0W[P`@Q\x80\x93\x81\x92c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x80\x15a\x05$Wa\x07\0\x91_\x91a\x07\x82W[P`\x01`\x01`\xC0\x1B\x03\x16a(\xC8V[\x91a\x07\x0B\x83Qa\x19\xD3V[\x93_[\x84Q\x81\x10\x15a\x07ZW\x80a\x07Ta\x07Ja\x07Da\x07>a\x070`\x01\x96\x8Ba\x1BjV[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`\xFF\x16\x90V[a\x05\xED\x83\x8Aa\x1A\xCCV[\x01a\x07\x0EV[P\x93`\x01\x92\x91\x96\x93Pa\x07m\x82\x87a\x1A\xCCV[Ra\x07x\x81\x86a\x1A\xCCV[P\x01\x93\x90\x93a\x06]V[a\x07\xA3\x91P` =\x81\x11a\x07\xA9W[a\x07\x9B\x81\x83a\x08VV[\x81\x01\x90a\x1BKV[\x87a\x06\xF1V[P=a\x07\x91V[a\x07\xD0\x91P\x82=\x81\x11a\x07\xD6W[a\x07\xC8\x81\x83a\x08VV[\x81\x01\x90a\x1B<V[\x88a\x06\xBBV[P=a\x07\xBEV[PPa\x07\xEF\x83`@Q\x93\x84\x93\x84a\x02SV[\x03\x90\xF3[a\x08\x15\x91P` =` \x11a\x08\x1BW[a\x08\r\x81\x83a\x08VV[\x81\x01\x90a\x19\x94V[_a\x03\x93V[P=a\x08\x03V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08QW`@RV[a\x08\"V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08QW`@RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02\x13WV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02\x13WV[`\x01`\x01`@\x1B\x03\x81\x11a\x08QW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x08\xC0\x82a\x08\x99V[\x91a\x08\xCE`@Q\x93\x84a\x08VV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x02\x13W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x02\x13W\x81` a\x02\xB7\x935\x91\x01a\x08\xB4V[4a\x02\x13W` 6`\x03\x19\x01\x12a\x02\x13W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13W`\x80`\x03\x19\x826\x03\x01\x12a\x02\x13W`@Qa\tA\x81a\x086V[\x81`\x04\x015a\tO\x81a\x08wV[\x81Ra\t]`$\x83\x01a\x08\x88V[` \x82\x01R`D\x82\x015`@\x82\x01R`d\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13Wa\t\x9C\x92`\x04a\t\x92\x926\x92\x01\x01a\x08\xEAV[``\x82\x01Ra\x1B{V[\0[`\x01`\x01`@\x1B\x03\x81\x11a\x08QW`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x02\x13W\x815a\t\xCC\x81a\t\x9EV[\x92a\t\xDA`@Q\x94\x85a\x08VV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\x13W` \x01\x90[\x82\x82\x10a\n\x02WPPP\x90V[` \x80\x91a\n\x0F\x84a\x08\x88V[\x81R\x01\x91\x01\x90a\t\xF5V[` `\x03\x19\x82\x01\x12a\x02\x13W`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x13Wa\x02\xB7\x91`\x04\x01a\t\xB5V[4a\x02\x13Wa\nR6a\n\x1AV[Pa\t\x9Ca)gV[\x91\x81`\x1F\x84\x01\x12\x15a\x02\x13W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\x13W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x02\x13WV[\x91\x90\x91``\x81\x84\x03\x12a\x02\x13W`@Q\x90``\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08QW`@R\x81\x93\x815`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13W\x82\x01\x91\x81`\x1F\x84\x01\x12\x15a\x02\x13Wa\n\xE9`@\x93\x92\x83` \x86\x955\x91\x01a\x08\xB4V[\x84R` \x81\x015` \x85\x01R\x015\x91\x01RV[4a\x02\x13W``6`\x03\x19\x01\x12a\x02\x13Wa\x0B\x18`\x045a\x08wV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13Wa\x0B7\x906\x90`\x04\x01a\n[V[PP`D5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13Wa\x0BX\x906\x90`\x04\x01a\n\x8BV[Pa\t\x9Ca\x1D`V[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13Wa\x0Bya)gV[`hTb\t:\x80\x81\x01\x80\x91\x11a\x0C\x12WB\x10a\x0B\xB6W`gTa\x0B\xA4\x90`\x01`\x01`\xA0\x1B\x03\x16a)\xBFV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FServiceManager: Slasher proposal`D\x82\x01Rm\x08\x19\x19[\x18^H\x1B\x9B\xDD\x08\x1BY]`\x92\x1B`d\x82\x01R`\x84\x90\xFD[a\x1A\x05V[4a\x02\x13W` 6`\x03\x19\x01\x12a\x02\x13Wa\x07\xEFa\x0C?`\x045a\x0C:\x81a\x08wV[a\x1E\x03V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x02\x17V[4a\x02\x13W` 6`\x03\x19\x01\x12a\x02\x13W`\x045a\x0Cp\x81a\x08wV[a\x0Cxa)gV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`eU\0[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W` `@Qb\t:\x80\x81R\xF3[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13Wa\rOa)gV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\x13W`@Qc4\xF6[\xFD`\xE2\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90_\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x05$Wa\x0E/W\0[\x80a\x0E=_a\t\x9C\x93a\x08VV[\x80a\x02\tV[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x13W` 6`\x03\x19\x01\x12a\x02\x13W\x7F/\x8A\xFC\x8Ax\xFD\x95\x8F3\x01\xC0#:\xA3&\xB9\xC4\xB9\xA2\x88Jt\x83\"}k\x05U\xAA\xA0:\xDBa\x0F\r`\x045a\x0E\xC8\x81a\x08wV[a\x0E\xD0a)gV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90UB`h\x81\x90U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1\0[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W` `\xFF`iT\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x13W`@6`\x03\x19\x01\x12a\x02\x13W`\x045a\x0Fy\x81a\x08wV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13Wa\x0F\x98\x906\x90`\x04\x01a\n\x8BV[\x90a\x0F\xCD3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1C\xD3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x02\x13W_\x92\x83\x92`@Q\x94\x85\x80\x94\x81\x93c\x99&\xEE}`\xE0\x1B\x83R`\x01\x80`\xA0\x1B\x03\x16`\x04\x83\x01R`@`$\x83\x01R`@a\x10B\x82Q```D\x86\x01R`\xA4\x85\x01\x90a\x1C\xAFV[\x91` \x81\x01Q`d\x85\x01R\x01Q`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\x05$Wa\x0E/W\0[4a\x02\x13W_` 6`\x03\x19\x01\x12a\x02\x13W`\x045a\x10\x82\x81a\x08wV[a\x10\xB63\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1C\xD3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x02\x13W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x05$Wa\x11!WP\x80\xF3[a\t\x9C\x91P_\x90a\x08VV[4a\x02\x13W_` 6`\x03\x19\x01\x12a\x02\x13W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13Wa\x11^\x906\x90`\x04\x01a\x08\xEAV[a\x11fa)gV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\x13W`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R` `\x04\x82\x01R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x11\xC4\x90`$\x83\x01\x90a\x1C\xAFV[\x03\x92Z\xF1\x80\x15a\x05$Wa\x11!WP\x80\xF3[4a\x02\x13Wa\x0BX6a\n\x1AV[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W`fT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13Wa\x12$a)gV[`\x01`iTa\x126`\xFF\x82\x16\x15a!<V[`\xFF\x19\x16\x17`iU\0[4a\x02\x13W`@6`\x03\x19\x01\x12a\x02\x13Wa\x12\\`\x045a\x08wV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13Wa\x12{\x906\x90`\x04\x01a\n[V[PPa\t\x9Ca\x1D`V[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90\x80`\x1F\x83\x01\x12\x15a\x02\x13W\x815a\x12\xE0\x81a\t\x9EV[\x92a\x12\xEE`@Q\x94\x85a\x08VV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\x13W` \x01\x90[\x82\x82\x10a\x13\x16WPPP\x90V[` \x80\x91\x835a\x13%\x81a\x08wV[\x81R\x01\x91\x01\x90a\x13\tV[4a\x02\x13W`@6`\x03\x19\x01\x12a\x02\x13W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13W6`#\x82\x01\x12\x15a\x02\x13W\x80`\x04\x015a\x13k\x81a\t\x9EV[\x91a\x13y`@Q\x93\x84a\x08VV[\x81\x83R`$` \x84\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x02\x13W`$\x81\x01\x92[\x82\x84\x10a\x13\xC7W`$5\x85`\x01`\x01`@\x1B\x03\x82\x11a\x02\x13Wa\x13\xC1a\t\x9C\x926\x90`\x04\x01a\x12\xC9V[\x90a!\x9CV[\x835`\x01`\x01`@\x1B\x03\x81\x11a\x02\x13W` \x91a\x13\xEB\x83\x92`$6\x91\x87\x01\x01a\t\xB5V[\x81R\x01\x93\x01\x92a\x13\x97V[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W`gT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13Wa\x07\xEFa\x0C?a\"SV[` `\x03\x19\x82\x01\x12a\x02\x13W`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x13Wa\x14c\x91`\x04\x01a\n[V[\x90\x91V[4a\x02\x13Wa\x14u6a\x149V[\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x02\x13W\x91`@Q\x92\x83\x91c\x010\xFC'`\xE5\x1B\x83R\x80`D\x84\x010`\x04\x86\x01R`@`$\x86\x01RR`d\x83\x01\x90`d\x81`\x05\x1B\x85\x01\x01\x92\x80\x92_\x91`>\x19\x816\x03\x01[\x84\x84\x10a\x15\x05W\x87_\x81\x80\x8A\x03\x81\x83\x8EZ\xF1\x80\x15a\x05$Wa\x0E/W\0[\x91\x93\x95\x90\x92\x94\x96P`c\x19\x89\x82\x03\x01\x84R\x865\x82\x81\x12\x15a\x02\x13W\x83\x01`@\x82\x01\x90c\xFF\xFF\xFF\xFFa\x155\x82a\x08\x88V[\x16\x83R` \x81\x015`\x1E\x19\x826\x03\x01\x81\x12\x15a\x02\x13W\x01\x90` \x825\x92\x01\x92`\x01`\x01`@\x1B\x03\x83\x11a\x02\x13W\x82`\x05\x1B6\x03\x84\x13a\x02\x13W\x82``\x92`@` \x84\x01RR\x01\x91\x90_\x90[\x80\x82\x10a\x15\xA2WPPP` \x80`\x01\x92\x98\x01\x94\x01\x94\x01\x91\x88\x96\x95\x93\x94\x91a\x14\xE7V[\x90\x91\x92` \x80`\x01\x92\x865a\x15\xB6\x81a\x08wV[\x84\x80`\xA0\x1B\x03\x16\x81R\x01\x94\x01\x92\x01\x90a\x15\x80V[4a\x02\x13W` 6`\x03\x19\x01\x12a\x02\x13W`\x045a\x15\xE7\x81a\x08wV[a\x15\xEFa)gV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x16FW`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x13W_6`\x03\x19\x01\x12a\x02\x13W` `hT`@Q\x90\x81R\xF3[4a\x02\x13Wa\x16\xED6a\x149V[`eT\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x19\x14W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91_[\x81\x81\x10a\x17rWP\x82;\x15a\x02\x13Wa\x17b\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c!\xF5\";`\xE1\x1B\x84R0`\x04\x85\x01a%1V[\x03\x92Z\xF1\x80\x15a\x05$Wa\x0E/W\0[_` a\x17\xCDa\x17\x90a\x05\x85\x83a\x17\x8A\x87\x89\x8Ba$\x80V[\x01a$\xA2V[`@a\x17\x9D\x86\x88\x8Aa$\x80V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x91\x015`D\x82\x01R\x93\x84\x92\x83\x91\x90\x82\x90`d\x82\x01\x90V[\x03\x92Z\xF1\x80\x15a\x05$Wa\x18\xF8W[Pa\x17\xF0a\x05\x85` a\x17\x8A\x84\x86\x88a$\x80V[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x86\x16`$\x82\x01R\x91\x90` \x90\x83\x90`D\x90\x82\x90Z\xFA\x80\x15a\x05$Wa\x18\x93` \x91_\x94\x85\x91a\x18\xDBW[Pa\x18`a\x18Ka\x05\x85\x85a\x17\x8A\x88\x8B\x8Da$\x80V[\x91`@a\x18Y\x87\x8A\x8Ca$\x80V[\x015a\x1D\x94V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x94\x85\x92\x83\x91\x90\x82\x90`D\x82\x01\x90V[\x03\x92Z\xF1\x91\x82\x15a\x05$W`\x01\x92a\x18\xADW[P\x01a\x17/V[a\x18\xCD\x90` =\x81\x11a\x18\xD4W[a\x18\xC5\x81\x83a\x08VV[\x81\x01\x90a$\xACV[P_a\x18\xA6V[P=a\x18\xBBV[a\x18\xF2\x91P\x83=\x81\x11a\x07\xD6Wa\x07\xC8\x81\x83a\x08VV[_a\x185V[a\x19\x0F\x90` =\x81\x11a\x18\xD4Wa\x18\xC5\x81\x83a\x08VV[a\x17\xDCV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FServiceManagerBase.onlyRewardsIn`D\x82\x01R\x7Fitiator: caller is not the rewar`d\x82\x01Rk29\x904\xB74\xBA4\xB0\xBA7\xB9`\xA1\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x90\x81` \x91\x03\x12a\x02\x13WQ`\xFF\x81\x16\x81\x03a\x02\x13W\x90V[`@Q=_\x82>=\x90\xFD[`@Q\x90a\x19\xC7` \x83a\x08VV[_\x80\x83R6` \x84\x017V[\x90a\x19\xDD\x82a\t\x9EV[a\x19\xEA`@Q\x91\x82a\x08VV[\x82\x81R\x80\x92a\x19\xFB`\x1F\x19\x91a\t\x9EV[\x01\x90` 6\x91\x017V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x16`\xFF\x81\x14a\x0C\x12W`\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x02\x13WQa\x02\xB7\x81a\x08wV[` \x81\x83\x03\x12a\x02\x13W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x13W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x02\x13W\x81Qa\x1Ar\x81a\t\x9EV[\x92a\x1A\x80`@Q\x94\x85a\x08VV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\x13W` \x01\x90[\x82\x82\x10a\x1A\xA8WPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a\x1A\x9BV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q\x82\x10\x15a\x1A\xE0W` \x91`\x05\x1B\x01\x01\x90V[a\x1A\xB8V[_\x19\x81\x14a\x0C\x12W`\x01\x01\x90V[\x90a\x1A\xFD\x82a\t\x9EV[a\x1B\n`@Q\x91\x82a\x08VV[\x82\x81R\x80\x92a\x1B\x1B`\x1F\x19\x91a\t\x9EV[\x01\x90_[\x82\x81\x10a\x1B+WPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a\x1B\x1FV[\x90\x81` \x91\x03\x12a\x02\x13WQ\x90V[\x90\x81` \x91\x03\x12a\x02\x13WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x02\x13W\x90V[\x90\x81Q\x81\x10\x15a\x1A\xE0W\x01` \x01\x90V[`fT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1CDW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\x13W``_\x91a\x1C#\x93\x83`@Q\x80\x96\x81\x95\x82\x94c\r\xCF\xB3\xF5`\xE3\x1B\x84R0`\x04\x85\x01R`@`$\x85\x01R`\x01\x80`\xA0\x1B\x03\x81Q\x16`D\x85\x01Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16`d\x85\x01R`@\x81\x01Q`\x84\x85\x01R\x01Q`\x80`\xA4\x84\x01R`\xC4\x83\x01\x90a\x1C\xAFV[\x03\x92Z\xF1\x80\x15a\x05$Wa\x1C4WPV[\x80a\x0E=_a\x1CB\x93a\x08VV[V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FServiceManagerBase.onlySlasher: `D\x82\x01R\x7Fcaller is not the slasher\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x15a\x1C\xDAWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FServiceManagerBase.onlyRegistryC`D\x82\x01R\x7Foordinator: caller is not the re`d\x82\x01Rq3\xB4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B`\x84\x82\x01R`\xA4\x90\xFD[a\x1CB3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1C\xD3V[\x91\x90\x82\x01\x80\x92\x11a\x0C\x12WV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x02\x13WV[\x90\x81`@\x91\x03\x12a\x02\x13W`@Q\x90`@\x82\x01\x90\x82\x82\x10`\x01`\x01`@\x1B\x03\x83\x11\x17a\x08QW` \x91`@R\x80Qa\x1D\xEE\x81a\x08wV[\x83R\x01Qa\x1D\xFB\x81a\x1D\xA1V[` \x82\x01R\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x05$Wa\x1E\x82\x91` \x91_\x91a!\x1FWP`@Q\x80\x93\x81\x92c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x85Z\xFA\x90\x81\x15a\x05$W_\x91a!\0W[P`\x01`\x01`\xC0\x1B\x03\x16\x90\x81\x15\x90\x81\x15a \xADW[Pa \xA4Wa\x1E\xB8\x90a(\xC8V[_\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x83[\x81Q\x85\x10\x15a\x1FpWa\x1F)` a\x1F\x06a\x07>a\x070\x89\x87a\x1BjV[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x87Z\xFA\x80\x15a\x05$W`\x01\x92a\x1FH\x92_\x92a\x1FPW[Pa\x1D\x94V[\x94\x01\x93a\x1E\xE8V[a\x1Fi\x91\x92P` =\x81\x11a\x07\xD6Wa\x07\xC8\x81\x83a\x08VV[\x90_a\x1FBV[a\x1F{\x91\x94Pa\x19\xD3V[\x92_\x90_[\x81Q\x81\x10\x15a \x9EWa\x1F\x99a\x07>a\x070\x83\x85a\x1BjV[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x90` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x05$W_\x92a ~W[P\x90_\x91[\x81\x83\x10a\x1F\xDEWPPP`\x01\x01a\x1F\x80V[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x05$Wa B\x8Ba\x04\xCC\x83a <a\x05\x85`\x01\x98a G\x98_\x91a PW[PQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92a\x1A\xCCV[a\x1A\xE5V[\x95\x01\x91\x90a\x1F\xCCV[a q\x91P`@=\x81\x11a wW[a i\x81\x83a\x08VV[\x81\x01\x90a\x1D\xB7V[_a .V[P=a _V[a \x97\x91\x92P` =\x81\x11a\x07\xD6Wa\x07\xC8\x81\x83a\x08VV[\x90_a\x1F\xC7V[PPPPV[Pa\x02\xB7a\x19\xB8V[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x91P` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x80\x15a\x05$W`\xFF\x91_\x91a \xE1W[P\x16\x15_a\x1E\xAAV[a \xFA\x91P` =` \x11a\x08\x1BWa\x08\r\x81\x83a\x08VV[_a \xD8V[a!\x19\x91P` =` \x11a\x07\xA9Wa\x07\x9B\x81\x83a\x08VV[_a\x1E\x95V[a!6\x91P\x82=\x84\x11a\x07\xD6Wa\x07\xC8\x81\x83a\x08VV[_a\x06\xBBV[\x15a!CWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FServiceManager: Migration Alread`D\x82\x01Rj\x1EH\x11\x9A[\x98[\x1A^\x99Y`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90a!\xA6a)gV[a!\xB5`\xFF`iT\x16\x15a!<V[\x80Q\x83Q\x03a!\xFAW_[\x81Q\x81\x10\x15a!\xF4W\x80a!\xEDa!\xDCa\x05x`\x01\x94\x86a\x1A\xCCV[a!\xE6\x83\x88a\x1A\xCCV[Q\x90a*xV[P\x01a!\xC0V[PP\x90PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FServiceManager: Input array leng`D\x82\x01Rj\x0E\x8D\x04\r\xAD.m\xAC.\x8Cm`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x05$W`\xFF\x91_\x91a$aW[P\x16\x80\x15a$WW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90_\x90\x81\x90[\x80\x83\x10a$\x13WPa\"\xEE\x91Pa\x19\xD3V[\x92_\x90_[`@Qc\x9A\xA1e=`\xE0\x1B\x81R` \x81`\x04\x81\x89Z\xFA\x80\x15a\x05$W`\xFF\x91_\x91a#\xF5W[P\x16\x81\x10\x15a#\xEEW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01\x81\x90R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x05$W_\x92a#\xCEW[P\x90_\x91[\x81\x83\x10a#hWPPP`\x01\x01a\"\xF3V[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x05$Wa B\x8Ba\x04\xCC\x83a <a\x05\x85`\x01\x98a#\xC5\x98_\x91a PWPQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95\x01\x91\x90a#VV[a#\xE7\x91\x92P` =\x81\x11a\x07\xD6Wa\x07\xC8\x81\x83a\x08VV[\x90_a#QV[P\x92PPPV[a$\r\x91P` =\x81\x11a\x08\x1BWa\x08\r\x81\x83a\x08VV[_a#\x19V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x90\x91\x90` \x81`$\x81\x88Z\xFA\x80\x15a\x05$W`\x01\x92a$N\x92_\x92a\x1FPWPa\x1D\x94V[\x92\x01\x91\x90a\"\xDCV[PPa\x02\xB7a\x19\xB8V[a$z\x91P` =` \x11a\x08\x1BWa\x08\r\x81\x83a\x08VV[_a\"\xA4V[\x91\x90\x81\x10\x15a\x1A\xE0W`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x02\x13W\x01\x90V[5a\x02\xB7\x81a\x08wV[\x90\x81` \x91\x03\x12a\x02\x13WQ\x80\x15\x15\x81\x03a\x02\x13W\x90V[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a$\xDDWPPP\x90V[\x90\x91\x92`@\x80`\x01\x92\x865a$\xF1\x81a\x08wV[\x84\x80`\xA0\x1B\x03\x16\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x88\x015a%\x15\x81a\x1D\xA1V[\x16` \x82\x01R\x01\x94\x01\x92\x91\x01a$\xD0V[5\x90a\x1CB\x82a\x08wV[\x92\x80\x91`@\x85\x01\x90`\x01\x80`\xA0\x1B\x03\x16\x85R`@` \x86\x01RR``\x83\x01\x90``\x81`\x05\x1B\x85\x01\x01\x93\x83_\x91`\x9E\x19\x826\x03\x01\x90[\x84\x84\x10a%wWPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`_\x19\x82\x82\x03\x01\x87R\x875\x83\x81\x12\x15a\x02\x13W\x84\x01\x90\x815`\x1E\x19\x836\x03\x01\x81\x12\x15a\x02\x13W\x82\x01\x91` \x835\x93\x01\x90`\x01`\x01`@\x1B\x03\x84\x11a\x02\x13W\x83`\x06\x1B6\x03\x82\x13a\x02\x13Wa&@\x83`\x80a&5\x81a%\xEB` \x98\x97`\x01\x9A`\xA0\x8B\x9AR`\xA0\x87\x01\x91a$\xC4V[\x95a&\ta%\xFA\x89\x83\x01a%&V[`\x01`\x01`\xA0\x1B\x03\x16\x86\x8A\x01RV[`@\x81\x015`@\x86\x01Ra&/a&\"``\x83\x01a\x08\x88V[c\xFF\xFF\xFF\xFF\x16``\x87\x01RV[\x01a\x08\x88V[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x99\x01\x97\x01\x95\x94\x01\x92\x91\x90a%fV[\x91\x90\x82\x03\x91\x82\x11a\x0C\x12WV[\x80Q`\x01\x81\x11\x15a'\x0EW`\x01\x1C\x91a&t\x83a\x19\xD3V[\x91a&\x88a&\x83\x85\x83Qa&OV[a\x19\xD3V[\x91_[\x85\x81\x10a&\xEBWP\x84[\x82Q\x81\x10\x15a&\xCAW\x80a&\xC4a&\xB1a\x05x`\x01\x94\x87a\x1A\xCCV[a\x04\xCCa&\xBE\x8A\x85a&OV[\x88a\x1A\xCCV[\x01a&\x95V[P\x93PP\x90a&\xD8\x90a&\\V[a&\xE2\x90\x91a&\\V[a\x02\xB7\x91a'\x12V[\x80a'\x08a&\xFEa\x05x`\x01\x94\x87a\x1A\xCCV[a\x04\xCC\x83\x89a\x1A\xCCV[\x01a&\x8BV[P\x90V[\x91\x82Q\x92\x82Qa'%a&\x83\x82\x87a\x1D\x94V[\x93_\x93_\x92_\x97[\x80\x87\x10\x80a(\x97W[\x15a(\x18Wa'Ha\x05x\x88\x88a\x1A\xCCV[a'Xa\x05\x85a\x05x\x88\x88a\x1A\xCCV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x10\x15a'\x9AWa'\x95a'\x82a\x05xa'{\x8Aa\x1A\xE5V[\x99\x89a\x1A\xCCV[a\x04\xCCa'\x8E\x8Ca\x1A\xE5V[\x9B\x8Ba\x1A\xCCV[a'-V[a'\xA7a\x05x\x88\x88a\x1A\xCCV[a'\xB7a\x05\x85a\x05x\x88\x88a\x1A\xCCV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x11\x15a'\xE1Wa'\x95a'\x82a\x05xa'\xDA\x88a\x1A\xE5V[\x97\x87a\x1A\xCCV[\x93a(\x12\x90a Ba'\xFFa\x05xa'\xF8\x8Ba\x1A\xE5V[\x9A\x8Aa\x1A\xCCV[a\x04\xCCa(\x0B\x8Da\x1A\xE5V[\x9C\x8Ca\x1A\xCCV[\x93a'-V[\x97\x95\x91\x97\x94\x90\x92\x93\x94[\x80\x83\x10a(mWPPP[\x80\x83\x10a(<WPPP\x81R\x90V[a(ha(Ua\x05xa(N\x86a\x1A\xE5V[\x95\x85a\x1A\xCCV[a\x04\xCCa(a\x87a\x1A\xE5V[\x96\x88a\x1A\xCCV[a(-V[a(\x92a(\x7Fa\x05xa(N\x86a\x1A\xE5V[a\x04\xCCa(\x8B\x8Aa\x1A\xE5V[\x99\x8Ba\x1A\xCCV[a(\"V[P\x81\x85\x10a'6V[\x90a(\xAA\x82a\x08\x99V[a(\xB7`@Q\x91\x82a\x08VV[\x82\x81R\x80\x92a\x19\xFB`\x1F\x19\x91a\x08\x99V[_\x81\x80[a)BWPa(\xDE\x90a\xFF\xFF\x16a(\xA0V[__[\x82Q\x82\x10\x80a)7W[\x15a)0W`\x01\x81\x1B\x84\x16a)\tW[a)\x04\x90a\x1A\xE5V[a(\xE1V[\x90`\x01a)\x04\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa)&\x82\x87a\x1BjV[S\x01\x91\x90Pa(\xFBV[PP\x90P\x90V[Pa\x01\0\x81\x10a(\xEBV[_\x19\x81\x01\x81\x81\x11a\x0C\x12Wa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a\x0C\x12W`\x01\x01\x90\x80a(\xCCV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a){WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`fT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE0\xD4\x9AT'D#\x18=\xAD\xEC\xBD\xF29\xEA\xACn\x06\xBA\x882\x0B&\xFE\x8C\xC5\xEC\x9D\x05\nc\x95\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`fUV[\x15a*$WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FServiceManager: Operator not in `D\x82\x01Requorum`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x92\x91\x90_\x90` \x81`$\x81\x88Z\xFA\x80\x15a\x05$Wa*\xFC\x95` \x92_\x92a+\x85W[P`@Q\x80\x80\x98\x81\x94c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91Z\xFA\x93\x84\x15a\x05$W_\x94a+dW[P_[\x82Q\x81\x10\x15a+^W\x80a+Xa+Sa+=a\x07Da+3`\x01\x96\x89a\x1A\xCCV[Qc\xFF\xFF\xFF\xFF\x16\x90V[\x84\x80`\xC0\x1B\x03\x89\x16`\xFF`\x01\x92\x16\x1C\x16`\x01\x14\x90V[a*\x1DV[\x01a+\x11V[P\x92PPV[a+~\x91\x94P` =` \x11a\x07\xA9Wa\x07\x9B\x81\x83a\x08VV[\x92_a+\x0EV[a+\x9D\x91\x92P\x83=\x85\x11a\x07\xD6Wa\x07\xC8\x81\x83a\x08VV[\x90_a*\xDBV\xFE\xA2dipfsX\"\x12 \xD8}5K\x1F\xA86d,\xE2#\x95\tWNTUU\x02\x89Y}=\x12\x04h\0\xDD\xFC\x1F\xED\xC2dsolcC\0\x08\x1B\x003",
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
    /**Function with signature `createOperatorSet((uint32,address[])[])` and selector `0xe61bbee8`.
```solidity
function createOperatorSet(IAllocationManagerTypes.CreateSetParams[] memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorSetCall {
        pub params: alloy::sol_types::private::Vec<
            <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createOperatorSet((uint32,address[])[])`](createOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorSetReturn {}
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
                    IAllocationManagerTypes::CreateSetParams,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createOperatorSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createOperatorSetCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOperatorSetCall {
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
            impl ::core::convert::From<createOperatorSetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createOperatorSetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createOperatorSetCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::CreateSetParams,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createOperatorSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createOperatorSet((uint32,address[])[])";
            const SELECTOR: [u8; 4] = [230u8, 27u8, 190u8, 232u8];
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
                        IAllocationManagerTypes::CreateSetParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.params),
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
    /**Function with signature `setAvsRegistrar()` and selector `0x72080075`.
```solidity
function setAvsRegistrar() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAvsRegistrarCall {}
    ///Container type for the return parameters of the [`setAvsRegistrar()`](setAvsRegistrarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAvsRegistrarReturn {}
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
            impl ::core::convert::From<setAvsRegistrarCall> for UnderlyingRustTuple<'_> {
                fn from(value: setAvsRegistrarCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAvsRegistrarCall {
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
            impl ::core::convert::From<setAvsRegistrarReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAvsRegistrarReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAvsRegistrarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAvsRegistrarCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAvsRegistrarReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAvsRegistrar()";
            const SELECTOR: [u8; 4] = [114u8, 8u8, 0u8, 117u8];
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
    /**Function with signature `slashOperator((address,uint32,uint256,string))` and selector `0x0ea43e43`.
```solidity
function slashOperator(IAllocationManagerTypes.SlashingParams memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorCall {
        pub params: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`slashOperator((address,uint32,uint256,string))`](slashOperatorCall) function.
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
            const SIGNATURE: &'static str = "slashOperator((address,uint32,uint256,string))";
            const SELECTOR: [u8; 4] = [14u8, 164u8, 62u8, 67u8];
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
        createOperatorSet(createOperatorSetCall),
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
        setAvsRegistrar(setAvsRegistrarCall),
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
            [14u8, 164u8, 62u8, 67u8],
            [21u8, 183u8, 188u8, 154u8],
            [30u8, 33u8, 153u8, 226u8],
            [38u8, 240u8, 23u8, 226u8],
            [51u8, 207u8, 183u8, 183u8],
            [59u8, 194u8, 140u8, 140u8],
            [103u8, 148u8, 12u8, 137u8],
            [107u8, 58u8, 167u8, 46u8],
            [113u8, 80u8, 24u8, 166u8],
            [114u8, 8u8, 0u8, 117u8],
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
            [230u8, 27u8, 190u8, 232u8],
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
        const COUNT: usize = 31usize;
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
                Self::createOperatorSet(_) => {
                    <createOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::setAvsRegistrar(_) => {
                    <setAvsRegistrarCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn setAvsRegistrar(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <setAvsRegistrarCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringServiceManagerCalls::setAvsRegistrar)
                    }
                    setAvsRegistrar
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
                    fn createOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls> {
                        <createOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::createOperatorSet,
                            )
                    }
                    createOperatorSet
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
                Self::createOperatorSet(inner) => {
                    <createOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::setAvsRegistrar(inner) => {
                    <setAvsRegistrarCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::createOperatorSet(inner) => {
                    <createOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setAvsRegistrar(inner) => {
                    <setAvsRegistrarCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        ///Creates a new call builder for the [`createOperatorSet`] function.
        pub fn createOperatorSet(
            &self,
            params: alloy::sol_types::private::Vec<
                <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createOperatorSetCall, N> {
            self.call_builder(&createOperatorSetCall { params })
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
        ///Creates a new call builder for the [`setAvsRegistrar`] function.
        pub fn setAvsRegistrar(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAvsRegistrarCall, N> {
            self.call_builder(&setAvsRegistrarCall {})
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
