///Module containing a contract's types and functions.
/**

```solidity
library IAllocationManagerTypes {
    struct CreateSetParams { uint32 operatorSetId; address[] strategies; }
    struct SlashingParams { address operator; uint32 operatorSetId; address[] strategies; uint256[] wadsToSlash; string description; }
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
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for CreateSetParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
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
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct SlashingParams { address operator; uint32 operatorSetId; address[] strategies; uint256[] wadsToSlash; string description; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SlashingParams {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub wadsToSlash:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            u32,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
            alloy::sol_types::private::String,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    value.wadsToSlash,
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
                    wadsToSlash: tuple.3,
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.wadsToSlash),
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
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SlashingParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for SlashingParams {
            const NAME: &'static str = "SlashingParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SlashingParams(address operator,uint32 operatorSetId,address[] strategies,uint256[] wadsToSlash,string description)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.wadsToSlash)
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
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wadsToSlash,
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wadsToSlash,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.description,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
    pub struct IAllocationManagerTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
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
        > IAllocationManagerTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IAllocationManagerTypes`](self) contract instance.

        See the [wrapper's documentation](`IAllocationManagerTypesInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
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
        > IAllocationManagerTypesInstance<T, P, N>
    {
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
        > IAllocationManagerTypesInstance<T, P, N>
    {
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for RewardsSubmission {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
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
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components.push(
                    <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_root_type(),
                );
                components.extend(
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    <alloy::sol_types::sol_data::Uint<96> as alloy_sol_types::SolType>::tokenize(
                        &self.multiplier,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for StrategyAndMultiplier {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
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
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
    pub struct IRewardsCoordinatorTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
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
        > IRewardsCoordinatorTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IRewardsCoordinatorTypes`](self) contract instance.

        See the [wrapper's documentation](`IRewardsCoordinatorTypesInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
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
        > IRewardsCoordinatorTypesInstance<T, P, N>
    {
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
        > IRewardsCoordinatorTypesInstance<T, P, N>
    {
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignatureWithSaltAndExpiry> for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithSaltAndExpiry) -> Self {
                (value.signature, value.salt, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignatureWithSaltAndExpiry {
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
        impl alloy_sol_types::private::SolTypeValue<Self> for SignatureWithSaltAndExpiry {
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
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SignatureWithSaltAndExpiry {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
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
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
            f.debug_tuple("ISignatureUtilsInstance")
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
        > ISignatureUtilsInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

        See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
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
        > ISignatureUtilsInstance<T, P, N>
    {
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
        > ISignatureUtilsInstance<T, P, N>
    {
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
        address[] strategies;
        uint256[] wadsToSlash;
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
    function addStrategyToOperatorSet(uint32 operatorSetId, address[] memory strategies) external;
    function allocationManager() external view returns (address);
    function avsDirectory() external view returns (address);
    function createAVSRewardsSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
    function createOperatorSets(IAllocationManagerTypes.CreateSetParams[] memory params) external;
    function deregisterOperatorFromAVS(address operator) external;
    function deregisterOperatorFromOperatorSets(address operator, uint32[] memory operatorSetIds) external;
    function getOperatorRestakedStrategies(address operator) external view returns (address[] memory);
    function getRestakeableStrategies() external view returns (address[] memory);
    function incredibleSquaringTaskManager() external view returns (address);
    function migrationFinalized() external view returns (bool);
    function owner() external view returns (address);
    function proposeNewSlasher(address newSlasher) external;
    function proposedSlasher() external view returns (address);
    function registerOperatorToAVS(address operator, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
    function removeStrategiesFromOperatorSet(uint32 operatorSetId, address[] memory strategies) external;
    function renounceOwnership() external;
    function rewardsInitiator() external view returns (address);
    function setAVSRegistrar(address registrar) external;
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
    "name": "addStrategyToOperatorSet",
    "inputs": [
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
    ],
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
    "name": "removeStrategiesFromOperatorSet",
    "inputs": [
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
    "name": "setAVSRegistrar",
    "inputs": [
      {
        "name": "registrar",
        "type": "address",
        "internalType": "contract IAVSRegistrar"
      }
    ],
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
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "wadsToSlash",
            "type": "uint256[]",
            "internalType": "uint256[]"
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
    ///0x6101403461023957601f6124aa38819003918201601f19168301916001600160401b0383118484101761023d5780849260c094604052833981010312610239578051906001600160a01b03821682036102395760208101516001600160a01b0381168103610239576040820151906001600160a01b03821682036102395760608301516001600160a01b03811690819003610239576080840151936001600160a01b03851685036102395760a00151946001600160a01b03861686036102395760805260a05260c05260e052610100525f5460ff8160081c166101e45760ff808216106101aa575b506101205260405161225890816102528239608051818181610acc01528181610b7901528181610cab0152611129015260a0518161018a015260c051818181610701015281816108f801528181610b4901528181610c8201528181610e9b015281816119320152611e7f015260e051818181611a4c01528181611ecb015261217801526101005181818161073101528181610808015281816108880152818161092b01528181610ec4015281816110bf01526112c0015261012051816110180152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f6100e7565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f5f3560e01c806326f017e2146114fd57806333cfb7b7146114d95780633bc28c8c1461145a5780633d0714221461117657806367940c89146111585780636b3aa72e146111135780636ecbccfe146110a4578063715018a61461104757806377ef731d14611002578063847d634f14610e175780638999817f14610d8e5780638d68349a14610d6b5780638da5cb5b14610d425780639926ee7d14610be1578063a364f4da14610b28578063a98fb35514610a90578063b134427114610a67578063c1a8e2c5146108b7578063ca8aa7c714610872578063ce7b5e4b146107ed578063e46f1816146107c4578063e481af9d14610798578063f25f1610146106d3578063f2fde38b1461060a578063fc299dee146105e1578063fcd1c375146105c35763fce36c7d14610143575f80fd5b3461022d57602036600319011261022d576004356001600160401b0381116105bf576101739036906004016117f4565b6065549192916001600160a01b0316330361053f577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031692825b82811061036e5750833b1561036a576040519163fce36c7d60e01b835280602484016020600486015252604483019060448160051b8501019280928691609e19813603015b84841061023b5788808981818e8183818f03925af180156102305761021c5750f35b816102269161169b565b61022d5780f35b80fd5b6040513d84823e3d90fd5b9091929394956043198882030184528635828112156103665783019060a08101918035601e1982360301811215610310578101602081359101936001600160401b038211610362578160061b3603851361036257819060a085525260c0830193908c905b8082106103145750505060208101356001600160a01b03811690819003610310579282608063ffffffff6102fc8260209796600199898099015260408101356040870152836102f0606083016116bc565b166060870152016116bc565b1691015298019401940192949391906101fa565b8b80fd5b91949091906001600160a01b0361032a8761160f565b16815260208601356bffffffffffffffffffffffff811680910361035e57602082015260409081019501916001019061029f565b8e80fd5b8c80fd5b8980fd5b8280fd5b8360206001600160a01b0361038e826103888689896120d0565b016120f2565b166064604061039e8689896120d0565b013560405194859384926323b872dd60e01b845233600485015230602485015260448401525af180156104de57610523575b506001600160a01b036103e960206103888487876120d0565b604051636eb1769f60e11b81523060048201526001600160a01b03881660248201529160209183916044918391165afa9081156104de5790859182916104e9575b50602090610495906104626001600160a01b0361044c85610388898c8c6120d0565b1691604061045b888b8b6120d0565b0135611824565b60405163095ea7b360e01b81526001600160a01b038b166004820152602481019190915293849283919082906044820190565b03925af180156104de5790600192916104b0575b50016101b5565b6104d09060203d81116104d7575b6104c8818361169b565b810190612106565b505f6104a9565b503d6104be565b6040513d87823e3d90fd5b9150506020813d821161051b575b816105046020938361169b565b810103126105175751849061049561042a565b5f80fd5b3d91506104f7565b61053a9060203d81116104d7576104c8818361169b565b6103d0565b60405162461bcd60e51b815260206004820152604c60248201527f536572766963654d616e61676572426173652e6f6e6c7952657761726473496e60448201527f69746961746f723a2063616c6c6572206973206e6f742074686520726577617260648201526b32399034b734ba34b0ba37b960a11b608482015260a490fd5b5080fd5b503461022d578060031936011261022d576020606854604051908152f35b503461022d578060031936011261022d576065546040516001600160a01b039091168152602090f35b503461022d57602036600319011261022d576106246115f9565b61062c61211e565b6001600160a01b0316801561067f57603380546001600160a01b0319811683179091556001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08380a380f35b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b503461022d57602036600319011261022d57806004356001600160a01b038116908190036107955761072f337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611de3565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156107935782916044839260405194859384926334f65bfd60e21b845230600485015260248401525af180156102305761021c5750f35b505b50fd5b503461022d578060031936011261022d576107c06107b4611e70565b60405191829182611623565b0390f35b503461022d578060031936011261022d576067546040516001600160a01b039091168152602090f35b503461022d57806107fd366117b8565b90610806612176565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561086d5761085c9284928360405180968195829463b66bd98960e01b84523060048501611db9565b03925af180156102305761021c5750f35b505050fd5b503461022d578060031936011261022d576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b503461022d57604036600319011261022d576108d16115f9565b906024356001600160401b0381116105bf576108f19036906004016117f4565b92610926337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611de3565b6040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031694909161095f83611680565b6001600160a01b0316825230602083019081529261097c826116cd565b9161098a604051938461169b565b808352602083019060051b820191368311610a6357905b828210610a4b5750505060408201908152843b15610a4757604051636e3492b560e01b815260206004820181905292516001600160a01b03908116602483015293519093166044840152516060606484015280516084840181905260a48401929190910190845b818110610a2b57505050818394818581819503925af180156102305761021c5750f35b825163ffffffff16845260209384019390920191600101610a08565b8380fd5b60208091610a58846116bc565b8152019101906109a1565b8680fd5b503461022d578060031936011261022d576066546040516001600160a01b039091168152602090f35b503461022d57602036600319011261022d57806004356001600160401b03811161079557610ac290369060040161179a565b610aca61211e565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156107935760405163a98fb35560e01b815260206004820152918391839182908490829061085c906024830190611d95565b503461022d57602036600319011261022d5780610b436115f9565b610b77337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611de3565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610793576040516351b27a6d60e11b81526001600160a01b0390911660048201529082908290602490829084905af180156102305761021c5750f35b503461022d57604036600319011261022d5780610bfc6115f9565b602435906001600160401b03821161079357606060031983360301126107935760405190610c2982611680565b82600401356001600160401b038111610d3e57830136602382011215610d3e57610c5d903690602460048201359101611764565b82526020820191602484013583526044604082019401358452610ca960018060a01b037f0000000000000000000000000000000000000000000000000000000000000000163314611de3565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610d3a5785610d1c93819560405197889687958694639926ee7d60e01b865260018060a01b0316600486015260406024860152516060604486015260a4850190611d95565b9151606484015251608483015203925af180156102305761021c5750f35b8580fd5b8480fd5b503461022d578060031936011261022d576033546040516001600160a01b039091168152602090f35b503461022d578060031936011261022d57602060ff606954166040519015158152f35b503461022d57602036600319011261022d577f2f8afc8a78fd958f3301c0233aa326b9c4b9a2884a7483227d6b0555aaa03adb610e11610dcc6115f9565b610dd461211e565b606780546001600160a01b0319166001600160a01b0392909216918217905542606881905560408051928352602083019190915290918291820190565b0390a180f35b503461022d57602036600319011261022d57600435906001600160401b03821161022d573660238301121561022d578160040135610e54816116cd565b92610e62604051948561169b565b81845260208401906024829360051b82010190368211610d3e5760248101925b828410610f8c5750610ec2925050506001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163314611de3565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561036a5760405191630130fc2760e51b835260448301943060048501526040602485015251809552606483019460648160051b8501019190855b818110610f4a57868087818180890381838c5af180156102305761021c5750f35b909192602080610f7d6001936063198a8203018c52604083895163ffffffff815116845201519181858201520190611d59565b95019801910196919096610f29565b83356001600160401b038111610a6357820160406023198236030112610a635760405190610fb982611665565b610fc5602482016116bc565b825260448101356001600160401b038111610ffe5791610fef6020949260248695369201016116e4565b83820152815201930192610e82565b8880fd5b503461022d578060031936011261022d576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b503461022d578060031936011261022d5761106061211e565b603380546001600160a01b0319811690915581906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b503461022d57806110b4366117b8565b906110bd612176565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561086d5761085c92849283604051809681958294630287f75160e51b84523060048501611db9565b503461022d578060031936011261022d576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b503461022d578060031936011261022d57602060405162093a808152f35b5034610517576020366003190112610517576004356001600160401b0381116105175760a06003198236030112610517576040519060a082018281106001600160401b03821117611446576040526111d08160040161160f565b82526111de602482016116bc565b906020830191825260448101356001600160401b0381116105175761120990600436918401016116e4565b916040840192835260648201356001600160401b038111610517578201913660238401121561051757600483013592611241846116cd565b9361124f604051958661169b565b808552602060048187019260051b840101019136831161051757602401905b82821061143657505050606085019283526084810135906001600160401b0382116105175760046112a2923692010161179a565b608085019081526066549091906001600160a01b031633036113cb577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031693843b156105175760408051633635205760e01b8152306004820152602481019190915295516001600160a01b03166044870152905163ffffffff1660648601525160a06084860152849291906113439060e4850190611d59565b9151916043198482030160a4850152602080845192838152019301905f5b8181106113b2575050505f836113878296948294516043198483030160c4850152611d95565b03925af180156113a757611399575080f35b6113a591505f9061169b565b005b6040513d5f823e3d90fd5b8251855287955060209485019490920191600101611361565b60405162461bcd60e51b815260206004820152603960248201527f536572766963654d616e61676572426173652e6f6e6c79536c61736865723a2060448201527f63616c6c6572206973206e6f742074686520736c6173686572000000000000006064820152608490fd5b813581526020918201910161126e565b634e487b7160e01b5f52604160045260245ffd5b34610517576020366003190112610517576114736115f9565b61147b61211e565b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555005b34610517576020366003190112610517576107c06107b46114f86115f9565b611912565b34610517575f3660031901126105175761151561211e565b60685462093a8081018091116115e557421061158957606754606654604080516001600160a01b03808416825284166020820181905292917fe0d49a54274423183dadecbdf239eaac6e06ba88320b26fe8cc5ec9d050a639591a16001600160a01b03199081169190911760665516606755005b60405162461bcd60e51b815260206004820152602e60248201527f536572766963654d616e616765723a20536c61736865722070726f706f73616c60448201526d0819195b185e481b9bdd081b595d60921b6064820152608490fd5b634e487b7160e01b5f52601160045260245ffd5b600435906001600160a01b038216820361051757565b35906001600160a01b038216820361051757565b60206040818301928281528451809452019201905f5b8181106116465750505090565b82516001600160a01b0316845260209384019390920191600101611639565b604081019081106001600160401b0382111761144657604052565b606081019081106001600160401b0382111761144657604052565b90601f801991011681019081106001600160401b0382111761144657604052565b359063ffffffff8216820361051757565b6001600160401b0381116114465760051b60200190565b9080601f830112156105175781356116fb816116cd565b92611709604051948561169b565b81845260208085019260051b82010192831161051757602001905b8282106117315750505090565b6020809161173e8461160f565b815201910190611724565b6001600160401b03811161144657601f01601f191660200190565b92919261177082611749565b9161177e604051938461169b565b829481845281830111610517578281602093845f960137010152565b9080601f83011215610517578160206117b593359101611764565b90565b9060406003198301126105175760043563ffffffff811681036105175791602435906001600160401b038211610517576117b5916004016116e4565b9181601f84011215610517578235916001600160401b038311610517576020808501948460051b01011161051757565b919082018092116115e557565b90816020910312610517575160ff811681036105175790565b90611854826116cd565b611861604051918261169b565b8281528092611872601f19916116cd565b0190602036910137565b90815181101561188d570160200190565b634e487b7160e01b5f52603260045260245ffd5b9081604091031261051757604051906118b982611665565b8051906001600160a01b038216820361051757602091835201516bffffffffffffffffffffffff8116810361051757602082015290565b805182101561188d5760209160051b010190565b5f1981146115e55760010190565b6040516309aa152760e11b81526001600160a01b039182166004820152907f000000000000000000000000000000000000000000000000000000000000000016602082602481845afa9182156113a7575f92611d25575b506040519163871ef04960e01b83526004830152602082602481845afa9182156113a7575f92611ce1575b506001600160c01b038216908115908115611c7e575b50611c6457805f915b611c3f575061ffff166119c581611749565b906119d3604051928361169b565b8082526119e2601f1991611749565b013660208301375f5f5b8251821080611c34575b15611a44576001811b84166001600160c01b0316611a1d575b611a1890611904565b6119ec565b906001611a189160ff60f81b8460f81b165f1a611a3a828761187c565b5301919050611a0f565b505f939250507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690835b8151851015611aff57611a8a858361187c565b5160f81c60405190633ca5a5f560e01b82526004820152602081602481875afa9081156113a7575f91611acd575b50611ac590600192611824565b940193611a77565b90506020813d8211611af7575b81611ae76020938361169b565b8101031261051757516001611ab8565b3d9150611ada565b611b0a91945061184a565b925f905f5b8151811015611c2e57611b22818361187c565b5160f81c60405190633ca5a5f560e01b8252806004830152602082602481895afa9182156113a7575f92611bfb575b50905f915b818310611b6857505050600101611b0f565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156113a757600192611bc4925f91611bcd575b50838060a01b03905116611bbe828d6118f0565b52611904565b95019190611b56565b611bee915060403d8111611bf4575b611be6818361169b565b8101906118a1565b5f611baa565b503d611bdc565b9091506020813d8211611c26575b81611c166020938361169b565b810103126105175751905f611b51565b3d9150611c09565b50505050565b5061010081106119f6565b5f1981018181116115e55761ffff9116911661ffff81146115e55760010190806119b3565b5050604051611c7460208261169b565b5f81525f36813790565b604051639aa1653d60e01b81529150602090829060049082905afa80156113a75760ff915f91611cb2575b5016155f6119aa565b611cd4915060203d602011611cda575b611ccc818361169b565b810190611831565b5f611ca9565b503d611cc2565b9091506020813d602011611d1d575b81611cfd6020938361169b565b8101031261051757516001600160c01b038116810361051757905f611994565b3d9150611cf0565b9091506020813d602011611d51575b81611d416020938361169b565b810103126105175751905f611969565b3d9150611d34565b90602080835192838152019201905f5b818110611d765750505090565b82516001600160a01b0316845260209384019390920191600101611d69565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b63ffffffff6117b5949360609360018060a01b031683521660208201528160408201520190611d59565b15611dea57565b60405162461bcd60e51b815260206004820152605260248201527f536572766963654d616e61676572426173652e6f6e6c7952656769737472794360448201527f6f6f7264696e61746f723a2063616c6c6572206973206e6f742074686520726560648201527133b4b9ba393c9031b7b7b93234b730ba37b960711b608482015260a490fd5b604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156113a75760ff915f916120b1575b50168015611c64577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316905f9081905b80831061203b5750611f0b915061184a565b925f905f5b604051639aa1653d60e01b8152602081600481895afa80156113a75760ff915f9161201d575b501681101561201657604051633ca5a5f560e01b815260ff821660048201819052602082602481895afa9182156113a7575f92611fe3575b50905f915b818310611f8557505050600101611f10565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156113a757600192611fda925f91611bcd5750838060a01b03905116611bbe828d6118f0565b95019190611f73565b9091506020813d821161200e575b81611ffe6020938361169b565b810103126105175751905f611f6e565b3d9150611ff1565b5092505050565b612035915060203d8111611cda57611ccc818361169b565b5f611f36565b90604051633ca5a5f560e01b815260ff84166004820152602081602481885afa9081156113a7575f9161207f575b5061207690600192611824565b92019190611ef9565b90506020813d82116120a9575b816120996020938361169b565b8101031261051757516001612069565b3d915061208c565b6120ca915060203d602011611cda57611ccc818361169b565b5f611ec1565b919081101561188d5760051b81013590609e1981360301821215610517570190565b356001600160a01b03811681036105175790565b90816020910312610517575180151581036105175790565b6033546001600160a01b0316330361213257565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036121a857565b60405162461bcd60e51b815260206004820152604660248201527f536572766963654d616e61676572426173652e6f6e6c795374616b655265676960448201527f737472793a2063616c6c6572206973206e6f7420746865207374616b6520726560648201526567697374727960d01b608482015260a490fdfea26469706673582212204d9a395fd2d7be3715f8c3b3d9264e68488d03ce1e0dbbabf108d7e6cb85e43064736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01@4a\x029W`\x1Fa$\xAA8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x02=W\x80\x84\x92`\xC0\x94`@R\x839\x81\x01\x03\x12a\x029W\x80Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x029W` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x029W`@\x82\x01Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x029W``\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x029W`\x80\x84\x01Q\x93`\x01`\x01`\xA0\x1B\x03\x85\x16\x85\x03a\x029W`\xA0\x01Q\x94`\x01`\x01`\xA0\x1B\x03\x86\x16\x86\x03a\x029W`\x80R`\xA0R`\xC0R`\xE0Ra\x01\0R_T`\xFF\x81`\x08\x1C\x16a\x01\xE4W`\xFF\x80\x82\x16\x10a\x01\xAAW[Pa\x01 R`@Qa\"X\x90\x81a\x02R\x829`\x80Q\x81\x81\x81a\n\xCC\x01R\x81\x81a\x0By\x01R\x81\x81a\x0C\xAB\x01Ra\x11)\x01R`\xA0Q\x81a\x01\x8A\x01R`\xC0Q\x81\x81\x81a\x07\x01\x01R\x81\x81a\x08\xF8\x01R\x81\x81a\x0BI\x01R\x81\x81a\x0C\x82\x01R\x81\x81a\x0E\x9B\x01R\x81\x81a\x192\x01Ra\x1E\x7F\x01R`\xE0Q\x81\x81\x81a\x1AL\x01R\x81\x81a\x1E\xCB\x01Ra!x\x01Ra\x01\0Q\x81\x81\x81a\x071\x01R\x81\x81a\x08\x08\x01R\x81\x81a\x08\x88\x01R\x81\x81a\t+\x01R\x81\x81a\x0E\xC4\x01R\x81\x81a\x10\xBF\x01Ra\x12\xC0\x01Ra\x01 Q\x81a\x10\x18\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\0\xE7V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c&\xF0\x17\xE2\x14a\x14\xFDW\x80c3\xCF\xB7\xB7\x14a\x14\xD9W\x80c;\xC2\x8C\x8C\x14a\x14ZW\x80c=\x07\x14\"\x14a\x11vW\x80cg\x94\x0C\x89\x14a\x11XW\x80ck:\xA7.\x14a\x11\x13W\x80cn\xCB\xCC\xFE\x14a\x10\xA4W\x80cqP\x18\xA6\x14a\x10GW\x80cw\xEFs\x1D\x14a\x10\x02W\x80c\x84}cO\x14a\x0E\x17W\x80c\x89\x99\x81\x7F\x14a\r\x8EW\x80c\x8Dh4\x9A\x14a\rkW\x80c\x8D\xA5\xCB[\x14a\rBW\x80c\x99&\xEE}\x14a\x0B\xE1W\x80c\xA3d\xF4\xDA\x14a\x0B(W\x80c\xA9\x8F\xB3U\x14a\n\x90W\x80c\xB14Bq\x14a\ngW\x80c\xC1\xA8\xE2\xC5\x14a\x08\xB7W\x80c\xCA\x8A\xA7\xC7\x14a\x08rW\x80c\xCE{^K\x14a\x07\xEDW\x80c\xE4o\x18\x16\x14a\x07\xC4W\x80c\xE4\x81\xAF\x9D\x14a\x07\x98W\x80c\xF2_\x16\x10\x14a\x06\xD3W\x80c\xF2\xFD\xE3\x8B\x14a\x06\nW\x80c\xFC)\x9D\xEE\x14a\x05\xE1W\x80c\xFC\xD1\xC3u\x14a\x05\xC3Wc\xFC\xE3l}\x14a\x01CW_\x80\xFD[4a\x02-W` 6`\x03\x19\x01\x12a\x02-W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x05\xBFWa\x01s\x906\x90`\x04\x01a\x17\xF4V[`eT\x91\x92\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a\x05?W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92\x82[\x82\x81\x10a\x03nWP\x83;\x15a\x03jW`@Q\x91c\xFC\xE3l}`\xE0\x1B\x83R\x80`$\x84\x01` `\x04\x86\x01RR`D\x83\x01\x90`D\x81`\x05\x1B\x85\x01\x01\x92\x80\x92\x86\x91`\x9E\x19\x816\x03\x01[\x84\x84\x10a\x02;W\x88\x80\x89\x81\x81\x8E\x81\x83\x81\x8F\x03\x92Z\xF1\x80\x15a\x020Wa\x02\x1CWP\xF3[\x81a\x02&\x91a\x16\x9BV[a\x02-W\x80\xF3[\x80\xFD[`@Q=\x84\x82>=\x90\xFD[\x90\x91\x92\x93\x94\x95`C\x19\x88\x82\x03\x01\x84R\x865\x82\x81\x12\x15a\x03fW\x83\x01\x90`\xA0\x81\x01\x91\x805`\x1E\x19\x826\x03\x01\x81\x12\x15a\x03\x10W\x81\x01` \x815\x91\x01\x93`\x01`\x01`@\x1B\x03\x82\x11a\x03bW\x81`\x06\x1B6\x03\x85\x13a\x03bW\x81\x90`\xA0\x85RR`\xC0\x83\x01\x93\x90\x8C\x90[\x80\x82\x10a\x03\x14WPPP` \x81\x015`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x03\x10W\x92\x82`\x80c\xFF\xFF\xFF\xFFa\x02\xFC\x82` \x97\x96`\x01\x99\x89\x80\x99\x01R`@\x81\x015`@\x87\x01R\x83a\x02\xF0``\x83\x01a\x16\xBCV[\x16``\x87\x01R\x01a\x16\xBCV[\x16\x91\x01R\x98\x01\x94\x01\x94\x01\x92\x94\x93\x91\x90a\x01\xFAV[\x8B\x80\xFD[\x91\x94\x90\x91\x90`\x01`\x01`\xA0\x1B\x03a\x03*\x87a\x16\x0FV[\x16\x81R` \x86\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\x03^W` \x82\x01R`@\x90\x81\x01\x95\x01\x91`\x01\x01\x90a\x02\x9FV[\x8E\x80\xFD[\x8C\x80\xFD[\x89\x80\xFD[\x82\x80\xFD[\x83` `\x01`\x01`\xA0\x1B\x03a\x03\x8E\x82a\x03\x88\x86\x89\x89a \xD0V[\x01a \xF2V[\x16`d`@a\x03\x9E\x86\x89\x89a \xD0V[\x015`@Q\x94\x85\x93\x84\x92c#\xB8r\xDD`\xE0\x1B\x84R3`\x04\x85\x01R0`$\x85\x01R`D\x84\x01RZ\xF1\x80\x15a\x04\xDEWa\x05#W[P`\x01`\x01`\xA0\x1B\x03a\x03\xE9` a\x03\x88\x84\x87\x87a \xD0V[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16`$\x82\x01R\x91` \x91\x83\x91`D\x91\x83\x91\x16Z\xFA\x90\x81\x15a\x04\xDEW\x90\x85\x91\x82\x91a\x04\xE9W[P` \x90a\x04\x95\x90a\x04b`\x01`\x01`\xA0\x1B\x03a\x04L\x85a\x03\x88\x89\x8C\x8Ca \xD0V[\x16\x91`@a\x04[\x88\x8B\x8Ba \xD0V[\x015a\x18$V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x93\x84\x92\x83\x91\x90\x82\x90`D\x82\x01\x90V[\x03\x92Z\xF1\x80\x15a\x04\xDEW\x90`\x01\x92\x91a\x04\xB0W[P\x01a\x01\xB5V[a\x04\xD0\x90` =\x81\x11a\x04\xD7W[a\x04\xC8\x81\x83a\x16\x9BV[\x81\x01\x90a!\x06V[P_a\x04\xA9V[P=a\x04\xBEV[`@Q=\x87\x82>=\x90\xFD[\x91PP` \x81=\x82\x11a\x05\x1BW[\x81a\x05\x04` \x93\x83a\x16\x9BV[\x81\x01\x03\x12a\x05\x17WQ\x84\x90a\x04\x95a\x04*V[_\x80\xFD[=\x91Pa\x04\xF7V[a\x05:\x90` =\x81\x11a\x04\xD7Wa\x04\xC8\x81\x83a\x16\x9BV[a\x03\xD0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FServiceManagerBase.onlyRewardsIn`D\x82\x01R\x7Fitiator: caller is not the rewar`d\x82\x01Rk29\x904\xB74\xBA4\xB0\xBA7\xB9`\xA1\x1B`\x84\x82\x01R`\xA4\x90\xFD[P\x80\xFD[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W` `hT`@Q\x90\x81R\xF3[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x02-W` 6`\x03\x19\x01\x12a\x02-Wa\x06$a\x15\xF9V[a\x06,a!\x1EV[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x06\x7FW`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x90\x91U`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x83\x80\xA3\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[P4a\x02-W` 6`\x03\x19\x01\x12a\x02-W\x80`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x07\x95Wa\x07/3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1D\xE3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x07\x93W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c4\xF6[\xFD`\xE2\x1B\x84R0`\x04\x85\x01R`$\x84\x01RZ\xF1\x80\x15a\x020Wa\x02\x1CWP\xF3[P[P\xFD[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-Wa\x07\xC0a\x07\xB4a\x1EpV[`@Q\x91\x82\x91\x82a\x16#V[\x03\x90\xF3[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W`gT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x02-W\x80a\x07\xFD6a\x17\xB8V[\x90a\x08\x06a!vV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x08mWa\x08\\\x92\x84\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xB6k\xD9\x89`\xE0\x1B\x84R0`\x04\x85\x01a\x1D\xB9V[\x03\x92Z\xF1\x80\x15a\x020Wa\x02\x1CWP\xF3[PPP\xFD[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x02-W`@6`\x03\x19\x01\x12a\x02-Wa\x08\xD1a\x15\xF9V[\x90`$5`\x01`\x01`@\x1B\x03\x81\x11a\x05\xBFWa\x08\xF1\x906\x90`\x04\x01a\x17\xF4V[\x92a\t&3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1D\xE3V[`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x94\x90\x91a\t_\x83a\x16\x80V[`\x01`\x01`\xA0\x1B\x03\x16\x82R0` \x83\x01\x90\x81R\x92a\t|\x82a\x16\xCDV[\x91a\t\x8A`@Q\x93\x84a\x16\x9BV[\x80\x83R` \x83\x01\x90`\x05\x1B\x82\x01\x916\x83\x11a\ncW\x90[\x82\x82\x10a\nKWPPP`@\x82\x01\x90\x81R\x84;\x15a\nGW`@Qcn4\x92\xB5`\xE0\x1B\x81R` `\x04\x82\x01\x81\x90R\x92Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`$\x83\x01R\x93Q\x90\x93\x16`D\x84\x01RQ```d\x84\x01R\x80Q`\x84\x84\x01\x81\x90R`\xA4\x84\x01\x92\x91\x90\x91\x01\x90\x84[\x81\x81\x10a\n+WPPP\x81\x83\x94\x81\x85\x81\x81\x95\x03\x92Z\xF1\x80\x15a\x020Wa\x02\x1CWP\xF3[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\n\x08V[\x83\x80\xFD[` \x80\x91a\nX\x84a\x16\xBCV[\x81R\x01\x91\x01\x90a\t\xA1V[\x86\x80\xFD[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W`fT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x02-W` 6`\x03\x19\x01\x12a\x02-W\x80`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x07\x95Wa\n\xC2\x906\x90`\x04\x01a\x17\x9AV[a\n\xCAa!\x1EV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x07\x93W`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R` `\x04\x82\x01R\x91\x83\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x08\\\x90`$\x83\x01\x90a\x1D\x95V[P4a\x02-W` 6`\x03\x19\x01\x12a\x02-W\x80a\x0BCa\x15\xF9V[a\x0Bw3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1D\xE3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x07\x93W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x82\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x020Wa\x02\x1CWP\xF3[P4a\x02-W`@6`\x03\x19\x01\x12a\x02-W\x80a\x0B\xFCa\x15\xF9V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x07\x93W```\x03\x19\x836\x03\x01\x12a\x07\x93W`@Q\x90a\x0C)\x82a\x16\x80V[\x82`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\r>W\x83\x016`#\x82\x01\x12\x15a\r>Wa\x0C]\x906\x90`$`\x04\x82\x015\x91\x01a\x17dV[\x82R` \x82\x01\x91`$\x84\x015\x83R`D`@\x82\x01\x94\x015\x84Ra\x0C\xA9`\x01\x80`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x14a\x1D\xE3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\r:W\x85a\r\x1C\x93\x81\x95`@Q\x97\x88\x96\x87\x95\x86\x94c\x99&\xEE}`\xE0\x1B\x86R`\x01\x80`\xA0\x1B\x03\x16`\x04\x86\x01R`@`$\x86\x01RQ```D\x86\x01R`\xA4\x85\x01\x90a\x1D\x95V[\x91Q`d\x84\x01RQ`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\x020Wa\x02\x1CWP\xF3[\x85\x80\xFD[\x84\x80\xFD[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W` `\xFF`iT\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x02-W` 6`\x03\x19\x01\x12a\x02-W\x7F/\x8A\xFC\x8Ax\xFD\x95\x8F3\x01\xC0#:\xA3&\xB9\xC4\xB9\xA2\x88Jt\x83\"}k\x05U\xAA\xA0:\xDBa\x0E\x11a\r\xCCa\x15\xF9V[a\r\xD4a!\x1EV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90UB`h\x81\x90U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1\x80\xF3[P4a\x02-W` 6`\x03\x19\x01\x12a\x02-W`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02-W6`#\x83\x01\x12\x15a\x02-W\x81`\x04\x015a\x0ET\x81a\x16\xCDV[\x92a\x0Eb`@Q\x94\x85a\x16\x9BV[\x81\x84R` \x84\x01\x90`$\x82\x93`\x05\x1B\x82\x01\x01\x906\x82\x11a\r>W`$\x81\x01\x92[\x82\x84\x10a\x0F\x8CWPa\x0E\xC2\x92PPP`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x14a\x1D\xE3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03jW`@Q\x91c\x010\xFC'`\xE5\x1B\x83R`D\x83\x01\x940`\x04\x85\x01R`@`$\x85\x01RQ\x80\x95R`d\x83\x01\x94`d\x81`\x05\x1B\x85\x01\x01\x91\x90\x85[\x81\x81\x10a\x0FJW\x86\x80\x87\x81\x81\x80\x89\x03\x81\x83\x8CZ\xF1\x80\x15a\x020Wa\x02\x1CWP\xF3[\x90\x91\x92` \x80a\x0F}`\x01\x93`c\x19\x8A\x82\x03\x01\x8CR`@\x83\x89Qc\xFF\xFF\xFF\xFF\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a\x1DYV[\x95\x01\x98\x01\x91\x01\x96\x91\x90\x96a\x0F)V[\x835`\x01`\x01`@\x1B\x03\x81\x11a\ncW\x82\x01`@`#\x19\x826\x03\x01\x12a\ncW`@Q\x90a\x0F\xB9\x82a\x16eV[a\x0F\xC5`$\x82\x01a\x16\xBCV[\x82R`D\x81\x015`\x01`\x01`@\x1B\x03\x81\x11a\x0F\xFEW\x91a\x0F\xEF` \x94\x92`$\x86\x956\x92\x01\x01a\x16\xE4V[\x83\x82\x01R\x81R\x01\x93\x01\x92a\x0E\x82V[\x88\x80\xFD[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-Wa\x10`a!\x1EV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[P4a\x02-W\x80a\x10\xB46a\x17\xB8V[\x90a\x10\xBDa!vV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x08mWa\x08\\\x92\x84\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\x02\x87\xF7Q`\xE5\x1B\x84R0`\x04\x85\x01a\x1D\xB9V[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W` `@Qb\t:\x80\x81R\xF3[P4a\x05\x17W` 6`\x03\x19\x01\x12a\x05\x17W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x05\x17W`\xA0`\x03\x19\x826\x03\x01\x12a\x05\x17W`@Q\x90`\xA0\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x14FW`@Ra\x11\xD0\x81`\x04\x01a\x16\x0FV[\x82Ra\x11\xDE`$\x82\x01a\x16\xBCV[\x90` \x83\x01\x91\x82R`D\x81\x015`\x01`\x01`@\x1B\x03\x81\x11a\x05\x17Wa\x12\t\x90`\x046\x91\x84\x01\x01a\x16\xE4V[\x91`@\x84\x01\x92\x83R`d\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x05\x17W\x82\x01\x916`#\x84\x01\x12\x15a\x05\x17W`\x04\x83\x015\x92a\x12A\x84a\x16\xCDV[\x93a\x12O`@Q\x95\x86a\x16\x9BV[\x80\x85R` `\x04\x81\x87\x01\x92`\x05\x1B\x84\x01\x01\x01\x916\x83\x11a\x05\x17W`$\x01\x90[\x82\x82\x10a\x146WPPP``\x85\x01\x92\x83R`\x84\x81\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x05\x17W`\x04a\x12\xA2\x926\x92\x01\x01a\x17\x9AV[`\x80\x85\x01\x90\x81R`fT\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x13\xCBW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x93\x84;\x15a\x05\x17W`@\x80Qc65 W`\xE0\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x95Q`\x01`\x01`\xA0\x1B\x03\x16`D\x87\x01R\x90Qc\xFF\xFF\xFF\xFF\x16`d\x86\x01RQ`\xA0`\x84\x86\x01R\x84\x92\x91\x90a\x13C\x90`\xE4\x85\x01\x90a\x1DYV[\x91Q\x91`C\x19\x84\x82\x03\x01`\xA4\x85\x01R` \x80\x84Q\x92\x83\x81R\x01\x93\x01\x90_[\x81\x81\x10a\x13\xB2WPPP_\x83a\x13\x87\x82\x96\x94\x82\x94Q`C\x19\x84\x83\x03\x01`\xC4\x85\x01Ra\x1D\x95V[\x03\x92Z\xF1\x80\x15a\x13\xA7Wa\x13\x99WP\x80\xF3[a\x13\xA5\x91P_\x90a\x16\x9BV[\0[`@Q=_\x82>=\x90\xFD[\x82Q\x85R\x87\x95P` \x94\x85\x01\x94\x90\x92\x01\x91`\x01\x01a\x13aV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FServiceManagerBase.onlySlasher: `D\x82\x01R\x7Fcaller is not the slasher\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x815\x81R` \x91\x82\x01\x91\x01a\x12nV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[4a\x05\x17W` 6`\x03\x19\x01\x12a\x05\x17Wa\x14sa\x15\xF9V[a\x14{a!\x1EV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`eU\0[4a\x05\x17W` 6`\x03\x19\x01\x12a\x05\x17Wa\x07\xC0a\x07\xB4a\x14\xF8a\x15\xF9V[a\x19\x12V[4a\x05\x17W_6`\x03\x19\x01\x12a\x05\x17Wa\x15\x15a!\x1EV[`hTb\t:\x80\x81\x01\x80\x91\x11a\x15\xE5WB\x10a\x15\x89W`gT`fT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01\x81\x90R\x92\x91\x7F\xE0\xD4\x9AT'D#\x18=\xAD\xEC\xBD\xF29\xEA\xACn\x06\xBA\x882\x0B&\xFE\x8C\xC5\xEC\x9D\x05\nc\x95\x91\xA1`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x91\x90\x91\x17`fU\x16`gU\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FServiceManager: Slasher proposal`D\x82\x01Rm\x08\x19\x19[\x18^H\x1B\x9B\xDD\x08\x1BY]`\x92\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x05\x17WV[5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x05\x17WV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x16FWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x169V[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x14FW`@RV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x14FW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x14FW`@RV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x05\x17WV[`\x01`\x01`@\x1B\x03\x81\x11a\x14FW`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x05\x17W\x815a\x16\xFB\x81a\x16\xCDV[\x92a\x17\t`@Q\x94\x85a\x16\x9BV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x05\x17W` \x01\x90[\x82\x82\x10a\x171WPPP\x90V[` \x80\x91a\x17>\x84a\x16\x0FV[\x81R\x01\x91\x01\x90a\x17$V[`\x01`\x01`@\x1B\x03\x81\x11a\x14FW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x17p\x82a\x17IV[\x91a\x17~`@Q\x93\x84a\x16\x9BV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x05\x17W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x05\x17W\x81` a\x17\xB5\x935\x91\x01a\x17dV[\x90V[\x90`@`\x03\x19\x83\x01\x12a\x05\x17W`\x045c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x05\x17W\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x05\x17Wa\x17\xB5\x91`\x04\x01a\x16\xE4V[\x91\x81`\x1F\x84\x01\x12\x15a\x05\x17W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x05\x17W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x05\x17WV[\x91\x90\x82\x01\x80\x92\x11a\x15\xE5WV[\x90\x81` \x91\x03\x12a\x05\x17WQ`\xFF\x81\x16\x81\x03a\x05\x17W\x90V[\x90a\x18T\x82a\x16\xCDV[a\x18a`@Q\x91\x82a\x16\x9BV[\x82\x81R\x80\x92a\x18r`\x1F\x19\x91a\x16\xCDV[\x01\x90` 6\x91\x017V[\x90\x81Q\x81\x10\x15a\x18\x8DW\x01` \x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90\x81`@\x91\x03\x12a\x05\x17W`@Q\x90a\x18\xB9\x82a\x16eV[\x80Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x05\x17W` \x91\x83R\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x05\x17W` \x82\x01R\x90V[\x80Q\x82\x10\x15a\x18\x8DW` \x91`\x05\x1B\x01\x01\x90V[_\x19\x81\x14a\x15\xE5W`\x01\x01\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x82`$\x81\x84Z\xFA\x91\x82\x15a\x13\xA7W_\x92a\x1D%W[P`@Q\x91c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01R` \x82`$\x81\x84Z\xFA\x91\x82\x15a\x13\xA7W_\x92a\x1C\xE1W[P`\x01`\x01`\xC0\x1B\x03\x82\x16\x90\x81\x15\x90\x81\x15a\x1C~W[Pa\x1CdW\x80_\x91[a\x1C?WPa\xFF\xFF\x16a\x19\xC5\x81a\x17IV[\x90a\x19\xD3`@Q\x92\x83a\x16\x9BV[\x80\x82Ra\x19\xE2`\x1F\x19\x91a\x17IV[\x016` \x83\x017__[\x82Q\x82\x10\x80a\x1C4W[\x15a\x1ADW`\x01\x81\x1B\x84\x16`\x01`\x01`\xC0\x1B\x03\x16a\x1A\x1DW[a\x1A\x18\x90a\x19\x04V[a\x19\xECV[\x90`\x01a\x1A\x18\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa\x1A:\x82\x87a\x18|V[S\x01\x91\x90Pa\x1A\x0FV[P_\x93\x92PP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x83[\x81Q\x85\x10\x15a\x1A\xFFWa\x1A\x8A\x85\x83a\x18|V[Q`\xF8\x1C`@Q\x90c<\xA5\xA5\xF5`\xE0\x1B\x82R`\x04\x82\x01R` \x81`$\x81\x87Z\xFA\x90\x81\x15a\x13\xA7W_\x91a\x1A\xCDW[Pa\x1A\xC5\x90`\x01\x92a\x18$V[\x94\x01\x93a\x1AwV[\x90P` \x81=\x82\x11a\x1A\xF7W[\x81a\x1A\xE7` \x93\x83a\x16\x9BV[\x81\x01\x03\x12a\x05\x17WQ`\x01a\x1A\xB8V[=\x91Pa\x1A\xDAV[a\x1B\n\x91\x94Pa\x18JV[\x92_\x90_[\x81Q\x81\x10\x15a\x1C.Wa\x1B\"\x81\x83a\x18|V[Q`\xF8\x1C`@Q\x90c<\xA5\xA5\xF5`\xE0\x1B\x82R\x80`\x04\x83\x01R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x13\xA7W_\x92a\x1B\xFBW[P\x90_\x91[\x81\x83\x10a\x1BhWPPP`\x01\x01a\x1B\x0FV[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x13\xA7W`\x01\x92a\x1B\xC4\x92_\x91a\x1B\xCDW[P\x83\x80`\xA0\x1B\x03\x90Q\x16a\x1B\xBE\x82\x8Da\x18\xF0V[Ra\x19\x04V[\x95\x01\x91\x90a\x1BVV[a\x1B\xEE\x91P`@=\x81\x11a\x1B\xF4W[a\x1B\xE6\x81\x83a\x16\x9BV[\x81\x01\x90a\x18\xA1V[_a\x1B\xAAV[P=a\x1B\xDCV[\x90\x91P` \x81=\x82\x11a\x1C&W[\x81a\x1C\x16` \x93\x83a\x16\x9BV[\x81\x01\x03\x12a\x05\x17WQ\x90_a\x1BQV[=\x91Pa\x1C\tV[PPPPV[Pa\x01\0\x81\x10a\x19\xF6V[_\x19\x81\x01\x81\x81\x11a\x15\xE5Wa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a\x15\xE5W`\x01\x01\x90\x80a\x19\xB3V[PP`@Qa\x1Ct` \x82a\x16\x9BV[_\x81R_6\x817\x90V[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x91P` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x80\x15a\x13\xA7W`\xFF\x91_\x91a\x1C\xB2W[P\x16\x15_a\x19\xAAV[a\x1C\xD4\x91P` =` \x11a\x1C\xDAW[a\x1C\xCC\x81\x83a\x16\x9BV[\x81\x01\x90a\x181V[_a\x1C\xA9V[P=a\x1C\xC2V[\x90\x91P` \x81=` \x11a\x1D\x1DW[\x81a\x1C\xFD` \x93\x83a\x16\x9BV[\x81\x01\x03\x12a\x05\x17WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x05\x17W\x90_a\x19\x94V[=\x91Pa\x1C\xF0V[\x90\x91P` \x81=` \x11a\x1DQW[\x81a\x1DA` \x93\x83a\x16\x9BV[\x81\x01\x03\x12a\x05\x17WQ\x90_a\x19iV[=\x91Pa\x1D4V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x1DvWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1DiV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[c\xFF\xFF\xFF\xFFa\x17\xB5\x94\x93``\x93`\x01\x80`\xA0\x1B\x03\x16\x83R\x16` \x82\x01R\x81`@\x82\x01R\x01\x90a\x1DYV[\x15a\x1D\xEAWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FServiceManagerBase.onlyRegistryC`D\x82\x01R\x7Foordinator: caller is not the re`d\x82\x01Rq3\xB4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B`\x84\x82\x01R`\xA4\x90\xFD[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x13\xA7W`\xFF\x91_\x91a \xB1W[P\x16\x80\x15a\x1CdW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90_\x90\x81\x90[\x80\x83\x10a ;WPa\x1F\x0B\x91Pa\x18JV[\x92_\x90_[`@Qc\x9A\xA1e=`\xE0\x1B\x81R` \x81`\x04\x81\x89Z\xFA\x80\x15a\x13\xA7W`\xFF\x91_\x91a \x1DW[P\x16\x81\x10\x15a \x16W`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01\x81\x90R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x13\xA7W_\x92a\x1F\xE3W[P\x90_\x91[\x81\x83\x10a\x1F\x85WPPP`\x01\x01a\x1F\x10V[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x13\xA7W`\x01\x92a\x1F\xDA\x92_\x91a\x1B\xCDWP\x83\x80`\xA0\x1B\x03\x90Q\x16a\x1B\xBE\x82\x8Da\x18\xF0V[\x95\x01\x91\x90a\x1FsV[\x90\x91P` \x81=\x82\x11a \x0EW[\x81a\x1F\xFE` \x93\x83a\x16\x9BV[\x81\x01\x03\x12a\x05\x17WQ\x90_a\x1FnV[=\x91Pa\x1F\xF1V[P\x92PPPV[a 5\x91P` =\x81\x11a\x1C\xDAWa\x1C\xCC\x81\x83a\x16\x9BV[_a\x1F6V[\x90`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R` \x81`$\x81\x88Z\xFA\x90\x81\x15a\x13\xA7W_\x91a \x7FW[Pa v\x90`\x01\x92a\x18$V[\x92\x01\x91\x90a\x1E\xF9V[\x90P` \x81=\x82\x11a \xA9W[\x81a \x99` \x93\x83a\x16\x9BV[\x81\x01\x03\x12a\x05\x17WQ`\x01a iV[=\x91Pa \x8CV[a \xCA\x91P` =` \x11a\x1C\xDAWa\x1C\xCC\x81\x83a\x16\x9BV[_a\x1E\xC1V[\x91\x90\x81\x10\x15a\x18\x8DW`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x05\x17W\x01\x90V[5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x05\x17W\x90V[\x90\x81` \x91\x03\x12a\x05\x17WQ\x80\x15\x15\x81\x03a\x05\x17W\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a!2WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a!\xA8WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FServiceManagerBase.onlyStakeRegi`D\x82\x01R\x7Fstry: caller is not the stake re`d\x82\x01Registry`\xD0\x1B`\x84\x82\x01R`\xA4\x90\xFD\xFE\xA2dipfsX\"\x12 M\x9A9_\xD2\xD7\xBE7\x15\xF8\xC3\xB3\xD9&NhH\x8D\x03\xCE\x1E\r\xBB\xAB\xF1\x08\xD7\xE6\xCB\x85\xE40dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f5f3560e01c806326f017e2146114fd57806333cfb7b7146114d95780633bc28c8c1461145a5780633d0714221461117657806367940c89146111585780636b3aa72e146111135780636ecbccfe146110a4578063715018a61461104757806377ef731d14611002578063847d634f14610e175780638999817f14610d8e5780638d68349a14610d6b5780638da5cb5b14610d425780639926ee7d14610be1578063a364f4da14610b28578063a98fb35514610a90578063b134427114610a67578063c1a8e2c5146108b7578063ca8aa7c714610872578063ce7b5e4b146107ed578063e46f1816146107c4578063e481af9d14610798578063f25f1610146106d3578063f2fde38b1461060a578063fc299dee146105e1578063fcd1c375146105c35763fce36c7d14610143575f80fd5b3461022d57602036600319011261022d576004356001600160401b0381116105bf576101739036906004016117f4565b6065549192916001600160a01b0316330361053f577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031692825b82811061036e5750833b1561036a576040519163fce36c7d60e01b835280602484016020600486015252604483019060448160051b8501019280928691609e19813603015b84841061023b5788808981818e8183818f03925af180156102305761021c5750f35b816102269161169b565b61022d5780f35b80fd5b6040513d84823e3d90fd5b9091929394956043198882030184528635828112156103665783019060a08101918035601e1982360301811215610310578101602081359101936001600160401b038211610362578160061b3603851361036257819060a085525260c0830193908c905b8082106103145750505060208101356001600160a01b03811690819003610310579282608063ffffffff6102fc8260209796600199898099015260408101356040870152836102f0606083016116bc565b166060870152016116bc565b1691015298019401940192949391906101fa565b8b80fd5b91949091906001600160a01b0361032a8761160f565b16815260208601356bffffffffffffffffffffffff811680910361035e57602082015260409081019501916001019061029f565b8e80fd5b8c80fd5b8980fd5b8280fd5b8360206001600160a01b0361038e826103888689896120d0565b016120f2565b166064604061039e8689896120d0565b013560405194859384926323b872dd60e01b845233600485015230602485015260448401525af180156104de57610523575b506001600160a01b036103e960206103888487876120d0565b604051636eb1769f60e11b81523060048201526001600160a01b03881660248201529160209183916044918391165afa9081156104de5790859182916104e9575b50602090610495906104626001600160a01b0361044c85610388898c8c6120d0565b1691604061045b888b8b6120d0565b0135611824565b60405163095ea7b360e01b81526001600160a01b038b166004820152602481019190915293849283919082906044820190565b03925af180156104de5790600192916104b0575b50016101b5565b6104d09060203d81116104d7575b6104c8818361169b565b810190612106565b505f6104a9565b503d6104be565b6040513d87823e3d90fd5b9150506020813d821161051b575b816105046020938361169b565b810103126105175751849061049561042a565b5f80fd5b3d91506104f7565b61053a9060203d81116104d7576104c8818361169b565b6103d0565b60405162461bcd60e51b815260206004820152604c60248201527f536572766963654d616e61676572426173652e6f6e6c7952657761726473496e60448201527f69746961746f723a2063616c6c6572206973206e6f742074686520726577617260648201526b32399034b734ba34b0ba37b960a11b608482015260a490fd5b5080fd5b503461022d578060031936011261022d576020606854604051908152f35b503461022d578060031936011261022d576065546040516001600160a01b039091168152602090f35b503461022d57602036600319011261022d576106246115f9565b61062c61211e565b6001600160a01b0316801561067f57603380546001600160a01b0319811683179091556001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08380a380f35b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b503461022d57602036600319011261022d57806004356001600160a01b038116908190036107955761072f337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611de3565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156107935782916044839260405194859384926334f65bfd60e21b845230600485015260248401525af180156102305761021c5750f35b505b50fd5b503461022d578060031936011261022d576107c06107b4611e70565b60405191829182611623565b0390f35b503461022d578060031936011261022d576067546040516001600160a01b039091168152602090f35b503461022d57806107fd366117b8565b90610806612176565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561086d5761085c9284928360405180968195829463b66bd98960e01b84523060048501611db9565b03925af180156102305761021c5750f35b505050fd5b503461022d578060031936011261022d576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b503461022d57604036600319011261022d576108d16115f9565b906024356001600160401b0381116105bf576108f19036906004016117f4565b92610926337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611de3565b6040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031694909161095f83611680565b6001600160a01b0316825230602083019081529261097c826116cd565b9161098a604051938461169b565b808352602083019060051b820191368311610a6357905b828210610a4b5750505060408201908152843b15610a4757604051636e3492b560e01b815260206004820181905292516001600160a01b03908116602483015293519093166044840152516060606484015280516084840181905260a48401929190910190845b818110610a2b57505050818394818581819503925af180156102305761021c5750f35b825163ffffffff16845260209384019390920191600101610a08565b8380fd5b60208091610a58846116bc565b8152019101906109a1565b8680fd5b503461022d578060031936011261022d576066546040516001600160a01b039091168152602090f35b503461022d57602036600319011261022d57806004356001600160401b03811161079557610ac290369060040161179a565b610aca61211e565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156107935760405163a98fb35560e01b815260206004820152918391839182908490829061085c906024830190611d95565b503461022d57602036600319011261022d5780610b436115f9565b610b77337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614611de3565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610793576040516351b27a6d60e11b81526001600160a01b0390911660048201529082908290602490829084905af180156102305761021c5750f35b503461022d57604036600319011261022d5780610bfc6115f9565b602435906001600160401b03821161079357606060031983360301126107935760405190610c2982611680565b82600401356001600160401b038111610d3e57830136602382011215610d3e57610c5d903690602460048201359101611764565b82526020820191602484013583526044604082019401358452610ca960018060a01b037f0000000000000000000000000000000000000000000000000000000000000000163314611de3565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610d3a5785610d1c93819560405197889687958694639926ee7d60e01b865260018060a01b0316600486015260406024860152516060604486015260a4850190611d95565b9151606484015251608483015203925af180156102305761021c5750f35b8580fd5b8480fd5b503461022d578060031936011261022d576033546040516001600160a01b039091168152602090f35b503461022d578060031936011261022d57602060ff606954166040519015158152f35b503461022d57602036600319011261022d577f2f8afc8a78fd958f3301c0233aa326b9c4b9a2884a7483227d6b0555aaa03adb610e11610dcc6115f9565b610dd461211e565b606780546001600160a01b0319166001600160a01b0392909216918217905542606881905560408051928352602083019190915290918291820190565b0390a180f35b503461022d57602036600319011261022d57600435906001600160401b03821161022d573660238301121561022d578160040135610e54816116cd565b92610e62604051948561169b565b81845260208401906024829360051b82010190368211610d3e5760248101925b828410610f8c5750610ec2925050506001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163314611de3565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561036a5760405191630130fc2760e51b835260448301943060048501526040602485015251809552606483019460648160051b8501019190855b818110610f4a57868087818180890381838c5af180156102305761021c5750f35b909192602080610f7d6001936063198a8203018c52604083895163ffffffff815116845201519181858201520190611d59565b95019801910196919096610f29565b83356001600160401b038111610a6357820160406023198236030112610a635760405190610fb982611665565b610fc5602482016116bc565b825260448101356001600160401b038111610ffe5791610fef6020949260248695369201016116e4565b83820152815201930192610e82565b8880fd5b503461022d578060031936011261022d576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b503461022d578060031936011261022d5761106061211e565b603380546001600160a01b0319811690915581906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b503461022d57806110b4366117b8565b906110bd612176565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561086d5761085c92849283604051809681958294630287f75160e51b84523060048501611db9565b503461022d578060031936011261022d576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b503461022d578060031936011261022d57602060405162093a808152f35b5034610517576020366003190112610517576004356001600160401b0381116105175760a06003198236030112610517576040519060a082018281106001600160401b03821117611446576040526111d08160040161160f565b82526111de602482016116bc565b906020830191825260448101356001600160401b0381116105175761120990600436918401016116e4565b916040840192835260648201356001600160401b038111610517578201913660238401121561051757600483013592611241846116cd565b9361124f604051958661169b565b808552602060048187019260051b840101019136831161051757602401905b82821061143657505050606085019283526084810135906001600160401b0382116105175760046112a2923692010161179a565b608085019081526066549091906001600160a01b031633036113cb577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031693843b156105175760408051633635205760e01b8152306004820152602481019190915295516001600160a01b03166044870152905163ffffffff1660648601525160a06084860152849291906113439060e4850190611d59565b9151916043198482030160a4850152602080845192838152019301905f5b8181106113b2575050505f836113878296948294516043198483030160c4850152611d95565b03925af180156113a757611399575080f35b6113a591505f9061169b565b005b6040513d5f823e3d90fd5b8251855287955060209485019490920191600101611361565b60405162461bcd60e51b815260206004820152603960248201527f536572766963654d616e61676572426173652e6f6e6c79536c61736865723a2060448201527f63616c6c6572206973206e6f742074686520736c6173686572000000000000006064820152608490fd5b813581526020918201910161126e565b634e487b7160e01b5f52604160045260245ffd5b34610517576020366003190112610517576114736115f9565b61147b61211e565b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555005b34610517576020366003190112610517576107c06107b46114f86115f9565b611912565b34610517575f3660031901126105175761151561211e565b60685462093a8081018091116115e557421061158957606754606654604080516001600160a01b03808416825284166020820181905292917fe0d49a54274423183dadecbdf239eaac6e06ba88320b26fe8cc5ec9d050a639591a16001600160a01b03199081169190911760665516606755005b60405162461bcd60e51b815260206004820152602e60248201527f536572766963654d616e616765723a20536c61736865722070726f706f73616c60448201526d0819195b185e481b9bdd081b595d60921b6064820152608490fd5b634e487b7160e01b5f52601160045260245ffd5b600435906001600160a01b038216820361051757565b35906001600160a01b038216820361051757565b60206040818301928281528451809452019201905f5b8181106116465750505090565b82516001600160a01b0316845260209384019390920191600101611639565b604081019081106001600160401b0382111761144657604052565b606081019081106001600160401b0382111761144657604052565b90601f801991011681019081106001600160401b0382111761144657604052565b359063ffffffff8216820361051757565b6001600160401b0381116114465760051b60200190565b9080601f830112156105175781356116fb816116cd565b92611709604051948561169b565b81845260208085019260051b82010192831161051757602001905b8282106117315750505090565b6020809161173e8461160f565b815201910190611724565b6001600160401b03811161144657601f01601f191660200190565b92919261177082611749565b9161177e604051938461169b565b829481845281830111610517578281602093845f960137010152565b9080601f83011215610517578160206117b593359101611764565b90565b9060406003198301126105175760043563ffffffff811681036105175791602435906001600160401b038211610517576117b5916004016116e4565b9181601f84011215610517578235916001600160401b038311610517576020808501948460051b01011161051757565b919082018092116115e557565b90816020910312610517575160ff811681036105175790565b90611854826116cd565b611861604051918261169b565b8281528092611872601f19916116cd565b0190602036910137565b90815181101561188d570160200190565b634e487b7160e01b5f52603260045260245ffd5b9081604091031261051757604051906118b982611665565b8051906001600160a01b038216820361051757602091835201516bffffffffffffffffffffffff8116810361051757602082015290565b805182101561188d5760209160051b010190565b5f1981146115e55760010190565b6040516309aa152760e11b81526001600160a01b039182166004820152907f000000000000000000000000000000000000000000000000000000000000000016602082602481845afa9182156113a7575f92611d25575b506040519163871ef04960e01b83526004830152602082602481845afa9182156113a7575f92611ce1575b506001600160c01b038216908115908115611c7e575b50611c6457805f915b611c3f575061ffff166119c581611749565b906119d3604051928361169b565b8082526119e2601f1991611749565b013660208301375f5f5b8251821080611c34575b15611a44576001811b84166001600160c01b0316611a1d575b611a1890611904565b6119ec565b906001611a189160ff60f81b8460f81b165f1a611a3a828761187c565b5301919050611a0f565b505f939250507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690835b8151851015611aff57611a8a858361187c565b5160f81c60405190633ca5a5f560e01b82526004820152602081602481875afa9081156113a7575f91611acd575b50611ac590600192611824565b940193611a77565b90506020813d8211611af7575b81611ae76020938361169b565b8101031261051757516001611ab8565b3d9150611ada565b611b0a91945061184a565b925f905f5b8151811015611c2e57611b22818361187c565b5160f81c60405190633ca5a5f560e01b8252806004830152602082602481895afa9182156113a7575f92611bfb575b50905f915b818310611b6857505050600101611b0f565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156113a757600192611bc4925f91611bcd575b50838060a01b03905116611bbe828d6118f0565b52611904565b95019190611b56565b611bee915060403d8111611bf4575b611be6818361169b565b8101906118a1565b5f611baa565b503d611bdc565b9091506020813d8211611c26575b81611c166020938361169b565b810103126105175751905f611b51565b3d9150611c09565b50505050565b5061010081106119f6565b5f1981018181116115e55761ffff9116911661ffff81146115e55760010190806119b3565b5050604051611c7460208261169b565b5f81525f36813790565b604051639aa1653d60e01b81529150602090829060049082905afa80156113a75760ff915f91611cb2575b5016155f6119aa565b611cd4915060203d602011611cda575b611ccc818361169b565b810190611831565b5f611ca9565b503d611cc2565b9091506020813d602011611d1d575b81611cfd6020938361169b565b8101031261051757516001600160c01b038116810361051757905f611994565b3d9150611cf0565b9091506020813d602011611d51575b81611d416020938361169b565b810103126105175751905f611969565b3d9150611d34565b90602080835192838152019201905f5b818110611d765750505090565b82516001600160a01b0316845260209384019390920191600101611d69565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b63ffffffff6117b5949360609360018060a01b031683521660208201528160408201520190611d59565b15611dea57565b60405162461bcd60e51b815260206004820152605260248201527f536572766963654d616e61676572426173652e6f6e6c7952656769737472794360448201527f6f6f7264696e61746f723a2063616c6c6572206973206e6f742074686520726560648201527133b4b9ba393c9031b7b7b93234b730ba37b960711b608482015260a490fd5b604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156113a75760ff915f916120b1575b50168015611c64577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316905f9081905b80831061203b5750611f0b915061184a565b925f905f5b604051639aa1653d60e01b8152602081600481895afa80156113a75760ff915f9161201d575b501681101561201657604051633ca5a5f560e01b815260ff821660048201819052602082602481895afa9182156113a7575f92611fe3575b50905f915b818310611f8557505050600101611f10565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156113a757600192611fda925f91611bcd5750838060a01b03905116611bbe828d6118f0565b95019190611f73565b9091506020813d821161200e575b81611ffe6020938361169b565b810103126105175751905f611f6e565b3d9150611ff1565b5092505050565b612035915060203d8111611cda57611ccc818361169b565b5f611f36565b90604051633ca5a5f560e01b815260ff84166004820152602081602481885afa9081156113a7575f9161207f575b5061207690600192611824565b92019190611ef9565b90506020813d82116120a9575b816120996020938361169b565b8101031261051757516001612069565b3d915061208c565b6120ca915060203d602011611cda57611ccc818361169b565b5f611ec1565b919081101561188d5760051b81013590609e1981360301821215610517570190565b356001600160a01b03811681036105175790565b90816020910312610517575180151581036105175790565b6033546001600160a01b0316330361213257565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036121a857565b60405162461bcd60e51b815260206004820152604660248201527f536572766963654d616e61676572426173652e6f6e6c795374616b655265676960448201527f737472793a2063616c6c6572206973206e6f7420746865207374616b6520726560648201526567697374727960d01b608482015260a490fdfea26469706673582212204d9a395fd2d7be3715f8c3b3d9264e68488d03ce1e0dbbabf108d7e6cb85e43064736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c&\xF0\x17\xE2\x14a\x14\xFDW\x80c3\xCF\xB7\xB7\x14a\x14\xD9W\x80c;\xC2\x8C\x8C\x14a\x14ZW\x80c=\x07\x14\"\x14a\x11vW\x80cg\x94\x0C\x89\x14a\x11XW\x80ck:\xA7.\x14a\x11\x13W\x80cn\xCB\xCC\xFE\x14a\x10\xA4W\x80cqP\x18\xA6\x14a\x10GW\x80cw\xEFs\x1D\x14a\x10\x02W\x80c\x84}cO\x14a\x0E\x17W\x80c\x89\x99\x81\x7F\x14a\r\x8EW\x80c\x8Dh4\x9A\x14a\rkW\x80c\x8D\xA5\xCB[\x14a\rBW\x80c\x99&\xEE}\x14a\x0B\xE1W\x80c\xA3d\xF4\xDA\x14a\x0B(W\x80c\xA9\x8F\xB3U\x14a\n\x90W\x80c\xB14Bq\x14a\ngW\x80c\xC1\xA8\xE2\xC5\x14a\x08\xB7W\x80c\xCA\x8A\xA7\xC7\x14a\x08rW\x80c\xCE{^K\x14a\x07\xEDW\x80c\xE4o\x18\x16\x14a\x07\xC4W\x80c\xE4\x81\xAF\x9D\x14a\x07\x98W\x80c\xF2_\x16\x10\x14a\x06\xD3W\x80c\xF2\xFD\xE3\x8B\x14a\x06\nW\x80c\xFC)\x9D\xEE\x14a\x05\xE1W\x80c\xFC\xD1\xC3u\x14a\x05\xC3Wc\xFC\xE3l}\x14a\x01CW_\x80\xFD[4a\x02-W` 6`\x03\x19\x01\x12a\x02-W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x05\xBFWa\x01s\x906\x90`\x04\x01a\x17\xF4V[`eT\x91\x92\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a\x05?W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92\x82[\x82\x81\x10a\x03nWP\x83;\x15a\x03jW`@Q\x91c\xFC\xE3l}`\xE0\x1B\x83R\x80`$\x84\x01` `\x04\x86\x01RR`D\x83\x01\x90`D\x81`\x05\x1B\x85\x01\x01\x92\x80\x92\x86\x91`\x9E\x19\x816\x03\x01[\x84\x84\x10a\x02;W\x88\x80\x89\x81\x81\x8E\x81\x83\x81\x8F\x03\x92Z\xF1\x80\x15a\x020Wa\x02\x1CWP\xF3[\x81a\x02&\x91a\x16\x9BV[a\x02-W\x80\xF3[\x80\xFD[`@Q=\x84\x82>=\x90\xFD[\x90\x91\x92\x93\x94\x95`C\x19\x88\x82\x03\x01\x84R\x865\x82\x81\x12\x15a\x03fW\x83\x01\x90`\xA0\x81\x01\x91\x805`\x1E\x19\x826\x03\x01\x81\x12\x15a\x03\x10W\x81\x01` \x815\x91\x01\x93`\x01`\x01`@\x1B\x03\x82\x11a\x03bW\x81`\x06\x1B6\x03\x85\x13a\x03bW\x81\x90`\xA0\x85RR`\xC0\x83\x01\x93\x90\x8C\x90[\x80\x82\x10a\x03\x14WPPP` \x81\x015`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x03\x10W\x92\x82`\x80c\xFF\xFF\xFF\xFFa\x02\xFC\x82` \x97\x96`\x01\x99\x89\x80\x99\x01R`@\x81\x015`@\x87\x01R\x83a\x02\xF0``\x83\x01a\x16\xBCV[\x16``\x87\x01R\x01a\x16\xBCV[\x16\x91\x01R\x98\x01\x94\x01\x94\x01\x92\x94\x93\x91\x90a\x01\xFAV[\x8B\x80\xFD[\x91\x94\x90\x91\x90`\x01`\x01`\xA0\x1B\x03a\x03*\x87a\x16\x0FV[\x16\x81R` \x86\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\x03^W` \x82\x01R`@\x90\x81\x01\x95\x01\x91`\x01\x01\x90a\x02\x9FV[\x8E\x80\xFD[\x8C\x80\xFD[\x89\x80\xFD[\x82\x80\xFD[\x83` `\x01`\x01`\xA0\x1B\x03a\x03\x8E\x82a\x03\x88\x86\x89\x89a \xD0V[\x01a \xF2V[\x16`d`@a\x03\x9E\x86\x89\x89a \xD0V[\x015`@Q\x94\x85\x93\x84\x92c#\xB8r\xDD`\xE0\x1B\x84R3`\x04\x85\x01R0`$\x85\x01R`D\x84\x01RZ\xF1\x80\x15a\x04\xDEWa\x05#W[P`\x01`\x01`\xA0\x1B\x03a\x03\xE9` a\x03\x88\x84\x87\x87a \xD0V[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16`$\x82\x01R\x91` \x91\x83\x91`D\x91\x83\x91\x16Z\xFA\x90\x81\x15a\x04\xDEW\x90\x85\x91\x82\x91a\x04\xE9W[P` \x90a\x04\x95\x90a\x04b`\x01`\x01`\xA0\x1B\x03a\x04L\x85a\x03\x88\x89\x8C\x8Ca \xD0V[\x16\x91`@a\x04[\x88\x8B\x8Ba \xD0V[\x015a\x18$V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x93\x84\x92\x83\x91\x90\x82\x90`D\x82\x01\x90V[\x03\x92Z\xF1\x80\x15a\x04\xDEW\x90`\x01\x92\x91a\x04\xB0W[P\x01a\x01\xB5V[a\x04\xD0\x90` =\x81\x11a\x04\xD7W[a\x04\xC8\x81\x83a\x16\x9BV[\x81\x01\x90a!\x06V[P_a\x04\xA9V[P=a\x04\xBEV[`@Q=\x87\x82>=\x90\xFD[\x91PP` \x81=\x82\x11a\x05\x1BW[\x81a\x05\x04` \x93\x83a\x16\x9BV[\x81\x01\x03\x12a\x05\x17WQ\x84\x90a\x04\x95a\x04*V[_\x80\xFD[=\x91Pa\x04\xF7V[a\x05:\x90` =\x81\x11a\x04\xD7Wa\x04\xC8\x81\x83a\x16\x9BV[a\x03\xD0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FServiceManagerBase.onlyRewardsIn`D\x82\x01R\x7Fitiator: caller is not the rewar`d\x82\x01Rk29\x904\xB74\xBA4\xB0\xBA7\xB9`\xA1\x1B`\x84\x82\x01R`\xA4\x90\xFD[P\x80\xFD[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W` `hT`@Q\x90\x81R\xF3[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x02-W` 6`\x03\x19\x01\x12a\x02-Wa\x06$a\x15\xF9V[a\x06,a!\x1EV[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x06\x7FW`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x90\x91U`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x83\x80\xA3\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[P4a\x02-W` 6`\x03\x19\x01\x12a\x02-W\x80`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x07\x95Wa\x07/3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1D\xE3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x07\x93W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c4\xF6[\xFD`\xE2\x1B\x84R0`\x04\x85\x01R`$\x84\x01RZ\xF1\x80\x15a\x020Wa\x02\x1CWP\xF3[P[P\xFD[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-Wa\x07\xC0a\x07\xB4a\x1EpV[`@Q\x91\x82\x91\x82a\x16#V[\x03\x90\xF3[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W`gT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x02-W\x80a\x07\xFD6a\x17\xB8V[\x90a\x08\x06a!vV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x08mWa\x08\\\x92\x84\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xB6k\xD9\x89`\xE0\x1B\x84R0`\x04\x85\x01a\x1D\xB9V[\x03\x92Z\xF1\x80\x15a\x020Wa\x02\x1CWP\xF3[PPP\xFD[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x02-W`@6`\x03\x19\x01\x12a\x02-Wa\x08\xD1a\x15\xF9V[\x90`$5`\x01`\x01`@\x1B\x03\x81\x11a\x05\xBFWa\x08\xF1\x906\x90`\x04\x01a\x17\xF4V[\x92a\t&3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1D\xE3V[`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x94\x90\x91a\t_\x83a\x16\x80V[`\x01`\x01`\xA0\x1B\x03\x16\x82R0` \x83\x01\x90\x81R\x92a\t|\x82a\x16\xCDV[\x91a\t\x8A`@Q\x93\x84a\x16\x9BV[\x80\x83R` \x83\x01\x90`\x05\x1B\x82\x01\x916\x83\x11a\ncW\x90[\x82\x82\x10a\nKWPPP`@\x82\x01\x90\x81R\x84;\x15a\nGW`@Qcn4\x92\xB5`\xE0\x1B\x81R` `\x04\x82\x01\x81\x90R\x92Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`$\x83\x01R\x93Q\x90\x93\x16`D\x84\x01RQ```d\x84\x01R\x80Q`\x84\x84\x01\x81\x90R`\xA4\x84\x01\x92\x91\x90\x91\x01\x90\x84[\x81\x81\x10a\n+WPPP\x81\x83\x94\x81\x85\x81\x81\x95\x03\x92Z\xF1\x80\x15a\x020Wa\x02\x1CWP\xF3[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\n\x08V[\x83\x80\xFD[` \x80\x91a\nX\x84a\x16\xBCV[\x81R\x01\x91\x01\x90a\t\xA1V[\x86\x80\xFD[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W`fT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x02-W` 6`\x03\x19\x01\x12a\x02-W\x80`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x07\x95Wa\n\xC2\x906\x90`\x04\x01a\x17\x9AV[a\n\xCAa!\x1EV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x07\x93W`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R` `\x04\x82\x01R\x91\x83\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x08\\\x90`$\x83\x01\x90a\x1D\x95V[P4a\x02-W` 6`\x03\x19\x01\x12a\x02-W\x80a\x0BCa\x15\xF9V[a\x0Bw3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1D\xE3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x07\x93W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x82\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x020Wa\x02\x1CWP\xF3[P4a\x02-W`@6`\x03\x19\x01\x12a\x02-W\x80a\x0B\xFCa\x15\xF9V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x07\x93W```\x03\x19\x836\x03\x01\x12a\x07\x93W`@Q\x90a\x0C)\x82a\x16\x80V[\x82`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\r>W\x83\x016`#\x82\x01\x12\x15a\r>Wa\x0C]\x906\x90`$`\x04\x82\x015\x91\x01a\x17dV[\x82R` \x82\x01\x91`$\x84\x015\x83R`D`@\x82\x01\x94\x015\x84Ra\x0C\xA9`\x01\x80`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x14a\x1D\xE3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\r:W\x85a\r\x1C\x93\x81\x95`@Q\x97\x88\x96\x87\x95\x86\x94c\x99&\xEE}`\xE0\x1B\x86R`\x01\x80`\xA0\x1B\x03\x16`\x04\x86\x01R`@`$\x86\x01RQ```D\x86\x01R`\xA4\x85\x01\x90a\x1D\x95V[\x91Q`d\x84\x01RQ`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\x020Wa\x02\x1CWP\xF3[\x85\x80\xFD[\x84\x80\xFD[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W` `\xFF`iT\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x02-W` 6`\x03\x19\x01\x12a\x02-W\x7F/\x8A\xFC\x8Ax\xFD\x95\x8F3\x01\xC0#:\xA3&\xB9\xC4\xB9\xA2\x88Jt\x83\"}k\x05U\xAA\xA0:\xDBa\x0E\x11a\r\xCCa\x15\xF9V[a\r\xD4a!\x1EV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90UB`h\x81\x90U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1\x80\xF3[P4a\x02-W` 6`\x03\x19\x01\x12a\x02-W`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02-W6`#\x83\x01\x12\x15a\x02-W\x81`\x04\x015a\x0ET\x81a\x16\xCDV[\x92a\x0Eb`@Q\x94\x85a\x16\x9BV[\x81\x84R` \x84\x01\x90`$\x82\x93`\x05\x1B\x82\x01\x01\x906\x82\x11a\r>W`$\x81\x01\x92[\x82\x84\x10a\x0F\x8CWPa\x0E\xC2\x92PPP`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x14a\x1D\xE3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03jW`@Q\x91c\x010\xFC'`\xE5\x1B\x83R`D\x83\x01\x940`\x04\x85\x01R`@`$\x85\x01RQ\x80\x95R`d\x83\x01\x94`d\x81`\x05\x1B\x85\x01\x01\x91\x90\x85[\x81\x81\x10a\x0FJW\x86\x80\x87\x81\x81\x80\x89\x03\x81\x83\x8CZ\xF1\x80\x15a\x020Wa\x02\x1CWP\xF3[\x90\x91\x92` \x80a\x0F}`\x01\x93`c\x19\x8A\x82\x03\x01\x8CR`@\x83\x89Qc\xFF\xFF\xFF\xFF\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a\x1DYV[\x95\x01\x98\x01\x91\x01\x96\x91\x90\x96a\x0F)V[\x835`\x01`\x01`@\x1B\x03\x81\x11a\ncW\x82\x01`@`#\x19\x826\x03\x01\x12a\ncW`@Q\x90a\x0F\xB9\x82a\x16eV[a\x0F\xC5`$\x82\x01a\x16\xBCV[\x82R`D\x81\x015`\x01`\x01`@\x1B\x03\x81\x11a\x0F\xFEW\x91a\x0F\xEF` \x94\x92`$\x86\x956\x92\x01\x01a\x16\xE4V[\x83\x82\x01R\x81R\x01\x93\x01\x92a\x0E\x82V[\x88\x80\xFD[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-Wa\x10`a!\x1EV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[P4a\x02-W\x80a\x10\xB46a\x17\xB8V[\x90a\x10\xBDa!vV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x08mWa\x08\\\x92\x84\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\x02\x87\xF7Q`\xE5\x1B\x84R0`\x04\x85\x01a\x1D\xB9V[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x02-W\x80`\x03\x196\x01\x12a\x02-W` `@Qb\t:\x80\x81R\xF3[P4a\x05\x17W` 6`\x03\x19\x01\x12a\x05\x17W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x05\x17W`\xA0`\x03\x19\x826\x03\x01\x12a\x05\x17W`@Q\x90`\xA0\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x14FW`@Ra\x11\xD0\x81`\x04\x01a\x16\x0FV[\x82Ra\x11\xDE`$\x82\x01a\x16\xBCV[\x90` \x83\x01\x91\x82R`D\x81\x015`\x01`\x01`@\x1B\x03\x81\x11a\x05\x17Wa\x12\t\x90`\x046\x91\x84\x01\x01a\x16\xE4V[\x91`@\x84\x01\x92\x83R`d\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x05\x17W\x82\x01\x916`#\x84\x01\x12\x15a\x05\x17W`\x04\x83\x015\x92a\x12A\x84a\x16\xCDV[\x93a\x12O`@Q\x95\x86a\x16\x9BV[\x80\x85R` `\x04\x81\x87\x01\x92`\x05\x1B\x84\x01\x01\x01\x916\x83\x11a\x05\x17W`$\x01\x90[\x82\x82\x10a\x146WPPP``\x85\x01\x92\x83R`\x84\x81\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x05\x17W`\x04a\x12\xA2\x926\x92\x01\x01a\x17\x9AV[`\x80\x85\x01\x90\x81R`fT\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x13\xCBW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x93\x84;\x15a\x05\x17W`@\x80Qc65 W`\xE0\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x95Q`\x01`\x01`\xA0\x1B\x03\x16`D\x87\x01R\x90Qc\xFF\xFF\xFF\xFF\x16`d\x86\x01RQ`\xA0`\x84\x86\x01R\x84\x92\x91\x90a\x13C\x90`\xE4\x85\x01\x90a\x1DYV[\x91Q\x91`C\x19\x84\x82\x03\x01`\xA4\x85\x01R` \x80\x84Q\x92\x83\x81R\x01\x93\x01\x90_[\x81\x81\x10a\x13\xB2WPPP_\x83a\x13\x87\x82\x96\x94\x82\x94Q`C\x19\x84\x83\x03\x01`\xC4\x85\x01Ra\x1D\x95V[\x03\x92Z\xF1\x80\x15a\x13\xA7Wa\x13\x99WP\x80\xF3[a\x13\xA5\x91P_\x90a\x16\x9BV[\0[`@Q=_\x82>=\x90\xFD[\x82Q\x85R\x87\x95P` \x94\x85\x01\x94\x90\x92\x01\x91`\x01\x01a\x13aV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FServiceManagerBase.onlySlasher: `D\x82\x01R\x7Fcaller is not the slasher\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x815\x81R` \x91\x82\x01\x91\x01a\x12nV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[4a\x05\x17W` 6`\x03\x19\x01\x12a\x05\x17Wa\x14sa\x15\xF9V[a\x14{a!\x1EV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`eU\0[4a\x05\x17W` 6`\x03\x19\x01\x12a\x05\x17Wa\x07\xC0a\x07\xB4a\x14\xF8a\x15\xF9V[a\x19\x12V[4a\x05\x17W_6`\x03\x19\x01\x12a\x05\x17Wa\x15\x15a!\x1EV[`hTb\t:\x80\x81\x01\x80\x91\x11a\x15\xE5WB\x10a\x15\x89W`gT`fT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01\x81\x90R\x92\x91\x7F\xE0\xD4\x9AT'D#\x18=\xAD\xEC\xBD\xF29\xEA\xACn\x06\xBA\x882\x0B&\xFE\x8C\xC5\xEC\x9D\x05\nc\x95\x91\xA1`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x91\x90\x91\x17`fU\x16`gU\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FServiceManager: Slasher proposal`D\x82\x01Rm\x08\x19\x19[\x18^H\x1B\x9B\xDD\x08\x1BY]`\x92\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x05\x17WV[5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x05\x17WV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x16FWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x169V[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x14FW`@RV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x14FW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x14FW`@RV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x05\x17WV[`\x01`\x01`@\x1B\x03\x81\x11a\x14FW`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x05\x17W\x815a\x16\xFB\x81a\x16\xCDV[\x92a\x17\t`@Q\x94\x85a\x16\x9BV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x05\x17W` \x01\x90[\x82\x82\x10a\x171WPPP\x90V[` \x80\x91a\x17>\x84a\x16\x0FV[\x81R\x01\x91\x01\x90a\x17$V[`\x01`\x01`@\x1B\x03\x81\x11a\x14FW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x17p\x82a\x17IV[\x91a\x17~`@Q\x93\x84a\x16\x9BV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x05\x17W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x05\x17W\x81` a\x17\xB5\x935\x91\x01a\x17dV[\x90V[\x90`@`\x03\x19\x83\x01\x12a\x05\x17W`\x045c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x05\x17W\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x05\x17Wa\x17\xB5\x91`\x04\x01a\x16\xE4V[\x91\x81`\x1F\x84\x01\x12\x15a\x05\x17W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x05\x17W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x05\x17WV[\x91\x90\x82\x01\x80\x92\x11a\x15\xE5WV[\x90\x81` \x91\x03\x12a\x05\x17WQ`\xFF\x81\x16\x81\x03a\x05\x17W\x90V[\x90a\x18T\x82a\x16\xCDV[a\x18a`@Q\x91\x82a\x16\x9BV[\x82\x81R\x80\x92a\x18r`\x1F\x19\x91a\x16\xCDV[\x01\x90` 6\x91\x017V[\x90\x81Q\x81\x10\x15a\x18\x8DW\x01` \x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90\x81`@\x91\x03\x12a\x05\x17W`@Q\x90a\x18\xB9\x82a\x16eV[\x80Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x05\x17W` \x91\x83R\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x05\x17W` \x82\x01R\x90V[\x80Q\x82\x10\x15a\x18\x8DW` \x91`\x05\x1B\x01\x01\x90V[_\x19\x81\x14a\x15\xE5W`\x01\x01\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x82`$\x81\x84Z\xFA\x91\x82\x15a\x13\xA7W_\x92a\x1D%W[P`@Q\x91c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01R` \x82`$\x81\x84Z\xFA\x91\x82\x15a\x13\xA7W_\x92a\x1C\xE1W[P`\x01`\x01`\xC0\x1B\x03\x82\x16\x90\x81\x15\x90\x81\x15a\x1C~W[Pa\x1CdW\x80_\x91[a\x1C?WPa\xFF\xFF\x16a\x19\xC5\x81a\x17IV[\x90a\x19\xD3`@Q\x92\x83a\x16\x9BV[\x80\x82Ra\x19\xE2`\x1F\x19\x91a\x17IV[\x016` \x83\x017__[\x82Q\x82\x10\x80a\x1C4W[\x15a\x1ADW`\x01\x81\x1B\x84\x16`\x01`\x01`\xC0\x1B\x03\x16a\x1A\x1DW[a\x1A\x18\x90a\x19\x04V[a\x19\xECV[\x90`\x01a\x1A\x18\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa\x1A:\x82\x87a\x18|V[S\x01\x91\x90Pa\x1A\x0FV[P_\x93\x92PP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x83[\x81Q\x85\x10\x15a\x1A\xFFWa\x1A\x8A\x85\x83a\x18|V[Q`\xF8\x1C`@Q\x90c<\xA5\xA5\xF5`\xE0\x1B\x82R`\x04\x82\x01R` \x81`$\x81\x87Z\xFA\x90\x81\x15a\x13\xA7W_\x91a\x1A\xCDW[Pa\x1A\xC5\x90`\x01\x92a\x18$V[\x94\x01\x93a\x1AwV[\x90P` \x81=\x82\x11a\x1A\xF7W[\x81a\x1A\xE7` \x93\x83a\x16\x9BV[\x81\x01\x03\x12a\x05\x17WQ`\x01a\x1A\xB8V[=\x91Pa\x1A\xDAV[a\x1B\n\x91\x94Pa\x18JV[\x92_\x90_[\x81Q\x81\x10\x15a\x1C.Wa\x1B\"\x81\x83a\x18|V[Q`\xF8\x1C`@Q\x90c<\xA5\xA5\xF5`\xE0\x1B\x82R\x80`\x04\x83\x01R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x13\xA7W_\x92a\x1B\xFBW[P\x90_\x91[\x81\x83\x10a\x1BhWPPP`\x01\x01a\x1B\x0FV[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x13\xA7W`\x01\x92a\x1B\xC4\x92_\x91a\x1B\xCDW[P\x83\x80`\xA0\x1B\x03\x90Q\x16a\x1B\xBE\x82\x8Da\x18\xF0V[Ra\x19\x04V[\x95\x01\x91\x90a\x1BVV[a\x1B\xEE\x91P`@=\x81\x11a\x1B\xF4W[a\x1B\xE6\x81\x83a\x16\x9BV[\x81\x01\x90a\x18\xA1V[_a\x1B\xAAV[P=a\x1B\xDCV[\x90\x91P` \x81=\x82\x11a\x1C&W[\x81a\x1C\x16` \x93\x83a\x16\x9BV[\x81\x01\x03\x12a\x05\x17WQ\x90_a\x1BQV[=\x91Pa\x1C\tV[PPPPV[Pa\x01\0\x81\x10a\x19\xF6V[_\x19\x81\x01\x81\x81\x11a\x15\xE5Wa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a\x15\xE5W`\x01\x01\x90\x80a\x19\xB3V[PP`@Qa\x1Ct` \x82a\x16\x9BV[_\x81R_6\x817\x90V[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x91P` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x80\x15a\x13\xA7W`\xFF\x91_\x91a\x1C\xB2W[P\x16\x15_a\x19\xAAV[a\x1C\xD4\x91P` =` \x11a\x1C\xDAW[a\x1C\xCC\x81\x83a\x16\x9BV[\x81\x01\x90a\x181V[_a\x1C\xA9V[P=a\x1C\xC2V[\x90\x91P` \x81=` \x11a\x1D\x1DW[\x81a\x1C\xFD` \x93\x83a\x16\x9BV[\x81\x01\x03\x12a\x05\x17WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x05\x17W\x90_a\x19\x94V[=\x91Pa\x1C\xF0V[\x90\x91P` \x81=` \x11a\x1DQW[\x81a\x1DA` \x93\x83a\x16\x9BV[\x81\x01\x03\x12a\x05\x17WQ\x90_a\x19iV[=\x91Pa\x1D4V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x1DvWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1DiV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[c\xFF\xFF\xFF\xFFa\x17\xB5\x94\x93``\x93`\x01\x80`\xA0\x1B\x03\x16\x83R\x16` \x82\x01R\x81`@\x82\x01R\x01\x90a\x1DYV[\x15a\x1D\xEAWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FServiceManagerBase.onlyRegistryC`D\x82\x01R\x7Foordinator: caller is not the re`d\x82\x01Rq3\xB4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B`\x84\x82\x01R`\xA4\x90\xFD[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x13\xA7W`\xFF\x91_\x91a \xB1W[P\x16\x80\x15a\x1CdW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90_\x90\x81\x90[\x80\x83\x10a ;WPa\x1F\x0B\x91Pa\x18JV[\x92_\x90_[`@Qc\x9A\xA1e=`\xE0\x1B\x81R` \x81`\x04\x81\x89Z\xFA\x80\x15a\x13\xA7W`\xFF\x91_\x91a \x1DW[P\x16\x81\x10\x15a \x16W`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01\x81\x90R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x13\xA7W_\x92a\x1F\xE3W[P\x90_\x91[\x81\x83\x10a\x1F\x85WPPP`\x01\x01a\x1F\x10V[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x13\xA7W`\x01\x92a\x1F\xDA\x92_\x91a\x1B\xCDWP\x83\x80`\xA0\x1B\x03\x90Q\x16a\x1B\xBE\x82\x8Da\x18\xF0V[\x95\x01\x91\x90a\x1FsV[\x90\x91P` \x81=\x82\x11a \x0EW[\x81a\x1F\xFE` \x93\x83a\x16\x9BV[\x81\x01\x03\x12a\x05\x17WQ\x90_a\x1FnV[=\x91Pa\x1F\xF1V[P\x92PPPV[a 5\x91P` =\x81\x11a\x1C\xDAWa\x1C\xCC\x81\x83a\x16\x9BV[_a\x1F6V[\x90`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R` \x81`$\x81\x88Z\xFA\x90\x81\x15a\x13\xA7W_\x91a \x7FW[Pa v\x90`\x01\x92a\x18$V[\x92\x01\x91\x90a\x1E\xF9V[\x90P` \x81=\x82\x11a \xA9W[\x81a \x99` \x93\x83a\x16\x9BV[\x81\x01\x03\x12a\x05\x17WQ`\x01a iV[=\x91Pa \x8CV[a \xCA\x91P` =` \x11a\x1C\xDAWa\x1C\xCC\x81\x83a\x16\x9BV[_a\x1E\xC1V[\x91\x90\x81\x10\x15a\x18\x8DW`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x05\x17W\x01\x90V[5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x05\x17W\x90V[\x90\x81` \x91\x03\x12a\x05\x17WQ\x80\x15\x15\x81\x03a\x05\x17W\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a!2WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a!\xA8WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FServiceManagerBase.onlyStakeRegi`D\x82\x01R\x7Fstry: caller is not the stake re`d\x82\x01Registry`\xD0\x1B`\x84\x82\x01R`\xA4\x90\xFD\xFE\xA2dipfsX\"\x12 M\x9A9_\xD2\xD7\xBE7\x15\xF8\xC3\xB3\xD9&NhH\x8D\x03\xCE\x1E\r\xBB\xAB\xF1\x08\xD7\xE6\xCB\x85\xE40dsolcC\0\x08\x1B\x003",
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8,
                    56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8,
                    96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.version,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8,
                    208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8,
                    175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "RewardsInitiatorUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    225u8, 28u8, 221u8, 241u8, 129u8, 106u8, 67u8, 49u8, 140u8, 161u8, 117u8,
                    187u8, 197u8, 44u8, 208u8, 24u8, 84u8, 54u8, 233u8, 203u8, 234u8, 215u8, 200u8,
                    58u8, 204u8, 84u8, 167u8, 62u8, 70u8, 23u8, 23u8, 227u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &RewardsInitiatorUpdated) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SlasherProposed(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    47u8, 138u8, 252u8, 138u8, 120u8, 253u8, 149u8, 143u8, 51u8, 1u8, 192u8, 35u8,
                    58u8, 163u8, 38u8, 185u8, 196u8, 185u8, 162u8, 136u8, 74u8, 116u8, 131u8, 34u8,
                    125u8, 107u8, 5u8, 85u8, 170u8, 160u8, 58u8, 219u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newSlasher,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SlasherUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    224u8, 212u8, 154u8, 84u8, 39u8, 68u8, 35u8, 24u8, 61u8, 173u8, 236u8, 189u8,
                    242u8, 57u8, 234u8, 172u8, 110u8, 6u8, 186u8, 136u8, 50u8, 11u8, 38u8, 254u8,
                    140u8, 197u8, 236u8, 157u8, 5u8, 10u8, 99u8, 149u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<SLASHER_PROPOSAL_DELAYCall> for UnderlyingRustTuple<'_> {
                fn from(value: SLASHER_PROPOSAL_DELAYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for SLASHER_PROPOSAL_DELAYCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<SLASHER_PROPOSAL_DELAYReturn> for UnderlyingRustTuple<'_> {
                fn from(value: SLASHER_PROPOSAL_DELAYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for SLASHER_PROPOSAL_DELAYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SLASHER_PROPOSAL_DELAYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = SLASHER_PROPOSAL_DELAYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<acceptProposedSlasherCall> for UnderlyingRustTuple<'_> {
                fn from(value: acceptProposedSlasherCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for acceptProposedSlasherCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<acceptProposedSlasherReturn> for UnderlyingRustTuple<'_> {
                fn from(value: acceptProposedSlasherReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for acceptProposedSlasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for acceptProposedSlasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = acceptProposedSlasherReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `addStrategyToOperatorSet(uint32,address[])` and selector `0x6ecbccfe`.
    ```solidity
    function addStrategyToOperatorSet(uint32 operatorSetId, address[] memory strategies) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategyToOperatorSetCall {
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`addStrategyToOperatorSet(uint32,address[])`](addStrategyToOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategyToOperatorSetReturn {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addStrategyToOperatorSetCall> for UnderlyingRustTuple<'_> {
                fn from(value: addStrategyToOperatorSetCall) -> Self {
                    (value.operatorSetId, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addStrategyToOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetId: tuple.0,
                        strategies: tuple.1,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addStrategyToOperatorSetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addStrategyToOperatorSetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addStrategyToOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addStrategyToOperatorSetCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = addStrategyToOperatorSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addStrategyToOperatorSet(uint32,address[])";
            const SELECTOR: [u8; 4] = [110u8, 203u8, 204u8, 254u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allocationManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allocationManagerCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allocationManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allocationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allocationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = allocationManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsDirectoryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<IRewardsCoordinatorTypes::RewardsSubmission>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createAVSRewardsSubmissionCall> for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionCall) -> Self {
                    (value.rewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createAVSRewardsSubmissionCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createAVSRewardsSubmissionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createAVSRewardsSubmissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createAVSRewardsSubmissionCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<IRewardsCoordinatorTypes::RewardsSubmission>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createAVSRewardsSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])";
            const SELECTOR: [u8; 4] = [252u8, 227u8, 108u8, 125u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsSubmission,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.rewardsSubmissions,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `createOperatorSets((uint32,address[])[])` and selector `0x847d634f`.
    ```solidity
    function createOperatorSets(IAllocationManagerTypes.CreateSetParams[] memory params) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorSetsCall {
        pub params: alloy::sol_types::private::Vec<
            <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createOperatorSets((uint32,address[])[])`](createOperatorSetsCall) function.
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<IAllocationManagerTypes::CreateSetParams>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createOperatorSetsCall> for UnderlyingRustTuple<'_> {
                fn from(value: createOperatorSetsCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createOperatorSetsCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createOperatorSetsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createOperatorSetsCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<IAllocationManagerTypes::CreateSetParams>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createOperatorSets((uint32,address[])[])";
            const SELECTOR: [u8; 4] = [132u8, 125u8, 99u8, 79u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::CreateSetParams,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.params
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromAVSCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorFromAVSCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromAVSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorFromAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorFromAVSCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromOperatorSetsCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromOperatorSetsCall) -> Self {
                    (value.operator, value.operatorSetIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorFromOperatorSetsCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromOperatorSetsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorFromOperatorSetsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorRestakedStrategiesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRestakedStrategiesCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorRestakedStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorRestakedStrategiesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRestakedStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorRestakedStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorRestakedStrategiesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorRestakedStrategiesReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRestakeableStrategiesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRestakeableStrategiesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRestakeableStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRestakeableStrategiesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRestakeableStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRestakeableStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRestakeableStrategiesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRestakeableStrategiesReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<incredibleSquaringTaskManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: incredibleSquaringTaskManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for incredibleSquaringTaskManagerCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<incredibleSquaringTaskManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: incredibleSquaringTaskManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for incredibleSquaringTaskManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for incredibleSquaringTaskManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = incredibleSquaringTaskManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<migrationFinalizedCall> for UnderlyingRustTuple<'_> {
                fn from(value: migrationFinalizedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for migrationFinalizedCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<migrationFinalizedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: migrationFinalizedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for migrationFinalizedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for migrationFinalizedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = migrationFinalizedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposeNewSlasherCall> for UnderlyingRustTuple<'_> {
                fn from(value: proposeNewSlasherCall) -> Self {
                    (value.newSlasher,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposeNewSlasherCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newSlasher: tuple.0,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposeNewSlasherReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proposeNewSlasherReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposeNewSlasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposeNewSlasherCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposeNewSlasherReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposedSlasherReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proposedSlasherReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposedSlasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposedSlasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposedSlasherReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub operatorSignature:
            <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorToAVSCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSCall) -> Self {
                    (value.operator, value.operatorSignature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorToAVSCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorToAVSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorToAVSReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorToAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "registerOperatorToAVS(address,(bytes,bytes32,uint256))";
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `removeStrategiesFromOperatorSet(uint32,address[])` and selector `0xce7b5e4b`.
    ```solidity
    function removeStrategiesFromOperatorSet(uint32 operatorSetId, address[] memory strategies) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesFromOperatorSetCall {
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`removeStrategiesFromOperatorSet(uint32,address[])`](removeStrategiesFromOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesFromOperatorSetReturn {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeStrategiesFromOperatorSetCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesFromOperatorSetCall) -> Self {
                    (value.operatorSetId, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeStrategiesFromOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetId: tuple.0,
                        strategies: tuple.1,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeStrategiesFromOperatorSetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesFromOperatorSetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeStrategiesFromOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeStrategiesFromOperatorSetCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeStrategiesFromOperatorSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeStrategiesFromOperatorSet(uint32,address[])";
            const SELECTOR: [u8; 4] = [206u8, 123u8, 94u8, 75u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<rewardsInitiatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: rewardsInitiatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rewardsInitiatorCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<rewardsInitiatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: rewardsInitiatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rewardsInitiatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rewardsInitiatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = rewardsInitiatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setAVSRegistrar(address)` and selector `0xf25f1610`.
    ```solidity
    function setAVSRegistrar(address registrar) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAVSRegistrarCall {
        pub registrar: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setAVSRegistrar(address)`](setAVSRegistrarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAVSRegistrarReturn {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setAVSRegistrarCall> for UnderlyingRustTuple<'_> {
                fn from(value: setAVSRegistrarCall) -> Self {
                    (value.registrar,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAVSRegistrarCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { registrar: tuple.0 }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setAVSRegistrarReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setAVSRegistrarReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAVSRegistrarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAVSRegistrarCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAVSRegistrarReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAVSRegistrar(address)";
            const SELECTOR: [u8; 4] = [242u8, 95u8, 22u8, 16u8];
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
                        &self.registrar,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setRewardsInitiatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsInitiatorCall) -> Self {
                    (value.newRewardsInitiator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setRewardsInitiatorCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setRewardsInitiatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsInitiatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setRewardsInitiatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setRewardsInitiatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setRewardsInitiatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `slashOperator((address,uint32,address[],uint256[],string))` and selector `0x3d071422`.
    ```solidity
    function slashOperator(IAllocationManagerTypes.SlashingParams memory params) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorCall {
        pub params:
            <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`slashOperator((address,uint32,address[],uint256[],string))`](slashOperatorCall) function.
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "slashOperator((address,uint32,address[],uint256[],string))";
            const SELECTOR: [u8; 4] = [61u8, 7u8, 20u8, 34u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = slasherReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<slasherProposalTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: slasherProposalTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slasherProposalTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<slasherProposalTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: slasherProposalTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slasherProposalTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slasherProposalTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = slasherProposalTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateAVSMetadataURICall> for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURICall) -> Self {
                    (value._metadataURI,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateAVSMetadataURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _metadataURI: tuple.0,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateAVSMetadataURIReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURIReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateAVSMetadataURIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateAVSMetadataURICall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateAVSMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    ///Container for all the [`IncredibleSquaringServiceManager`](self) function calls.
    pub enum IncredibleSquaringServiceManagerCalls {
        SLASHER_PROPOSAL_DELAY(SLASHER_PROPOSAL_DELAYCall),
        acceptProposedSlasher(acceptProposedSlasherCall),
        addStrategyToOperatorSet(addStrategyToOperatorSetCall),
        allocationManager(allocationManagerCall),
        avsDirectory(avsDirectoryCall),
        createAVSRewardsSubmission(createAVSRewardsSubmissionCall),
        createOperatorSets(createOperatorSetsCall),
        deregisterOperatorFromAVS(deregisterOperatorFromAVSCall),
        deregisterOperatorFromOperatorSets(deregisterOperatorFromOperatorSetsCall),
        getOperatorRestakedStrategies(getOperatorRestakedStrategiesCall),
        getRestakeableStrategies(getRestakeableStrategiesCall),
        incredibleSquaringTaskManager(incredibleSquaringTaskManagerCall),
        migrationFinalized(migrationFinalizedCall),
        owner(ownerCall),
        proposeNewSlasher(proposeNewSlasherCall),
        proposedSlasher(proposedSlasherCall),
        registerOperatorToAVS(registerOperatorToAVSCall),
        removeStrategiesFromOperatorSet(removeStrategiesFromOperatorSetCall),
        renounceOwnership(renounceOwnershipCall),
        rewardsInitiator(rewardsInitiatorCall),
        setAVSRegistrar(setAVSRegistrarCall),
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
            [38u8, 240u8, 23u8, 226u8],
            [51u8, 207u8, 183u8, 183u8],
            [59u8, 194u8, 140u8, 140u8],
            [61u8, 7u8, 20u8, 34u8],
            [103u8, 148u8, 12u8, 137u8],
            [107u8, 58u8, 167u8, 46u8],
            [110u8, 203u8, 204u8, 254u8],
            [113u8, 80u8, 24u8, 166u8],
            [119u8, 239u8, 115u8, 29u8],
            [132u8, 125u8, 99u8, 79u8],
            [137u8, 153u8, 129u8, 127u8],
            [141u8, 104u8, 52u8, 154u8],
            [141u8, 165u8, 203u8, 91u8],
            [153u8, 38u8, 238u8, 125u8],
            [163u8, 100u8, 244u8, 218u8],
            [169u8, 143u8, 179u8, 85u8],
            [177u8, 52u8, 66u8, 113u8],
            [193u8, 168u8, 226u8, 197u8],
            [202u8, 138u8, 167u8, 199u8],
            [206u8, 123u8, 94u8, 75u8],
            [228u8, 111u8, 24u8, 22u8],
            [228u8, 129u8, 175u8, 157u8],
            [242u8, 95u8, 22u8, 16u8],
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
        const COUNT: usize = 27usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::SLASHER_PROPOSAL_DELAY(_) => {
                    <SLASHER_PROPOSAL_DELAYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::acceptProposedSlasher(_) => {
                    <acceptProposedSlasherCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addStrategyToOperatorSet(_) => {
                    <addStrategyToOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avsDirectory(_) => <avsDirectoryCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::getOperatorRestakedStrategies(_) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRestakeableStrategies(_) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::incredibleSquaringTaskManager(_) => {
                    <incredibleSquaringTaskManagerCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::removeStrategiesFromOperatorSet(_) => {
                    <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rewardsInitiator(_) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAVSRegistrar(_) => {
                    <setAVSRegistrarCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setRewardsInitiator(_) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashOperator(_) => <slashOperatorCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<
                IncredibleSquaringServiceManagerCalls,
            >] = &[
                {
                    fn acceptProposedSlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <acceptProposedSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::acceptProposedSlasher)
                    }
                    acceptProposedSlasher
                },
                {
                    fn getOperatorRestakedStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
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
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::setRewardsInitiator)
                    }
                    setRewardsInitiator
                },
                {
                    fn slashOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <slashOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::slashOperator)
                    }
                    slashOperator
                },
                {
                    fn SLASHER_PROPOSAL_DELAY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <SLASHER_PROPOSAL_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::SLASHER_PROPOSAL_DELAY)
                    }
                    SLASHER_PROPOSAL_DELAY
                },
                {
                    fn avsDirectory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <avsDirectoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::avsDirectory)
                    }
                    avsDirectory
                },
                {
                    fn addStrategyToOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <addStrategyToOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::addStrategyToOperatorSet)
                    }
                    addStrategyToOperatorSet
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn incredibleSquaringTaskManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
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
                    fn createOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <createOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::createOperatorSets)
                    }
                    createOperatorSets
                },
                {
                    fn proposeNewSlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <proposeNewSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::proposeNewSlasher)
                    }
                    proposeNewSlasher
                },
                {
                    fn migrationFinalized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <migrationFinalizedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::migrationFinalized)
                    }
                    migrationFinalized
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringServiceManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn registerOperatorToAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::registerOperatorToAVS)
                    }
                    registerOperatorToAVS
                },
                {
                    fn deregisterOperatorFromAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::deregisterOperatorFromAVS)
                    }
                    deregisterOperatorFromAVS
                },
                {
                    fn updateAVSMetadataURI(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::updateAVSMetadataURI)
                    }
                    updateAVSMetadataURI
                },
                {
                    fn slasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <slasherCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringServiceManagerCalls::slasher)
                    }
                    slasher
                },
                {
                    fn deregisterOperatorFromOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
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
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn removeStrategiesFromOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringServiceManagerCalls::removeStrategiesFromOperatorSet,
                            )
                    }
                    removeStrategiesFromOperatorSet
                },
                {
                    fn proposedSlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <proposedSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::proposedSlasher)
                    }
                    proposedSlasher
                },
                {
                    fn getRestakeableStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::getRestakeableStrategies)
                    }
                    getRestakeableStrategies
                },
                {
                    fn setAVSRegistrar(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <setAVSRegistrarCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::setAVSRegistrar)
                    }
                    setAVSRegistrar
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn rewardsInitiator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::rewardsInitiator)
                    }
                    rewardsInitiator
                },
                {
                    fn slasherProposalTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
                        <slasherProposalTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringServiceManagerCalls::slasherProposalTimestamp)
                    }
                    slasherProposalTimestamp
                },
                {
                    fn createAVSRewardsSubmission(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringServiceManagerCalls>
                    {
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
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
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
                Self::addStrategyToOperatorSet(inner) => {
                    <addStrategyToOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getOperatorRestakedStrategies(inner) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::removeStrategiesFromOperatorSet(inner) => {
                    <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::setAVSRegistrar(inner) => {
                    <setAVSRegistrarCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::addStrategyToOperatorSet(inner) => {
                    <addStrategyToOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getOperatorRestakedStrategies(inner) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::removeStrategiesFromOperatorSet(inner) => {
                    <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setAVSRegistrar(inner) => {
                    <setAVSRegistrarCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                47u8, 138u8, 252u8, 138u8, 120u8, 253u8, 149u8, 143u8, 51u8, 1u8, 192u8, 35u8,
                58u8, 163u8, 38u8, 185u8, 196u8, 185u8, 162u8, 136u8, 74u8, 116u8, 131u8, 34u8,
                125u8, 107u8, 5u8, 85u8, 170u8, 160u8, 58u8, 219u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8, 208u8,
                164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8, 175u8, 227u8,
                180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                224u8, 212u8, 154u8, 84u8, 39u8, 68u8, 35u8, 24u8, 61u8, 173u8, 236u8, 189u8,
                242u8, 57u8, 234u8, 172u8, 110u8, 6u8, 186u8, 136u8, 50u8, 11u8, 38u8, 254u8,
                140u8, 197u8, 236u8, 157u8, 5u8, 10u8, 99u8, 149u8,
            ],
            [
                225u8, 28u8, 221u8, 241u8, 129u8, 106u8, 67u8, 49u8, 140u8, 161u8, 117u8, 187u8,
                197u8, 44u8, 208u8, 24u8, 84u8, 54u8, 233u8, 203u8, 234u8, 215u8, 200u8, 58u8,
                204u8, 84u8, 167u8, 62u8, 70u8, 23u8, 23u8, 227u8,
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
                        topics, data, validate,
                    )
                    .map(Self::Initialized)
                }
                Some(<OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OwnershipTransferred)
                }
                Some(<RewardsInitiatorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RewardsInitiatorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::RewardsInitiatorUpdated)
                }
                Some(<SlasherProposed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SlasherProposed as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::SlasherProposed)
                }
                Some(<SlasherUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SlasherUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::SlasherUpdated)
                }
                _ => alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                    name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                    log: alloy_sol_types::private::Box::new(
                        alloy_sol_types::private::LogData::new_unchecked(
                            topics.to_vec(),
                            data.to_vec().into(),
                        ),
                    ),
                }),
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for IncredibleSquaringServiceManagerEvents {
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
        Output = alloy_contract::Result<IncredibleSquaringServiceManagerInstance<T, P, N>>,
    > {
        IncredibleSquaringServiceManagerInstance::<T, P, N>::deploy(
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
        IncredibleSquaringServiceManagerInstance::<T, P, N>::deploy_builder(
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
    pub struct IncredibleSquaringServiceManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IncredibleSquaringServiceManagerInstance<T, P, N> {
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
        > IncredibleSquaringServiceManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IncredibleSquaringServiceManager`](self) contract instance.

        See the [wrapper's documentation](`IncredibleSquaringServiceManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
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
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _avsDirectory,
                        _registryCoordinator,
                        _stakeRegistry,
                        rewards_coordinator,
                        allocationManager,
                        _incredibleSquaringTaskManager,
                    })[..],
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
    impl<T, P: ::core::clone::Clone, N> IncredibleSquaringServiceManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IncredibleSquaringServiceManagerInstance<T, P, N> {
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
        > IncredibleSquaringServiceManagerInstance<T, P, N>
    {
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
        ///Creates a new call builder for the [`addStrategyToOperatorSet`] function.
        pub fn addStrategyToOperatorSet(
            &self,
            operatorSetId: u32,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, addStrategyToOperatorSetCall, N> {
            self.call_builder(&addStrategyToOperatorSetCall {
                operatorSetId,
                strategies,
            })
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
        }
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(&self) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall {})
        }
        ///Creates a new call builder for the [`createAVSRewardsSubmission`] function.
        pub fn createAVSRewardsSubmission(
            &self,
            rewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createAVSRewardsSubmissionCall, N> {
            self.call_builder(&createAVSRewardsSubmissionCall { rewardsSubmissions })
        }
        ///Creates a new call builder for the [`createOperatorSets`] function.
        pub fn createOperatorSets(
            &self,
            params: alloy::sol_types::private::Vec<
                <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createOperatorSetsCall, N> {
            self.call_builder(&createOperatorSetsCall { params })
        }
        ///Creates a new call builder for the [`deregisterOperatorFromAVS`] function.
        pub fn deregisterOperatorFromAVS(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorFromAVSCall, N> {
            self.call_builder(&deregisterOperatorFromAVSCall { operator })
        }
        ///Creates a new call builder for the [`deregisterOperatorFromOperatorSets`] function.
        pub fn deregisterOperatorFromOperatorSets(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorFromOperatorSetsCall, N>
        {
            self.call_builder(&deregisterOperatorFromOperatorSetsCall {
                operator,
                operatorSetIds,
            })
        }
        ///Creates a new call builder for the [`getOperatorRestakedStrategies`] function.
        pub fn getOperatorRestakedStrategies(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorRestakedStrategiesCall, N> {
            self.call_builder(&getOperatorRestakedStrategiesCall { operator })
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
        ) -> alloy_contract::SolCallBuilder<T, &P, incredibleSquaringTaskManagerCall, N> {
            self.call_builder(&incredibleSquaringTaskManagerCall {})
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
            self.call_builder(&proposeNewSlasherCall { newSlasher })
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
            self.call_builder(&registerOperatorToAVSCall {
                operator,
                operatorSignature,
            })
        }
        ///Creates a new call builder for the [`removeStrategiesFromOperatorSet`] function.
        pub fn removeStrategiesFromOperatorSet(
            &self,
            operatorSetId: u32,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeStrategiesFromOperatorSetCall, N> {
            self.call_builder(&removeStrategiesFromOperatorSetCall {
                operatorSetId,
                strategies,
            })
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
        ///Creates a new call builder for the [`setAVSRegistrar`] function.
        pub fn setAVSRegistrar(
            &self,
            registrar: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAVSRegistrarCall, N> {
            self.call_builder(&setAVSRegistrarCall { registrar })
        }
        ///Creates a new call builder for the [`setRewardsInitiator`] function.
        pub fn setRewardsInitiator(
            &self,
            newRewardsInitiator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setRewardsInitiatorCall, N> {
            self.call_builder(&setRewardsInitiatorCall {
                newRewardsInitiator,
            })
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
            self.call_builder(&updateAVSMetadataURICall { _metadataURI })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > IncredibleSquaringServiceManagerInstance<T, P, N>
    {
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
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
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
        pub fn SlasherProposed_filter(&self) -> alloy_contract::Event<T, &P, SlasherProposed, N> {
            self.event_filter::<SlasherProposed>()
        }
        ///Creates a new event filter for the [`SlasherUpdated`] event.
        pub fn SlasherUpdated_filter(&self) -> alloy_contract::Event<T, &P, SlasherUpdated, N> {
            self.event_filter::<SlasherUpdated>()
        }
    }
}
