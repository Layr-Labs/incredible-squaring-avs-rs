///Module containing a contract's types and functions.
/**

```solidity
library BN254 {
    struct G1Point { uint256 X; uint256 Y; }
    struct G2Point { uint256[2] X; uint256[2] Y; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod BN254 {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct G1Point { uint256 X; uint256 Y; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone, Debug)]
    pub struct G1Point {
        pub X: alloy::sol_types::private::primitives::aliases::U256,
        pub Y: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<G1Point> for UnderlyingRustTuple<'_> {
            fn from(value: G1Point) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G1Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    X: tuple.0,
                    Y: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G1Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G1Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.X,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.Y,
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
        impl alloy_sol_types::SolType for G1Point {
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
        impl alloy_sol_types::SolStruct for G1Point {
            const NAME: &'static str = "G1Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("G1Point(uint256 X,uint256 Y)")
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
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.X)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.Y)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G1Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.X)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.Y)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.X, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.Y, out);
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
    struct G2Point { uint256[2] X; uint256[2] Y; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G2Point {
        pub X: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
        pub Y: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
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
            alloy::sol_types::sol_data::FixedArray<alloy::sol_types::sol_data::Uint<256>, 2usize>,
            alloy::sol_types::sol_data::FixedArray<alloy::sol_types::sol_data::Uint<256>, 2usize>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
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
        impl ::core::convert::From<G2Point> for UnderlyingRustTuple<'_> {
            fn from(value: G2Point) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G2Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    X: tuple.0,
                    Y: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G2Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G2Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.X),
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.Y),
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
        impl alloy_sol_types::SolType for G2Point {
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
        impl alloy_sol_types::SolStruct for G2Point {
            const NAME: &'static str = "G2Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("G2Point(uint256[2] X,uint256[2] Y)")
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
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.X)
                    .0,
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.Y)
                    .0,
                ]
                .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G2Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.X
                    )
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.Y
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.X, out
                );
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.Y, out
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
    /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

    See the [wrapper's documentation](`BN254Instance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BN254Instance<T, P, N> {
        BN254Instance::<T, P, N>::new(address, provider)
    }
    /**A [`BN254`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`BN254`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BN254Instance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BN254Instance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BN254Instance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > BN254Instance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

        See the [wrapper's documentation](`BN254Instance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> BN254Instance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BN254Instance<T, P, N> {
            BN254Instance {
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
        > BN254Instance<T, P, N>
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
        > BN254Instance<T, P, N>
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
library IBLSSignatureChecker {
    struct NonSignerStakesAndSignature { uint32[] nonSignerQuorumBitmapIndices; BN254.G1Point[] nonSignerPubkeys; BN254.G1Point[] quorumApks; BN254.G2Point apkG2; BN254.G1Point sigma; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    struct QuorumStakeTotals { uint96[] signedStakeForQuorum; uint96[] totalStakeForQuorum; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IBLSSignatureChecker {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct NonSignerStakesAndSignature { uint32[] nonSignerQuorumBitmapIndices; BN254.G1Point[] nonSignerPubkeys; BN254.G1Point[] quorumApks; BN254.G2Point apkG2; BN254.G1Point sigma; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonSignerStakesAndSignature {
        pub nonSignerQuorumBitmapIndices: alloy::sol_types::private::Vec<u32>,
        pub nonSignerPubkeys:
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
        pub quorumApks:
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub quorumApkIndices: alloy::sol_types::private::Vec<u32>,
        pub totalStakeIndices: alloy::sol_types::private::Vec<u32>,
        pub nonSignerStakeIndices:
            alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<BN254::G1Point>,
            alloy::sol_types::sol_data::Array<BN254::G1Point>,
            BN254::G2Point,
            BN254::G1Point,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            >,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
            <BN254::G2Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
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
        impl ::core::convert::From<NonSignerStakesAndSignature> for UnderlyingRustTuple<'_> {
            fn from(value: NonSignerStakesAndSignature) -> Self {
                (
                    value.nonSignerQuorumBitmapIndices,
                    value.nonSignerPubkeys,
                    value.quorumApks,
                    value.apkG2,
                    value.sigma,
                    value.quorumApkIndices,
                    value.totalStakeIndices,
                    value.nonSignerStakeIndices,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonSignerStakesAndSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    nonSignerQuorumBitmapIndices: tuple.0,
                    nonSignerPubkeys: tuple.1,
                    quorumApks: tuple.2,
                    apkG2: tuple.3,
                    sigma: tuple.4,
                    quorumApkIndices: tuple.5,
                    totalStakeIndices: tuple.6,
                    nonSignerStakeIndices: tuple.7,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for NonSignerStakesAndSignature {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for NonSignerStakesAndSignature {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.nonSignerQuorumBitmapIndices,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerPubkeys),
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumApks),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.apkG2),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.sigma),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumApkIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalStakeIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerStakeIndices),
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
        impl alloy_sol_types::SolType for NonSignerStakesAndSignature {
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
        impl alloy_sol_types::SolStruct for NonSignerStakesAndSignature {
            const NAME: &'static str = "NonSignerStakesAndSignature";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "NonSignerStakesAndSignature(uint32[] nonSignerQuorumBitmapIndices,BN254.G1Point[] nonSignerPubkeys,BN254.G1Point[] quorumApks,BN254.G2Point apkG2,BN254.G1Point sigma,uint32[] quorumApkIndices,uint32[] totalStakeIndices,uint32[][] nonSignerStakeIndices)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(4);
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G2Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G2Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerQuorumBitmapIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerPubkeys,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.quorumApks)
                        .0,
                    <BN254::G2Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.apkG2,
                        )
                        .0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sigma,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.quorumApkIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalStakeIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerStakeIndices,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for NonSignerStakesAndSignature {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerQuorumBitmapIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerPubkeys,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumApks,
                    )
                    + <BN254::G2Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.apkG2,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sigma,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumApkIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalStakeIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerStakeIndices,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerQuorumBitmapIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    BN254::G1Point,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerPubkeys,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    BN254::G1Point,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumApks,
                    out,
                );
                <BN254::G2Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.apkG2,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sigma,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumApkIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalStakeIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerStakeIndices,
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
    struct QuorumStakeTotals { uint96[] signedStakeForQuorum; uint96[] totalStakeForQuorum; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumStakeTotals {
        pub signedStakeForQuorum:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
        pub totalStakeForQuorum:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
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
        impl ::core::convert::From<QuorumStakeTotals> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumStakeTotals) -> Self {
                (value.signedStakeForQuorum, value.totalStakeForQuorum)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumStakeTotals {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signedStakeForQuorum: tuple.0,
                    totalStakeForQuorum: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for QuorumStakeTotals {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for QuorumStakeTotals {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::tokenize(&self.signedStakeForQuorum),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalStakeForQuorum),
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
        impl alloy_sol_types::SolType for QuorumStakeTotals {
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
        impl alloy_sol_types::SolStruct for QuorumStakeTotals {
            const NAME: &'static str = "QuorumStakeTotals";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QuorumStakeTotals(uint96[] signedStakeForQuorum,uint96[] totalStakeForQuorum)",
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signedStakeForQuorum,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalStakeForQuorum,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for QuorumStakeTotals {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signedStakeForQuorum,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalStakeForQuorum,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<96>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signedStakeForQuorum,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<96>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalStakeForQuorum,
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
    /**Creates a new wrapper around an on-chain [`IBLSSignatureChecker`](self) contract instance.

    See the [wrapper's documentation](`IBLSSignatureCheckerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IBLSSignatureCheckerInstance<T, P, N> {
        IBLSSignatureCheckerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IBLSSignatureChecker`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IBLSSignatureChecker`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IBLSSignatureCheckerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IBLSSignatureCheckerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IBLSSignatureCheckerInstance")
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
        > IBLSSignatureCheckerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IBLSSignatureChecker`](self) contract instance.

        See the [wrapper's documentation](`IBLSSignatureCheckerInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IBLSSignatureCheckerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IBLSSignatureCheckerInstance<T, P, N> {
            IBLSSignatureCheckerInstance {
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
        > IBLSSignatureCheckerInstance<T, P, N>
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
        > IBLSSignatureCheckerInstance<T, P, N>
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
library IIncredibleSquaringTaskManager {
    struct Task { uint256 numberToBeSquared; uint32 taskCreatedBlock; bytes quorumNumbers; uint32 quorumThresholdPercentage; }
    struct TaskResponse { uint32 referenceTaskIndex; uint256 numberSquared; }
    struct TaskResponseMetadata { uint32 taskResponsedBlock; bytes32 hashOfNonSigners; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IIncredibleSquaringTaskManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct Task { uint256 numberToBeSquared; uint32 taskCreatedBlock; bytes quorumNumbers; uint32 quorumThresholdPercentage; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone, Debug)]
    pub struct Task {
        pub numberToBeSquared: alloy::sol_types::private::primitives::aliases::U256,
        pub taskCreatedBlock: u32,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        pub quorumThresholdPercentage: u32,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            u32,
            alloy::sol_types::private::Bytes,
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
        impl ::core::convert::From<Task> for UnderlyingRustTuple<'_> {
            fn from(value: Task) -> Self {
                (
                    value.numberToBeSquared,
                    value.taskCreatedBlock,
                    value.quorumNumbers,
                    value.quorumThresholdPercentage,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Task {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    numberToBeSquared: tuple.0,
                    taskCreatedBlock: tuple.1,
                    quorumNumbers: tuple.2,
                    quorumThresholdPercentage: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Task {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Task {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.numberToBeSquared,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.taskCreatedBlock,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumThresholdPercentage,
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
        impl alloy_sol_types::SolType for Task {
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
        impl alloy_sol_types::SolStruct for Task {
            const NAME: &'static str = "Task";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Task(uint256 numberToBeSquared,uint32 taskCreatedBlock,bytes quorumNumbers,uint32 quorumThresholdPercentage)",
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
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.numberToBeSquared,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.taskCreatedBlock,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.quorumNumbers,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.quorumThresholdPercentage,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Task {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.numberToBeSquared,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.taskCreatedBlock,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumNumbers,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumThresholdPercentage,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.numberToBeSquared,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.taskCreatedBlock,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumNumbers,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumThresholdPercentage,
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
    struct TaskResponse { uint32 referenceTaskIndex; uint256 numberSquared; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TaskResponse {
        pub referenceTaskIndex: u32,
        pub numberSquared: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, alloy::sol_types::private::primitives::aliases::U256);
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
        impl ::core::convert::From<TaskResponse> for UnderlyingRustTuple<'_> {
            fn from(value: TaskResponse) -> Self {
                (value.referenceTaskIndex, value.numberSquared)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TaskResponse {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    referenceTaskIndex: tuple.0,
                    numberSquared: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TaskResponse {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TaskResponse {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.referenceTaskIndex,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.numberSquared,
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
        impl alloy_sol_types::SolType for TaskResponse {
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
        impl alloy_sol_types::SolStruct for TaskResponse {
            const NAME: &'static str = "TaskResponse";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TaskResponse(uint32 referenceTaskIndex,uint256 numberSquared)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.referenceTaskIndex,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.numberSquared)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TaskResponse {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.referenceTaskIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.numberSquared,
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
                    &rust.referenceTaskIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.numberSquared,
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
    struct TaskResponseMetadata { uint32 taskResponsedBlock; bytes32 hashOfNonSigners; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone, Debug)]
    pub struct TaskResponseMetadata {
        pub taskResponsedBlock: u32,
        pub hashOfNonSigners: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, alloy::sol_types::private::FixedBytes<32>);
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
        impl ::core::convert::From<TaskResponseMetadata> for UnderlyingRustTuple<'_> {
            fn from(value: TaskResponseMetadata) -> Self {
                (value.taskResponsedBlock, value.hashOfNonSigners)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TaskResponseMetadata {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    taskResponsedBlock: tuple.0,
                    hashOfNonSigners: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TaskResponseMetadata {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TaskResponseMetadata {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.taskResponsedBlock),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.hashOfNonSigners),
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
        impl alloy_sol_types::SolType for TaskResponseMetadata {
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
        impl alloy_sol_types::SolStruct for TaskResponseMetadata {
            const NAME: &'static str = "TaskResponseMetadata";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TaskResponseMetadata(uint32 taskResponsedBlock,bytes32 hashOfNonSigners)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.taskResponsedBlock,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.hashOfNonSigners,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TaskResponseMetadata {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.taskResponsedBlock,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.hashOfNonSigners,
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
                    &rust.taskResponsedBlock,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.hashOfNonSigners,
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
    use serde::Deserialize;
    use serde::Serialize;
    /**Creates a new wrapper around an on-chain [`IIncredibleSquaringTaskManager`](self) contract instance.

    See the [wrapper's documentation](`IIncredibleSquaringTaskManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IIncredibleSquaringTaskManagerInstance<T, P, N> {
        IIncredibleSquaringTaskManagerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IIncredibleSquaringTaskManager`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IIncredibleSquaringTaskManager`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IIncredibleSquaringTaskManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IIncredibleSquaringTaskManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IIncredibleSquaringTaskManagerInstance")
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
        > IIncredibleSquaringTaskManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IIncredibleSquaringTaskManager`](self) contract instance.

        See the [wrapper's documentation](`IIncredibleSquaringTaskManagerInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IIncredibleSquaringTaskManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IIncredibleSquaringTaskManagerInstance<T, P, N> {
            IIncredibleSquaringTaskManagerInstance {
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
        > IIncredibleSquaringTaskManagerInstance<T, P, N>
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
        > IIncredibleSquaringTaskManagerInstance<T, P, N>
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
library OperatorStateRetriever {
    struct CheckSignaturesIndices { uint32[] nonSignerQuorumBitmapIndices; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    struct Operator { address operator; bytes32 operatorId; uint96 stake; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod OperatorStateRetriever {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct CheckSignaturesIndices { uint32[] nonSignerQuorumBitmapIndices; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CheckSignaturesIndices {
        pub nonSignerQuorumBitmapIndices: alloy::sol_types::private::Vec<u32>,
        pub quorumApkIndices: alloy::sol_types::private::Vec<u32>,
        pub totalStakeIndices: alloy::sol_types::private::Vec<u32>,
        pub nonSignerStakeIndices:
            alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            >,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
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
        impl ::core::convert::From<CheckSignaturesIndices> for UnderlyingRustTuple<'_> {
            fn from(value: CheckSignaturesIndices) -> Self {
                (
                    value.nonSignerQuorumBitmapIndices,
                    value.quorumApkIndices,
                    value.totalStakeIndices,
                    value.nonSignerStakeIndices,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CheckSignaturesIndices {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    nonSignerQuorumBitmapIndices: tuple.0,
                    quorumApkIndices: tuple.1,
                    totalStakeIndices: tuple.2,
                    nonSignerStakeIndices: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CheckSignaturesIndices {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CheckSignaturesIndices {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.nonSignerQuorumBitmapIndices,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumApkIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalStakeIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerStakeIndices),
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
        impl alloy_sol_types::SolType for CheckSignaturesIndices {
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
        impl alloy_sol_types::SolStruct for CheckSignaturesIndices {
            const NAME: &'static str = "CheckSignaturesIndices";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CheckSignaturesIndices(uint32[] nonSignerQuorumBitmapIndices,uint32[] quorumApkIndices,uint32[] totalStakeIndices,uint32[][] nonSignerStakeIndices)",
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerQuorumBitmapIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.quorumApkIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalStakeIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerStakeIndices,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CheckSignaturesIndices {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerQuorumBitmapIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumApkIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalStakeIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerStakeIndices,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerQuorumBitmapIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumApkIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalStakeIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerStakeIndices,
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
    struct Operator { address operator; bytes32 operatorId; uint96 stake; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Operator {
        pub operator: alloy::sol_types::private::Address,
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub stake: alloy::sol_types::private::primitives::aliases::U96,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<Operator> for UnderlyingRustTuple<'_> {
            fn from(value: Operator) -> Self {
                (value.operator, value.operatorId, value.stake)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Operator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operator: tuple.0,
                    operatorId: tuple.1,
                    stake: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Operator {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Operator {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.stake),
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
        impl alloy_sol_types::SolType for Operator {
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
        impl alloy_sol_types::SolStruct for Operator {
            const NAME: &'static str = "Operator";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Operator(address operator,bytes32 operatorId,uint96 stake)",
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.stake)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Operator {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.stake)
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
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stake,
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
    /**Creates a new wrapper around an on-chain [`OperatorStateRetriever`](self) contract instance.

    See the [wrapper's documentation](`OperatorStateRetrieverInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> OperatorStateRetrieverInstance<T, P, N> {
        OperatorStateRetrieverInstance::<T, P, N>::new(address, provider)
    }
    /**A [`OperatorStateRetriever`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`OperatorStateRetriever`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OperatorStateRetrieverInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for OperatorStateRetrieverInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OperatorStateRetrieverInstance")
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
        > OperatorStateRetrieverInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`OperatorStateRetriever`](self) contract instance.

        See the [wrapper's documentation](`OperatorStateRetrieverInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> OperatorStateRetrieverInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OperatorStateRetrieverInstance<T, P, N> {
            OperatorStateRetrieverInstance {
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
        > OperatorStateRetrieverInstance<T, P, N>
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
        > OperatorStateRetrieverInstance<T, P, N>
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
library BN254 {
    struct G1Point {
        uint256 X;
        uint256 Y;
    }
    struct G2Point {
        uint256[2] X;
        uint256[2] Y;
    }
}

library IBLSSignatureChecker {
    struct NonSignerStakesAndSignature {
        uint32[] nonSignerQuorumBitmapIndices;
        BN254.G1Point[] nonSignerPubkeys;
        BN254.G1Point[] quorumApks;
        BN254.G2Point apkG2;
        BN254.G1Point sigma;
        uint32[] quorumApkIndices;
        uint32[] totalStakeIndices;
        uint32[][] nonSignerStakeIndices;
    }
    struct QuorumStakeTotals {
        uint96[] signedStakeForQuorum;
        uint96[] totalStakeForQuorum;
    }
}

library IIncredibleSquaringTaskManager {
    struct Task {
        uint256 numberToBeSquared;
        uint32 taskCreatedBlock;
        bytes quorumNumbers;
        uint32 quorumThresholdPercentage;
    }
    struct TaskResponse {
        uint32 referenceTaskIndex;
        uint256 numberSquared;
    }
    struct TaskResponseMetadata {
        uint32 taskResponsedBlock;
        bytes32 hashOfNonSigners;
    }
}

library OperatorStateRetriever {
    struct CheckSignaturesIndices {
        uint32[] nonSignerQuorumBitmapIndices;
        uint32[] quorumApkIndices;
        uint32[] totalStakeIndices;
        uint32[][] nonSignerStakeIndices;
    }
    struct Operator {
        address operator;
        bytes32 operatorId;
        uint96 stake;
    }
}

interface IncredibleSquaringTaskManager {
    error CurrentlyPaused();
    error InputAddressZero();
    error InvalidNewPausedStatus();
    error OnlyPauser();
    error OnlyUnpauser();

    event Initialized(uint8 version);
    event NewTaskCreated(uint32 indexed taskIndex, IIncredibleSquaringTaskManager.Task task);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event StaleStakesForbiddenUpdate(bool value);
    event TaskChallengedSuccessfully(uint32 indexed taskIndex, address indexed challenger);
    event TaskChallengedUnsuccessfully(uint32 indexed taskIndex, address indexed challenger);
    event TaskCompleted(uint32 indexed taskIndex);
    event TaskResponded(IIncredibleSquaringTaskManager.TaskResponse taskResponse, IIncredibleSquaringTaskManager.TaskResponseMetadata taskResponseMetadata);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _registryCoordinator, address _pauserRegistry, uint32 _taskResponseWindowBlock, address _instantSlasher, address _allocationManager, address _serviceManager);

    function TASK_CHALLENGE_WINDOW_BLOCK() external view returns (uint32);
    function TASK_RESPONSE_WINDOW_BLOCK() external view returns (uint32);
    function aggregator() external view returns (address);
    function allTaskHashes(uint32) external view returns (bytes32);
    function allTaskResponses(uint32) external view returns (bytes32);
    function allocationManager() external view returns (address);
    function blsApkRegistry() external view returns (address);
    function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureChecker.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureChecker.QuorumStakeTotals memory, bytes32);
    function createNewTask(uint256 numberToBeSquared, uint32 quorumThresholdPercentage, bytes memory quorumNumbers) external;
    function delegation() external view returns (address);
    function generator() external view returns (address);
    function getBatchOperatorFromId(address registryCoordinator, bytes32[] memory operatorIds) external view returns (address[] memory operators);
    function getBatchOperatorId(address registryCoordinator, address[] memory operators) external view returns (bytes32[] memory operatorIds);
    function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes memory quorumNumbers, bytes32[] memory nonSignerOperatorIds) external view returns (OperatorStateRetriever.CheckSignaturesIndices memory);
    function getOperatorState(address registryCoordinator, bytes memory quorumNumbers, uint32 blockNumber) external view returns (OperatorStateRetriever.Operator[][] memory);
    function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) external view returns (uint256, OperatorStateRetriever.Operator[][] memory);
    function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] memory operatorIds, uint32 blockNumber) external view returns (uint256[] memory);
    function getTaskResponseWindowBlock() external view returns (uint32);
    function initialize(address initialOwner, address _aggregator, address _generator) external;
    function instantSlasher() external view returns (address);
    function latestTaskNum() external view returns (uint32);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function raiseAndResolveChallenge(IIncredibleSquaringTaskManager.Task memory task, IIncredibleSquaringTaskManager.TaskResponse memory taskResponse, IIncredibleSquaringTaskManager.TaskResponseMetadata memory taskResponseMetadata, BN254.G1Point[] memory pubkeysOfNonSigningOperators) external;
    function registryCoordinator() external view returns (address);
    function renounceOwnership() external;
    function respondToTask(IIncredibleSquaringTaskManager.Task memory task, IIncredibleSquaringTaskManager.TaskResponse memory taskResponse, IBLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature) external;
    function serviceManager() external view returns (address);
    function setStaleStakesForbidden(bool value) external;
    function stakeRegistry() external view returns (address);
    function staleStakesForbidden() external view returns (bool);
    function taskNumber() external view returns (uint32);
    function taskSuccesfullyChallenged(uint32) external view returns (bool);
    function transferOwnership(address newOwner) external;
    function trySignatureAndApkVerification(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma) external view returns (bool pairingSuccessful, bool siganatureIsValid);
    function unpause(uint256 newPausedStatus) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_registryCoordinator",
        "type": "address",
        "internalType": "contract IRegistryCoordinator"
      },
      {
        "name": "_pauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      },
      {
        "name": "_taskResponseWindowBlock",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_instantSlasher",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_allocationManager",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_serviceManager",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "TASK_CHALLENGE_WINDOW_BLOCK",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "TASK_RESPONSE_WINDOW_BLOCK",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "aggregator",
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
    "name": "allTaskHashes",
    "inputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "allTaskResponses",
    "inputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
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
    "name": "blsApkRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IBLSApkRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkSignatures",
    "inputs": [
      {
        "name": "msgHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "referenceBlockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IBLSSignatureChecker.NonSignerStakesAndSignature",
        "components": [
          {
            "name": "nonSignerQuorumBitmapIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerPubkeys",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApks",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "apkG2",
            "type": "tuple",
            "internalType": "struct BN254.G2Point",
            "components": [
              {
                "name": "X",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              },
              {
                "name": "Y",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              }
            ]
          },
          {
            "name": "sigma",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApkIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "totalStakeIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerStakeIndices",
            "type": "uint32[][]",
            "internalType": "uint32[][]"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IBLSSignatureChecker.QuorumStakeTotals",
        "components": [
          {
            "name": "signedStakeForQuorum",
            "type": "uint96[]",
            "internalType": "uint96[]"
          },
          {
            "name": "totalStakeForQuorum",
            "type": "uint96[]",
            "internalType": "uint96[]"
          }
        ]
      },
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "createNewTask",
    "inputs": [
      {
        "name": "numberToBeSquared",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "quorumThresholdPercentage",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IDelegationManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "generator",
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
    "name": "getBatchOperatorFromId",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract IRegistryCoordinator"
      },
      {
        "name": "operatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getBatchOperatorId",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract IRegistryCoordinator"
      },
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [
      {
        "name": "operatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCheckSignaturesIndices",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract IRegistryCoordinator"
      },
      {
        "name": "referenceBlockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "nonSignerOperatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct OperatorStateRetriever.CheckSignaturesIndices",
        "components": [
          {
            "name": "nonSignerQuorumBitmapIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "quorumApkIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "totalStakeIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerStakeIndices",
            "type": "uint32[][]",
            "internalType": "uint32[][]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorState",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract IRegistryCoordinator"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[][]",
        "internalType": "struct OperatorStateRetriever.Operator[][]",
        "components": [
          {
            "name": "operator",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "stake",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorState",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract IRegistryCoordinator"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "tuple[][]",
        "internalType": "struct OperatorStateRetriever.Operator[][]",
        "components": [
          {
            "name": "operator",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "stake",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQuorumBitmapsAtBlockNumber",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract IRegistryCoordinator"
      },
      {
        "name": "operatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTaskResponseWindowBlock",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "initialOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_aggregator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_generator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "instantSlasher",
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
    "name": "latestTaskNum",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "pause",
    "inputs": [
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "pauseAll",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [
      {
        "name": "index",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
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
    "name": "paused",
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
    "name": "pauserRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "raiseAndResolveChallenge",
    "inputs": [
      {
        "name": "task",
        "type": "tuple",
        "internalType": "struct IIncredibleSquaringTaskManager.Task",
        "components": [
          {
            "name": "numberToBeSquared",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "quorumNumbers",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "quorumThresholdPercentage",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "taskResponse",
        "type": "tuple",
        "internalType": "struct IIncredibleSquaringTaskManager.TaskResponse",
        "components": [
          {
            "name": "referenceTaskIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "numberSquared",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "taskResponseMetadata",
        "type": "tuple",
        "internalType": "struct IIncredibleSquaringTaskManager.TaskResponseMetadata",
        "components": [
          {
            "name": "taskResponsedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "hashOfNonSigners",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "pubkeysOfNonSigningOperators",
        "type": "tuple[]",
        "internalType": "struct BN254.G1Point[]",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
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
    "name": "registryCoordinator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IRegistryCoordinator"
      }
    ],
    "stateMutability": "view"
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
    "name": "respondToTask",
    "inputs": [
      {
        "name": "task",
        "type": "tuple",
        "internalType": "struct IIncredibleSquaringTaskManager.Task",
        "components": [
          {
            "name": "numberToBeSquared",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "quorumNumbers",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "quorumThresholdPercentage",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "taskResponse",
        "type": "tuple",
        "internalType": "struct IIncredibleSquaringTaskManager.TaskResponse",
        "components": [
          {
            "name": "referenceTaskIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "numberSquared",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "nonSignerStakesAndSignature",
        "type": "tuple",
        "internalType": "struct IBLSSignatureChecker.NonSignerStakesAndSignature",
        "components": [
          {
            "name": "nonSignerQuorumBitmapIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerPubkeys",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApks",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "apkG2",
            "type": "tuple",
            "internalType": "struct BN254.G2Point",
            "components": [
              {
                "name": "X",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              },
              {
                "name": "Y",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              }
            ]
          },
          {
            "name": "sigma",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApkIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "totalStakeIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerStakeIndices",
            "type": "uint32[][]",
            "internalType": "uint32[][]"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "serviceManager",
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
    "name": "setStaleStakesForbidden",
    "inputs": [
      {
        "name": "value",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "stakeRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStakeRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "staleStakesForbidden",
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
    "name": "taskNumber",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "taskSuccesfullyChallenged",
    "inputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
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
    "name": "trySignatureAndApkVerification",
    "inputs": [
      {
        "name": "msgHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "apk",
        "type": "tuple",
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "apkG2",
        "type": "tuple",
        "internalType": "struct BN254.G2Point",
        "components": [
          {
            "name": "X",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          },
          {
            "name": "Y",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          }
        ]
      },
      {
        "name": "sigma",
        "type": "tuple",
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "pairingSuccessful",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "siganatureIsValid",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "unpause",
    "inputs": [
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "NewTaskCreated",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "task",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IIncredibleSquaringTaskManager.Task",
        "components": [
          {
            "name": "numberToBeSquared",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "quorumNumbers",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "quorumThresholdPercentage",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
    "name": "Paused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StaleStakesForbiddenUpdate",
    "inputs": [
      {
        "name": "value",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TaskChallengedSuccessfully",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "challenger",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TaskChallengedUnsuccessfully",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "challenger",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TaskCompleted",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TaskResponded",
    "inputs": [
      {
        "name": "taskResponse",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IIncredibleSquaringTaskManager.TaskResponse",
        "components": [
          {
            "name": "referenceTaskIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "numberSquared",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "taskResponseMetadata",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IIncredibleSquaringTaskManager.TaskResponseMetadata",
        "components": [
          {
            "name": "taskResponsedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "hashOfNonSigners",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Unpaused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "CurrentlyPaused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputAddressZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNewPausedStatus",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyPauser",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyUnpauser",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IncredibleSquaringTaskManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610140806040523461024f5760c081615417803803809161002082856102fa565b83398101031261024f5780516001600160a01b038116919082810361024f5760208201516001600160a01b0381169390919084830361024f5760408401519163ffffffff8316830361024f5761007860608601610331565b9361009160a061008a60808901610331565b9701610331565b96156102eb5760805260a052604051636830483560e01b8152602081600481855afa90811561025b575f916102a8575b5060c052604051632efa2ca360e11b815290602090829060049082905afa90811561025b575f91610266575b5060e05260c05160405163df5cf72360e01b815290602090829060049082906001600160a01b03165afa90811561025b575f91610215575b50610100526101205260cf80546001600160a01b03199081166001600160a01b039384161790915560d0805482169383169390931790925560d1805490921692169190911790556040516150d1908161034682396080518181816102ba01528181610de0015281816119ef0152611e30015260a0518181816109530152818161181c015281816135ba01528181613dc901528181613e9f0152614431015260c0518181816115f9015281816141a101526142ec015260e0518181816115b5015281816134da01526140dd015261010051818181611d2c0152613fb2015261012051818181610585015261120d0152f35b90506020813d602011610253575b81610230602093836102fa565b8101031261024f57516001600160a01b038116810361024f575f610125565b5f80fd5b3d9150610223565b6040513d5f823e3d90fd5b90506020813d6020116102a0575b81610281602093836102fa565b8101031261024f57516001600160a01b038116810361024f575f6100ed565b3d9150610274565b90506020813d6020116102e3575b816102c3602093836102fa565b8101031261024f57516001600160a01b038116810361024f5760046100c1565b3d91506102b6565b6339b190bb60e11b5f5260045ffd5b601f909101601f19168101906001600160401b0382119082101761031d57604052565b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b038216820361024f5756fe60806040526004361015610011575f80fd5b5f3560e01c8063136439dd1461028a578063171f1d5b146102855780631ad43189146101db578063245a7bfc146102805780632cb223d51461027b5780632d89f6fc1461027657806331b36bd9146102715780633563b0d11461026c5780633998fdd314610267578063416c7e5e146102625780634d2b57fe1461025d5780634f739f7414610258578063595c6a67146102535780635ac86ab71461024e5780635baec9a0146102495780635c155662146102445780635c975abb1461023f5780635decc3f51461023a5780635df459461461023557806368304835146102305780636b532e9e1461022b5780636b92787e146102265780636d14a987146102215780636efb46361461021c578063715018a61461021757806372d18e8d146102085780637afa1eed14610212578063886f11951461020d5780638b00ce7c146102085780638da5cb5b146102035780639b290e98146101fe578063b98d0908146101f9578063c0c53b8b146101f4578063ca8aa7c7146101ef578063cefdc1d4146101ea578063df5cf723146101e5578063f2fde38b146101e0578063f5c9899d146101db578063f63c5bab146101d65763fabc1cbc146101d1575f80fd5b611e07565b611dec565b610569565b611d5b565b611d17565b611bd3565b611b94565b611a90565b611a6e565b611a46565b611a1e565b61198f565b6119da565b6119b2565b611934565b611887565b611807565b611699565b611628565b6115e4565b6115a0565b611562565b611545565b6113cc565b6110e2565b610e33565b610db5565b610d0e565b610b03565b610921565b6108ef565b610875565b6106cb565b610623565b6105ea565b6105a9565b610501565b3461034a57602036600319011261034a5760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156103455761031492610300915f91610316575b50611f09565b61030f60665482811614611f1f565b61464e565b005b610338915060203d60201161033e575b610330818361039d565b810190611ee9565b5f6102fa565b503d610326565b611efe565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761037d57604052565b61034e565b608081019081106001600160401b0382111761037d57604052565b90601f801991011681019081106001600160401b0382111761037d57604052565b604051906103ce6101008361039d565b565b604051906103ce60408361039d565b604051906103ce60608361039d565b906103ce604051928361039d565b60409060e319011261034a576040519061041582610362565b60e4358252610104356020830152565b919082604091031261034a5760405161043d81610362565b6020808294803584520135910152565b9080601f8301121561034a576040519161046860408461039d565b82906040810192831161034a57905b8282106104845750505090565b8135815260209182019101610477565b90608060631983011261034a576040516104ad81610362565b60206104c882946104bf81606461044d565b845260a461044d565b910152565b919060808382031261034a5760206104c8604051926104eb84610362565b604084966104f9838261044d565b86520161044d565b3461034a5761012036600319011261034a57600435604036602319011261034a57610559604091825161053381610362565b6024358152604435602082015261054936610494565b90610553366103fc565b92611f73565b8251911515825215156020820152f35b3461034a575f36600319011261034a57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b3461034a575f36600319011261034a5760cd546040516001600160a01b039091168152602090f35b63ffffffff81160361034a57565b35906103ce826105d1565b3461034a57602036600319011261034a5763ffffffff60043561060c816105d1565b165f5260cb602052602060405f2054604051908152f35b3461034a57602036600319011261034a5763ffffffff600435610645816105d1565b165f5260ca602052602060405f2054604051908152f35b6001600160a01b0381160361034a57565b6001600160401b03811161037d5760051b60200190565b90602080835192838152019201905f5b8181106106a15750505090565b8251845260209384019390920191600101610694565b9060206106c8928181520190610684565b90565b3461034a57604036600319011261034a576004356106e88161065c565b602435906001600160401b03821161034a573660238301121561034a578160040135916107148361066d565b92610722604051948561039d565b8084526024602085019160051b8301019136831161034a57602401905b8282106107635761075f61075386866120dd565b604051918291826106b7565b0390f35b6020809183356107728161065c565b81520191019061073f565b6001600160401b03811161037d57601f01601f191660200190565b9291926107a48261077d565b916107b2604051938461039d565b82948184528183011161034a578281602093845f960137010152565b9080602083519182815201916020808360051b8301019401925f915b8383106107f957505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b80821061083957505050602080600192970193019301919392906107ea565b909192602060606001926001600160601b0360408851868060a01b0381511684528581015186850152015116604082015201940192019061081a565b3461034a57606036600319011261034a576004356108928161065c565b6024356001600160401b03811161034a573660238201121561034a5761075f916108c96108db923690602481600401359101610798565b604435916108d6836105d1565b61231b565b6040519182916020835260208301906107ce565b3461034a575f36600319011261034a5760d1546040516001600160a01b039091168152602090f35b8015150361034a57565b3461034a57602036600319011261034a5760043561093e81610917565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610345575f91610a35575b506001600160a01b031633036109a45761031490614b07565b60405162461bcd60e51b815260206004820152605c60248201527f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e6160448201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560648201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000608482015260a490fd5b610a57915060203d602011610a5d575b610a4f818361039d565b8101906121a2565b5f61098b565b503d610a45565b9080601f8301121561034a578135610a7b8161066d565b92610a89604051948561039d565b81845260208085019260051b82010192831161034a57602001905b828210610ab15750505090565b8135815260209182019101610aa4565b60206040818301928281528451809452019201905f5b818110610ae45750505090565b82516001600160a01b0316845260209384019390920191600101610ad7565b3461034a57604036600319011261034a57600435610b208161065c565b6024356001600160401b03811161034a57610b3f903690600401610a64565b610b49815161207b565b916001600160a01b03165f5b8251811015610be657806020610b6e610b8e93866120ba565b5160405180948192630a5aec1960e21b8352600483019190602083019252565b0381865afa91821561034557600192610bc2915f91610bc8575b50610bb383886120ba565b6001600160a01b039091169052565b01610b55565b610be0915060203d8111610a5d57610a4f818361039d565b5f610ba8565b6040518061075f8682610ac1565b9181601f8401121561034a578235916001600160401b03831161034a576020838186019501011161034a57565b90602080835192838152019201905f5b818110610c3e5750505090565b825163ffffffff16845260209384019390920191600101610c31565b90602082526060610ca8610c93610c7d84516080602088015260a0870190610c21565b6020850151868203601f19016040880152610c21565b6040840151858203601f190184870152610c21565b910151916080601f1982840301910152815180825260208201916020808360051b8301019401925f915b838310610ce157505050505090565b9091929394602080610cff600193601f198682030187528951610c21565b97019301930191939290610cd2565b3461034a57608036600319011261034a57600435610d2b8161065c565b60243590610d38826105d1565b6044356001600160401b03811161034a57610d57903690600401610bf4565b91606435926001600160401b03841161034a573660238501121561034a578360040135926001600160401b03841161034a573660248560051b8701011161034a5761075f956024610da99601936128b1565b60405191829182610c5a565b3461034a575f36600319011261034a5760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561034557610e20915f916103165750611f09565b61031461461a565b60ff81160361034a57565b3461034a57602036600319011261034a576020600160ff600435610e5681610e28565b161b806066541614604051908152f35b9081608091031261034a5790565b604090602319011261034a57602490565b9080601f8301121561034a578135610e9c8161066d565b92610eaa604051948561039d565b81845260208085019260051b82010192831161034a57602001905b828210610ed25750505090565b602080918335610ee1816105d1565b815201910190610ec5565b81601f8201121561034a578035610f028161066d565b92610f10604051948561039d565b81845260208085019260061b8401019281841161034a57602001915b838310610f3a575050505090565b6020604091610f498486610425565b815201920191610f2c565b9080601f8301121561034a578135610f6b8161066d565b92610f79604051948561039d565b81845260208085019260051b8201019183831161034a5760208201905b838210610fa557505050505090565b81356001600160401b03811161034a57602091610fc787848094880101610e85565b815201910190610f96565b9190916101808184031261034a57610fe86103be565b9281356001600160401b03811161034a5781611005918401610e85565b845260208201356001600160401b03811161034a5781611026918401610eec565b602085015260408201356001600160401b03811161034a578161104a918401610eec565b604085015261105c81606084016104cd565b606085015261106e8160e08401610425565b60808501526101208201356001600160401b03811161034a5781611093918401610e85565b60a08501526101408201356001600160401b03811161034a57816110b8918401610e85565b60c08501526101608201356001600160401b03811161034a576110db9201610f54565b60e0830152565b3461034a57608036600319011261034a576004356001600160401b03811161034a57611112903690600401610e66565b61111b36610e74565b906064356001600160401b03811161034a5761113b903690600401610fd2565b60cd549092906001600160a01b0316330361134e5761115e602083949301612d19565b9161126161116f6040860186612d23565b9290946111cf61118160608901612d19565b976040516111a581611197602082019485612d55565b03601f19810183528261039d565b5190206111c86111b488612d19565b63ffffffff165f5260ca60205260405f2090565b5414612ddc565b6111f96111f26111de87612d19565b63ffffffff165f5260cb60205260405f2090565b5415612e4e565b8363ffffffff43169661124361123b6112327f000000000000000000000000000000000000000000000000000000000000000086612edf565b63ffffffff1690565b891115612ef9565b6040516020810190611259816111978b85612f79565b519020613cec565b919060ff5f9616955b8281106112ec577f349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a8686866112ac6112a06103d0565b63ffffffff9094168452565b602083015260405160208101906112c88161119786868661305c565b5190206112d76111de83612d19565b556112e76040519283928361305c565b0390a1005b8061134861132461131f61131361130660019688516120ba565b516001600160601b031690565b6001600160601b031690565b612f89565b6113416113138b61133c6113068760208b01516120ba565b612fc8565b1115612feb565b0161126a565b60405162461bcd60e51b815260206004820152601d60248201527f41676772656761746f72206d757374206265207468652063616c6c65720000006044820152606490fd5b60206040818301928281528451809452019201905f5b8181106113b65750505090565b82518452602093840193909201916001016113a9565b3461034a57606036600319011261034a576004356113e98161065c565b6024356001600160401b03811161034a57611408903690600401610a64565b60443591611415836105d1565b6040516361c8a12f60e11b8152906001600160a01b03165f828061143d868860048401613086565b0381845afa918215610345575f92611521575b5061145b835161207b565b935f5b84518110156115135761147181866120ba565b519060208361148d61148384896120ba565b5163ffffffff1690565b6040516304ec635160e01b8152600481019590955263ffffffff918216602486015216604484015282606481875afa8015610345576001925f916114e5575b50828060c01b03166114de82896120ba565b520161145e565b611506915060203d811161150c575b6114fe818361039d565b8101906127a6565b5f6114cc565b503d6114f4565b6040518061075f8882611393565b61153e9192503d805f833e611536818361039d565b810190612675565b905f611450565b3461034a575f36600319011261034a576020606654604051908152f35b3461034a57602036600319011261034a5763ffffffff600435611584816105d1565b165f5260cc602052602060ff60405f2054166040519015158152f35b3461034a575f36600319011261034a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461034a575f36600319011261034a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461034a5760c036600319011261034a576004356001600160401b03811161034a57611658903690600401610e66565b61166136610e74565b90604036606319011261034a5760a4356001600160401b03811161034a57610314926116936064923690600401610eec565b92613374565b3461034a57606036600319011261034a576024356004356116b9826105d1565b6044356001600160401b03811161034a576116d8903690600401610bf4565b60ce5491939092916001600160a01b031633036117b857610314936117a293611722611729936117066137c5565b9586524363ffffffff16602087015263ffffffff166060860152565b3691610798565b604082015260405160208101906117448161119785856137e9565b5190206117596111b460c95463ffffffff1690565b5560c95463ffffffff16907f1695b8d06ec800b4615e745cfb5bd00c1f2875615d42925c3b5afa543bb24c486040518061179a63ffffffff861694826137e9565b0390a2612eaf565b63ffffffff1663ffffffff1960c954161760c955565b60405162461bcd60e51b815260206004820152602160248201527f5461736b2067656e657261746f72206d757374206265207468652063616c6c656044820152603960f91b6064820152608490fd5b3461034a575f36600319011261034a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b90602080835192838152019201905f5b8181106118685750505090565b82516001600160601b031684526020938401939092019160010161185b565b3461034a57608036600319011261034a576004356024356001600160401b03811161034a576118ba903690600401610bf4565b90916044356118c8816105d1565b606435926001600160401b03841161034a5761192a946118ef6118f5953690600401610fd2565b93613cec565b6040519283926040845260206119168251604080880152608087019061184b565b910151848203603f1901606086015261184b565b9060208301520390f35b3461034a575f36600319011261034a5761194c614d68565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461034a575f36600319011261034a57602063ffffffff60c95416604051908152f35b3461034a575f36600319011261034a5760ce546040516001600160a01b039091168152602090f35b3461034a575f36600319011261034a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461034a575f36600319011261034a576033546040516001600160a01b039091168152602090f35b3461034a575f36600319011261034a5760cf546040516001600160a01b039091168152602090f35b3461034a575f36600319011261034a57602060ff609754166040519015158152f35b3461034a57606036600319011261034a57600435611aad8161065c565b611b08602435611abc8161065c565b60443590611ac98261065c565b5f5493611aee60ff600887901c161580968197611b86575b8115611b66575b50614576565b84611aff600160ff195f5416175f55565b611b4f576145d9565b611b0e57005b611b1c61ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989080602081016112e7565b611b6161010061ff00195f5416175f55565b6145d9565b303b15915081611b78575b505f611ae8565b60ff1660011490505f611b71565b600160ff8216109150611ae1565b3461034a575f36600319011261034a5760d0546040516001600160a01b039091168152602090f35b6040906106c89392815281602082015201906107ce565b3461034a57606036600319011261034a57600435611bf08161065c565b602435604435611bff816105d1565b611c40611c0a612059565b9280611c15856120ad565b526040516361c8a12f60e11b81526001600160a01b0386169490925f91849182918760048401613086565b0381875afa9384156103455783611c6a611232611483611c9f986020975f91611cfd575b506120ad565b92604051968794859384936304ec635160e01b85526004850163ffffffff604092959493606083019683521660208201520152565b03915afa801561034557611cce925f91611cde575b506001600160c01b031692611cc884614e08565b9061231b565b9061075f60405192839283611bbc565b611cf7915060203d60201161150c576114fe818361039d565b5f611cb4565b611d1191503d805f833e611536818361039d565b5f611c64565b3461034a575f36600319011261034a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461034a57602036600319011261034a57600435611d788161065c565b611d80614d68565b6001600160a01b03811615611d985761031490614dc0565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b3461034a575f36600319011261034a57602060405160648152f35b3461034a57602036600319011261034a5760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610345575f91611eca575b506001600160a01b03163303611ebb57611e89606654198219811614611f1f565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b611ee3915060203d602011610a5d57610a4f818361039d565b5f611e68565b9081602091031261034a57516106c881610917565b6040513d5f823e3d90fd5b15611f1057565b631d77d47760e21b5f5260045ffd5b15611f2657565b63c61dca5d60e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b906002811015611f5a5760051b0190565b611f35565b634e487b7160e01b5f52601260045260245ffd5b61204f61202c6120559561202661201f85875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e0840152610100830152611ff681610120840103601f19810183528261039d565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b80966146c4565b9061470a565b9261202661204161203b614792565b94614889565b9161204a6149a5565b6146c4565b916149d9565b9091565b6040805190919061206a838261039d565b6001815291601f1901366020840137565b906120858261066d565b612092604051918261039d565b82815280926120a3601f199161066d565b0190602036910137565b805115611f5a5760200190565b8051821015611f5a5760209160051b010190565b9081602091031261034a575190565b9190916120ea835161207b565b925f5b815181101561219d5780602061211661210961213f94866120ba565b516001600160a01b031690565b6040516309aa152760e11b81526001600160a01b03909116600482015292839081906024820190565b03816001600160a01b0388165afa8015610345576001925f9161216f575b5061216882886120ba565b52016120ed565b612190915060203d8111612196575b612188818361039d565b8101906120ce565b5f61215d565b503d61217e565b505050565b9081602091031261034a57516106c88161065c565b906121c18261066d565b6121ce604051918261039d565b828152602081936121e1601f199161066d565b0191015f5b8281106121f257505050565b6060828201526020016121e6565b908151811015611f5a570160200190565b60208183031261034a578051906001600160401b03821161034a57019080601f8301121561034a5781516122448161066d565b92612252604051948561039d565b81845260208085019260051b82010192831161034a57602001905b82821061227a5750505090565b815181526020918201910161226d565b906122948261066d565b6122a1604051918261039d565b82815280926122b2601f199161066d565b015f5b8181106122c157505050565b6040519060608201918083106001600160401b0384111761037d576020926040525f81525f838201525f6040820152828286010152016122b5565b9081602091031261034a57516001600160601b038116810361034a5790565b604051636830483560e01b815293919291906001600160a01b0316602085600481845afa948515610345575f95612630575b50604051634f4c91e160e11b815294602086600481855afa918215610345576004965f9361260e575b5060209060405197888092632efa2ca360e11b82525afa958615610345575f966125ed575b506123a985939295516121b7565b945f935b80518510156125e3576123da6123d46123c68784612200565b516001600160f81b03191690565b60f81c90565b604051638902624560e01b815260ff8216600482015263ffffffff88166024820152909490925f846044816001600160a01b0385165afa938415610345575f946125bf575b5061242a845161228a565b612434888b6120ba565b5261243f878a6120ba565b505f5b84518110156125ae5780602061245b61247d93886120ba565b518d60405180809681946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa918215610345575f9261258e575b506124a381876120ba565b518a60208a6124b2858b6120ba565b5160405163fa28c62760e01b8152600481019190915260ff91909116602482015263ffffffff929092166044830152816064816001600160a01b038d165afa938415610345576125458c8f6125406001986125579789975f9261255e575b5061252b61251c6103df565b6001600160a01b039098168852565b60208701526001600160601b03166040860152565b6120ba565b519061255183836120ba565b526120ba565b5001612442565b61258091925060203d8111612587575b612578818361039d565b8101906122fc565b905f612510565b503d61256e565b6125a791925060203d8111610a5d57610a4f818361039d565b905f612498565b5060019096019590945091506123ad565b6125dc9194503d805f833e6125d4818361039d565b810190612211565b925f61241f565b5050509350505090565b61260791965060203d602011610a5d57610a4f818361039d565b945f61239b565b602091935061262990823d8411610a5d57610a4f818361039d565b9290612376565b61264a91955060203d602011610a5d57610a4f818361039d565b935f61234d565b6040519061265e82610382565b606080838181528160208201528160408201520152565b60208183031261034a578051906001600160401b03821161034a57019080601f8301121561034a5781516126a88161066d565b926126b6604051948561039d565b81845260208085019260051b82010192831161034a57602001905b8282106126de5750505090565b6020809183516126ed816105d1565b8152019101906126d1565b63ffffffff909116815260406020820181905281018390526001600160fb1b03831161034a5760609260051b809284830137010190565b908060209392818452848401375f828201840152601f01601f1916010190565b60409063ffffffff6106c89593168152816020820152019161272f565b634e487b7160e01b5f52601160045260245ffd5b60ff1660ff81146127915760010190565b61276c565b9190811015611f5a5760051b0190565b9081602091031261034a57516001600160c01b038116810361034a5790565b156127cc57565b60405162461bcd60e51b815260206004820152605c60248201527f4f70657261746f7253746174655265747269657665722e676574436865636b5360448201527f69676e617475726573496e64696365733a206f70657261746f72206d7573742060648201527f6265207265676973746572656420617420626c6f636b6e756d62657200000000608482015260a490fd5b90821015611f5a570190565b9081602091031261034a57516106c8816105d1565b5f1981146127915760010190565b916128aa60209263ffffffff9296959660408652604086019161272f565b9416910152565b95939495929091926128c1612651565b50604051636830483560e01b8152936001600160a01b03919091169190602085600481865afa948515610345575f95612cf8575b506128fe612651565b946040516361c8a12f60e11b81525f818061291e8d8d8b600485016126f8565b0381885afa908115610345575f91612cde575b5086526040516340e03a8160e11b81526001600160a01b039190911692905f818061296185878b6004850161274f565b0381875afa908115610345575f91612cc4575b506040870152612983816121b7565b9860608701998a525f5b60ff811683811015612c0f57885f6129b6838f6129a98861207b565b90519061255183836120ba565b505f8a868f5b818410612a39575050505090508c6129d38261207b565b915f5b818110612a00575050916129f5916129fb9493519061255183836120ba565b50612780565b61298d565b80612a33612a1e611483600194612a188a89516120ba565b516120ba565b612a2883886120ba565b9063ffffffff169052565b016129d6565b61148384612a4e8160209695612a5695612796565b3597516120ba565b6040516304ec635160e01b8152600481019690965263ffffffff9182166024870152166044850152836064818d5afa801561034557888f888a918f94612afb6001612aee81938d809d5f92612be3575b506123d4612aca612ad892612ac3878060c01b03861615156127c5565b8b8d61285d565b356001600160f81b03191690565b6001600160c01b0391821660ff919091161c1690565b166001600160c01b031690565b14612b17575b5050505050600191925001908a918a868f6129bc565b8597612b3993612b3260209799986123d495612aca95612796565b359561285d565b60405163dd9846b960e01b8152600481019290925260ff16602482015263ffffffff939093166044840152826064818c5afa908115610345578f612b9790612b9c9383886001975f93612bab575b50612a1890612a289394516120ba565b61287e565b905082918a888f888a91612b01565b612a28935090612bd4612a189260203d8111612bdc575b612bcc818361039d565b810190612869565b935090612b87565b503d612bc2565b612ad8919250612aca612c066123d49260203d811161150c576114fe818361039d565b93925050612aa6565b505050929095975060049496506020915060405194858092632efa2ca360e11b82525afa90811561034557612c65945f948593612ca3575b5060405163354952a360e21b8152958694859384936004850161288c565b03916001600160a01b03165afa908115610345575f91612c89575b50602082015290565b612c9d91503d805f833e611536818361039d565b5f612c80565b612cbd91935060203d602011610a5d57610a4f818361039d565b915f612c47565b612cd891503d805f833e611536818361039d565b5f612974565b612cf291503d805f833e611536818361039d565b5f612931565b612d1291955060203d602011610a5d57610a4f818361039d565b935f6128f5565b356106c8816105d1565b903590601e198136030182121561034a57018035906001600160401b03821161034a5760200191813603831361034a57565b602081528135602082015263ffffffff6020830135612d73816105d1565b1660408201526040820135601e198336030181121561034a578201906020823592016001600160401b03831161034a57823603811361034a57612dd16060612dca6080936106c896858488015260a087019161272f565b95016105df565b63ffffffff16910152565b15612de357565b60405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b15612e5557565b60405162461bcd60e51b815260206004820152602c60248201527f41676772656761746f722068617320616c726561647920726573706f6e64656460448201526b20746f20746865207461736b60a01b6064820152608490fd5b63ffffffff60019116019063ffffffff821161279157565b63ffffffff60649116019063ffffffff821161279157565b9063ffffffff8091169116019063ffffffff821161279157565b15612f0057565b60405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b6064820152608490fd5b6020809163ffffffff8135612f6f816105d1565b1684520135910152565b6040810192916103ce9190612f5b565b9060648202918083046064149015171561279157565b9060068202918083046006149015171561279157565b8181029291811591840414171561279157565b906001600160601b03809116911602906001600160601b03821691820361279157565b15612ff257565b608460405162461bcd60e51b815260206004820152604060248201527f5369676e61746f7269657320646f206e6f74206f776e206174206c656173742060448201527f7468726573686f6c642070657263656e74616765206f6620612071756f72756d6064820152fd5b9092916020606091613072846080810197612f5b565b63ffffffff81511660408501520151910152565b60409063ffffffff6106c894931681528160208201520190610684565b156130aa57565b60405162461bcd60e51b815260206004820152602160248201527f5461736b206861736e2774206265656e20726573706f6e64656420746f2079656044820152601d60fa1b6064820152608490fd5b909291602060609161310f846080810197612f5b565b63ffffffff813561311f816105d1565b1660408501520135910152565b1561313357565b60405162461bcd60e51b815260206004820152603d60248201527f5461736b20726573706f6e736520646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b156131a557565b60405162461bcd60e51b815260206004820152604360248201527f54686520726573706f6e736520746f2074686973207461736b2068617320616c60448201527f7265616479206265656e206368616c6c656e676564207375636365737366756c606482015262363c9760e91b608482015260a490fd5b1561322357565b60405162461bcd60e51b815260206004820152603760248201527f546865206368616c6c656e676520706572696f6420666f72207468697320746160448201527f736b2068617320616c726561647920657870697265642e0000000000000000006064820152608490fd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b8181106132b85750505090565b82518452602093840193909201916001016132ab565b156132d557565b60405162461bcd60e51b815260206004820152605060248201527f546865207075626b657973206f66206e6f6e2d7369676e696e67206f7065726160448201527f746f727320737570706c69656420627920746865206368616c6c656e6765722060648201526f30b932903737ba1031b7b93932b1ba1760811b608482015260a490fd5b1561336057565b634e487b7160e01b5f52600160045260245ffd5b9392919093600161338486612d19565b95602061344084356133af6133a78b63ffffffff165f5260cb60205260405f2090565b5415156130a3565b6133ea6133ca8b63ffffffff165f5260cb60205260405f2090565b54604051858101906133e1816111978d8b866130f9565b5190201461312c565b61341561340f6134088c63ffffffff165f5260cc60205260405f2090565b5460ff1690565b1561319e565b61343a61342c6112326134278a612d19565b612ec7565b63ffffffff4316111561321c565b80612fb5565b910135141461379357613453835161207b565b935f5b8451811015613493578061348261346f600193886120ba565b5180515f526020015160205260405f2090565b61348c82896120ba565b5201613456565b5090929391946134cd602085019660206134ac89612d19565b6040516134c1816111978a868301958661328e565b519020910135146132ce565b6134d7855161207b565b957f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316945f5b87518110156135875780602061351e61353e93896120ba565b516040518094819263745dcd7360e11b8352600483019190602083019252565b03818b5afa91821561034557600192613563915f91613569575b50610bb3838d6120ba565b01613505565b613581915060203d8111610a5d57610a4f818361039d565b5f613558565b506135e793959796506135b0929450906135a86135b8926040810190612d23565b939091612d19565b923691610798565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031661231b565b60d15460d0546001600160a01b03908116738a791620dd6260079bf849dc5567adc3f2fdc31814939116915f915b8151831015613735575f955b61362b84846120ba565b51518710156137285790829160208061364c8a612a188961366e9a996120ba565b510151604051809781926308f6629d60e31b8352600483019190602083019252565b03818d5afa948515610345575f95613708575b505f5b89518110156136fa576136a961369d612109838d6120ba565b6001600160a01b031690565b6001600160a01b038716146136c057600101613684565b5096600191929394505b6136ea60206136d76103d0565b6001600160a01b03891681520160019052565b6136f387613359565b0195613621565b5096600191929394506136ca565b61372191955060203d8111610a5d57610a4f818361039d565b935f613681565b9550600190920191613615565b95505050505091506137656137588263ffffffff165f5260cc60205260405f2090565b805460ff19166001179055565b63ffffffff3391167fc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec5f80a3565b5050509063ffffffff3391167ffd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb055f80a3565b604051906137d282610382565b5f6060838281528260208201528160408201520152565b602063ffffffff606060c0948385528051848601528284820151166040860152604081015160808387015280519485918260a0890152018787015e5f8585018701520151166080830152601f01601f1916010190565b6040519061384c82610362565b60606020838281520152565b1561385f57565b60405162461bcd60e51b815260206004820152603760248201525f51602061507c5f395f51905f5260448201527f7265733a20656d7074792071756f72756d20696e7075740000000000000000006064820152608490fd5b156138be57565b60405162461bcd60e51b815260206004820152604160248201525f51602061507c5f395f51905f5260448201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d6174636064820152600d60fb1b608482015260a490fd5b1561392757565b60a460405162461bcd60e51b815260206004820152604460248201525f51602061507c5f395f51905f5260448201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d6064820152630c2e8c6d60e31b6084820152fd5b1561399257565b60405162461bcd60e51b815260206004820152603c60248201525f51602061507c5f395f51905f5260448201527f7265733a20696e76616c6964207265666572656e636520626c6f636b000000006064820152608490fd5b9081602091031261034a57516106c881610e28565b5f1981019190821161279157565b15613a1457565b608460405162461bcd60e51b815260206004820152604060248201525f51602061507c5f395f51905f5260448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f727465646064820152fd5b906001820180921161279157565b906002820180921161279157565b906003820180921161279157565b906004820180921161279157565b906005820180921161279157565b9190820180921161279157565b15613ac557565b60405162461bcd60e51b815260206004820152606660248201525f51602061507c5f395f51905f5260448201527f7265733a205374616b6552656769737472792075706461746573206d7573742060648201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060848201526577696e646f7760d01b60a482015260c490fd5b9081602091031261034a575167ffffffffffffffff198116810361034a5790565b15613b7a57565b60405162461bcd60e51b815260206004820152606160248201525f51602061507c5f395f51905f5260448201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460648201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d2061706084820152606b60f81b60a482015260c490fd5b906001600160601b03809116911603906001600160601b03821161279157565b15613c2957565b60405162461bcd60e51b815260206004820152604360248201525f51602061507c5f395f51905f5260448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a490fd5b15613c9457565b60405162461bcd60e51b815260206004820152603960248201525f51602061507c5f395f51905f5260448201527f7265733a207369676e617475726520697320696e76616c6964000000000000006064820152608490fd5b949392909193613cfa61383f565b50613d06851515613858565b604084015151851480614568575b8061455a575b8061454c575b613d29906138b7565b613d3b60208501515185515114613920565b613d5263ffffffff431663ffffffff84161061398b565b613d5a6103d0565b5f81525f602082015292613d6c61383f565b613d758761207b565b6020820152613d838761207b565b8152613d8d61383f565b92613d9c60208801515161207b565b8452613dac60208801515161207b565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561034557613e15915f9161451d575b50613e10368b87610798565b614b45565b985f965b60208901518051891015613f7457602088613e696114838c613e618f96868e613e4661346f8680956120ba565b613e5384848401516120ba565b5282613f41575b01516120ba565b5195516120ba565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa918215610345576120268a613f168f613f0f8f8460208f92613f0693613efe8460019e613f1c9e5f91613f24575b508f8060c01b031692516120ba565b5201516120ba565b51938d516120ba565b5116614bcc565b90614bfd565b970196613e19565b613f3b9150863d811161150c576114fe818361039d565b5f613eef565b613f6f613f5184848401516120ba565b51613f6884840151613f62876139ff565b906120ba565b5110613a0d565b613e5a565b50909597949650613f89919893929950614ce3565b91613f9660975460ff1690565b908115614515576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610345575f916144f6575b5091905b5f925b8184106140475750505050509261402e61402961402261404195856111979860806060602099015192015192611f73565b9190613c22565b613c8d565b015160405192839160208301958661328e565b51902090565b92989596909399919794878b888c888d6143f0575b6114838260a061409c6123d4612aca846140a49761409661408861346f8f9c604060209f9e01516120ba565b67ffffffffffffffff191690565b9b61285d565b9701516120ba565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa908115610345576141686114838f958f906141608f978f96848f61415a60c096614153848f60209f90613e5a612aca996040936123d49c5f916143c2575b5067ffffffffffffffff19918216911614613b73565b519061470a565b9c61285d565b9601516120ba565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa908115610345576141f5918c8f925f9261439e575b5060206141e7929301516120ba565b906001600160601b03169052565b6142158c6141e78c61420e6113068260208601516120ba565b92516120ba565b5f985f5b60208a015151811015614385578b8d6142578961424a6123d4612aca868f8961424291516120ba565b51948761285d565b60ff161c60019081161490565b614266575b5050600101614219565b8a8a6142e8859f948f9686612a188f9360e061429f6114839560206142976123d4612aca839f6142a89c899161285d565b9a01516120ba565b519b01516120ba565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610345578f614354908f936001959486955f9261435f575b5061434e6141e79293519361434961130684876120ba565b613c02565b926120ba565b019a90508b8d61425c565b6141e7925061437e61434e9160203d811161258757612578818361039d565b9250614331565b5093919796996001919699509a94929a01929190613ff1565b6141e792506143bb602091823d811161258757612578818361039d565b92506141d8565b60206143e392503d81116143e9575b6143db818361039d565b810190613b52565b5f61413d565b503d6143d1565b61442d945061440a92506123d491612aca9160209561285d565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa8015610345576020896140a48f938f60a08f976123d4612aca8f8f9061409661408861346f8f60408b96918f88936114839f6144b1906144b79361409c9f5f926144cd575b5063ffffffff809116931690613ab1565b11613abe565b505050505050975050505050509293505061405c565b602063ffffffff92935082916144ee913d811161219657612188818361039d565b9291506144a0565b61450f915060203d602011612bdc57612bcc818361039d565b5f613fea565b5f9190613fee565b61453f915060203d602011614545575b614537818361039d565b8101906139ea565b5f613e04565b503d61452d565b5060e0840151518514613d20565b5060c0840151518514613d1a565b5060a0840151518514613d14565b1561457d57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b6145e290614dc0565b60018060a01b03166001600160601b0360a01b60cd54161760cd5560018060a01b03166001600160601b0360a01b60ce54161760ce55565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b6040519061468d82610362565b5f6020838281520152565b604051906101806146a9818461039d565b368337565b604051906146bd60208361039d565b6020368337565b919060409060606146d3614680565b94859260208551926146e5858561039d565b8436853780518452015160208301528482015260076107cf195a01fa1561470857565bfe5b60209291608060409261471b614680565b9586938186519361472c868661039d565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa8015614708571561475d57565b60405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606490fd5b60405161479e81610362565b60409081516147ad838261039d565b82368237815260208251916147c2848461039d565b83368437015280516147d4828261039d565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed602082015281519061482a838361039d565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d602083015261487f8351938461039d565b8252602082015290565b5f51602061505c5f395f51905f52906148a0614680565b505f919006602060c0835b6149a0575f935f51602061505c5f395f51905f52600381868181800909086040516148d6858261039d565b843682378481856040516148ea828261039d565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f51602061505c5f395f51905f5260a082015260056107cf195a01fa8015614708576149549061500f565b51916149a0575f51602061505c5f395f51905f528280091461498b57505f51602061505c5f395f51905f5260015f940892936148ab565b929350506149976103d0565b92835282015290565b611f5f565b6149ad614680565b506040516149ba81610362565b600181526002602082015290565b90600c811015611f5a5760051b0190565b939290916149e760406103ee565b94855260208501526149f960406103ee565b9182526020820152614a09614698565b925f5b60028110614a3657505050602061018092614a256146ae565b93849160086201d4c0fa9151151590565b80614a42600192612f9f565b614a4c8285611f49565b5151614a5882896149c8565b526020614a658386611f49565b510151614a7a614a7483613a6b565b896149c8565b52614a858286611f49565b515151614a94614a7483613a79565b52614aaa614aa28387611f49565b515160200190565b51614ab7614a7483613a87565b526020614ac48387611f49565b51015151614ad4614a7483613a95565b52614b00614afa614af36020614aea868a611f49565b51015160200190565b5192613aa3565b886149c8565b5201614a0c565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff196097541660ff821617609755604051908152a1565b906001614b5360ff93614f28565b928392161b1115614b615790565b60405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c7565006064820152608490fd5b805f915b614bd8575090565b5f1981018181116127915761ffff9116911661ffff8114612791576001019080614bd0565b90614c06614680565b5061ffff811690610200821015614cab5760018214614ca657614c276103d0565b5f81525f602082015292906001905f925b61ffff8316851015614c4c57505050505090565b600161ffff831660ff86161c811614614c86575b6001614c7c614c718360ff9461470a565b9460011b61fffe1690565b9401169291614c38565b946001614c7c614c71614c9b8960ff9561470a565b989350505050614c60565b505090565b60405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b6044820152606490fd5b614ceb614680565b50805190811580614d5c575b15614d18575050604051614d0c60408261039d565b5f81525f602082015290565b60205f51602061505c5f395f51905f52910151065f51602061505c5f395f51905f52035f51602061505c5f395f51905f528111612791576040519161487f83610362565b50602081015115614cf7565b6033546001600160a01b03163303614d7c57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b61ffff614e1482614bcc565b16614e1e8161077d565b90614e2c604051928361039d565b808252614e3b601f199161077d565b013660208301375f5f5b8251821080614e9b575b15614e94576001811b8416614e6d575b614e689061287e565b614e45565b906001614e689160ff60f81b8460f81b165f1a614e8a8287612200565b5301919050614e5f565b5050905090565b506101008110614e4f565b15614ead57565b60405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a490fd5b90610100825111614f9857815115614f9357614f56614f4c6123d46123c6856120ad565b60ff600191161b90565b6001905b8351821015614f8e57600190614f79614f4c6123d46123c68689612200565b90614f85818311614ea6565b17910190614f5a565b925050565b5f9150565b60a460405162461bcd60e51b815260206004820152604460248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b6084820152fd5b1561501657565b60405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000006044820152606490fdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47424c535369676e6174757265436865636b65722e636865636b5369676e617475a2646970667358221220fd22d4ffb94c0825ebfcc1a6ad4e1d20cd0c7fde6159bf4e7f547cce4ac852f764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01@\x80`@R4a\x02OW`\xC0\x81aT\x17\x808\x03\x80\x91a\0 \x82\x85a\x02\xFAV[\x839\x81\x01\x03\x12a\x02OW\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x91\x90\x82\x81\x03a\x02OW` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x93\x90\x91\x90\x84\x83\x03a\x02OW`@\x84\x01Q\x91c\xFF\xFF\xFF\xFF\x83\x16\x83\x03a\x02OWa\0x``\x86\x01a\x031V[\x93a\0\x91`\xA0a\0\x8A`\x80\x89\x01a\x031V[\x97\x01a\x031V[\x96\x15a\x02\xEBW`\x80R`\xA0R`@Qch0H5`\xE0\x1B\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x02[W_\x91a\x02\xA8W[P`\xC0R`@Qc.\xFA,\xA3`\xE1\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x90\x81\x15a\x02[W_\x91a\x02fW[P`\xE0R`\xC0Q`@Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x02[W_\x91a\x02\x15W[Pa\x01\0Ra\x01 R`\xCF\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\xD0\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD1\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90U`@QaP\xD1\x90\x81a\x03F\x829`\x80Q\x81\x81\x81a\x02\xBA\x01R\x81\x81a\r\xE0\x01R\x81\x81a\x19\xEF\x01Ra\x1E0\x01R`\xA0Q\x81\x81\x81a\tS\x01R\x81\x81a\x18\x1C\x01R\x81\x81a5\xBA\x01R\x81\x81a=\xC9\x01R\x81\x81a>\x9F\x01RaD1\x01R`\xC0Q\x81\x81\x81a\x15\xF9\x01R\x81\x81aA\xA1\x01RaB\xEC\x01R`\xE0Q\x81\x81\x81a\x15\xB5\x01R\x81\x81a4\xDA\x01Ra@\xDD\x01Ra\x01\0Q\x81\x81\x81a\x1D,\x01Ra?\xB2\x01Ra\x01 Q\x81\x81\x81a\x05\x85\x01Ra\x12\r\x01R\xF3[\x90P` \x81=` \x11a\x02SW[\x81a\x020` \x93\x83a\x02\xFAV[\x81\x01\x03\x12a\x02OWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02OW_a\x01%V[_\x80\xFD[=\x91Pa\x02#V[`@Q=_\x82>=\x90\xFD[\x90P` \x81=` \x11a\x02\xA0W[\x81a\x02\x81` \x93\x83a\x02\xFAV[\x81\x01\x03\x12a\x02OWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02OW_a\0\xEDV[=\x91Pa\x02tV[\x90P` \x81=` \x11a\x02\xE3W[\x81a\x02\xC3` \x93\x83a\x02\xFAV[\x81\x01\x03\x12a\x02OWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02OW`\x04a\0\xC1V[=\x91Pa\x02\xB6V[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x03\x1DW`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02OWV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x13d9\xDD\x14a\x02\x8AW\x80c\x17\x1F\x1D[\x14a\x02\x85W\x80c\x1A\xD41\x89\x14a\x01\xDBW\x80c$Z{\xFC\x14a\x02\x80W\x80c,\xB2#\xD5\x14a\x02{W\x80c-\x89\xF6\xFC\x14a\x02vW\x80c1\xB3k\xD9\x14a\x02qW\x80c5c\xB0\xD1\x14a\x02lW\x80c9\x98\xFD\xD3\x14a\x02gW\x80cAl~^\x14a\x02bW\x80cM+W\xFE\x14a\x02]W\x80cOs\x9Ft\x14a\x02XW\x80cY\\jg\x14a\x02SW\x80cZ\xC8j\xB7\x14a\x02NW\x80c[\xAE\xC9\xA0\x14a\x02IW\x80c\\\x15Vb\x14a\x02DW\x80c\\\x97Z\xBB\x14a\x02?W\x80c]\xEC\xC3\xF5\x14a\x02:W\x80c]\xF4YF\x14a\x025W\x80ch0H5\x14a\x020W\x80ckS.\x9E\x14a\x02+W\x80ck\x92x~\x14a\x02&W\x80cm\x14\xA9\x87\x14a\x02!W\x80cn\xFBF6\x14a\x02\x1CW\x80cqP\x18\xA6\x14a\x02\x17W\x80cr\xD1\x8E\x8D\x14a\x02\x08W\x80cz\xFA\x1E\xED\x14a\x02\x12W\x80c\x88o\x11\x95\x14a\x02\rW\x80c\x8B\0\xCE|\x14a\x02\x08W\x80c\x8D\xA5\xCB[\x14a\x02\x03W\x80c\x9B)\x0E\x98\x14a\x01\xFEW\x80c\xB9\x8D\t\x08\x14a\x01\xF9W\x80c\xC0\xC5;\x8B\x14a\x01\xF4W\x80c\xCA\x8A\xA7\xC7\x14a\x01\xEFW\x80c\xCE\xFD\xC1\xD4\x14a\x01\xEAW\x80c\xDF\\\xF7#\x14a\x01\xE5W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xE0W\x80c\xF5\xC9\x89\x9D\x14a\x01\xDBW\x80c\xF6<[\xAB\x14a\x01\xD6Wc\xFA\xBC\x1C\xBC\x14a\x01\xD1W_\x80\xFD[a\x1E\x07V[a\x1D\xECV[a\x05iV[a\x1D[V[a\x1D\x17V[a\x1B\xD3V[a\x1B\x94V[a\x1A\x90V[a\x1AnV[a\x1AFV[a\x1A\x1EV[a\x19\x8FV[a\x19\xDAV[a\x19\xB2V[a\x194V[a\x18\x87V[a\x18\x07V[a\x16\x99V[a\x16(V[a\x15\xE4V[a\x15\xA0V[a\x15bV[a\x15EV[a\x13\xCCV[a\x10\xE2V[a\x0E3V[a\r\xB5V[a\r\x0EV[a\x0B\x03V[a\t!V[a\x08\xEFV[a\x08uV[a\x06\xCBV[a\x06#V[a\x05\xEAV[a\x05\xA9V[a\x05\x01V[4a\x03JW` 6`\x03\x19\x01\x12a\x03JW`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03EWa\x03\x14\x92a\x03\0\x91_\x91a\x03\x16W[Pa\x1F\tV[a\x03\x0F`fT\x82\x81\x16\x14a\x1F\x1FV[aFNV[\0[a\x038\x91P` =` \x11a\x03>W[a\x030\x81\x83a\x03\x9DV[\x81\x01\x90a\x1E\xE9V[_a\x02\xFAV[P=a\x03&V[a\x1E\xFEV[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03}W`@RV[a\x03NV[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03}W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03}W`@RV[`@Q\x90a\x03\xCEa\x01\0\x83a\x03\x9DV[V[`@Q\x90a\x03\xCE`@\x83a\x03\x9DV[`@Q\x90a\x03\xCE``\x83a\x03\x9DV[\x90a\x03\xCE`@Q\x92\x83a\x03\x9DV[`@\x90`\xE3\x19\x01\x12a\x03JW`@Q\x90a\x04\x15\x82a\x03bV[`\xE45\x82Ra\x01\x045` \x83\x01RV[\x91\x90\x82`@\x91\x03\x12a\x03JW`@Qa\x04=\x81a\x03bV[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03JW`@Q\x91a\x04h`@\x84a\x03\x9DV[\x82\x90`@\x81\x01\x92\x83\x11a\x03JW\x90[\x82\x82\x10a\x04\x84WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x04wV[\x90`\x80`c\x19\x83\x01\x12a\x03JW`@Qa\x04\xAD\x81a\x03bV[` a\x04\xC8\x82\x94a\x04\xBF\x81`da\x04MV[\x84R`\xA4a\x04MV[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x03JW` a\x04\xC8`@Q\x92a\x04\xEB\x84a\x03bV[`@\x84\x96a\x04\xF9\x83\x82a\x04MV[\x86R\x01a\x04MV[4a\x03JWa\x01 6`\x03\x19\x01\x12a\x03JW`\x045`@6`#\x19\x01\x12a\x03JWa\x05Y`@\x91\x82Qa\x053\x81a\x03bV[`$5\x81R`D5` \x82\x01Ra\x05I6a\x04\x94V[\x90a\x05S6a\x03\xFCV[\x92a\x1FsV[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`\xCDT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03JWV[5\x90a\x03\xCE\x82a\x05\xD1V[4a\x03JW` 6`\x03\x19\x01\x12a\x03JWc\xFF\xFF\xFF\xFF`\x045a\x06\x0C\x81a\x05\xD1V[\x16_R`\xCB` R` `@_ T`@Q\x90\x81R\xF3[4a\x03JW` 6`\x03\x19\x01\x12a\x03JWc\xFF\xFF\xFF\xFF`\x045a\x06E\x81a\x05\xD1V[\x16_R`\xCA` R` `@_ T`@Q\x90\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03JWV[`\x01`\x01`@\x1B\x03\x81\x11a\x03}W`\x05\x1B` \x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x06\xA1WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\x94V[\x90` a\x06\xC8\x92\x81\x81R\x01\x90a\x06\x84V[\x90V[4a\x03JW`@6`\x03\x19\x01\x12a\x03JW`\x045a\x06\xE8\x81a\x06\\V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03JW6`#\x83\x01\x12\x15a\x03JW\x81`\x04\x015\x91a\x07\x14\x83a\x06mV[\x92a\x07\"`@Q\x94\x85a\x03\x9DV[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x03JW`$\x01\x90[\x82\x82\x10a\x07cWa\x07_a\x07S\x86\x86a \xDDV[`@Q\x91\x82\x91\x82a\x06\xB7V[\x03\x90\xF3[` \x80\x91\x835a\x07r\x81a\x06\\V[\x81R\x01\x91\x01\x90a\x07?V[`\x01`\x01`@\x1B\x03\x81\x11a\x03}W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x07\xA4\x82a\x07}V[\x91a\x07\xB2`@Q\x93\x84a\x03\x9DV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03JW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x07\xF9WPPPPP\x90V[\x90\x91\x92\x93\x94`\x1F\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x089WPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x07\xEAV[\x90\x91\x92` ```\x01\x92`\x01`\x01``\x1B\x03`@\x88Q\x86\x80`\xA0\x1B\x03\x81Q\x16\x84R\x85\x81\x01Q\x86\x85\x01R\x01Q\x16`@\x82\x01R\x01\x94\x01\x92\x01\x90a\x08\x1AV[4a\x03JW``6`\x03\x19\x01\x12a\x03JW`\x045a\x08\x92\x81a\x06\\V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03JW6`#\x82\x01\x12\x15a\x03JWa\x07_\x91a\x08\xC9a\x08\xDB\x926\x90`$\x81`\x04\x015\x91\x01a\x07\x98V[`D5\x91a\x08\xD6\x83a\x05\xD1V[a#\x1BV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x07\xCEV[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`\xD1T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x03JWV[4a\x03JW` 6`\x03\x19\x01\x12a\x03JW`\x045a\t>\x81a\t\x17V[`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03EW_\x91a\n5W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\t\xA4Wa\x03\x14\x90aK\x07V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FBLSSignatureChecker.onlyCoordina`D\x82\x01R\x7FtorOwner: caller is not the owne`d\x82\x01R\x7Fr of the registryCoordinator\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[a\nW\x91P` =` \x11a\n]W[a\nO\x81\x83a\x03\x9DV[\x81\x01\x90a!\xA2V[_a\t\x8BV[P=a\nEV[\x90\x80`\x1F\x83\x01\x12\x15a\x03JW\x815a\n{\x81a\x06mV[\x92a\n\x89`@Q\x94\x85a\x03\x9DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03JW` \x01\x90[\x82\x82\x10a\n\xB1WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\n\xA4V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\n\xE4WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\n\xD7V[4a\x03JW`@6`\x03\x19\x01\x12a\x03JW`\x045a\x0B \x81a\x06\\V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x0B?\x906\x90`\x04\x01a\ndV[a\x0BI\x81Qa {V[\x91`\x01`\x01`\xA0\x1B\x03\x16_[\x82Q\x81\x10\x15a\x0B\xE6W\x80` a\x0Bna\x0B\x8E\x93\x86a \xBAV[Q`@Q\x80\x94\x81\x92c\nZ\xEC\x19`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x91\x82\x15a\x03EW`\x01\x92a\x0B\xC2\x91_\x91a\x0B\xC8W[Pa\x0B\xB3\x83\x88a \xBAV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\x0BUV[a\x0B\xE0\x91P` =\x81\x11a\n]Wa\nO\x81\x83a\x03\x9DV[_a\x0B\xA8V[`@Q\x80a\x07_\x86\x82a\n\xC1V[\x91\x81`\x1F\x84\x01\x12\x15a\x03JW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03JW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03JWV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0C>WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0C1V[\x90` \x82R``a\x0C\xA8a\x0C\x93a\x0C}\x84Q`\x80` \x88\x01R`\xA0\x87\x01\x90a\x0C!V[` \x85\x01Q\x86\x82\x03`\x1F\x19\x01`@\x88\x01Ra\x0C!V[`@\x84\x01Q\x85\x82\x03`\x1F\x19\x01\x84\x87\x01Ra\x0C!V[\x91\x01Q\x91`\x80`\x1F\x19\x82\x84\x03\x01\x91\x01R\x81Q\x80\x82R` \x82\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x0C\xE1WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a\x0C\xFF`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\x0C!V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x0C\xD2V[4a\x03JW`\x806`\x03\x19\x01\x12a\x03JW`\x045a\r+\x81a\x06\\V[`$5\x90a\r8\x82a\x05\xD1V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\rW\x906\x90`\x04\x01a\x0B\xF4V[\x91`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03JW6`#\x85\x01\x12\x15a\x03JW\x83`\x04\x015\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03JW6`$\x85`\x05\x1B\x87\x01\x01\x11a\x03JWa\x07_\x95`$a\r\xA9\x96\x01\x93a(\xB1V[`@Q\x91\x82\x91\x82a\x0CZV[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03EWa\x0E \x91_\x91a\x03\x16WPa\x1F\tV[a\x03\x14aF\x1AV[`\xFF\x81\x16\x03a\x03JWV[4a\x03JW` 6`\x03\x19\x01\x12a\x03JW` `\x01`\xFF`\x045a\x0EV\x81a\x0E(V[\x16\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[\x90\x81`\x80\x91\x03\x12a\x03JW\x90V[`@\x90`#\x19\x01\x12a\x03JW`$\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x03JW\x815a\x0E\x9C\x81a\x06mV[\x92a\x0E\xAA`@Q\x94\x85a\x03\x9DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03JW` \x01\x90[\x82\x82\x10a\x0E\xD2WPPP\x90V[` \x80\x91\x835a\x0E\xE1\x81a\x05\xD1V[\x81R\x01\x91\x01\x90a\x0E\xC5V[\x81`\x1F\x82\x01\x12\x15a\x03JW\x805a\x0F\x02\x81a\x06mV[\x92a\x0F\x10`@Q\x94\x85a\x03\x9DV[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x03JW` \x01\x91[\x83\x83\x10a\x0F:WPPPP\x90V[` `@\x91a\x0FI\x84\x86a\x04%V[\x81R\x01\x92\x01\x91a\x0F,V[\x90\x80`\x1F\x83\x01\x12\x15a\x03JW\x815a\x0Fk\x81a\x06mV[\x92a\x0Fy`@Q\x94\x85a\x03\x9DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x03JW` \x82\x01\x90[\x83\x82\x10a\x0F\xA5WPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03JW` \x91a\x0F\xC7\x87\x84\x80\x94\x88\x01\x01a\x0E\x85V[\x81R\x01\x91\x01\x90a\x0F\x96V[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x03JWa\x0F\xE8a\x03\xBEV[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03JW\x81a\x10\x05\x91\x84\x01a\x0E\x85V[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03JW\x81a\x10&\x91\x84\x01a\x0E\xECV[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03JW\x81a\x10J\x91\x84\x01a\x0E\xECV[`@\x85\x01Ra\x10\\\x81``\x84\x01a\x04\xCDV[``\x85\x01Ra\x10n\x81`\xE0\x84\x01a\x04%V[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03JW\x81a\x10\x93\x91\x84\x01a\x0E\x85V[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03JW\x81a\x10\xB8\x91\x84\x01a\x0E\x85V[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x10\xDB\x92\x01a\x0FTV[`\xE0\x83\x01RV[4a\x03JW`\x806`\x03\x19\x01\x12a\x03JW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x11\x12\x906\x90`\x04\x01a\x0EfV[a\x11\x1B6a\x0EtV[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x11;\x906\x90`\x04\x01a\x0F\xD2V[`\xCDT\x90\x92\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x13NWa\x11^` \x83\x94\x93\x01a-\x19V[\x91a\x12aa\x11o`@\x86\x01\x86a-#V[\x92\x90\x94a\x11\xCFa\x11\x81``\x89\x01a-\x19V[\x97`@Qa\x11\xA5\x81a\x11\x97` \x82\x01\x94\x85a-UV[\x03`\x1F\x19\x81\x01\x83R\x82a\x03\x9DV[Q\x90 a\x11\xC8a\x11\xB4\x88a-\x19V[c\xFF\xFF\xFF\xFF\x16_R`\xCA` R`@_ \x90V[T\x14a-\xDCV[a\x11\xF9a\x11\xF2a\x11\xDE\x87a-\x19V[c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15a.NV[\x83c\xFF\xFF\xFF\xFFC\x16\x96a\x12Ca\x12;a\x122\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a.\xDFV[c\xFF\xFF\xFF\xFF\x16\x90V[\x89\x11\x15a.\xF9V[`@Q` \x81\x01\x90a\x12Y\x81a\x11\x97\x8B\x85a/yV[Q\x90 a<\xECV[\x91\x90`\xFF_\x96\x16\x95[\x82\x81\x10a\x12\xECW\x7F4\x9C\x1E\xE6\x0EN\x89r\xEE\x9D\xBAd,\x17tT=\\A6\x87\x9B\x7FL\xAA\xF0K\xF8\x1AHz*\x86\x86\x86a\x12\xACa\x12\xA0a\x03\xD0V[c\xFF\xFF\xFF\xFF\x90\x94\x16\x84RV[` \x83\x01R`@Q` \x81\x01\x90a\x12\xC8\x81a\x11\x97\x86\x86\x86a0\\V[Q\x90 a\x12\xD7a\x11\xDE\x83a-\x19V[Ua\x12\xE7`@Q\x92\x83\x92\x83a0\\V[\x03\x90\xA1\0[\x80a\x13Ha\x13$a\x13\x1Fa\x13\x13a\x13\x06`\x01\x96\x88Qa \xBAV[Q`\x01`\x01``\x1B\x03\x16\x90V[`\x01`\x01``\x1B\x03\x16\x90V[a/\x89V[a\x13Aa\x13\x13\x8Ba\x13<a\x13\x06\x87` \x8B\x01Qa \xBAV[a/\xC8V[\x11\x15a/\xEBV[\x01a\x12jV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAggregator must be the caller\0\0\0`D\x82\x01R`d\x90\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x13\xB6WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x13\xA9V[4a\x03JW``6`\x03\x19\x01\x12a\x03JW`\x045a\x13\xE9\x81a\x06\\V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x14\x08\x906\x90`\x04\x01a\ndV[`D5\x91a\x14\x15\x83a\x05\xD1V[`@Qca\xC8\xA1/`\xE1\x1B\x81R\x90`\x01`\x01`\xA0\x1B\x03\x16_\x82\x80a\x14=\x86\x88`\x04\x84\x01a0\x86V[\x03\x81\x84Z\xFA\x91\x82\x15a\x03EW_\x92a\x15!W[Pa\x14[\x83Qa {V[\x93_[\x84Q\x81\x10\x15a\x15\x13Wa\x14q\x81\x86a \xBAV[Q\x90` \x83a\x14\x8Da\x14\x83\x84\x89a \xBAV[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x95\x90\x95Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81\x87Z\xFA\x80\x15a\x03EW`\x01\x92_\x91a\x14\xE5W[P\x82\x80`\xC0\x1B\x03\x16a\x14\xDE\x82\x89a \xBAV[R\x01a\x14^V[a\x15\x06\x91P` =\x81\x11a\x15\x0CW[a\x14\xFE\x81\x83a\x03\x9DV[\x81\x01\x90a'\xA6V[_a\x14\xCCV[P=a\x14\xF4V[`@Q\x80a\x07_\x88\x82a\x13\x93V[a\x15>\x91\x92P=\x80_\x83>a\x156\x81\x83a\x03\x9DV[\x81\x01\x90a&uV[\x90_a\x14PV[4a\x03JW_6`\x03\x19\x01\x12a\x03JW` `fT`@Q\x90\x81R\xF3[4a\x03JW` 6`\x03\x19\x01\x12a\x03JWc\xFF\xFF\xFF\xFF`\x045a\x15\x84\x81a\x05\xD1V[\x16_R`\xCC` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03JW`\xC06`\x03\x19\x01\x12a\x03JW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x16X\x906\x90`\x04\x01a\x0EfV[a\x16a6a\x0EtV[\x90`@6`c\x19\x01\x12a\x03JW`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x03\x14\x92a\x16\x93`d\x926\x90`\x04\x01a\x0E\xECV[\x92a3tV[4a\x03JW``6`\x03\x19\x01\x12a\x03JW`$5`\x045a\x16\xB9\x82a\x05\xD1V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x16\xD8\x906\x90`\x04\x01a\x0B\xF4V[`\xCET\x91\x93\x90\x92\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a\x17\xB8Wa\x03\x14\x93a\x17\xA2\x93a\x17\"a\x17)\x93a\x17\x06a7\xC5V[\x95\x86RCc\xFF\xFF\xFF\xFF\x16` \x87\x01Rc\xFF\xFF\xFF\xFF\x16``\x86\x01RV[6\x91a\x07\x98V[`@\x82\x01R`@Q` \x81\x01\x90a\x17D\x81a\x11\x97\x85\x85a7\xE9V[Q\x90 a\x17Ya\x11\xB4`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90V[U`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90\x7F\x16\x95\xB8\xD0n\xC8\0\xB4a^t\\\xFB[\xD0\x0C\x1F(ua]B\x92\\;Z\xFAT;\xB2LH`@Q\x80a\x17\x9Ac\xFF\xFF\xFF\xFF\x86\x16\x94\x82a7\xE9V[\x03\x90\xA2a.\xAFV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19`\xC9T\x16\x17`\xC9UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask generator must be the calle`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x18hWPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x18[V[4a\x03JW`\x806`\x03\x19\x01\x12a\x03JW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x18\xBA\x906\x90`\x04\x01a\x0B\xF4V[\x90\x91`D5a\x18\xC8\x81a\x05\xD1V[`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03JWa\x19*\x94a\x18\xEFa\x18\xF5\x956\x90`\x04\x01a\x0F\xD2V[\x93a<\xECV[`@Q\x92\x83\x92`@\x84R` a\x19\x16\x82Q`@\x80\x88\x01R`\x80\x87\x01\x90a\x18KV[\x91\x01Q\x84\x82\x03`?\x19\x01``\x86\x01Ra\x18KV[\x90` \x83\x01R\x03\x90\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JWa\x19LaMhV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03JW_6`\x03\x19\x01\x12a\x03JW` c\xFF\xFF\xFF\xFF`\xC9T\x16`@Q\x90\x81R\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`\xCET`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`\xCFT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW` `\xFF`\x97T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03JW``6`\x03\x19\x01\x12a\x03JW`\x045a\x1A\xAD\x81a\x06\\V[a\x1B\x08`$5a\x1A\xBC\x81a\x06\\V[`D5\x90a\x1A\xC9\x82a\x06\\V[_T\x93a\x1A\xEE`\xFF`\x08\x87\x90\x1C\x16\x15\x80\x96\x81\x97a\x1B\x86W[\x81\x15a\x1BfW[PaEvV[\x84a\x1A\xFF`\x01`\xFF\x19_T\x16\x17_UV[a\x1BOWaE\xD9V[a\x1B\x0EW\0[a\x1B\x1Ca\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\x12\xE7V[a\x1Baa\x01\0a\xFF\0\x19_T\x16\x17_UV[aE\xD9V[0;\x15\x91P\x81a\x1BxW[P_a\x1A\xE8V[`\xFF\x16`\x01\x14\x90P_a\x1BqV[`\x01`\xFF\x82\x16\x10\x91Pa\x1A\xE1V[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`\xD0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[`@\x90a\x06\xC8\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x07\xCEV[4a\x03JW``6`\x03\x19\x01\x12a\x03JW`\x045a\x1B\xF0\x81a\x06\\V[`$5`D5a\x1B\xFF\x81a\x05\xD1V[a\x1C@a\x1C\na YV[\x92\x80a\x1C\x15\x85a \xADV[R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x94\x90\x92_\x91\x84\x91\x82\x91\x87`\x04\x84\x01a0\x86V[\x03\x81\x87Z\xFA\x93\x84\x15a\x03EW\x83a\x1Cja\x122a\x14\x83a\x1C\x9F\x98` \x97_\x91a\x1C\xFDW[Pa \xADV[\x92`@Q\x96\x87\x94\x85\x93\x84\x93c\x04\xECcQ`\xE0\x1B\x85R`\x04\x85\x01c\xFF\xFF\xFF\xFF`@\x92\x95\x94\x93``\x83\x01\x96\x83R\x16` \x82\x01R\x01RV[\x03\x91Z\xFA\x80\x15a\x03EWa\x1C\xCE\x92_\x91a\x1C\xDEW[P`\x01`\x01`\xC0\x1B\x03\x16\x92a\x1C\xC8\x84aN\x08V[\x90a#\x1BV[\x90a\x07_`@Q\x92\x83\x92\x83a\x1B\xBCV[a\x1C\xF7\x91P` =` \x11a\x15\x0CWa\x14\xFE\x81\x83a\x03\x9DV[_a\x1C\xB4V[a\x1D\x11\x91P=\x80_\x83>a\x156\x81\x83a\x03\x9DV[_a\x1CdV[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03JW` 6`\x03\x19\x01\x12a\x03JW`\x045a\x1Dx\x81a\x06\\V[a\x1D\x80aMhV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x1D\x98Wa\x03\x14\x90aM\xC0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03JW_6`\x03\x19\x01\x12a\x03JW` `@Q`d\x81R\xF3[4a\x03JW` 6`\x03\x19\x01\x12a\x03JW`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03EW_\x91a\x1E\xCAW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1E\xBBWa\x1E\x89`fT\x19\x82\x19\x81\x16\x14a\x1F\x1FV[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a\x1E\xE3\x91P` =` \x11a\n]Wa\nO\x81\x83a\x03\x9DV[_a\x1EhV[\x90\x81` \x91\x03\x12a\x03JWQa\x06\xC8\x81a\t\x17V[`@Q=_\x82>=\x90\xFD[\x15a\x1F\x10WV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a\x1F&WV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a\x1FZW`\x05\x1B\x01\x90V[a\x1F5V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a Oa ,a U\x95a &a \x1F\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x1F\xF6\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x03\x9DV[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96aF\xC4V[\x90aG\nV[\x92a &a Aa ;aG\x92V[\x94aH\x89V[\x91a JaI\xA5V[aF\xC4V[\x91aI\xD9V[\x90\x91V[`@\x80Q\x90\x91\x90a j\x83\x82a\x03\x9DV[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x90a \x85\x82a\x06mV[a \x92`@Q\x91\x82a\x03\x9DV[\x82\x81R\x80\x92a \xA3`\x1F\x19\x91a\x06mV[\x01\x90` 6\x91\x017V[\x80Q\x15a\x1FZW` \x01\x90V[\x80Q\x82\x10\x15a\x1FZW` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x03JWQ\x90V[\x91\x90\x91a \xEA\x83Qa {V[\x92_[\x81Q\x81\x10\x15a!\x9DW\x80` a!\x16a!\ta!?\x94\x86a \xBAV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x92\x83\x90\x81\x90`$\x82\x01\x90V[\x03\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x80\x15a\x03EW`\x01\x92_\x91a!oW[Pa!h\x82\x88a \xBAV[R\x01a \xEDV[a!\x90\x91P` =\x81\x11a!\x96W[a!\x88\x81\x83a\x03\x9DV[\x81\x01\x90a \xCEV[_a!]V[P=a!~V[PPPV[\x90\x81` \x91\x03\x12a\x03JWQa\x06\xC8\x81a\x06\\V[\x90a!\xC1\x82a\x06mV[a!\xCE`@Q\x91\x82a\x03\x9DV[\x82\x81R` \x81\x93a!\xE1`\x1F\x19\x91a\x06mV[\x01\x91\x01_[\x82\x81\x10a!\xF2WPPPV[``\x82\x82\x01R` \x01a!\xE6V[\x90\x81Q\x81\x10\x15a\x1FZW\x01` \x01\x90V[` \x81\x83\x03\x12a\x03JW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03JW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03JW\x81Qa\"D\x81a\x06mV[\x92a\"R`@Q\x94\x85a\x03\x9DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03JW` \x01\x90[\x82\x82\x10a\"zWPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a\"mV[\x90a\"\x94\x82a\x06mV[a\"\xA1`@Q\x91\x82a\x03\x9DV[\x82\x81R\x80\x92a\"\xB2`\x1F\x19\x91a\x06mV[\x01_[\x81\x81\x10a\"\xC1WPPPV[`@Q\x90``\x82\x01\x91\x80\x83\x10`\x01`\x01`@\x1B\x03\x84\x11\x17a\x03}W` \x92`@R_\x81R_\x83\x82\x01R_`@\x82\x01R\x82\x82\x86\x01\x01R\x01a\"\xB5V[\x90\x81` \x91\x03\x12a\x03JWQ`\x01`\x01``\x1B\x03\x81\x16\x81\x03a\x03JW\x90V[`@Qch0H5`\xE0\x1B\x81R\x93\x91\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16` \x85`\x04\x81\x84Z\xFA\x94\x85\x15a\x03EW_\x95a&0W[P`@QcOL\x91\xE1`\xE1\x1B\x81R\x94` \x86`\x04\x81\x85Z\xFA\x91\x82\x15a\x03EW`\x04\x96_\x93a&\x0EW[P` \x90`@Q\x97\x88\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x95\x86\x15a\x03EW_\x96a%\xEDW[Pa#\xA9\x85\x93\x92\x95Qa!\xB7V[\x94_\x93[\x80Q\x85\x10\x15a%\xE3Wa#\xDAa#\xD4a#\xC6\x87\x84a\"\0V[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`@Qc\x89\x02bE`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x88\x16`$\x82\x01R\x90\x94\x90\x92_\x84`D\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x93\x84\x15a\x03EW_\x94a%\xBFW[Pa$*\x84Qa\"\x8AV[a$4\x88\x8Ba \xBAV[Ra$?\x87\x8Aa \xBAV[P_[\x84Q\x81\x10\x15a%\xAEW\x80` a$[a$}\x93\x88a \xBAV[Q\x8D`@Q\x80\x80\x96\x81\x94c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03EW_\x92a%\x8EW[Pa$\xA3\x81\x87a \xBAV[Q\x8A` \x8Aa$\xB2\x85\x8Ba \xBAV[Q`@Qc\xFA(\xC6'`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x91\x90\x91\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x8D\x16Z\xFA\x93\x84\x15a\x03EWa%E\x8C\x8Fa%@`\x01\x98a%W\x97\x89\x97_\x92a%^W[Pa%+a%\x1Ca\x03\xDFV[`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88RV[` \x87\x01R`\x01`\x01``\x1B\x03\x16`@\x86\x01RV[a \xBAV[Q\x90a%Q\x83\x83a \xBAV[Ra \xBAV[P\x01a$BV[a%\x80\x91\x92P` =\x81\x11a%\x87W[a%x\x81\x83a\x03\x9DV[\x81\x01\x90a\"\xFCV[\x90_a%\x10V[P=a%nV[a%\xA7\x91\x92P` =\x81\x11a\n]Wa\nO\x81\x83a\x03\x9DV[\x90_a$\x98V[P`\x01\x90\x96\x01\x95\x90\x94P\x91Pa#\xADV[a%\xDC\x91\x94P=\x80_\x83>a%\xD4\x81\x83a\x03\x9DV[\x81\x01\x90a\"\x11V[\x92_a$\x1FV[PPP\x93PPP\x90V[a&\x07\x91\x96P` =` \x11a\n]Wa\nO\x81\x83a\x03\x9DV[\x94_a#\x9BV[` \x91\x93Pa&)\x90\x82=\x84\x11a\n]Wa\nO\x81\x83a\x03\x9DV[\x92\x90a#vV[a&J\x91\x95P` =` \x11a\n]Wa\nO\x81\x83a\x03\x9DV[\x93_a#MV[`@Q\x90a&^\x82a\x03\x82V[``\x80\x83\x81\x81R\x81` \x82\x01R\x81`@\x82\x01R\x01RV[` \x81\x83\x03\x12a\x03JW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03JW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03JW\x81Qa&\xA8\x81a\x06mV[\x92a&\xB6`@Q\x94\x85a\x03\x9DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03JW` \x01\x90[\x82\x82\x10a&\xDEWPPP\x90V[` \x80\x91\x83Qa&\xED\x81a\x05\xD1V[\x81R\x01\x91\x01\x90a&\xD1V[c\xFF\xFF\xFF\xFF\x90\x91\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x83\x90R`\x01`\x01`\xFB\x1B\x03\x83\x11a\x03JW``\x92`\x05\x1B\x80\x92\x84\x83\x017\x01\x01\x90V[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@\x90c\xFF\xFF\xFF\xFFa\x06\xC8\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a'/V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x16`\xFF\x81\x14a'\x91W`\x01\x01\x90V[a'lV[\x91\x90\x81\x10\x15a\x1FZW`\x05\x1B\x01\x90V[\x90\x81` \x91\x03\x12a\x03JWQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x03JW\x90V[\x15a'\xCCWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FOperatorStateRetriever.getCheckS`D\x82\x01R\x7FignaturesIndices: operator must `d\x82\x01R\x7Fbe registered at blocknumber\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[\x90\x82\x10\x15a\x1FZW\x01\x90V[\x90\x81` \x91\x03\x12a\x03JWQa\x06\xC8\x81a\x05\xD1V[_\x19\x81\x14a'\x91W`\x01\x01\x90V[\x91a(\xAA` \x92c\xFF\xFF\xFF\xFF\x92\x96\x95\x96`@\x86R`@\x86\x01\x91a'/V[\x94\x16\x91\x01RV[\x95\x93\x94\x95\x92\x90\x91\x92a(\xC1a&QV[P`@Qch0H5`\xE0\x1B\x81R\x93`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91\x90` \x85`\x04\x81\x86Z\xFA\x94\x85\x15a\x03EW_\x95a,\xF8W[Pa(\xFEa&QV[\x94`@Qca\xC8\xA1/`\xE1\x1B\x81R_\x81\x80a)\x1E\x8D\x8D\x8B`\x04\x85\x01a&\xF8V[\x03\x81\x88Z\xFA\x90\x81\x15a\x03EW_\x91a,\xDEW[P\x86R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x92\x90_\x81\x80a)a\x85\x87\x8B`\x04\x85\x01a'OV[\x03\x81\x87Z\xFA\x90\x81\x15a\x03EW_\x91a,\xC4W[P`@\x87\x01Ra)\x83\x81a!\xB7V[\x98``\x87\x01\x99\x8AR_[`\xFF\x81\x16\x83\x81\x10\x15a,\x0FW\x88_a)\xB6\x83\x8Fa)\xA9\x88a {V[\x90Q\x90a%Q\x83\x83a \xBAV[P_\x8A\x86\x8F[\x81\x84\x10a*9WPPPP\x90P\x8Ca)\xD3\x82a {V[\x91_[\x81\x81\x10a*\0WPP\x91a)\xF5\x91a)\xFB\x94\x93Q\x90a%Q\x83\x83a \xBAV[Pa'\x80V[a)\x8DV[\x80a*3a*\x1Ea\x14\x83`\x01\x94a*\x18\x8A\x89Qa \xBAV[Qa \xBAV[a*(\x83\x88a \xBAV[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[\x01a)\xD6V[a\x14\x83\x84a*N\x81` \x96\x95a*V\x95a'\x96V[5\x97Qa \xBAV[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x87\x01R\x16`D\x85\x01R\x83`d\x81\x8DZ\xFA\x80\x15a\x03EW\x88\x8F\x88\x8A\x91\x8F\x94a*\xFB`\x01a*\xEE\x81\x93\x8D\x80\x9D_\x92a+\xE3W[Pa#\xD4a*\xCAa*\xD8\x92a*\xC3\x87\x80`\xC0\x1B\x03\x86\x16\x15\x15a'\xC5V[\x8B\x8Da(]V[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\x01`\x01`\xC0\x1B\x03\x91\x82\x16`\xFF\x91\x90\x91\x16\x1C\x16\x90V[\x16`\x01`\x01`\xC0\x1B\x03\x16\x90V[\x14a+\x17W[PPPPP`\x01\x91\x92P\x01\x90\x8A\x91\x8A\x86\x8Fa)\xBCV[\x85\x97a+9\x93a+2` \x97\x99\x98a#\xD4\x95a*\xCA\x95a'\x96V[5\x95a(]V[`@Qc\xDD\x98F\xB9`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\xFF\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`D\x84\x01R\x82`d\x81\x8CZ\xFA\x90\x81\x15a\x03EW\x8Fa+\x97\x90a+\x9C\x93\x83\x88`\x01\x97_\x93a+\xABW[Pa*\x18\x90a*(\x93\x94Qa \xBAV[a(~V[\x90P\x82\x91\x8A\x88\x8F\x88\x8A\x91a+\x01V[a*(\x93P\x90a+\xD4a*\x18\x92` =\x81\x11a+\xDCW[a+\xCC\x81\x83a\x03\x9DV[\x81\x01\x90a(iV[\x93P\x90a+\x87V[P=a+\xC2V[a*\xD8\x91\x92Pa*\xCAa,\x06a#\xD4\x92` =\x81\x11a\x15\x0CWa\x14\xFE\x81\x83a\x03\x9DV[\x93\x92PPa*\xA6V[PPP\x92\x90\x95\x97P`\x04\x94\x96P` \x91P`@Q\x94\x85\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x90\x81\x15a\x03EWa,e\x94_\x94\x85\x93a,\xA3W[P`@Qc5IR\xA3`\xE2\x1B\x81R\x95\x86\x94\x85\x93\x84\x93`\x04\x85\x01a(\x8CV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03EW_\x91a,\x89W[P` \x82\x01R\x90V[a,\x9D\x91P=\x80_\x83>a\x156\x81\x83a\x03\x9DV[_a,\x80V[a,\xBD\x91\x93P` =` \x11a\n]Wa\nO\x81\x83a\x03\x9DV[\x91_a,GV[a,\xD8\x91P=\x80_\x83>a\x156\x81\x83a\x03\x9DV[_a)tV[a,\xF2\x91P=\x80_\x83>a\x156\x81\x83a\x03\x9DV[_a)1V[a-\x12\x91\x95P` =` \x11a\n]Wa\nO\x81\x83a\x03\x9DV[\x93_a(\xF5V[5a\x06\xC8\x81a\x05\xD1V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03JW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03JW` \x01\x91\x816\x03\x83\x13a\x03JWV[` \x81R\x815` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x015a-s\x81a\x05\xD1V[\x16`@\x82\x01R`@\x82\x015`\x1E\x19\x836\x03\x01\x81\x12\x15a\x03JW\x82\x01\x90` \x825\x92\x01`\x01`\x01`@\x1B\x03\x83\x11a\x03JW\x826\x03\x81\x13a\x03JWa-\xD1``a-\xCA`\x80\x93a\x06\xC8\x96\x85\x84\x88\x01R`\xA0\x87\x01\x91a'/V[\x95\x01a\x05\xDFV[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x15a-\xE3WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a.UWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAggregator has already responded`D\x82\x01Rk to the task`\xA0\x1B`d\x82\x01R`\x84\x90\xFD[c\xFF\xFF\xFF\xFF`\x01\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a'\x91WV[c\xFF\xFF\xFF\xFF`d\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a'\x91WV[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a'\x91WV[\x15a/\0WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAggregator has responded to the `D\x82\x01Rltask too late`\x98\x1B`d\x82\x01R`\x84\x90\xFD[` \x80\x91c\xFF\xFF\xFF\xFF\x815a/o\x81a\x05\xD1V[\x16\x84R\x015\x91\x01RV[`@\x81\x01\x92\x91a\x03\xCE\x91\x90a/[V[\x90`d\x82\x02\x91\x80\x83\x04`d\x14\x90\x15\x17\x15a'\x91WV[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a'\x91WV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a'\x91WV[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x02\x90`\x01`\x01``\x1B\x03\x82\x16\x91\x82\x03a'\x91WV[\x15a/\xF2WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FSignatories do not own at least `D\x82\x01R\x7Fthreshold percentage of a quorum`d\x82\x01R\xFD[\x90\x92\x91` ``\x91a0r\x84`\x80\x81\x01\x97a/[V[c\xFF\xFF\xFF\xFF\x81Q\x16`@\x85\x01R\x01Q\x91\x01RV[`@\x90c\xFF\xFF\xFF\xFFa\x06\xC8\x94\x93\x16\x81R\x81` \x82\x01R\x01\x90a\x06\x84V[\x15a0\xAAWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask hasn't been responded to ye`D\x82\x01R`\x1D`\xFA\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x92\x91` ``\x91a1\x0F\x84`\x80\x81\x01\x97a/[V[c\xFF\xFF\xFF\xFF\x815a1\x1F\x81a\x05\xD1V[\x16`@\x85\x01R\x015\x91\x01RV[\x15a13WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FTask response does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a1\xA5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FThe response to this task has al`D\x82\x01R\x7Fready been challenged successful`d\x82\x01Rb6<\x97`\xE9\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a2#WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FThe challenge period for this ta`D\x82\x01R\x7Fsk has already expired.\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a2\xB8WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a2\xABV[\x15a2\xD5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FThe pubkeys of non-signing opera`D\x82\x01R\x7Ftors supplied by the challenger `d\x82\x01Ro0\xB92\x9077\xBA\x101\xB7\xB992\xB1\xBA\x17`\x81\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a3`WV[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[\x93\x92\x91\x90\x93`\x01a3\x84\x86a-\x19V[\x95` a4@\x845a3\xAFa3\xA7\x8Bc\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15\x15a0\xA3V[a3\xEAa3\xCA\x8Bc\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T`@Q\x85\x81\x01\x90a3\xE1\x81a\x11\x97\x8D\x8B\x86a0\xF9V[Q\x90 \x14a1,V[a4\x15a4\x0Fa4\x08\x8Cc\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[T`\xFF\x16\x90V[\x15a1\x9EV[a4:a4,a\x122a4'\x8Aa-\x19V[a.\xC7V[c\xFF\xFF\xFF\xFFC\x16\x11\x15a2\x1CV[\x80a/\xB5V[\x91\x015\x14\x14a7\x93Wa4S\x83Qa {V[\x93_[\x84Q\x81\x10\x15a4\x93W\x80a4\x82a4o`\x01\x93\x88a \xBAV[Q\x80Q_R` \x01Q` R`@_ \x90V[a4\x8C\x82\x89a \xBAV[R\x01a4VV[P\x90\x92\x93\x91\x94a4\xCD` \x85\x01\x96` a4\xAC\x89a-\x19V[`@Qa4\xC1\x81a\x11\x97\x8A\x86\x83\x01\x95\x86a2\x8EV[Q\x90 \x91\x015\x14a2\xCEV[a4\xD7\x85Qa {V[\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x94_[\x87Q\x81\x10\x15a5\x87W\x80` a5\x1Ea5>\x93\x89a \xBAV[Q`@Q\x80\x94\x81\x92ct]\xCDs`\xE1\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8BZ\xFA\x91\x82\x15a\x03EW`\x01\x92a5c\x91_\x91a5iW[Pa\x0B\xB3\x83\x8Da \xBAV[\x01a5\x05V[a5\x81\x91P` =\x81\x11a\n]Wa\nO\x81\x83a\x03\x9DV[_a5XV[Pa5\xE7\x93\x95\x97\x96Pa5\xB0\x92\x94P\x90a5\xA8a5\xB8\x92`@\x81\x01\x90a-#V[\x93\x90\x91a-\x19V[\x926\x91a\x07\x98V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a#\x1BV[`\xD1T`\xD0T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16s\x8Ay\x16 \xDDb`\x07\x9B\xF8I\xDCUg\xAD\xC3\xF2\xFD\xC3\x18\x14\x93\x91\x16\x91_\x91[\x81Q\x83\x10\x15a75W_\x95[a6+\x84\x84a \xBAV[QQ\x87\x10\x15a7(W\x90\x82\x91` \x80a6L\x8Aa*\x18\x89a6n\x9A\x99a \xBAV[Q\x01Q`@Q\x80\x97\x81\x92c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8DZ\xFA\x94\x85\x15a\x03EW_\x95a7\x08W[P_[\x89Q\x81\x10\x15a6\xFAWa6\xA9a6\x9Da!\t\x83\x8Da \xBAV[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a6\xC0W`\x01\x01a6\x84V[P\x96`\x01\x91\x92\x93\x94P[a6\xEA` a6\xD7a\x03\xD0V[`\x01`\x01`\xA0\x1B\x03\x89\x16\x81R\x01`\x01\x90RV[a6\xF3\x87a3YV[\x01\x95a6!V[P\x96`\x01\x91\x92\x93\x94Pa6\xCAV[a7!\x91\x95P` =\x81\x11a\n]Wa\nO\x81\x83a\x03\x9DV[\x93_a6\x81V[\x95P`\x01\x90\x92\x01\x91a6\x15V[\x95PPPPP\x91Pa7ea7X\x82c\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[c\xFF\xFF\xFF\xFF3\x91\x16\x7F\xC2\r\x1B\xB0\xF1b6\x800k\x83\xD4\xFFK\xB9\x9A+\xEB\x9D\x86\xD9x2\xF3\xCA@\xFD\x13\xA2\x9D\xF1\xEC_\x80\xA3V[PPP\x90c\xFF\xFF\xFF\xFF3\x91\x16\x7F\xFD>&\xBE\xEBYg\xFCZW\xA0Di\x14\xEA\xBCE\xB4\xAAGLg\xA5\x1BKQ`\xCA\xC6\r\xDB\x05_\x80\xA3V[`@Q\x90a7\xD2\x82a\x03\x82V[_``\x83\x82\x81R\x82` \x82\x01R\x81`@\x82\x01R\x01RV[` c\xFF\xFF\xFF\xFF```\xC0\x94\x83\x85R\x80Q\x84\x86\x01R\x82\x84\x82\x01Q\x16`@\x86\x01R`@\x81\x01Q`\x80\x83\x87\x01R\x80Q\x94\x85\x91\x82`\xA0\x89\x01R\x01\x87\x87\x01^_\x85\x85\x01\x87\x01R\x01Q\x16`\x80\x83\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@Q\x90a8L\x82a\x03bV[``` \x83\x82\x81R\x01RV[\x15a8_WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: empty quorum input\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a8\xBEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: input quorum length mismatc`d\x82\x01R`\r`\xFB\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a9'WV[`\xA4`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: input nonsigner length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R\xFD[\x15a9\x92WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: invalid reference block\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x90\x81` \x91\x03\x12a\x03JWQa\x06\xC8\x81a\x0E(V[_\x19\x81\x01\x91\x90\x82\x11a'\x91WV[\x15a:\x14WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R\xFD[\x90`\x01\x82\x01\x80\x92\x11a'\x91WV[\x90`\x02\x82\x01\x80\x92\x11a'\x91WV[\x90`\x03\x82\x01\x80\x92\x11a'\x91WV[\x90`\x04\x82\x01\x80\x92\x11a'\x91WV[\x90`\x05\x82\x01\x80\x92\x11a'\x91WV[\x91\x90\x82\x01\x80\x92\x11a'\x91WV[\x15a:\xC5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: StakeRegistry updates must `d\x82\x01R\x7Fbe within withdrawalDelayBlocks `\x84\x82\x01Rewindow`\xD0\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x90\x81` \x91\x03\x12a\x03JWQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x03JW\x90V[\x15a;zWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: quorumApk hash in storage d`d\x82\x01R\x7Foes not match provided quorum ap`\x84\x82\x01R`k`\xF8\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a'\x91WV[\x15a<)WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a<\x94WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x94\x93\x92\x90\x91\x93a<\xFAa8?V[Pa=\x06\x85\x15\x15a8XV[`@\x84\x01QQ\x85\x14\x80aEhW[\x80aEZW[\x80aELW[a=)\x90a8\xB7V[a=;` \x85\x01QQ\x85QQ\x14a9 V[a=Rc\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a9\x8BV[a=Za\x03\xD0V[_\x81R_` \x82\x01R\x92a=la8?V[a=u\x87a {V[` \x82\x01Ra=\x83\x87a {V[\x81Ra=\x8Da8?V[\x92a=\x9C` \x88\x01QQa {V[\x84Ra=\xAC` \x88\x01QQa {V[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03EWa>\x15\x91_\x91aE\x1DW[Pa>\x106\x8B\x87a\x07\x98V[aKEV[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a?tW` \x88a>ia\x14\x83\x8Ca>a\x8F\x96\x86\x8Ea>Fa4o\x86\x80\x95a \xBAV[a>S\x84\x84\x84\x01Qa \xBAV[R\x82a?AW[\x01Qa \xBAV[Q\x95Qa \xBAV[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x03EWa &\x8Aa?\x16\x8Fa?\x0F\x8F\x84` \x8F\x92a?\x06\x93a>\xFE\x84`\x01\x9Ea?\x1C\x9E_\x91a?$W[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa \xBAV[R\x01Qa \xBAV[Q\x93\x8DQa \xBAV[Q\x16aK\xCCV[\x90aK\xFDV[\x97\x01\x96a>\x19V[a?;\x91P\x86=\x81\x11a\x15\x0CWa\x14\xFE\x81\x83a\x03\x9DV[_a>\xEFV[a?oa?Q\x84\x84\x84\x01Qa \xBAV[Qa?h\x84\x84\x01Qa?b\x87a9\xFFV[\x90a \xBAV[Q\x10a:\rV[a>ZV[P\x90\x95\x97\x94\x96Pa?\x89\x91\x98\x93\x92\x99PaL\xE3V[\x91a?\x96`\x97T`\xFF\x16\x90V[\x90\x81\x15aE\x15W`@Qc\x18\x89\x1F\xD7`\xE3\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03EW_\x91aD\xF6W[P\x91\x90[_\x92[\x81\x84\x10a@GWPPPPP\x92a@.a@)a@\"a@A\x95\x85a\x11\x97\x98`\x80``` \x99\x01Q\x92\x01Q\x92a\x1FsV[\x91\x90a<\"V[a<\x8DV[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a2\x8EV[Q\x90 \x90V[\x92\x98\x95\x96\x90\x93\x99\x91\x97\x94\x87\x8B\x88\x8C\x88\x8DaC\xF0W[a\x14\x83\x82`\xA0a@\x9Ca#\xD4a*\xCA\x84a@\xA4\x97a@\x96a@\x88a4o\x8F\x9C`@` \x9F\x9E\x01Qa \xBAV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x9Ba(]V[\x97\x01Qa \xBAV[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x95\x90\x95\x16`\x04\x86\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03EWaAha\x14\x83\x8F\x95\x8F\x90aA`\x8F\x97\x8F\x96\x84\x8FaAZ`\xC0\x96aAS\x84\x8F` \x9F\x90a>Za*\xCA\x99`@\x93a#\xD4\x9C_\x91aC\xC2W[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a;sV[Q\x90aG\nV[\x9Ca(]V[\x96\x01Qa \xBAV[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03EWaA\xF5\x91\x8C\x8F\x92_\x92aC\x9EW[P` aA\xE7\x92\x93\x01Qa \xBAV[\x90`\x01`\x01``\x1B\x03\x16\x90RV[aB\x15\x8CaA\xE7\x8CaB\x0Ea\x13\x06\x82` \x86\x01Qa \xBAV[\x92Qa \xBAV[_\x98_[` \x8A\x01QQ\x81\x10\x15aC\x85W\x8B\x8DaBW\x89aBJa#\xD4a*\xCA\x86\x8F\x89aBB\x91Qa \xBAV[Q\x94\x87a(]V[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[aBfW[PP`\x01\x01aB\x19V[\x8A\x8AaB\xE8\x85\x9F\x94\x8F\x96\x86a*\x18\x8F\x93`\xE0aB\x9Fa\x14\x83\x95` aB\x97a#\xD4a*\xCA\x83\x9FaB\xA8\x9C\x89\x91a(]V[\x9A\x01Qa \xBAV[Q\x9B\x01Qa \xBAV[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x96\x90\x96R\x91\x90\x94\x16`d\x85\x01R\x83\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03EW\x8FaCT\x90\x8F\x93`\x01\x95\x94\x86\x95_\x92aC_W[PaCNaA\xE7\x92\x93Q\x93aCIa\x13\x06\x84\x87a \xBAV[a<\x02V[\x92a \xBAV[\x01\x9A\x90P\x8B\x8DaB\\V[aA\xE7\x92PaC~aCN\x91` =\x81\x11a%\x87Wa%x\x81\x83a\x03\x9DV[\x92PaC1V[P\x93\x91\x97\x96\x99`\x01\x91\x96\x99P\x9A\x94\x92\x9A\x01\x92\x91\x90a?\xF1V[aA\xE7\x92PaC\xBB` \x91\x82=\x81\x11a%\x87Wa%x\x81\x83a\x03\x9DV[\x92PaA\xD8V[` aC\xE3\x92P=\x81\x11aC\xE9W[aC\xDB\x81\x83a\x03\x9DV[\x81\x01\x90a;RV[_aA=V[P=aC\xD1V[aD-\x94PaD\n\x92Pa#\xD4\x91a*\xCA\x91` \x95a(]V[`@Qc\x12M\x06!`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03EW` \x89a@\xA4\x8F\x93\x8F`\xA0\x8F\x97a#\xD4a*\xCA\x8F\x8F\x90a@\x96a@\x88a4o\x8F`@\x8B\x96\x91\x8F\x88\x93a\x14\x83\x9FaD\xB1\x90aD\xB7\x93a@\x9C\x9F_\x92aD\xCDW[Pc\xFF\xFF\xFF\xFF\x80\x91\x16\x93\x16\x90a:\xB1V[\x11a:\xBEV[PPPPPP\x97PPPPPP\x92\x93PPa@\\V[` c\xFF\xFF\xFF\xFF\x92\x93P\x82\x91aD\xEE\x91=\x81\x11a!\x96Wa!\x88\x81\x83a\x03\x9DV[\x92\x91PaD\xA0V[aE\x0F\x91P` =` \x11a+\xDCWa+\xCC\x81\x83a\x03\x9DV[_a?\xEAV[_\x91\x90a?\xEEV[aE?\x91P` =` \x11aEEW[aE7\x81\x83a\x03\x9DV[\x81\x01\x90a9\xEAV[_a>\x04V[P=aE-V[P`\xE0\x84\x01QQ\x85\x14a= V[P`\xC0\x84\x01QQ\x85\x14a=\x1AV[P`\xA0\x84\x01QQ\x85\x14a=\x14V[\x15aE}WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[aE\xE2\x90aM\xC0V[`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`\xCDT\x16\x17`\xCDU`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`\xCET\x16\x17`\xCEUV[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`@Q\x90aF\x8D\x82a\x03bV[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80aF\xA9\x81\x84a\x03\x9DV[6\x837V[`@Q\x90aF\xBD` \x83a\x03\x9DV[` 6\x837V[\x91\x90`@\x90``aF\xD3aF\x80V[\x94\x85\x92` \x85Q\x92aF\xE5\x85\x85a\x03\x9DV[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15aG\x08WV[\xFE[` \x92\x91`\x80`@\x92aG\x1BaF\x80V[\x95\x86\x93\x81\x86Q\x93aG,\x86\x86a\x03\x9DV[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15aG\x08W\x15aG]WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x90\xFD[`@QaG\x9E\x81a\x03bV[`@\x90\x81QaG\xAD\x83\x82a\x03\x9DV[\x826\x827\x81R` \x82Q\x91aG\xC2\x84\x84a\x03\x9DV[\x836\x847\x01R\x80QaG\xD4\x82\x82a\x03\x9DV[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90aH*\x83\x83a\x03\x9DV[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01RaH\x7F\x83Q\x93\x84a\x03\x9DV[\x82R` \x82\x01R\x90V[_Q` aP\\_9_Q\x90_R\x90aH\xA0aF\x80V[P_\x91\x90\x06` `\xC0\x83[aI\xA0W_\x93_Q` aP\\_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@QaH\xD6\x85\x82a\x03\x9DV[\x846\x827\x84\x81\x85`@QaH\xEA\x82\x82a\x03\x9DV[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aP\\_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15aG\x08WaIT\x90aP\x0FV[Q\x91aI\xA0W_Q` aP\\_9_Q\x90_R\x82\x80\t\x14aI\x8BWP_Q` aP\\_9_Q\x90_R`\x01_\x94\x08\x92\x93aH\xABV[\x92\x93PPaI\x97a\x03\xD0V[\x92\x83R\x82\x01R\x90V[a\x1F_V[aI\xADaF\x80V[P`@QaI\xBA\x81a\x03bV[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x0C\x81\x10\x15a\x1FZW`\x05\x1B\x01\x90V[\x93\x92\x90\x91aI\xE7`@a\x03\xEEV[\x94\x85R` \x85\x01RaI\xF9`@a\x03\xEEV[\x91\x82R` \x82\x01RaJ\taF\x98V[\x92_[`\x02\x81\x10aJ6WPPP` a\x01\x80\x92aJ%aF\xAEV[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80aJB`\x01\x92a/\x9FV[aJL\x82\x85a\x1FIV[QQaJX\x82\x89aI\xC8V[R` aJe\x83\x86a\x1FIV[Q\x01QaJzaJt\x83a:kV[\x89aI\xC8V[RaJ\x85\x82\x86a\x1FIV[QQQaJ\x94aJt\x83a:yV[RaJ\xAAaJ\xA2\x83\x87a\x1FIV[QQ` \x01\x90V[QaJ\xB7aJt\x83a:\x87V[R` aJ\xC4\x83\x87a\x1FIV[Q\x01QQaJ\xD4aJt\x83a:\x95V[RaK\0aJ\xFAaJ\xF3` aJ\xEA\x86\x8Aa\x1FIV[Q\x01Q` \x01\x90V[Q\x92a:\xA3V[\x88aI\xC8V[R\x01aJ\x0CV[` \x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x91\x15\x15`\xFF\x19`\x97T\x16`\xFF\x82\x16\x17`\x97U`@Q\x90\x81R\xA1V[\x90`\x01aKS`\xFF\x93aO(V[\x92\x83\x92\x16\x1B\x11\x15aKaW\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x90\xFD[\x80_\x91[aK\xD8WP\x90V[_\x19\x81\x01\x81\x81\x11a'\x91Wa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a'\x91W`\x01\x01\x90\x80aK\xD0V[\x90aL\x06aF\x80V[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15aL\xABW`\x01\x82\x14aL\xA6WaL'a\x03\xD0V[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15aLLWPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14aL\x86W[`\x01aL|aLq\x83`\xFF\x94aG\nV[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91aL8V[\x94`\x01aL|aLqaL\x9B\x89`\xFF\x95aG\nV[\x98\x93PPPPaL`V[PP\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x90\xFD[aL\xEBaF\x80V[P\x80Q\x90\x81\x15\x80aM\\W[\x15aM\x18WPP`@QaM\x0C`@\x82a\x03\x9DV[_\x81R_` \x82\x01R\x90V[` _Q` aP\\_9_Q\x90_R\x91\x01Q\x06_Q` aP\\_9_Q\x90_R\x03_Q` aP\\_9_Q\x90_R\x81\x11a'\x91W`@Q\x91aH\x7F\x83a\x03bV[P` \x81\x01Q\x15aL\xF7V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aM|WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[a\xFF\xFFaN\x14\x82aK\xCCV[\x16aN\x1E\x81a\x07}V[\x90aN,`@Q\x92\x83a\x03\x9DV[\x80\x82RaN;`\x1F\x19\x91a\x07}V[\x016` \x83\x017__[\x82Q\x82\x10\x80aN\x9BW[\x15aN\x94W`\x01\x81\x1B\x84\x16aNmW[aNh\x90a(~V[aNEV[\x90`\x01aNh\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1AaN\x8A\x82\x87a\"\0V[S\x01\x91\x90PaN_V[PP\x90P\x90V[Pa\x01\0\x81\x10aNOV[\x15aN\xADWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x90a\x01\0\x82Q\x11aO\x98W\x81Q\x15aO\x93WaOVaOLa#\xD4a#\xC6\x85a \xADV[`\xFF`\x01\x91\x16\x1B\x90V[`\x01\x90[\x83Q\x82\x10\x15aO\x8EW`\x01\x90aOyaOLa#\xD4a#\xC6\x86\x89a\"\0V[\x90aO\x85\x81\x83\x11aN\xA6V[\x17\x91\x01\x90aOZV[\x92PPV[_\x91PV[`\xA4`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R\xFD[\x15aP\x16WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGBLSSignatureChecker.checkSignatu\xA2dipfsX\"\x12 \xFD\"\xD4\xFF\xB9L\x08%\xEB\xFC\xC1\xA6\xADN\x1D \xCD\x0C\x7F\xDEaY\xBFN\x7FT|\xCEJ\xC8R\xF7dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c8063136439dd1461028a578063171f1d5b146102855780631ad43189146101db578063245a7bfc146102805780632cb223d51461027b5780632d89f6fc1461027657806331b36bd9146102715780633563b0d11461026c5780633998fdd314610267578063416c7e5e146102625780634d2b57fe1461025d5780634f739f7414610258578063595c6a67146102535780635ac86ab71461024e5780635baec9a0146102495780635c155662146102445780635c975abb1461023f5780635decc3f51461023a5780635df459461461023557806368304835146102305780636b532e9e1461022b5780636b92787e146102265780636d14a987146102215780636efb46361461021c578063715018a61461021757806372d18e8d146102085780637afa1eed14610212578063886f11951461020d5780638b00ce7c146102085780638da5cb5b146102035780639b290e98146101fe578063b98d0908146101f9578063c0c53b8b146101f4578063ca8aa7c7146101ef578063cefdc1d4146101ea578063df5cf723146101e5578063f2fde38b146101e0578063f5c9899d146101db578063f63c5bab146101d65763fabc1cbc146101d1575f80fd5b611e07565b611dec565b610569565b611d5b565b611d17565b611bd3565b611b94565b611a90565b611a6e565b611a46565b611a1e565b61198f565b6119da565b6119b2565b611934565b611887565b611807565b611699565b611628565b6115e4565b6115a0565b611562565b611545565b6113cc565b6110e2565b610e33565b610db5565b610d0e565b610b03565b610921565b6108ef565b610875565b6106cb565b610623565b6105ea565b6105a9565b610501565b3461034a57602036600319011261034a5760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156103455761031492610300915f91610316575b50611f09565b61030f60665482811614611f1f565b61464e565b005b610338915060203d60201161033e575b610330818361039d565b810190611ee9565b5f6102fa565b503d610326565b611efe565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761037d57604052565b61034e565b608081019081106001600160401b0382111761037d57604052565b90601f801991011681019081106001600160401b0382111761037d57604052565b604051906103ce6101008361039d565b565b604051906103ce60408361039d565b604051906103ce60608361039d565b906103ce604051928361039d565b60409060e319011261034a576040519061041582610362565b60e4358252610104356020830152565b919082604091031261034a5760405161043d81610362565b6020808294803584520135910152565b9080601f8301121561034a576040519161046860408461039d565b82906040810192831161034a57905b8282106104845750505090565b8135815260209182019101610477565b90608060631983011261034a576040516104ad81610362565b60206104c882946104bf81606461044d565b845260a461044d565b910152565b919060808382031261034a5760206104c8604051926104eb84610362565b604084966104f9838261044d565b86520161044d565b3461034a5761012036600319011261034a57600435604036602319011261034a57610559604091825161053381610362565b6024358152604435602082015261054936610494565b90610553366103fc565b92611f73565b8251911515825215156020820152f35b3461034a575f36600319011261034a57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b3461034a575f36600319011261034a5760cd546040516001600160a01b039091168152602090f35b63ffffffff81160361034a57565b35906103ce826105d1565b3461034a57602036600319011261034a5763ffffffff60043561060c816105d1565b165f5260cb602052602060405f2054604051908152f35b3461034a57602036600319011261034a5763ffffffff600435610645816105d1565b165f5260ca602052602060405f2054604051908152f35b6001600160a01b0381160361034a57565b6001600160401b03811161037d5760051b60200190565b90602080835192838152019201905f5b8181106106a15750505090565b8251845260209384019390920191600101610694565b9060206106c8928181520190610684565b90565b3461034a57604036600319011261034a576004356106e88161065c565b602435906001600160401b03821161034a573660238301121561034a578160040135916107148361066d565b92610722604051948561039d565b8084526024602085019160051b8301019136831161034a57602401905b8282106107635761075f61075386866120dd565b604051918291826106b7565b0390f35b6020809183356107728161065c565b81520191019061073f565b6001600160401b03811161037d57601f01601f191660200190565b9291926107a48261077d565b916107b2604051938461039d565b82948184528183011161034a578281602093845f960137010152565b9080602083519182815201916020808360051b8301019401925f915b8383106107f957505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b80821061083957505050602080600192970193019301919392906107ea565b909192602060606001926001600160601b0360408851868060a01b0381511684528581015186850152015116604082015201940192019061081a565b3461034a57606036600319011261034a576004356108928161065c565b6024356001600160401b03811161034a573660238201121561034a5761075f916108c96108db923690602481600401359101610798565b604435916108d6836105d1565b61231b565b6040519182916020835260208301906107ce565b3461034a575f36600319011261034a5760d1546040516001600160a01b039091168152602090f35b8015150361034a57565b3461034a57602036600319011261034a5760043561093e81610917565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610345575f91610a35575b506001600160a01b031633036109a45761031490614b07565b60405162461bcd60e51b815260206004820152605c60248201527f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e6160448201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560648201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000608482015260a490fd5b610a57915060203d602011610a5d575b610a4f818361039d565b8101906121a2565b5f61098b565b503d610a45565b9080601f8301121561034a578135610a7b8161066d565b92610a89604051948561039d565b81845260208085019260051b82010192831161034a57602001905b828210610ab15750505090565b8135815260209182019101610aa4565b60206040818301928281528451809452019201905f5b818110610ae45750505090565b82516001600160a01b0316845260209384019390920191600101610ad7565b3461034a57604036600319011261034a57600435610b208161065c565b6024356001600160401b03811161034a57610b3f903690600401610a64565b610b49815161207b565b916001600160a01b03165f5b8251811015610be657806020610b6e610b8e93866120ba565b5160405180948192630a5aec1960e21b8352600483019190602083019252565b0381865afa91821561034557600192610bc2915f91610bc8575b50610bb383886120ba565b6001600160a01b039091169052565b01610b55565b610be0915060203d8111610a5d57610a4f818361039d565b5f610ba8565b6040518061075f8682610ac1565b9181601f8401121561034a578235916001600160401b03831161034a576020838186019501011161034a57565b90602080835192838152019201905f5b818110610c3e5750505090565b825163ffffffff16845260209384019390920191600101610c31565b90602082526060610ca8610c93610c7d84516080602088015260a0870190610c21565b6020850151868203601f19016040880152610c21565b6040840151858203601f190184870152610c21565b910151916080601f1982840301910152815180825260208201916020808360051b8301019401925f915b838310610ce157505050505090565b9091929394602080610cff600193601f198682030187528951610c21565b97019301930191939290610cd2565b3461034a57608036600319011261034a57600435610d2b8161065c565b60243590610d38826105d1565b6044356001600160401b03811161034a57610d57903690600401610bf4565b91606435926001600160401b03841161034a573660238501121561034a578360040135926001600160401b03841161034a573660248560051b8701011161034a5761075f956024610da99601936128b1565b60405191829182610c5a565b3461034a575f36600319011261034a5760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561034557610e20915f916103165750611f09565b61031461461a565b60ff81160361034a57565b3461034a57602036600319011261034a576020600160ff600435610e5681610e28565b161b806066541614604051908152f35b9081608091031261034a5790565b604090602319011261034a57602490565b9080601f8301121561034a578135610e9c8161066d565b92610eaa604051948561039d565b81845260208085019260051b82010192831161034a57602001905b828210610ed25750505090565b602080918335610ee1816105d1565b815201910190610ec5565b81601f8201121561034a578035610f028161066d565b92610f10604051948561039d565b81845260208085019260061b8401019281841161034a57602001915b838310610f3a575050505090565b6020604091610f498486610425565b815201920191610f2c565b9080601f8301121561034a578135610f6b8161066d565b92610f79604051948561039d565b81845260208085019260051b8201019183831161034a5760208201905b838210610fa557505050505090565b81356001600160401b03811161034a57602091610fc787848094880101610e85565b815201910190610f96565b9190916101808184031261034a57610fe86103be565b9281356001600160401b03811161034a5781611005918401610e85565b845260208201356001600160401b03811161034a5781611026918401610eec565b602085015260408201356001600160401b03811161034a578161104a918401610eec565b604085015261105c81606084016104cd565b606085015261106e8160e08401610425565b60808501526101208201356001600160401b03811161034a5781611093918401610e85565b60a08501526101408201356001600160401b03811161034a57816110b8918401610e85565b60c08501526101608201356001600160401b03811161034a576110db9201610f54565b60e0830152565b3461034a57608036600319011261034a576004356001600160401b03811161034a57611112903690600401610e66565b61111b36610e74565b906064356001600160401b03811161034a5761113b903690600401610fd2565b60cd549092906001600160a01b0316330361134e5761115e602083949301612d19565b9161126161116f6040860186612d23565b9290946111cf61118160608901612d19565b976040516111a581611197602082019485612d55565b03601f19810183528261039d565b5190206111c86111b488612d19565b63ffffffff165f5260ca60205260405f2090565b5414612ddc565b6111f96111f26111de87612d19565b63ffffffff165f5260cb60205260405f2090565b5415612e4e565b8363ffffffff43169661124361123b6112327f000000000000000000000000000000000000000000000000000000000000000086612edf565b63ffffffff1690565b891115612ef9565b6040516020810190611259816111978b85612f79565b519020613cec565b919060ff5f9616955b8281106112ec577f349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a8686866112ac6112a06103d0565b63ffffffff9094168452565b602083015260405160208101906112c88161119786868661305c565b5190206112d76111de83612d19565b556112e76040519283928361305c565b0390a1005b8061134861132461131f61131361130660019688516120ba565b516001600160601b031690565b6001600160601b031690565b612f89565b6113416113138b61133c6113068760208b01516120ba565b612fc8565b1115612feb565b0161126a565b60405162461bcd60e51b815260206004820152601d60248201527f41676772656761746f72206d757374206265207468652063616c6c65720000006044820152606490fd5b60206040818301928281528451809452019201905f5b8181106113b65750505090565b82518452602093840193909201916001016113a9565b3461034a57606036600319011261034a576004356113e98161065c565b6024356001600160401b03811161034a57611408903690600401610a64565b60443591611415836105d1565b6040516361c8a12f60e11b8152906001600160a01b03165f828061143d868860048401613086565b0381845afa918215610345575f92611521575b5061145b835161207b565b935f5b84518110156115135761147181866120ba565b519060208361148d61148384896120ba565b5163ffffffff1690565b6040516304ec635160e01b8152600481019590955263ffffffff918216602486015216604484015282606481875afa8015610345576001925f916114e5575b50828060c01b03166114de82896120ba565b520161145e565b611506915060203d811161150c575b6114fe818361039d565b8101906127a6565b5f6114cc565b503d6114f4565b6040518061075f8882611393565b61153e9192503d805f833e611536818361039d565b810190612675565b905f611450565b3461034a575f36600319011261034a576020606654604051908152f35b3461034a57602036600319011261034a5763ffffffff600435611584816105d1565b165f5260cc602052602060ff60405f2054166040519015158152f35b3461034a575f36600319011261034a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461034a575f36600319011261034a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461034a5760c036600319011261034a576004356001600160401b03811161034a57611658903690600401610e66565b61166136610e74565b90604036606319011261034a5760a4356001600160401b03811161034a57610314926116936064923690600401610eec565b92613374565b3461034a57606036600319011261034a576024356004356116b9826105d1565b6044356001600160401b03811161034a576116d8903690600401610bf4565b60ce5491939092916001600160a01b031633036117b857610314936117a293611722611729936117066137c5565b9586524363ffffffff16602087015263ffffffff166060860152565b3691610798565b604082015260405160208101906117448161119785856137e9565b5190206117596111b460c95463ffffffff1690565b5560c95463ffffffff16907f1695b8d06ec800b4615e745cfb5bd00c1f2875615d42925c3b5afa543bb24c486040518061179a63ffffffff861694826137e9565b0390a2612eaf565b63ffffffff1663ffffffff1960c954161760c955565b60405162461bcd60e51b815260206004820152602160248201527f5461736b2067656e657261746f72206d757374206265207468652063616c6c656044820152603960f91b6064820152608490fd5b3461034a575f36600319011261034a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b90602080835192838152019201905f5b8181106118685750505090565b82516001600160601b031684526020938401939092019160010161185b565b3461034a57608036600319011261034a576004356024356001600160401b03811161034a576118ba903690600401610bf4565b90916044356118c8816105d1565b606435926001600160401b03841161034a5761192a946118ef6118f5953690600401610fd2565b93613cec565b6040519283926040845260206119168251604080880152608087019061184b565b910151848203603f1901606086015261184b565b9060208301520390f35b3461034a575f36600319011261034a5761194c614d68565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461034a575f36600319011261034a57602063ffffffff60c95416604051908152f35b3461034a575f36600319011261034a5760ce546040516001600160a01b039091168152602090f35b3461034a575f36600319011261034a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461034a575f36600319011261034a576033546040516001600160a01b039091168152602090f35b3461034a575f36600319011261034a5760cf546040516001600160a01b039091168152602090f35b3461034a575f36600319011261034a57602060ff609754166040519015158152f35b3461034a57606036600319011261034a57600435611aad8161065c565b611b08602435611abc8161065c565b60443590611ac98261065c565b5f5493611aee60ff600887901c161580968197611b86575b8115611b66575b50614576565b84611aff600160ff195f5416175f55565b611b4f576145d9565b611b0e57005b611b1c61ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989080602081016112e7565b611b6161010061ff00195f5416175f55565b6145d9565b303b15915081611b78575b505f611ae8565b60ff1660011490505f611b71565b600160ff8216109150611ae1565b3461034a575f36600319011261034a5760d0546040516001600160a01b039091168152602090f35b6040906106c89392815281602082015201906107ce565b3461034a57606036600319011261034a57600435611bf08161065c565b602435604435611bff816105d1565b611c40611c0a612059565b9280611c15856120ad565b526040516361c8a12f60e11b81526001600160a01b0386169490925f91849182918760048401613086565b0381875afa9384156103455783611c6a611232611483611c9f986020975f91611cfd575b506120ad565b92604051968794859384936304ec635160e01b85526004850163ffffffff604092959493606083019683521660208201520152565b03915afa801561034557611cce925f91611cde575b506001600160c01b031692611cc884614e08565b9061231b565b9061075f60405192839283611bbc565b611cf7915060203d60201161150c576114fe818361039d565b5f611cb4565b611d1191503d805f833e611536818361039d565b5f611c64565b3461034a575f36600319011261034a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461034a57602036600319011261034a57600435611d788161065c565b611d80614d68565b6001600160a01b03811615611d985761031490614dc0565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b3461034a575f36600319011261034a57602060405160648152f35b3461034a57602036600319011261034a5760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610345575f91611eca575b506001600160a01b03163303611ebb57611e89606654198219811614611f1f565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b611ee3915060203d602011610a5d57610a4f818361039d565b5f611e68565b9081602091031261034a57516106c881610917565b6040513d5f823e3d90fd5b15611f1057565b631d77d47760e21b5f5260045ffd5b15611f2657565b63c61dca5d60e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b906002811015611f5a5760051b0190565b611f35565b634e487b7160e01b5f52601260045260245ffd5b61204f61202c6120559561202661201f85875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e0840152610100830152611ff681610120840103601f19810183528261039d565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b80966146c4565b9061470a565b9261202661204161203b614792565b94614889565b9161204a6149a5565b6146c4565b916149d9565b9091565b6040805190919061206a838261039d565b6001815291601f1901366020840137565b906120858261066d565b612092604051918261039d565b82815280926120a3601f199161066d565b0190602036910137565b805115611f5a5760200190565b8051821015611f5a5760209160051b010190565b9081602091031261034a575190565b9190916120ea835161207b565b925f5b815181101561219d5780602061211661210961213f94866120ba565b516001600160a01b031690565b6040516309aa152760e11b81526001600160a01b03909116600482015292839081906024820190565b03816001600160a01b0388165afa8015610345576001925f9161216f575b5061216882886120ba565b52016120ed565b612190915060203d8111612196575b612188818361039d565b8101906120ce565b5f61215d565b503d61217e565b505050565b9081602091031261034a57516106c88161065c565b906121c18261066d565b6121ce604051918261039d565b828152602081936121e1601f199161066d565b0191015f5b8281106121f257505050565b6060828201526020016121e6565b908151811015611f5a570160200190565b60208183031261034a578051906001600160401b03821161034a57019080601f8301121561034a5781516122448161066d565b92612252604051948561039d565b81845260208085019260051b82010192831161034a57602001905b82821061227a5750505090565b815181526020918201910161226d565b906122948261066d565b6122a1604051918261039d565b82815280926122b2601f199161066d565b015f5b8181106122c157505050565b6040519060608201918083106001600160401b0384111761037d576020926040525f81525f838201525f6040820152828286010152016122b5565b9081602091031261034a57516001600160601b038116810361034a5790565b604051636830483560e01b815293919291906001600160a01b0316602085600481845afa948515610345575f95612630575b50604051634f4c91e160e11b815294602086600481855afa918215610345576004965f9361260e575b5060209060405197888092632efa2ca360e11b82525afa958615610345575f966125ed575b506123a985939295516121b7565b945f935b80518510156125e3576123da6123d46123c68784612200565b516001600160f81b03191690565b60f81c90565b604051638902624560e01b815260ff8216600482015263ffffffff88166024820152909490925f846044816001600160a01b0385165afa938415610345575f946125bf575b5061242a845161228a565b612434888b6120ba565b5261243f878a6120ba565b505f5b84518110156125ae5780602061245b61247d93886120ba565b518d60405180809681946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa918215610345575f9261258e575b506124a381876120ba565b518a60208a6124b2858b6120ba565b5160405163fa28c62760e01b8152600481019190915260ff91909116602482015263ffffffff929092166044830152816064816001600160a01b038d165afa938415610345576125458c8f6125406001986125579789975f9261255e575b5061252b61251c6103df565b6001600160a01b039098168852565b60208701526001600160601b03166040860152565b6120ba565b519061255183836120ba565b526120ba565b5001612442565b61258091925060203d8111612587575b612578818361039d565b8101906122fc565b905f612510565b503d61256e565b6125a791925060203d8111610a5d57610a4f818361039d565b905f612498565b5060019096019590945091506123ad565b6125dc9194503d805f833e6125d4818361039d565b810190612211565b925f61241f565b5050509350505090565b61260791965060203d602011610a5d57610a4f818361039d565b945f61239b565b602091935061262990823d8411610a5d57610a4f818361039d565b9290612376565b61264a91955060203d602011610a5d57610a4f818361039d565b935f61234d565b6040519061265e82610382565b606080838181528160208201528160408201520152565b60208183031261034a578051906001600160401b03821161034a57019080601f8301121561034a5781516126a88161066d565b926126b6604051948561039d565b81845260208085019260051b82010192831161034a57602001905b8282106126de5750505090565b6020809183516126ed816105d1565b8152019101906126d1565b63ffffffff909116815260406020820181905281018390526001600160fb1b03831161034a5760609260051b809284830137010190565b908060209392818452848401375f828201840152601f01601f1916010190565b60409063ffffffff6106c89593168152816020820152019161272f565b634e487b7160e01b5f52601160045260245ffd5b60ff1660ff81146127915760010190565b61276c565b9190811015611f5a5760051b0190565b9081602091031261034a57516001600160c01b038116810361034a5790565b156127cc57565b60405162461bcd60e51b815260206004820152605c60248201527f4f70657261746f7253746174655265747269657665722e676574436865636b5360448201527f69676e617475726573496e64696365733a206f70657261746f72206d7573742060648201527f6265207265676973746572656420617420626c6f636b6e756d62657200000000608482015260a490fd5b90821015611f5a570190565b9081602091031261034a57516106c8816105d1565b5f1981146127915760010190565b916128aa60209263ffffffff9296959660408652604086019161272f565b9416910152565b95939495929091926128c1612651565b50604051636830483560e01b8152936001600160a01b03919091169190602085600481865afa948515610345575f95612cf8575b506128fe612651565b946040516361c8a12f60e11b81525f818061291e8d8d8b600485016126f8565b0381885afa908115610345575f91612cde575b5086526040516340e03a8160e11b81526001600160a01b039190911692905f818061296185878b6004850161274f565b0381875afa908115610345575f91612cc4575b506040870152612983816121b7565b9860608701998a525f5b60ff811683811015612c0f57885f6129b6838f6129a98861207b565b90519061255183836120ba565b505f8a868f5b818410612a39575050505090508c6129d38261207b565b915f5b818110612a00575050916129f5916129fb9493519061255183836120ba565b50612780565b61298d565b80612a33612a1e611483600194612a188a89516120ba565b516120ba565b612a2883886120ba565b9063ffffffff169052565b016129d6565b61148384612a4e8160209695612a5695612796565b3597516120ba565b6040516304ec635160e01b8152600481019690965263ffffffff9182166024870152166044850152836064818d5afa801561034557888f888a918f94612afb6001612aee81938d809d5f92612be3575b506123d4612aca612ad892612ac3878060c01b03861615156127c5565b8b8d61285d565b356001600160f81b03191690565b6001600160c01b0391821660ff919091161c1690565b166001600160c01b031690565b14612b17575b5050505050600191925001908a918a868f6129bc565b8597612b3993612b3260209799986123d495612aca95612796565b359561285d565b60405163dd9846b960e01b8152600481019290925260ff16602482015263ffffffff939093166044840152826064818c5afa908115610345578f612b9790612b9c9383886001975f93612bab575b50612a1890612a289394516120ba565b61287e565b905082918a888f888a91612b01565b612a28935090612bd4612a189260203d8111612bdc575b612bcc818361039d565b810190612869565b935090612b87565b503d612bc2565b612ad8919250612aca612c066123d49260203d811161150c576114fe818361039d565b93925050612aa6565b505050929095975060049496506020915060405194858092632efa2ca360e11b82525afa90811561034557612c65945f948593612ca3575b5060405163354952a360e21b8152958694859384936004850161288c565b03916001600160a01b03165afa908115610345575f91612c89575b50602082015290565b612c9d91503d805f833e611536818361039d565b5f612c80565b612cbd91935060203d602011610a5d57610a4f818361039d565b915f612c47565b612cd891503d805f833e611536818361039d565b5f612974565b612cf291503d805f833e611536818361039d565b5f612931565b612d1291955060203d602011610a5d57610a4f818361039d565b935f6128f5565b356106c8816105d1565b903590601e198136030182121561034a57018035906001600160401b03821161034a5760200191813603831361034a57565b602081528135602082015263ffffffff6020830135612d73816105d1565b1660408201526040820135601e198336030181121561034a578201906020823592016001600160401b03831161034a57823603811361034a57612dd16060612dca6080936106c896858488015260a087019161272f565b95016105df565b63ffffffff16910152565b15612de357565b60405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b15612e5557565b60405162461bcd60e51b815260206004820152602c60248201527f41676772656761746f722068617320616c726561647920726573706f6e64656460448201526b20746f20746865207461736b60a01b6064820152608490fd5b63ffffffff60019116019063ffffffff821161279157565b63ffffffff60649116019063ffffffff821161279157565b9063ffffffff8091169116019063ffffffff821161279157565b15612f0057565b60405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b6064820152608490fd5b6020809163ffffffff8135612f6f816105d1565b1684520135910152565b6040810192916103ce9190612f5b565b9060648202918083046064149015171561279157565b9060068202918083046006149015171561279157565b8181029291811591840414171561279157565b906001600160601b03809116911602906001600160601b03821691820361279157565b15612ff257565b608460405162461bcd60e51b815260206004820152604060248201527f5369676e61746f7269657320646f206e6f74206f776e206174206c656173742060448201527f7468726573686f6c642070657263656e74616765206f6620612071756f72756d6064820152fd5b9092916020606091613072846080810197612f5b565b63ffffffff81511660408501520151910152565b60409063ffffffff6106c894931681528160208201520190610684565b156130aa57565b60405162461bcd60e51b815260206004820152602160248201527f5461736b206861736e2774206265656e20726573706f6e64656420746f2079656044820152601d60fa1b6064820152608490fd5b909291602060609161310f846080810197612f5b565b63ffffffff813561311f816105d1565b1660408501520135910152565b1561313357565b60405162461bcd60e51b815260206004820152603d60248201527f5461736b20726573706f6e736520646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b156131a557565b60405162461bcd60e51b815260206004820152604360248201527f54686520726573706f6e736520746f2074686973207461736b2068617320616c60448201527f7265616479206265656e206368616c6c656e676564207375636365737366756c606482015262363c9760e91b608482015260a490fd5b1561322357565b60405162461bcd60e51b815260206004820152603760248201527f546865206368616c6c656e676520706572696f6420666f72207468697320746160448201527f736b2068617320616c726561647920657870697265642e0000000000000000006064820152608490fd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b8181106132b85750505090565b82518452602093840193909201916001016132ab565b156132d557565b60405162461bcd60e51b815260206004820152605060248201527f546865207075626b657973206f66206e6f6e2d7369676e696e67206f7065726160448201527f746f727320737570706c69656420627920746865206368616c6c656e6765722060648201526f30b932903737ba1031b7b93932b1ba1760811b608482015260a490fd5b1561336057565b634e487b7160e01b5f52600160045260245ffd5b9392919093600161338486612d19565b95602061344084356133af6133a78b63ffffffff165f5260cb60205260405f2090565b5415156130a3565b6133ea6133ca8b63ffffffff165f5260cb60205260405f2090565b54604051858101906133e1816111978d8b866130f9565b5190201461312c565b61341561340f6134088c63ffffffff165f5260cc60205260405f2090565b5460ff1690565b1561319e565b61343a61342c6112326134278a612d19565b612ec7565b63ffffffff4316111561321c565b80612fb5565b910135141461379357613453835161207b565b935f5b8451811015613493578061348261346f600193886120ba565b5180515f526020015160205260405f2090565b61348c82896120ba565b5201613456565b5090929391946134cd602085019660206134ac89612d19565b6040516134c1816111978a868301958661328e565b519020910135146132ce565b6134d7855161207b565b957f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316945f5b87518110156135875780602061351e61353e93896120ba565b516040518094819263745dcd7360e11b8352600483019190602083019252565b03818b5afa91821561034557600192613563915f91613569575b50610bb3838d6120ba565b01613505565b613581915060203d8111610a5d57610a4f818361039d565b5f613558565b506135e793959796506135b0929450906135a86135b8926040810190612d23565b939091612d19565b923691610798565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031661231b565b60d15460d0546001600160a01b03908116738a791620dd6260079bf849dc5567adc3f2fdc31814939116915f915b8151831015613735575f955b61362b84846120ba565b51518710156137285790829160208061364c8a612a188961366e9a996120ba565b510151604051809781926308f6629d60e31b8352600483019190602083019252565b03818d5afa948515610345575f95613708575b505f5b89518110156136fa576136a961369d612109838d6120ba565b6001600160a01b031690565b6001600160a01b038716146136c057600101613684565b5096600191929394505b6136ea60206136d76103d0565b6001600160a01b03891681520160019052565b6136f387613359565b0195613621565b5096600191929394506136ca565b61372191955060203d8111610a5d57610a4f818361039d565b935f613681565b9550600190920191613615565b95505050505091506137656137588263ffffffff165f5260cc60205260405f2090565b805460ff19166001179055565b63ffffffff3391167fc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec5f80a3565b5050509063ffffffff3391167ffd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb055f80a3565b604051906137d282610382565b5f6060838281528260208201528160408201520152565b602063ffffffff606060c0948385528051848601528284820151166040860152604081015160808387015280519485918260a0890152018787015e5f8585018701520151166080830152601f01601f1916010190565b6040519061384c82610362565b60606020838281520152565b1561385f57565b60405162461bcd60e51b815260206004820152603760248201525f51602061507c5f395f51905f5260448201527f7265733a20656d7074792071756f72756d20696e7075740000000000000000006064820152608490fd5b156138be57565b60405162461bcd60e51b815260206004820152604160248201525f51602061507c5f395f51905f5260448201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d6174636064820152600d60fb1b608482015260a490fd5b1561392757565b60a460405162461bcd60e51b815260206004820152604460248201525f51602061507c5f395f51905f5260448201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d6064820152630c2e8c6d60e31b6084820152fd5b1561399257565b60405162461bcd60e51b815260206004820152603c60248201525f51602061507c5f395f51905f5260448201527f7265733a20696e76616c6964207265666572656e636520626c6f636b000000006064820152608490fd5b9081602091031261034a57516106c881610e28565b5f1981019190821161279157565b15613a1457565b608460405162461bcd60e51b815260206004820152604060248201525f51602061507c5f395f51905f5260448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f727465646064820152fd5b906001820180921161279157565b906002820180921161279157565b906003820180921161279157565b906004820180921161279157565b906005820180921161279157565b9190820180921161279157565b15613ac557565b60405162461bcd60e51b815260206004820152606660248201525f51602061507c5f395f51905f5260448201527f7265733a205374616b6552656769737472792075706461746573206d7573742060648201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060848201526577696e646f7760d01b60a482015260c490fd5b9081602091031261034a575167ffffffffffffffff198116810361034a5790565b15613b7a57565b60405162461bcd60e51b815260206004820152606160248201525f51602061507c5f395f51905f5260448201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460648201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d2061706084820152606b60f81b60a482015260c490fd5b906001600160601b03809116911603906001600160601b03821161279157565b15613c2957565b60405162461bcd60e51b815260206004820152604360248201525f51602061507c5f395f51905f5260448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a490fd5b15613c9457565b60405162461bcd60e51b815260206004820152603960248201525f51602061507c5f395f51905f5260448201527f7265733a207369676e617475726520697320696e76616c6964000000000000006064820152608490fd5b949392909193613cfa61383f565b50613d06851515613858565b604084015151851480614568575b8061455a575b8061454c575b613d29906138b7565b613d3b60208501515185515114613920565b613d5263ffffffff431663ffffffff84161061398b565b613d5a6103d0565b5f81525f602082015292613d6c61383f565b613d758761207b565b6020820152613d838761207b565b8152613d8d61383f565b92613d9c60208801515161207b565b8452613dac60208801515161207b565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561034557613e15915f9161451d575b50613e10368b87610798565b614b45565b985f965b60208901518051891015613f7457602088613e696114838c613e618f96868e613e4661346f8680956120ba565b613e5384848401516120ba565b5282613f41575b01516120ba565b5195516120ba565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa918215610345576120268a613f168f613f0f8f8460208f92613f0693613efe8460019e613f1c9e5f91613f24575b508f8060c01b031692516120ba565b5201516120ba565b51938d516120ba565b5116614bcc565b90614bfd565b970196613e19565b613f3b9150863d811161150c576114fe818361039d565b5f613eef565b613f6f613f5184848401516120ba565b51613f6884840151613f62876139ff565b906120ba565b5110613a0d565b613e5a565b50909597949650613f89919893929950614ce3565b91613f9660975460ff1690565b908115614515576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610345575f916144f6575b5091905b5f925b8184106140475750505050509261402e61402961402261404195856111979860806060602099015192015192611f73565b9190613c22565b613c8d565b015160405192839160208301958661328e565b51902090565b92989596909399919794878b888c888d6143f0575b6114838260a061409c6123d4612aca846140a49761409661408861346f8f9c604060209f9e01516120ba565b67ffffffffffffffff191690565b9b61285d565b9701516120ba565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa908115610345576141686114838f958f906141608f978f96848f61415a60c096614153848f60209f90613e5a612aca996040936123d49c5f916143c2575b5067ffffffffffffffff19918216911614613b73565b519061470a565b9c61285d565b9601516120ba565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa908115610345576141f5918c8f925f9261439e575b5060206141e7929301516120ba565b906001600160601b03169052565b6142158c6141e78c61420e6113068260208601516120ba565b92516120ba565b5f985f5b60208a015151811015614385578b8d6142578961424a6123d4612aca868f8961424291516120ba565b51948761285d565b60ff161c60019081161490565b614266575b5050600101614219565b8a8a6142e8859f948f9686612a188f9360e061429f6114839560206142976123d4612aca839f6142a89c899161285d565b9a01516120ba565b519b01516120ba565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610345578f614354908f936001959486955f9261435f575b5061434e6141e79293519361434961130684876120ba565b613c02565b926120ba565b019a90508b8d61425c565b6141e7925061437e61434e9160203d811161258757612578818361039d565b9250614331565b5093919796996001919699509a94929a01929190613ff1565b6141e792506143bb602091823d811161258757612578818361039d565b92506141d8565b60206143e392503d81116143e9575b6143db818361039d565b810190613b52565b5f61413d565b503d6143d1565b61442d945061440a92506123d491612aca9160209561285d565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa8015610345576020896140a48f938f60a08f976123d4612aca8f8f9061409661408861346f8f60408b96918f88936114839f6144b1906144b79361409c9f5f926144cd575b5063ffffffff809116931690613ab1565b11613abe565b505050505050975050505050509293505061405c565b602063ffffffff92935082916144ee913d811161219657612188818361039d565b9291506144a0565b61450f915060203d602011612bdc57612bcc818361039d565b5f613fea565b5f9190613fee565b61453f915060203d602011614545575b614537818361039d565b8101906139ea565b5f613e04565b503d61452d565b5060e0840151518514613d20565b5060c0840151518514613d1a565b5060a0840151518514613d14565b1561457d57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b6145e290614dc0565b60018060a01b03166001600160601b0360a01b60cd54161760cd5560018060a01b03166001600160601b0360a01b60ce54161760ce55565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b6040519061468d82610362565b5f6020838281520152565b604051906101806146a9818461039d565b368337565b604051906146bd60208361039d565b6020368337565b919060409060606146d3614680565b94859260208551926146e5858561039d565b8436853780518452015160208301528482015260076107cf195a01fa1561470857565bfe5b60209291608060409261471b614680565b9586938186519361472c868661039d565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa8015614708571561475d57565b60405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606490fd5b60405161479e81610362565b60409081516147ad838261039d565b82368237815260208251916147c2848461039d565b83368437015280516147d4828261039d565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed602082015281519061482a838361039d565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d602083015261487f8351938461039d565b8252602082015290565b5f51602061505c5f395f51905f52906148a0614680565b505f919006602060c0835b6149a0575f935f51602061505c5f395f51905f52600381868181800909086040516148d6858261039d565b843682378481856040516148ea828261039d565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f51602061505c5f395f51905f5260a082015260056107cf195a01fa8015614708576149549061500f565b51916149a0575f51602061505c5f395f51905f528280091461498b57505f51602061505c5f395f51905f5260015f940892936148ab565b929350506149976103d0565b92835282015290565b611f5f565b6149ad614680565b506040516149ba81610362565b600181526002602082015290565b90600c811015611f5a5760051b0190565b939290916149e760406103ee565b94855260208501526149f960406103ee565b9182526020820152614a09614698565b925f5b60028110614a3657505050602061018092614a256146ae565b93849160086201d4c0fa9151151590565b80614a42600192612f9f565b614a4c8285611f49565b5151614a5882896149c8565b526020614a658386611f49565b510151614a7a614a7483613a6b565b896149c8565b52614a858286611f49565b515151614a94614a7483613a79565b52614aaa614aa28387611f49565b515160200190565b51614ab7614a7483613a87565b526020614ac48387611f49565b51015151614ad4614a7483613a95565b52614b00614afa614af36020614aea868a611f49565b51015160200190565b5192613aa3565b886149c8565b5201614a0c565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff196097541660ff821617609755604051908152a1565b906001614b5360ff93614f28565b928392161b1115614b615790565b60405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c7565006064820152608490fd5b805f915b614bd8575090565b5f1981018181116127915761ffff9116911661ffff8114612791576001019080614bd0565b90614c06614680565b5061ffff811690610200821015614cab5760018214614ca657614c276103d0565b5f81525f602082015292906001905f925b61ffff8316851015614c4c57505050505090565b600161ffff831660ff86161c811614614c86575b6001614c7c614c718360ff9461470a565b9460011b61fffe1690565b9401169291614c38565b946001614c7c614c71614c9b8960ff9561470a565b989350505050614c60565b505090565b60405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b6044820152606490fd5b614ceb614680565b50805190811580614d5c575b15614d18575050604051614d0c60408261039d565b5f81525f602082015290565b60205f51602061505c5f395f51905f52910151065f51602061505c5f395f51905f52035f51602061505c5f395f51905f528111612791576040519161487f83610362565b50602081015115614cf7565b6033546001600160a01b03163303614d7c57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b61ffff614e1482614bcc565b16614e1e8161077d565b90614e2c604051928361039d565b808252614e3b601f199161077d565b013660208301375f5f5b8251821080614e9b575b15614e94576001811b8416614e6d575b614e689061287e565b614e45565b906001614e689160ff60f81b8460f81b165f1a614e8a8287612200565b5301919050614e5f565b5050905090565b506101008110614e4f565b15614ead57565b60405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a490fd5b90610100825111614f9857815115614f9357614f56614f4c6123d46123c6856120ad565b60ff600191161b90565b6001905b8351821015614f8e57600190614f79614f4c6123d46123c68689612200565b90614f85818311614ea6565b17910190614f5a565b925050565b5f9150565b60a460405162461bcd60e51b815260206004820152604460248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b6084820152fd5b1561501657565b60405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000006044820152606490fdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47424c535369676e6174757265436865636b65722e636865636b5369676e617475a2646970667358221220fd22d4ffb94c0825ebfcc1a6ad4e1d20cd0c7fde6159bf4e7f547cce4ac852f764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x13d9\xDD\x14a\x02\x8AW\x80c\x17\x1F\x1D[\x14a\x02\x85W\x80c\x1A\xD41\x89\x14a\x01\xDBW\x80c$Z{\xFC\x14a\x02\x80W\x80c,\xB2#\xD5\x14a\x02{W\x80c-\x89\xF6\xFC\x14a\x02vW\x80c1\xB3k\xD9\x14a\x02qW\x80c5c\xB0\xD1\x14a\x02lW\x80c9\x98\xFD\xD3\x14a\x02gW\x80cAl~^\x14a\x02bW\x80cM+W\xFE\x14a\x02]W\x80cOs\x9Ft\x14a\x02XW\x80cY\\jg\x14a\x02SW\x80cZ\xC8j\xB7\x14a\x02NW\x80c[\xAE\xC9\xA0\x14a\x02IW\x80c\\\x15Vb\x14a\x02DW\x80c\\\x97Z\xBB\x14a\x02?W\x80c]\xEC\xC3\xF5\x14a\x02:W\x80c]\xF4YF\x14a\x025W\x80ch0H5\x14a\x020W\x80ckS.\x9E\x14a\x02+W\x80ck\x92x~\x14a\x02&W\x80cm\x14\xA9\x87\x14a\x02!W\x80cn\xFBF6\x14a\x02\x1CW\x80cqP\x18\xA6\x14a\x02\x17W\x80cr\xD1\x8E\x8D\x14a\x02\x08W\x80cz\xFA\x1E\xED\x14a\x02\x12W\x80c\x88o\x11\x95\x14a\x02\rW\x80c\x8B\0\xCE|\x14a\x02\x08W\x80c\x8D\xA5\xCB[\x14a\x02\x03W\x80c\x9B)\x0E\x98\x14a\x01\xFEW\x80c\xB9\x8D\t\x08\x14a\x01\xF9W\x80c\xC0\xC5;\x8B\x14a\x01\xF4W\x80c\xCA\x8A\xA7\xC7\x14a\x01\xEFW\x80c\xCE\xFD\xC1\xD4\x14a\x01\xEAW\x80c\xDF\\\xF7#\x14a\x01\xE5W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xE0W\x80c\xF5\xC9\x89\x9D\x14a\x01\xDBW\x80c\xF6<[\xAB\x14a\x01\xD6Wc\xFA\xBC\x1C\xBC\x14a\x01\xD1W_\x80\xFD[a\x1E\x07V[a\x1D\xECV[a\x05iV[a\x1D[V[a\x1D\x17V[a\x1B\xD3V[a\x1B\x94V[a\x1A\x90V[a\x1AnV[a\x1AFV[a\x1A\x1EV[a\x19\x8FV[a\x19\xDAV[a\x19\xB2V[a\x194V[a\x18\x87V[a\x18\x07V[a\x16\x99V[a\x16(V[a\x15\xE4V[a\x15\xA0V[a\x15bV[a\x15EV[a\x13\xCCV[a\x10\xE2V[a\x0E3V[a\r\xB5V[a\r\x0EV[a\x0B\x03V[a\t!V[a\x08\xEFV[a\x08uV[a\x06\xCBV[a\x06#V[a\x05\xEAV[a\x05\xA9V[a\x05\x01V[4a\x03JW` 6`\x03\x19\x01\x12a\x03JW`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03EWa\x03\x14\x92a\x03\0\x91_\x91a\x03\x16W[Pa\x1F\tV[a\x03\x0F`fT\x82\x81\x16\x14a\x1F\x1FV[aFNV[\0[a\x038\x91P` =` \x11a\x03>W[a\x030\x81\x83a\x03\x9DV[\x81\x01\x90a\x1E\xE9V[_a\x02\xFAV[P=a\x03&V[a\x1E\xFEV[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03}W`@RV[a\x03NV[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03}W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03}W`@RV[`@Q\x90a\x03\xCEa\x01\0\x83a\x03\x9DV[V[`@Q\x90a\x03\xCE`@\x83a\x03\x9DV[`@Q\x90a\x03\xCE``\x83a\x03\x9DV[\x90a\x03\xCE`@Q\x92\x83a\x03\x9DV[`@\x90`\xE3\x19\x01\x12a\x03JW`@Q\x90a\x04\x15\x82a\x03bV[`\xE45\x82Ra\x01\x045` \x83\x01RV[\x91\x90\x82`@\x91\x03\x12a\x03JW`@Qa\x04=\x81a\x03bV[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03JW`@Q\x91a\x04h`@\x84a\x03\x9DV[\x82\x90`@\x81\x01\x92\x83\x11a\x03JW\x90[\x82\x82\x10a\x04\x84WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x04wV[\x90`\x80`c\x19\x83\x01\x12a\x03JW`@Qa\x04\xAD\x81a\x03bV[` a\x04\xC8\x82\x94a\x04\xBF\x81`da\x04MV[\x84R`\xA4a\x04MV[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x03JW` a\x04\xC8`@Q\x92a\x04\xEB\x84a\x03bV[`@\x84\x96a\x04\xF9\x83\x82a\x04MV[\x86R\x01a\x04MV[4a\x03JWa\x01 6`\x03\x19\x01\x12a\x03JW`\x045`@6`#\x19\x01\x12a\x03JWa\x05Y`@\x91\x82Qa\x053\x81a\x03bV[`$5\x81R`D5` \x82\x01Ra\x05I6a\x04\x94V[\x90a\x05S6a\x03\xFCV[\x92a\x1FsV[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`\xCDT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03JWV[5\x90a\x03\xCE\x82a\x05\xD1V[4a\x03JW` 6`\x03\x19\x01\x12a\x03JWc\xFF\xFF\xFF\xFF`\x045a\x06\x0C\x81a\x05\xD1V[\x16_R`\xCB` R` `@_ T`@Q\x90\x81R\xF3[4a\x03JW` 6`\x03\x19\x01\x12a\x03JWc\xFF\xFF\xFF\xFF`\x045a\x06E\x81a\x05\xD1V[\x16_R`\xCA` R` `@_ T`@Q\x90\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03JWV[`\x01`\x01`@\x1B\x03\x81\x11a\x03}W`\x05\x1B` \x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x06\xA1WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\x94V[\x90` a\x06\xC8\x92\x81\x81R\x01\x90a\x06\x84V[\x90V[4a\x03JW`@6`\x03\x19\x01\x12a\x03JW`\x045a\x06\xE8\x81a\x06\\V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03JW6`#\x83\x01\x12\x15a\x03JW\x81`\x04\x015\x91a\x07\x14\x83a\x06mV[\x92a\x07\"`@Q\x94\x85a\x03\x9DV[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x03JW`$\x01\x90[\x82\x82\x10a\x07cWa\x07_a\x07S\x86\x86a \xDDV[`@Q\x91\x82\x91\x82a\x06\xB7V[\x03\x90\xF3[` \x80\x91\x835a\x07r\x81a\x06\\V[\x81R\x01\x91\x01\x90a\x07?V[`\x01`\x01`@\x1B\x03\x81\x11a\x03}W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x07\xA4\x82a\x07}V[\x91a\x07\xB2`@Q\x93\x84a\x03\x9DV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03JW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x07\xF9WPPPPP\x90V[\x90\x91\x92\x93\x94`\x1F\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x089WPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x07\xEAV[\x90\x91\x92` ```\x01\x92`\x01`\x01``\x1B\x03`@\x88Q\x86\x80`\xA0\x1B\x03\x81Q\x16\x84R\x85\x81\x01Q\x86\x85\x01R\x01Q\x16`@\x82\x01R\x01\x94\x01\x92\x01\x90a\x08\x1AV[4a\x03JW``6`\x03\x19\x01\x12a\x03JW`\x045a\x08\x92\x81a\x06\\V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03JW6`#\x82\x01\x12\x15a\x03JWa\x07_\x91a\x08\xC9a\x08\xDB\x926\x90`$\x81`\x04\x015\x91\x01a\x07\x98V[`D5\x91a\x08\xD6\x83a\x05\xD1V[a#\x1BV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x07\xCEV[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`\xD1T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x03JWV[4a\x03JW` 6`\x03\x19\x01\x12a\x03JW`\x045a\t>\x81a\t\x17V[`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03EW_\x91a\n5W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\t\xA4Wa\x03\x14\x90aK\x07V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FBLSSignatureChecker.onlyCoordina`D\x82\x01R\x7FtorOwner: caller is not the owne`d\x82\x01R\x7Fr of the registryCoordinator\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[a\nW\x91P` =` \x11a\n]W[a\nO\x81\x83a\x03\x9DV[\x81\x01\x90a!\xA2V[_a\t\x8BV[P=a\nEV[\x90\x80`\x1F\x83\x01\x12\x15a\x03JW\x815a\n{\x81a\x06mV[\x92a\n\x89`@Q\x94\x85a\x03\x9DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03JW` \x01\x90[\x82\x82\x10a\n\xB1WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\n\xA4V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\n\xE4WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\n\xD7V[4a\x03JW`@6`\x03\x19\x01\x12a\x03JW`\x045a\x0B \x81a\x06\\V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x0B?\x906\x90`\x04\x01a\ndV[a\x0BI\x81Qa {V[\x91`\x01`\x01`\xA0\x1B\x03\x16_[\x82Q\x81\x10\x15a\x0B\xE6W\x80` a\x0Bna\x0B\x8E\x93\x86a \xBAV[Q`@Q\x80\x94\x81\x92c\nZ\xEC\x19`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x91\x82\x15a\x03EW`\x01\x92a\x0B\xC2\x91_\x91a\x0B\xC8W[Pa\x0B\xB3\x83\x88a \xBAV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\x0BUV[a\x0B\xE0\x91P` =\x81\x11a\n]Wa\nO\x81\x83a\x03\x9DV[_a\x0B\xA8V[`@Q\x80a\x07_\x86\x82a\n\xC1V[\x91\x81`\x1F\x84\x01\x12\x15a\x03JW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03JW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03JWV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0C>WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0C1V[\x90` \x82R``a\x0C\xA8a\x0C\x93a\x0C}\x84Q`\x80` \x88\x01R`\xA0\x87\x01\x90a\x0C!V[` \x85\x01Q\x86\x82\x03`\x1F\x19\x01`@\x88\x01Ra\x0C!V[`@\x84\x01Q\x85\x82\x03`\x1F\x19\x01\x84\x87\x01Ra\x0C!V[\x91\x01Q\x91`\x80`\x1F\x19\x82\x84\x03\x01\x91\x01R\x81Q\x80\x82R` \x82\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x0C\xE1WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a\x0C\xFF`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\x0C!V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x0C\xD2V[4a\x03JW`\x806`\x03\x19\x01\x12a\x03JW`\x045a\r+\x81a\x06\\V[`$5\x90a\r8\x82a\x05\xD1V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\rW\x906\x90`\x04\x01a\x0B\xF4V[\x91`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03JW6`#\x85\x01\x12\x15a\x03JW\x83`\x04\x015\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03JW6`$\x85`\x05\x1B\x87\x01\x01\x11a\x03JWa\x07_\x95`$a\r\xA9\x96\x01\x93a(\xB1V[`@Q\x91\x82\x91\x82a\x0CZV[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03EWa\x0E \x91_\x91a\x03\x16WPa\x1F\tV[a\x03\x14aF\x1AV[`\xFF\x81\x16\x03a\x03JWV[4a\x03JW` 6`\x03\x19\x01\x12a\x03JW` `\x01`\xFF`\x045a\x0EV\x81a\x0E(V[\x16\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[\x90\x81`\x80\x91\x03\x12a\x03JW\x90V[`@\x90`#\x19\x01\x12a\x03JW`$\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x03JW\x815a\x0E\x9C\x81a\x06mV[\x92a\x0E\xAA`@Q\x94\x85a\x03\x9DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03JW` \x01\x90[\x82\x82\x10a\x0E\xD2WPPP\x90V[` \x80\x91\x835a\x0E\xE1\x81a\x05\xD1V[\x81R\x01\x91\x01\x90a\x0E\xC5V[\x81`\x1F\x82\x01\x12\x15a\x03JW\x805a\x0F\x02\x81a\x06mV[\x92a\x0F\x10`@Q\x94\x85a\x03\x9DV[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x03JW` \x01\x91[\x83\x83\x10a\x0F:WPPPP\x90V[` `@\x91a\x0FI\x84\x86a\x04%V[\x81R\x01\x92\x01\x91a\x0F,V[\x90\x80`\x1F\x83\x01\x12\x15a\x03JW\x815a\x0Fk\x81a\x06mV[\x92a\x0Fy`@Q\x94\x85a\x03\x9DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x03JW` \x82\x01\x90[\x83\x82\x10a\x0F\xA5WPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03JW` \x91a\x0F\xC7\x87\x84\x80\x94\x88\x01\x01a\x0E\x85V[\x81R\x01\x91\x01\x90a\x0F\x96V[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x03JWa\x0F\xE8a\x03\xBEV[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03JW\x81a\x10\x05\x91\x84\x01a\x0E\x85V[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03JW\x81a\x10&\x91\x84\x01a\x0E\xECV[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03JW\x81a\x10J\x91\x84\x01a\x0E\xECV[`@\x85\x01Ra\x10\\\x81``\x84\x01a\x04\xCDV[``\x85\x01Ra\x10n\x81`\xE0\x84\x01a\x04%V[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03JW\x81a\x10\x93\x91\x84\x01a\x0E\x85V[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03JW\x81a\x10\xB8\x91\x84\x01a\x0E\x85V[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x10\xDB\x92\x01a\x0FTV[`\xE0\x83\x01RV[4a\x03JW`\x806`\x03\x19\x01\x12a\x03JW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x11\x12\x906\x90`\x04\x01a\x0EfV[a\x11\x1B6a\x0EtV[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x11;\x906\x90`\x04\x01a\x0F\xD2V[`\xCDT\x90\x92\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x13NWa\x11^` \x83\x94\x93\x01a-\x19V[\x91a\x12aa\x11o`@\x86\x01\x86a-#V[\x92\x90\x94a\x11\xCFa\x11\x81``\x89\x01a-\x19V[\x97`@Qa\x11\xA5\x81a\x11\x97` \x82\x01\x94\x85a-UV[\x03`\x1F\x19\x81\x01\x83R\x82a\x03\x9DV[Q\x90 a\x11\xC8a\x11\xB4\x88a-\x19V[c\xFF\xFF\xFF\xFF\x16_R`\xCA` R`@_ \x90V[T\x14a-\xDCV[a\x11\xF9a\x11\xF2a\x11\xDE\x87a-\x19V[c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15a.NV[\x83c\xFF\xFF\xFF\xFFC\x16\x96a\x12Ca\x12;a\x122\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a.\xDFV[c\xFF\xFF\xFF\xFF\x16\x90V[\x89\x11\x15a.\xF9V[`@Q` \x81\x01\x90a\x12Y\x81a\x11\x97\x8B\x85a/yV[Q\x90 a<\xECV[\x91\x90`\xFF_\x96\x16\x95[\x82\x81\x10a\x12\xECW\x7F4\x9C\x1E\xE6\x0EN\x89r\xEE\x9D\xBAd,\x17tT=\\A6\x87\x9B\x7FL\xAA\xF0K\xF8\x1AHz*\x86\x86\x86a\x12\xACa\x12\xA0a\x03\xD0V[c\xFF\xFF\xFF\xFF\x90\x94\x16\x84RV[` \x83\x01R`@Q` \x81\x01\x90a\x12\xC8\x81a\x11\x97\x86\x86\x86a0\\V[Q\x90 a\x12\xD7a\x11\xDE\x83a-\x19V[Ua\x12\xE7`@Q\x92\x83\x92\x83a0\\V[\x03\x90\xA1\0[\x80a\x13Ha\x13$a\x13\x1Fa\x13\x13a\x13\x06`\x01\x96\x88Qa \xBAV[Q`\x01`\x01``\x1B\x03\x16\x90V[`\x01`\x01``\x1B\x03\x16\x90V[a/\x89V[a\x13Aa\x13\x13\x8Ba\x13<a\x13\x06\x87` \x8B\x01Qa \xBAV[a/\xC8V[\x11\x15a/\xEBV[\x01a\x12jV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAggregator must be the caller\0\0\0`D\x82\x01R`d\x90\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x13\xB6WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x13\xA9V[4a\x03JW``6`\x03\x19\x01\x12a\x03JW`\x045a\x13\xE9\x81a\x06\\V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x14\x08\x906\x90`\x04\x01a\ndV[`D5\x91a\x14\x15\x83a\x05\xD1V[`@Qca\xC8\xA1/`\xE1\x1B\x81R\x90`\x01`\x01`\xA0\x1B\x03\x16_\x82\x80a\x14=\x86\x88`\x04\x84\x01a0\x86V[\x03\x81\x84Z\xFA\x91\x82\x15a\x03EW_\x92a\x15!W[Pa\x14[\x83Qa {V[\x93_[\x84Q\x81\x10\x15a\x15\x13Wa\x14q\x81\x86a \xBAV[Q\x90` \x83a\x14\x8Da\x14\x83\x84\x89a \xBAV[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x95\x90\x95Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81\x87Z\xFA\x80\x15a\x03EW`\x01\x92_\x91a\x14\xE5W[P\x82\x80`\xC0\x1B\x03\x16a\x14\xDE\x82\x89a \xBAV[R\x01a\x14^V[a\x15\x06\x91P` =\x81\x11a\x15\x0CW[a\x14\xFE\x81\x83a\x03\x9DV[\x81\x01\x90a'\xA6V[_a\x14\xCCV[P=a\x14\xF4V[`@Q\x80a\x07_\x88\x82a\x13\x93V[a\x15>\x91\x92P=\x80_\x83>a\x156\x81\x83a\x03\x9DV[\x81\x01\x90a&uV[\x90_a\x14PV[4a\x03JW_6`\x03\x19\x01\x12a\x03JW` `fT`@Q\x90\x81R\xF3[4a\x03JW` 6`\x03\x19\x01\x12a\x03JWc\xFF\xFF\xFF\xFF`\x045a\x15\x84\x81a\x05\xD1V[\x16_R`\xCC` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03JW`\xC06`\x03\x19\x01\x12a\x03JW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x16X\x906\x90`\x04\x01a\x0EfV[a\x16a6a\x0EtV[\x90`@6`c\x19\x01\x12a\x03JW`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x03\x14\x92a\x16\x93`d\x926\x90`\x04\x01a\x0E\xECV[\x92a3tV[4a\x03JW``6`\x03\x19\x01\x12a\x03JW`$5`\x045a\x16\xB9\x82a\x05\xD1V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x16\xD8\x906\x90`\x04\x01a\x0B\xF4V[`\xCET\x91\x93\x90\x92\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a\x17\xB8Wa\x03\x14\x93a\x17\xA2\x93a\x17\"a\x17)\x93a\x17\x06a7\xC5V[\x95\x86RCc\xFF\xFF\xFF\xFF\x16` \x87\x01Rc\xFF\xFF\xFF\xFF\x16``\x86\x01RV[6\x91a\x07\x98V[`@\x82\x01R`@Q` \x81\x01\x90a\x17D\x81a\x11\x97\x85\x85a7\xE9V[Q\x90 a\x17Ya\x11\xB4`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90V[U`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90\x7F\x16\x95\xB8\xD0n\xC8\0\xB4a^t\\\xFB[\xD0\x0C\x1F(ua]B\x92\\;Z\xFAT;\xB2LH`@Q\x80a\x17\x9Ac\xFF\xFF\xFF\xFF\x86\x16\x94\x82a7\xE9V[\x03\x90\xA2a.\xAFV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19`\xC9T\x16\x17`\xC9UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask generator must be the calle`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x18hWPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x18[V[4a\x03JW`\x806`\x03\x19\x01\x12a\x03JW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03JWa\x18\xBA\x906\x90`\x04\x01a\x0B\xF4V[\x90\x91`D5a\x18\xC8\x81a\x05\xD1V[`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03JWa\x19*\x94a\x18\xEFa\x18\xF5\x956\x90`\x04\x01a\x0F\xD2V[\x93a<\xECV[`@Q\x92\x83\x92`@\x84R` a\x19\x16\x82Q`@\x80\x88\x01R`\x80\x87\x01\x90a\x18KV[\x91\x01Q\x84\x82\x03`?\x19\x01``\x86\x01Ra\x18KV[\x90` \x83\x01R\x03\x90\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JWa\x19LaMhV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03JW_6`\x03\x19\x01\x12a\x03JW` c\xFF\xFF\xFF\xFF`\xC9T\x16`@Q\x90\x81R\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`\xCET`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`\xCFT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03JW_6`\x03\x19\x01\x12a\x03JW` `\xFF`\x97T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03JW``6`\x03\x19\x01\x12a\x03JW`\x045a\x1A\xAD\x81a\x06\\V[a\x1B\x08`$5a\x1A\xBC\x81a\x06\\V[`D5\x90a\x1A\xC9\x82a\x06\\V[_T\x93a\x1A\xEE`\xFF`\x08\x87\x90\x1C\x16\x15\x80\x96\x81\x97a\x1B\x86W[\x81\x15a\x1BfW[PaEvV[\x84a\x1A\xFF`\x01`\xFF\x19_T\x16\x17_UV[a\x1BOWaE\xD9V[a\x1B\x0EW\0[a\x1B\x1Ca\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\x12\xE7V[a\x1Baa\x01\0a\xFF\0\x19_T\x16\x17_UV[aE\xD9V[0;\x15\x91P\x81a\x1BxW[P_a\x1A\xE8V[`\xFF\x16`\x01\x14\x90P_a\x1BqV[`\x01`\xFF\x82\x16\x10\x91Pa\x1A\xE1V[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`\xD0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[`@\x90a\x06\xC8\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x07\xCEV[4a\x03JW``6`\x03\x19\x01\x12a\x03JW`\x045a\x1B\xF0\x81a\x06\\V[`$5`D5a\x1B\xFF\x81a\x05\xD1V[a\x1C@a\x1C\na YV[\x92\x80a\x1C\x15\x85a \xADV[R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x94\x90\x92_\x91\x84\x91\x82\x91\x87`\x04\x84\x01a0\x86V[\x03\x81\x87Z\xFA\x93\x84\x15a\x03EW\x83a\x1Cja\x122a\x14\x83a\x1C\x9F\x98` \x97_\x91a\x1C\xFDW[Pa \xADV[\x92`@Q\x96\x87\x94\x85\x93\x84\x93c\x04\xECcQ`\xE0\x1B\x85R`\x04\x85\x01c\xFF\xFF\xFF\xFF`@\x92\x95\x94\x93``\x83\x01\x96\x83R\x16` \x82\x01R\x01RV[\x03\x91Z\xFA\x80\x15a\x03EWa\x1C\xCE\x92_\x91a\x1C\xDEW[P`\x01`\x01`\xC0\x1B\x03\x16\x92a\x1C\xC8\x84aN\x08V[\x90a#\x1BV[\x90a\x07_`@Q\x92\x83\x92\x83a\x1B\xBCV[a\x1C\xF7\x91P` =` \x11a\x15\x0CWa\x14\xFE\x81\x83a\x03\x9DV[_a\x1C\xB4V[a\x1D\x11\x91P=\x80_\x83>a\x156\x81\x83a\x03\x9DV[_a\x1CdV[4a\x03JW_6`\x03\x19\x01\x12a\x03JW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03JW` 6`\x03\x19\x01\x12a\x03JW`\x045a\x1Dx\x81a\x06\\V[a\x1D\x80aMhV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x1D\x98Wa\x03\x14\x90aM\xC0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03JW_6`\x03\x19\x01\x12a\x03JW` `@Q`d\x81R\xF3[4a\x03JW` 6`\x03\x19\x01\x12a\x03JW`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03EW_\x91a\x1E\xCAW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1E\xBBWa\x1E\x89`fT\x19\x82\x19\x81\x16\x14a\x1F\x1FV[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a\x1E\xE3\x91P` =` \x11a\n]Wa\nO\x81\x83a\x03\x9DV[_a\x1EhV[\x90\x81` \x91\x03\x12a\x03JWQa\x06\xC8\x81a\t\x17V[`@Q=_\x82>=\x90\xFD[\x15a\x1F\x10WV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a\x1F&WV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a\x1FZW`\x05\x1B\x01\x90V[a\x1F5V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a Oa ,a U\x95a &a \x1F\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x1F\xF6\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x03\x9DV[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96aF\xC4V[\x90aG\nV[\x92a &a Aa ;aG\x92V[\x94aH\x89V[\x91a JaI\xA5V[aF\xC4V[\x91aI\xD9V[\x90\x91V[`@\x80Q\x90\x91\x90a j\x83\x82a\x03\x9DV[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x90a \x85\x82a\x06mV[a \x92`@Q\x91\x82a\x03\x9DV[\x82\x81R\x80\x92a \xA3`\x1F\x19\x91a\x06mV[\x01\x90` 6\x91\x017V[\x80Q\x15a\x1FZW` \x01\x90V[\x80Q\x82\x10\x15a\x1FZW` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x03JWQ\x90V[\x91\x90\x91a \xEA\x83Qa {V[\x92_[\x81Q\x81\x10\x15a!\x9DW\x80` a!\x16a!\ta!?\x94\x86a \xBAV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x92\x83\x90\x81\x90`$\x82\x01\x90V[\x03\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x80\x15a\x03EW`\x01\x92_\x91a!oW[Pa!h\x82\x88a \xBAV[R\x01a \xEDV[a!\x90\x91P` =\x81\x11a!\x96W[a!\x88\x81\x83a\x03\x9DV[\x81\x01\x90a \xCEV[_a!]V[P=a!~V[PPPV[\x90\x81` \x91\x03\x12a\x03JWQa\x06\xC8\x81a\x06\\V[\x90a!\xC1\x82a\x06mV[a!\xCE`@Q\x91\x82a\x03\x9DV[\x82\x81R` \x81\x93a!\xE1`\x1F\x19\x91a\x06mV[\x01\x91\x01_[\x82\x81\x10a!\xF2WPPPV[``\x82\x82\x01R` \x01a!\xE6V[\x90\x81Q\x81\x10\x15a\x1FZW\x01` \x01\x90V[` \x81\x83\x03\x12a\x03JW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03JW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03JW\x81Qa\"D\x81a\x06mV[\x92a\"R`@Q\x94\x85a\x03\x9DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03JW` \x01\x90[\x82\x82\x10a\"zWPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a\"mV[\x90a\"\x94\x82a\x06mV[a\"\xA1`@Q\x91\x82a\x03\x9DV[\x82\x81R\x80\x92a\"\xB2`\x1F\x19\x91a\x06mV[\x01_[\x81\x81\x10a\"\xC1WPPPV[`@Q\x90``\x82\x01\x91\x80\x83\x10`\x01`\x01`@\x1B\x03\x84\x11\x17a\x03}W` \x92`@R_\x81R_\x83\x82\x01R_`@\x82\x01R\x82\x82\x86\x01\x01R\x01a\"\xB5V[\x90\x81` \x91\x03\x12a\x03JWQ`\x01`\x01``\x1B\x03\x81\x16\x81\x03a\x03JW\x90V[`@Qch0H5`\xE0\x1B\x81R\x93\x91\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16` \x85`\x04\x81\x84Z\xFA\x94\x85\x15a\x03EW_\x95a&0W[P`@QcOL\x91\xE1`\xE1\x1B\x81R\x94` \x86`\x04\x81\x85Z\xFA\x91\x82\x15a\x03EW`\x04\x96_\x93a&\x0EW[P` \x90`@Q\x97\x88\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x95\x86\x15a\x03EW_\x96a%\xEDW[Pa#\xA9\x85\x93\x92\x95Qa!\xB7V[\x94_\x93[\x80Q\x85\x10\x15a%\xE3Wa#\xDAa#\xD4a#\xC6\x87\x84a\"\0V[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`@Qc\x89\x02bE`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x88\x16`$\x82\x01R\x90\x94\x90\x92_\x84`D\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x93\x84\x15a\x03EW_\x94a%\xBFW[Pa$*\x84Qa\"\x8AV[a$4\x88\x8Ba \xBAV[Ra$?\x87\x8Aa \xBAV[P_[\x84Q\x81\x10\x15a%\xAEW\x80` a$[a$}\x93\x88a \xBAV[Q\x8D`@Q\x80\x80\x96\x81\x94c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03EW_\x92a%\x8EW[Pa$\xA3\x81\x87a \xBAV[Q\x8A` \x8Aa$\xB2\x85\x8Ba \xBAV[Q`@Qc\xFA(\xC6'`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x91\x90\x91\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x8D\x16Z\xFA\x93\x84\x15a\x03EWa%E\x8C\x8Fa%@`\x01\x98a%W\x97\x89\x97_\x92a%^W[Pa%+a%\x1Ca\x03\xDFV[`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88RV[` \x87\x01R`\x01`\x01``\x1B\x03\x16`@\x86\x01RV[a \xBAV[Q\x90a%Q\x83\x83a \xBAV[Ra \xBAV[P\x01a$BV[a%\x80\x91\x92P` =\x81\x11a%\x87W[a%x\x81\x83a\x03\x9DV[\x81\x01\x90a\"\xFCV[\x90_a%\x10V[P=a%nV[a%\xA7\x91\x92P` =\x81\x11a\n]Wa\nO\x81\x83a\x03\x9DV[\x90_a$\x98V[P`\x01\x90\x96\x01\x95\x90\x94P\x91Pa#\xADV[a%\xDC\x91\x94P=\x80_\x83>a%\xD4\x81\x83a\x03\x9DV[\x81\x01\x90a\"\x11V[\x92_a$\x1FV[PPP\x93PPP\x90V[a&\x07\x91\x96P` =` \x11a\n]Wa\nO\x81\x83a\x03\x9DV[\x94_a#\x9BV[` \x91\x93Pa&)\x90\x82=\x84\x11a\n]Wa\nO\x81\x83a\x03\x9DV[\x92\x90a#vV[a&J\x91\x95P` =` \x11a\n]Wa\nO\x81\x83a\x03\x9DV[\x93_a#MV[`@Q\x90a&^\x82a\x03\x82V[``\x80\x83\x81\x81R\x81` \x82\x01R\x81`@\x82\x01R\x01RV[` \x81\x83\x03\x12a\x03JW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03JW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03JW\x81Qa&\xA8\x81a\x06mV[\x92a&\xB6`@Q\x94\x85a\x03\x9DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03JW` \x01\x90[\x82\x82\x10a&\xDEWPPP\x90V[` \x80\x91\x83Qa&\xED\x81a\x05\xD1V[\x81R\x01\x91\x01\x90a&\xD1V[c\xFF\xFF\xFF\xFF\x90\x91\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x83\x90R`\x01`\x01`\xFB\x1B\x03\x83\x11a\x03JW``\x92`\x05\x1B\x80\x92\x84\x83\x017\x01\x01\x90V[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@\x90c\xFF\xFF\xFF\xFFa\x06\xC8\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a'/V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x16`\xFF\x81\x14a'\x91W`\x01\x01\x90V[a'lV[\x91\x90\x81\x10\x15a\x1FZW`\x05\x1B\x01\x90V[\x90\x81` \x91\x03\x12a\x03JWQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x03JW\x90V[\x15a'\xCCWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FOperatorStateRetriever.getCheckS`D\x82\x01R\x7FignaturesIndices: operator must `d\x82\x01R\x7Fbe registered at blocknumber\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[\x90\x82\x10\x15a\x1FZW\x01\x90V[\x90\x81` \x91\x03\x12a\x03JWQa\x06\xC8\x81a\x05\xD1V[_\x19\x81\x14a'\x91W`\x01\x01\x90V[\x91a(\xAA` \x92c\xFF\xFF\xFF\xFF\x92\x96\x95\x96`@\x86R`@\x86\x01\x91a'/V[\x94\x16\x91\x01RV[\x95\x93\x94\x95\x92\x90\x91\x92a(\xC1a&QV[P`@Qch0H5`\xE0\x1B\x81R\x93`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91\x90` \x85`\x04\x81\x86Z\xFA\x94\x85\x15a\x03EW_\x95a,\xF8W[Pa(\xFEa&QV[\x94`@Qca\xC8\xA1/`\xE1\x1B\x81R_\x81\x80a)\x1E\x8D\x8D\x8B`\x04\x85\x01a&\xF8V[\x03\x81\x88Z\xFA\x90\x81\x15a\x03EW_\x91a,\xDEW[P\x86R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x92\x90_\x81\x80a)a\x85\x87\x8B`\x04\x85\x01a'OV[\x03\x81\x87Z\xFA\x90\x81\x15a\x03EW_\x91a,\xC4W[P`@\x87\x01Ra)\x83\x81a!\xB7V[\x98``\x87\x01\x99\x8AR_[`\xFF\x81\x16\x83\x81\x10\x15a,\x0FW\x88_a)\xB6\x83\x8Fa)\xA9\x88a {V[\x90Q\x90a%Q\x83\x83a \xBAV[P_\x8A\x86\x8F[\x81\x84\x10a*9WPPPP\x90P\x8Ca)\xD3\x82a {V[\x91_[\x81\x81\x10a*\0WPP\x91a)\xF5\x91a)\xFB\x94\x93Q\x90a%Q\x83\x83a \xBAV[Pa'\x80V[a)\x8DV[\x80a*3a*\x1Ea\x14\x83`\x01\x94a*\x18\x8A\x89Qa \xBAV[Qa \xBAV[a*(\x83\x88a \xBAV[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[\x01a)\xD6V[a\x14\x83\x84a*N\x81` \x96\x95a*V\x95a'\x96V[5\x97Qa \xBAV[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x87\x01R\x16`D\x85\x01R\x83`d\x81\x8DZ\xFA\x80\x15a\x03EW\x88\x8F\x88\x8A\x91\x8F\x94a*\xFB`\x01a*\xEE\x81\x93\x8D\x80\x9D_\x92a+\xE3W[Pa#\xD4a*\xCAa*\xD8\x92a*\xC3\x87\x80`\xC0\x1B\x03\x86\x16\x15\x15a'\xC5V[\x8B\x8Da(]V[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\x01`\x01`\xC0\x1B\x03\x91\x82\x16`\xFF\x91\x90\x91\x16\x1C\x16\x90V[\x16`\x01`\x01`\xC0\x1B\x03\x16\x90V[\x14a+\x17W[PPPPP`\x01\x91\x92P\x01\x90\x8A\x91\x8A\x86\x8Fa)\xBCV[\x85\x97a+9\x93a+2` \x97\x99\x98a#\xD4\x95a*\xCA\x95a'\x96V[5\x95a(]V[`@Qc\xDD\x98F\xB9`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\xFF\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`D\x84\x01R\x82`d\x81\x8CZ\xFA\x90\x81\x15a\x03EW\x8Fa+\x97\x90a+\x9C\x93\x83\x88`\x01\x97_\x93a+\xABW[Pa*\x18\x90a*(\x93\x94Qa \xBAV[a(~V[\x90P\x82\x91\x8A\x88\x8F\x88\x8A\x91a+\x01V[a*(\x93P\x90a+\xD4a*\x18\x92` =\x81\x11a+\xDCW[a+\xCC\x81\x83a\x03\x9DV[\x81\x01\x90a(iV[\x93P\x90a+\x87V[P=a+\xC2V[a*\xD8\x91\x92Pa*\xCAa,\x06a#\xD4\x92` =\x81\x11a\x15\x0CWa\x14\xFE\x81\x83a\x03\x9DV[\x93\x92PPa*\xA6V[PPP\x92\x90\x95\x97P`\x04\x94\x96P` \x91P`@Q\x94\x85\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x90\x81\x15a\x03EWa,e\x94_\x94\x85\x93a,\xA3W[P`@Qc5IR\xA3`\xE2\x1B\x81R\x95\x86\x94\x85\x93\x84\x93`\x04\x85\x01a(\x8CV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03EW_\x91a,\x89W[P` \x82\x01R\x90V[a,\x9D\x91P=\x80_\x83>a\x156\x81\x83a\x03\x9DV[_a,\x80V[a,\xBD\x91\x93P` =` \x11a\n]Wa\nO\x81\x83a\x03\x9DV[\x91_a,GV[a,\xD8\x91P=\x80_\x83>a\x156\x81\x83a\x03\x9DV[_a)tV[a,\xF2\x91P=\x80_\x83>a\x156\x81\x83a\x03\x9DV[_a)1V[a-\x12\x91\x95P` =` \x11a\n]Wa\nO\x81\x83a\x03\x9DV[\x93_a(\xF5V[5a\x06\xC8\x81a\x05\xD1V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03JW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03JW` \x01\x91\x816\x03\x83\x13a\x03JWV[` \x81R\x815` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x015a-s\x81a\x05\xD1V[\x16`@\x82\x01R`@\x82\x015`\x1E\x19\x836\x03\x01\x81\x12\x15a\x03JW\x82\x01\x90` \x825\x92\x01`\x01`\x01`@\x1B\x03\x83\x11a\x03JW\x826\x03\x81\x13a\x03JWa-\xD1``a-\xCA`\x80\x93a\x06\xC8\x96\x85\x84\x88\x01R`\xA0\x87\x01\x91a'/V[\x95\x01a\x05\xDFV[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x15a-\xE3WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a.UWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAggregator has already responded`D\x82\x01Rk to the task`\xA0\x1B`d\x82\x01R`\x84\x90\xFD[c\xFF\xFF\xFF\xFF`\x01\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a'\x91WV[c\xFF\xFF\xFF\xFF`d\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a'\x91WV[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a'\x91WV[\x15a/\0WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAggregator has responded to the `D\x82\x01Rltask too late`\x98\x1B`d\x82\x01R`\x84\x90\xFD[` \x80\x91c\xFF\xFF\xFF\xFF\x815a/o\x81a\x05\xD1V[\x16\x84R\x015\x91\x01RV[`@\x81\x01\x92\x91a\x03\xCE\x91\x90a/[V[\x90`d\x82\x02\x91\x80\x83\x04`d\x14\x90\x15\x17\x15a'\x91WV[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a'\x91WV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a'\x91WV[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x02\x90`\x01`\x01``\x1B\x03\x82\x16\x91\x82\x03a'\x91WV[\x15a/\xF2WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FSignatories do not own at least `D\x82\x01R\x7Fthreshold percentage of a quorum`d\x82\x01R\xFD[\x90\x92\x91` ``\x91a0r\x84`\x80\x81\x01\x97a/[V[c\xFF\xFF\xFF\xFF\x81Q\x16`@\x85\x01R\x01Q\x91\x01RV[`@\x90c\xFF\xFF\xFF\xFFa\x06\xC8\x94\x93\x16\x81R\x81` \x82\x01R\x01\x90a\x06\x84V[\x15a0\xAAWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask hasn't been responded to ye`D\x82\x01R`\x1D`\xFA\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x92\x91` ``\x91a1\x0F\x84`\x80\x81\x01\x97a/[V[c\xFF\xFF\xFF\xFF\x815a1\x1F\x81a\x05\xD1V[\x16`@\x85\x01R\x015\x91\x01RV[\x15a13WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FTask response does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a1\xA5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FThe response to this task has al`D\x82\x01R\x7Fready been challenged successful`d\x82\x01Rb6<\x97`\xE9\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a2#WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FThe challenge period for this ta`D\x82\x01R\x7Fsk has already expired.\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a2\xB8WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a2\xABV[\x15a2\xD5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FThe pubkeys of non-signing opera`D\x82\x01R\x7Ftors supplied by the challenger `d\x82\x01Ro0\xB92\x9077\xBA\x101\xB7\xB992\xB1\xBA\x17`\x81\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a3`WV[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[\x93\x92\x91\x90\x93`\x01a3\x84\x86a-\x19V[\x95` a4@\x845a3\xAFa3\xA7\x8Bc\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15\x15a0\xA3V[a3\xEAa3\xCA\x8Bc\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T`@Q\x85\x81\x01\x90a3\xE1\x81a\x11\x97\x8D\x8B\x86a0\xF9V[Q\x90 \x14a1,V[a4\x15a4\x0Fa4\x08\x8Cc\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[T`\xFF\x16\x90V[\x15a1\x9EV[a4:a4,a\x122a4'\x8Aa-\x19V[a.\xC7V[c\xFF\xFF\xFF\xFFC\x16\x11\x15a2\x1CV[\x80a/\xB5V[\x91\x015\x14\x14a7\x93Wa4S\x83Qa {V[\x93_[\x84Q\x81\x10\x15a4\x93W\x80a4\x82a4o`\x01\x93\x88a \xBAV[Q\x80Q_R` \x01Q` R`@_ \x90V[a4\x8C\x82\x89a \xBAV[R\x01a4VV[P\x90\x92\x93\x91\x94a4\xCD` \x85\x01\x96` a4\xAC\x89a-\x19V[`@Qa4\xC1\x81a\x11\x97\x8A\x86\x83\x01\x95\x86a2\x8EV[Q\x90 \x91\x015\x14a2\xCEV[a4\xD7\x85Qa {V[\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x94_[\x87Q\x81\x10\x15a5\x87W\x80` a5\x1Ea5>\x93\x89a \xBAV[Q`@Q\x80\x94\x81\x92ct]\xCDs`\xE1\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8BZ\xFA\x91\x82\x15a\x03EW`\x01\x92a5c\x91_\x91a5iW[Pa\x0B\xB3\x83\x8Da \xBAV[\x01a5\x05V[a5\x81\x91P` =\x81\x11a\n]Wa\nO\x81\x83a\x03\x9DV[_a5XV[Pa5\xE7\x93\x95\x97\x96Pa5\xB0\x92\x94P\x90a5\xA8a5\xB8\x92`@\x81\x01\x90a-#V[\x93\x90\x91a-\x19V[\x926\x91a\x07\x98V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a#\x1BV[`\xD1T`\xD0T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16s\x8Ay\x16 \xDDb`\x07\x9B\xF8I\xDCUg\xAD\xC3\xF2\xFD\xC3\x18\x14\x93\x91\x16\x91_\x91[\x81Q\x83\x10\x15a75W_\x95[a6+\x84\x84a \xBAV[QQ\x87\x10\x15a7(W\x90\x82\x91` \x80a6L\x8Aa*\x18\x89a6n\x9A\x99a \xBAV[Q\x01Q`@Q\x80\x97\x81\x92c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8DZ\xFA\x94\x85\x15a\x03EW_\x95a7\x08W[P_[\x89Q\x81\x10\x15a6\xFAWa6\xA9a6\x9Da!\t\x83\x8Da \xBAV[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a6\xC0W`\x01\x01a6\x84V[P\x96`\x01\x91\x92\x93\x94P[a6\xEA` a6\xD7a\x03\xD0V[`\x01`\x01`\xA0\x1B\x03\x89\x16\x81R\x01`\x01\x90RV[a6\xF3\x87a3YV[\x01\x95a6!V[P\x96`\x01\x91\x92\x93\x94Pa6\xCAV[a7!\x91\x95P` =\x81\x11a\n]Wa\nO\x81\x83a\x03\x9DV[\x93_a6\x81V[\x95P`\x01\x90\x92\x01\x91a6\x15V[\x95PPPPP\x91Pa7ea7X\x82c\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[c\xFF\xFF\xFF\xFF3\x91\x16\x7F\xC2\r\x1B\xB0\xF1b6\x800k\x83\xD4\xFFK\xB9\x9A+\xEB\x9D\x86\xD9x2\xF3\xCA@\xFD\x13\xA2\x9D\xF1\xEC_\x80\xA3V[PPP\x90c\xFF\xFF\xFF\xFF3\x91\x16\x7F\xFD>&\xBE\xEBYg\xFCZW\xA0Di\x14\xEA\xBCE\xB4\xAAGLg\xA5\x1BKQ`\xCA\xC6\r\xDB\x05_\x80\xA3V[`@Q\x90a7\xD2\x82a\x03\x82V[_``\x83\x82\x81R\x82` \x82\x01R\x81`@\x82\x01R\x01RV[` c\xFF\xFF\xFF\xFF```\xC0\x94\x83\x85R\x80Q\x84\x86\x01R\x82\x84\x82\x01Q\x16`@\x86\x01R`@\x81\x01Q`\x80\x83\x87\x01R\x80Q\x94\x85\x91\x82`\xA0\x89\x01R\x01\x87\x87\x01^_\x85\x85\x01\x87\x01R\x01Q\x16`\x80\x83\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@Q\x90a8L\x82a\x03bV[``` \x83\x82\x81R\x01RV[\x15a8_WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: empty quorum input\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a8\xBEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: input quorum length mismatc`d\x82\x01R`\r`\xFB\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a9'WV[`\xA4`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: input nonsigner length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R\xFD[\x15a9\x92WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: invalid reference block\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x90\x81` \x91\x03\x12a\x03JWQa\x06\xC8\x81a\x0E(V[_\x19\x81\x01\x91\x90\x82\x11a'\x91WV[\x15a:\x14WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R\xFD[\x90`\x01\x82\x01\x80\x92\x11a'\x91WV[\x90`\x02\x82\x01\x80\x92\x11a'\x91WV[\x90`\x03\x82\x01\x80\x92\x11a'\x91WV[\x90`\x04\x82\x01\x80\x92\x11a'\x91WV[\x90`\x05\x82\x01\x80\x92\x11a'\x91WV[\x91\x90\x82\x01\x80\x92\x11a'\x91WV[\x15a:\xC5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: StakeRegistry updates must `d\x82\x01R\x7Fbe within withdrawalDelayBlocks `\x84\x82\x01Rewindow`\xD0\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x90\x81` \x91\x03\x12a\x03JWQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x03JW\x90V[\x15a;zWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: quorumApk hash in storage d`d\x82\x01R\x7Foes not match provided quorum ap`\x84\x82\x01R`k`\xF8\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a'\x91WV[\x15a<)WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a<\x94WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R_Q` aP|_9_Q\x90_R`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x94\x93\x92\x90\x91\x93a<\xFAa8?V[Pa=\x06\x85\x15\x15a8XV[`@\x84\x01QQ\x85\x14\x80aEhW[\x80aEZW[\x80aELW[a=)\x90a8\xB7V[a=;` \x85\x01QQ\x85QQ\x14a9 V[a=Rc\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a9\x8BV[a=Za\x03\xD0V[_\x81R_` \x82\x01R\x92a=la8?V[a=u\x87a {V[` \x82\x01Ra=\x83\x87a {V[\x81Ra=\x8Da8?V[\x92a=\x9C` \x88\x01QQa {V[\x84Ra=\xAC` \x88\x01QQa {V[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03EWa>\x15\x91_\x91aE\x1DW[Pa>\x106\x8B\x87a\x07\x98V[aKEV[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a?tW` \x88a>ia\x14\x83\x8Ca>a\x8F\x96\x86\x8Ea>Fa4o\x86\x80\x95a \xBAV[a>S\x84\x84\x84\x01Qa \xBAV[R\x82a?AW[\x01Qa \xBAV[Q\x95Qa \xBAV[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x03EWa &\x8Aa?\x16\x8Fa?\x0F\x8F\x84` \x8F\x92a?\x06\x93a>\xFE\x84`\x01\x9Ea?\x1C\x9E_\x91a?$W[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa \xBAV[R\x01Qa \xBAV[Q\x93\x8DQa \xBAV[Q\x16aK\xCCV[\x90aK\xFDV[\x97\x01\x96a>\x19V[a?;\x91P\x86=\x81\x11a\x15\x0CWa\x14\xFE\x81\x83a\x03\x9DV[_a>\xEFV[a?oa?Q\x84\x84\x84\x01Qa \xBAV[Qa?h\x84\x84\x01Qa?b\x87a9\xFFV[\x90a \xBAV[Q\x10a:\rV[a>ZV[P\x90\x95\x97\x94\x96Pa?\x89\x91\x98\x93\x92\x99PaL\xE3V[\x91a?\x96`\x97T`\xFF\x16\x90V[\x90\x81\x15aE\x15W`@Qc\x18\x89\x1F\xD7`\xE3\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03EW_\x91aD\xF6W[P\x91\x90[_\x92[\x81\x84\x10a@GWPPPPP\x92a@.a@)a@\"a@A\x95\x85a\x11\x97\x98`\x80``` \x99\x01Q\x92\x01Q\x92a\x1FsV[\x91\x90a<\"V[a<\x8DV[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a2\x8EV[Q\x90 \x90V[\x92\x98\x95\x96\x90\x93\x99\x91\x97\x94\x87\x8B\x88\x8C\x88\x8DaC\xF0W[a\x14\x83\x82`\xA0a@\x9Ca#\xD4a*\xCA\x84a@\xA4\x97a@\x96a@\x88a4o\x8F\x9C`@` \x9F\x9E\x01Qa \xBAV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x9Ba(]V[\x97\x01Qa \xBAV[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x95\x90\x95\x16`\x04\x86\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03EWaAha\x14\x83\x8F\x95\x8F\x90aA`\x8F\x97\x8F\x96\x84\x8FaAZ`\xC0\x96aAS\x84\x8F` \x9F\x90a>Za*\xCA\x99`@\x93a#\xD4\x9C_\x91aC\xC2W[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a;sV[Q\x90aG\nV[\x9Ca(]V[\x96\x01Qa \xBAV[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03EWaA\xF5\x91\x8C\x8F\x92_\x92aC\x9EW[P` aA\xE7\x92\x93\x01Qa \xBAV[\x90`\x01`\x01``\x1B\x03\x16\x90RV[aB\x15\x8CaA\xE7\x8CaB\x0Ea\x13\x06\x82` \x86\x01Qa \xBAV[\x92Qa \xBAV[_\x98_[` \x8A\x01QQ\x81\x10\x15aC\x85W\x8B\x8DaBW\x89aBJa#\xD4a*\xCA\x86\x8F\x89aBB\x91Qa \xBAV[Q\x94\x87a(]V[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[aBfW[PP`\x01\x01aB\x19V[\x8A\x8AaB\xE8\x85\x9F\x94\x8F\x96\x86a*\x18\x8F\x93`\xE0aB\x9Fa\x14\x83\x95` aB\x97a#\xD4a*\xCA\x83\x9FaB\xA8\x9C\x89\x91a(]V[\x9A\x01Qa \xBAV[Q\x9B\x01Qa \xBAV[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x96\x90\x96R\x91\x90\x94\x16`d\x85\x01R\x83\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03EW\x8FaCT\x90\x8F\x93`\x01\x95\x94\x86\x95_\x92aC_W[PaCNaA\xE7\x92\x93Q\x93aCIa\x13\x06\x84\x87a \xBAV[a<\x02V[\x92a \xBAV[\x01\x9A\x90P\x8B\x8DaB\\V[aA\xE7\x92PaC~aCN\x91` =\x81\x11a%\x87Wa%x\x81\x83a\x03\x9DV[\x92PaC1V[P\x93\x91\x97\x96\x99`\x01\x91\x96\x99P\x9A\x94\x92\x9A\x01\x92\x91\x90a?\xF1V[aA\xE7\x92PaC\xBB` \x91\x82=\x81\x11a%\x87Wa%x\x81\x83a\x03\x9DV[\x92PaA\xD8V[` aC\xE3\x92P=\x81\x11aC\xE9W[aC\xDB\x81\x83a\x03\x9DV[\x81\x01\x90a;RV[_aA=V[P=aC\xD1V[aD-\x94PaD\n\x92Pa#\xD4\x91a*\xCA\x91` \x95a(]V[`@Qc\x12M\x06!`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03EW` \x89a@\xA4\x8F\x93\x8F`\xA0\x8F\x97a#\xD4a*\xCA\x8F\x8F\x90a@\x96a@\x88a4o\x8F`@\x8B\x96\x91\x8F\x88\x93a\x14\x83\x9FaD\xB1\x90aD\xB7\x93a@\x9C\x9F_\x92aD\xCDW[Pc\xFF\xFF\xFF\xFF\x80\x91\x16\x93\x16\x90a:\xB1V[\x11a:\xBEV[PPPPPP\x97PPPPPP\x92\x93PPa@\\V[` c\xFF\xFF\xFF\xFF\x92\x93P\x82\x91aD\xEE\x91=\x81\x11a!\x96Wa!\x88\x81\x83a\x03\x9DV[\x92\x91PaD\xA0V[aE\x0F\x91P` =` \x11a+\xDCWa+\xCC\x81\x83a\x03\x9DV[_a?\xEAV[_\x91\x90a?\xEEV[aE?\x91P` =` \x11aEEW[aE7\x81\x83a\x03\x9DV[\x81\x01\x90a9\xEAV[_a>\x04V[P=aE-V[P`\xE0\x84\x01QQ\x85\x14a= V[P`\xC0\x84\x01QQ\x85\x14a=\x1AV[P`\xA0\x84\x01QQ\x85\x14a=\x14V[\x15aE}WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[aE\xE2\x90aM\xC0V[`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`\xCDT\x16\x17`\xCDU`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`\xCET\x16\x17`\xCEUV[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`@Q\x90aF\x8D\x82a\x03bV[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80aF\xA9\x81\x84a\x03\x9DV[6\x837V[`@Q\x90aF\xBD` \x83a\x03\x9DV[` 6\x837V[\x91\x90`@\x90``aF\xD3aF\x80V[\x94\x85\x92` \x85Q\x92aF\xE5\x85\x85a\x03\x9DV[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15aG\x08WV[\xFE[` \x92\x91`\x80`@\x92aG\x1BaF\x80V[\x95\x86\x93\x81\x86Q\x93aG,\x86\x86a\x03\x9DV[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15aG\x08W\x15aG]WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x90\xFD[`@QaG\x9E\x81a\x03bV[`@\x90\x81QaG\xAD\x83\x82a\x03\x9DV[\x826\x827\x81R` \x82Q\x91aG\xC2\x84\x84a\x03\x9DV[\x836\x847\x01R\x80QaG\xD4\x82\x82a\x03\x9DV[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90aH*\x83\x83a\x03\x9DV[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01RaH\x7F\x83Q\x93\x84a\x03\x9DV[\x82R` \x82\x01R\x90V[_Q` aP\\_9_Q\x90_R\x90aH\xA0aF\x80V[P_\x91\x90\x06` `\xC0\x83[aI\xA0W_\x93_Q` aP\\_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@QaH\xD6\x85\x82a\x03\x9DV[\x846\x827\x84\x81\x85`@QaH\xEA\x82\x82a\x03\x9DV[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aP\\_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15aG\x08WaIT\x90aP\x0FV[Q\x91aI\xA0W_Q` aP\\_9_Q\x90_R\x82\x80\t\x14aI\x8BWP_Q` aP\\_9_Q\x90_R`\x01_\x94\x08\x92\x93aH\xABV[\x92\x93PPaI\x97a\x03\xD0V[\x92\x83R\x82\x01R\x90V[a\x1F_V[aI\xADaF\x80V[P`@QaI\xBA\x81a\x03bV[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x0C\x81\x10\x15a\x1FZW`\x05\x1B\x01\x90V[\x93\x92\x90\x91aI\xE7`@a\x03\xEEV[\x94\x85R` \x85\x01RaI\xF9`@a\x03\xEEV[\x91\x82R` \x82\x01RaJ\taF\x98V[\x92_[`\x02\x81\x10aJ6WPPP` a\x01\x80\x92aJ%aF\xAEV[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80aJB`\x01\x92a/\x9FV[aJL\x82\x85a\x1FIV[QQaJX\x82\x89aI\xC8V[R` aJe\x83\x86a\x1FIV[Q\x01QaJzaJt\x83a:kV[\x89aI\xC8V[RaJ\x85\x82\x86a\x1FIV[QQQaJ\x94aJt\x83a:yV[RaJ\xAAaJ\xA2\x83\x87a\x1FIV[QQ` \x01\x90V[QaJ\xB7aJt\x83a:\x87V[R` aJ\xC4\x83\x87a\x1FIV[Q\x01QQaJ\xD4aJt\x83a:\x95V[RaK\0aJ\xFAaJ\xF3` aJ\xEA\x86\x8Aa\x1FIV[Q\x01Q` \x01\x90V[Q\x92a:\xA3V[\x88aI\xC8V[R\x01aJ\x0CV[` \x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x91\x15\x15`\xFF\x19`\x97T\x16`\xFF\x82\x16\x17`\x97U`@Q\x90\x81R\xA1V[\x90`\x01aKS`\xFF\x93aO(V[\x92\x83\x92\x16\x1B\x11\x15aKaW\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x90\xFD[\x80_\x91[aK\xD8WP\x90V[_\x19\x81\x01\x81\x81\x11a'\x91Wa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a'\x91W`\x01\x01\x90\x80aK\xD0V[\x90aL\x06aF\x80V[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15aL\xABW`\x01\x82\x14aL\xA6WaL'a\x03\xD0V[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15aLLWPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14aL\x86W[`\x01aL|aLq\x83`\xFF\x94aG\nV[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91aL8V[\x94`\x01aL|aLqaL\x9B\x89`\xFF\x95aG\nV[\x98\x93PPPPaL`V[PP\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x90\xFD[aL\xEBaF\x80V[P\x80Q\x90\x81\x15\x80aM\\W[\x15aM\x18WPP`@QaM\x0C`@\x82a\x03\x9DV[_\x81R_` \x82\x01R\x90V[` _Q` aP\\_9_Q\x90_R\x91\x01Q\x06_Q` aP\\_9_Q\x90_R\x03_Q` aP\\_9_Q\x90_R\x81\x11a'\x91W`@Q\x91aH\x7F\x83a\x03bV[P` \x81\x01Q\x15aL\xF7V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aM|WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[a\xFF\xFFaN\x14\x82aK\xCCV[\x16aN\x1E\x81a\x07}V[\x90aN,`@Q\x92\x83a\x03\x9DV[\x80\x82RaN;`\x1F\x19\x91a\x07}V[\x016` \x83\x017__[\x82Q\x82\x10\x80aN\x9BW[\x15aN\x94W`\x01\x81\x1B\x84\x16aNmW[aNh\x90a(~V[aNEV[\x90`\x01aNh\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1AaN\x8A\x82\x87a\"\0V[S\x01\x91\x90PaN_V[PP\x90P\x90V[Pa\x01\0\x81\x10aNOV[\x15aN\xADWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x90a\x01\0\x82Q\x11aO\x98W\x81Q\x15aO\x93WaOVaOLa#\xD4a#\xC6\x85a \xADV[`\xFF`\x01\x91\x16\x1B\x90V[`\x01\x90[\x83Q\x82\x10\x15aO\x8EW`\x01\x90aOyaOLa#\xD4a#\xC6\x86\x89a\"\0V[\x90aO\x85\x81\x83\x11aN\xA6V[\x17\x91\x01\x90aOZV[\x92PPV[_\x91PV[`\xA4`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R\xFD[\x15aP\x16WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGBLSSignatureChecker.checkSignatu\xA2dipfsX\"\x12 \xFD\"\xD4\xFF\xB9L\x08%\xEB\xFC\xC1\xA6\xADN\x1D \xCD\x0C\x7F\xDEaY\xBFN\x7FT|\xCEJ\xC8R\xF7dsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `CurrentlyPaused()` and selector `0x840a48d5`.
    ```solidity
    error CurrentlyPaused();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CurrentlyPaused {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<CurrentlyPaused> for UnderlyingRustTuple<'_> {
            fn from(value: CurrentlyPaused) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CurrentlyPaused {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CurrentlyPaused {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CurrentlyPaused()";
            const SELECTOR: [u8; 4] = [132u8, 10u8, 72u8, 213u8];
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
        }
    };
    /**Custom error with signature `InputAddressZero()` and selector `0x73632176`.
    ```solidity
    error InputAddressZero();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputAddressZero {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<InputAddressZero> for UnderlyingRustTuple<'_> {
            fn from(value: InputAddressZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputAddressZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputAddressZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputAddressZero()";
            const SELECTOR: [u8; 4] = [115u8, 99u8, 33u8, 118u8];
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
        }
    };
    /**Custom error with signature `InvalidNewPausedStatus()` and selector `0xc61dca5d`.
    ```solidity
    error InvalidNewPausedStatus();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNewPausedStatus {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<InvalidNewPausedStatus> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidNewPausedStatus) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidNewPausedStatus {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidNewPausedStatus {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidNewPausedStatus()";
            const SELECTOR: [u8; 4] = [198u8, 29u8, 202u8, 93u8];
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
        }
    };
    /**Custom error with signature `OnlyPauser()` and selector `0x75df51dc`.
    ```solidity
    error OnlyPauser();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyPauser {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<OnlyPauser> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyPauser) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyPauser {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyPauser {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyPauser()";
            const SELECTOR: [u8; 4] = [117u8, 223u8, 81u8, 220u8];
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
        }
    };
    /**Custom error with signature `OnlyUnpauser()` and selector `0x794821ff`.
    ```solidity
    error OnlyUnpauser();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyUnpauser {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<OnlyUnpauser> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyUnpauser) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyUnpauser {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyUnpauser {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyUnpauser()";
            const SELECTOR: [u8; 4] = [121u8, 72u8, 33u8, 255u8];
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
        }
    };
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
    /**Event with signature `NewTaskCreated(uint32,(uint256,uint32,bytes,uint32))` and selector `0x1695b8d06ec800b4615e745cfb5bd00c1f2875615d42925c3b5afa543bb24c48`.
    ```solidity
    event NewTaskCreated(uint32 indexed taskIndex, IIncredibleSquaringTaskManager.Task task);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewTaskCreated {
        #[allow(missing_docs)]
        pub taskIndex: u32,
        #[allow(missing_docs)]
        pub task: <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for NewTaskCreated {
            type DataTuple<'a> = (IIncredibleSquaringTaskManager::Task,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "NewTaskCreated(uint32,(uint256,uint32,bytes,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    22u8, 149u8, 184u8, 208u8, 110u8, 200u8, 0u8, 180u8, 97u8, 94u8, 116u8, 92u8,
                    251u8, 91u8, 208u8, 12u8, 31u8, 40u8, 117u8, 97u8, 93u8, 66u8, 146u8, 92u8,
                    59u8, 90u8, 250u8, 84u8, 59u8, 178u8, 76u8, 72u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskIndex: topics.1,
                    task: data.0,
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
                    <IIncredibleSquaringTaskManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.taskIndex.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewTaskCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewTaskCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewTaskCreated) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `Paused(address,uint256)` and selector `0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d`.
    ```solidity
    event Paused(address indexed account, uint256 newPausedStatus);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Paused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Paused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Paused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8, 188u8,
                    152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8, 11u8, 182u8,
                    224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    newPausedStatus: data.0,
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newPausedStatus,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
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
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Paused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Paused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Paused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StaleStakesForbiddenUpdate(bool)` and selector `0x40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc`.
    ```solidity
    event StaleStakesForbiddenUpdate(bool value);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StaleStakesForbiddenUpdate {
        #[allow(missing_docs)]
        pub value: bool,
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
        impl alloy_sol_types::SolEvent for StaleStakesForbiddenUpdate {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "StaleStakesForbiddenUpdate(bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    64u8, 228u8, 237u8, 136u8, 10u8, 41u8, 224u8, 246u8, 221u8, 206u8, 48u8, 116u8,
                    87u8, 251u8, 117u8, 205u8, 223u8, 79u8, 238u8, 247u8, 211u8, 236u8, 176u8,
                    48u8, 27u8, 253u8, 244u8, 151u8, 106u8, 14u8, 45u8, 252u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { value: data.0 }
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.value,
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
        impl alloy_sol_types::private::IntoLogData for StaleStakesForbiddenUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StaleStakesForbiddenUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StaleStakesForbiddenUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TaskChallengedSuccessfully(uint32,address)` and selector `0xc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec`.
    ```solidity
    event TaskChallengedSuccessfully(uint32 indexed taskIndex, address indexed challenger);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskChallengedSuccessfully {
        #[allow(missing_docs)]
        pub taskIndex: u32,
        #[allow(missing_docs)]
        pub challenger: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for TaskChallengedSuccessfully {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "TaskChallengedSuccessfully(uint32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    194u8, 13u8, 27u8, 176u8, 241u8, 98u8, 54u8, 128u8, 48u8, 107u8, 131u8, 212u8,
                    255u8, 75u8, 185u8, 154u8, 43u8, 235u8, 157u8, 134u8, 217u8, 120u8, 50u8,
                    243u8, 202u8, 64u8, 253u8, 19u8, 162u8, 157u8, 241u8, 236u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskIndex: topics.1,
                    challenger: topics.2,
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
                    self.taskIndex.clone(),
                    self.challenger.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.challenger,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TaskChallengedSuccessfully {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskChallengedSuccessfully> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TaskChallengedSuccessfully) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TaskChallengedUnsuccessfully(uint32,address)` and selector `0xfd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb05`.
    ```solidity
    event TaskChallengedUnsuccessfully(uint32 indexed taskIndex, address indexed challenger);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskChallengedUnsuccessfully {
        #[allow(missing_docs)]
        pub taskIndex: u32,
        #[allow(missing_docs)]
        pub challenger: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for TaskChallengedUnsuccessfully {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "TaskChallengedUnsuccessfully(uint32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    253u8, 62u8, 38u8, 190u8, 235u8, 89u8, 103u8, 252u8, 90u8, 87u8, 160u8, 68u8,
                    105u8, 20u8, 234u8, 188u8, 69u8, 180u8, 170u8, 71u8, 76u8, 103u8, 165u8, 27u8,
                    75u8, 81u8, 96u8, 202u8, 198u8, 13u8, 219u8, 5u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskIndex: topics.1,
                    challenger: topics.2,
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
                    self.taskIndex.clone(),
                    self.challenger.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.challenger,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TaskChallengedUnsuccessfully {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskChallengedUnsuccessfully> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TaskChallengedUnsuccessfully) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TaskCompleted(uint32)` and selector `0x9a144f228a931b9d0d1696fbcdaf310b24b5d2d21e799db623fc986a0f547430`.
    ```solidity
    event TaskCompleted(uint32 indexed taskIndex);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskCompleted {
        #[allow(missing_docs)]
        pub taskIndex: u32,
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
        impl alloy_sol_types::SolEvent for TaskCompleted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "TaskCompleted(uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    154u8, 20u8, 79u8, 34u8, 138u8, 147u8, 27u8, 157u8, 13u8, 22u8, 150u8, 251u8,
                    205u8, 175u8, 49u8, 11u8, 36u8, 181u8, 210u8, 210u8, 30u8, 121u8, 157u8, 182u8,
                    35u8, 252u8, 152u8, 106u8, 15u8, 84u8, 116u8, 48u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskIndex: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.taskIndex.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TaskCompleted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskCompleted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TaskCompleted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TaskResponded((uint32,uint256),(uint32,bytes32))` and selector `0x349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a`.
    ```solidity
    event TaskResponded(IIncredibleSquaringTaskManager.TaskResponse taskResponse, IIncredibleSquaringTaskManager.TaskResponseMetadata taskResponseMetadata);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskResponded {
        #[allow(missing_docs)]
        pub taskResponse: <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponseMetadata: <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for TaskResponded {
            type DataTuple<'a> = (
                IIncredibleSquaringTaskManager::TaskResponse,
                IIncredibleSquaringTaskManager::TaskResponseMetadata,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TaskResponded((uint32,uint256),(uint32,bytes32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    52u8, 156u8, 30u8, 230u8, 14u8, 78u8, 137u8, 114u8, 238u8, 157u8, 186u8, 100u8,
                    44u8, 23u8, 116u8, 84u8, 61u8, 92u8, 65u8, 54u8, 135u8, 155u8, 127u8, 76u8,
                    170u8, 240u8, 75u8, 248u8, 26u8, 72u8, 122u8, 42u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskResponse: data.0,
                    taskResponseMetadata: data.1,
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
                    <IIncredibleSquaringTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponse,
                    ),
                    <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponseMetadata,
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
        impl alloy_sol_types::private::IntoLogData for TaskResponded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskResponded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TaskResponded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Unpaused(address,uint256)` and selector `0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c`.
    ```solidity
    event Unpaused(address indexed account, uint256 newPausedStatus);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Unpaused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Unpaused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Unpaused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                    188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                    107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    newPausedStatus: data.0,
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newPausedStatus,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
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
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Unpaused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Unpaused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Unpaused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(address _registryCoordinator, address _pauserRegistry, uint32 _taskResponseWindowBlock, address _instantSlasher, address _allocationManager, address _serviceManager);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _registryCoordinator: alloy::sol_types::private::Address,
        pub _pauserRegistry: alloy::sol_types::private::Address,
        pub _taskResponseWindowBlock: u32,
        pub _instantSlasher: alloy::sol_types::private::Address,
        pub _allocationManager: alloy::sol_types::private::Address,
        pub _serviceManager: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u32,
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
                        value._registryCoordinator,
                        value._pauserRegistry,
                        value._taskResponseWindowBlock,
                        value._instantSlasher,
                        value._allocationManager,
                        value._serviceManager,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _registryCoordinator: tuple.0,
                        _pauserRegistry: tuple.1,
                        _taskResponseWindowBlock: tuple.2,
                        _instantSlasher: tuple.3,
                        _allocationManager: tuple.4,
                        _serviceManager: tuple.5,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
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
                        &self._registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._taskResponseWindowBlock,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._instantSlasher,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._serviceManager,
                    ),
                )
            }
        }
    };
    /**Function with signature `TASK_CHALLENGE_WINDOW_BLOCK()` and selector `0xf63c5bab`.
    ```solidity
    function TASK_CHALLENGE_WINDOW_BLOCK() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_CHALLENGE_WINDOW_BLOCKCall {}
    ///Container type for the return parameters of the [`TASK_CHALLENGE_WINDOW_BLOCK()`](TASK_CHALLENGE_WINDOW_BLOCKCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_CHALLENGE_WINDOW_BLOCKReturn {
        pub _0: u32,
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
            impl ::core::convert::From<TASK_CHALLENGE_WINDOW_BLOCKCall> for UnderlyingRustTuple<'_> {
                fn from(value: TASK_CHALLENGE_WINDOW_BLOCKCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TASK_CHALLENGE_WINDOW_BLOCKCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<TASK_CHALLENGE_WINDOW_BLOCKReturn> for UnderlyingRustTuple<'_> {
                fn from(value: TASK_CHALLENGE_WINDOW_BLOCKReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TASK_CHALLENGE_WINDOW_BLOCKReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TASK_CHALLENGE_WINDOW_BLOCKCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = TASK_CHALLENGE_WINDOW_BLOCKReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TASK_CHALLENGE_WINDOW_BLOCK()";
            const SELECTOR: [u8; 4] = [246u8, 60u8, 91u8, 171u8];
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
    /**Function with signature `TASK_RESPONSE_WINDOW_BLOCK()` and selector `0x1ad43189`.
    ```solidity
    function TASK_RESPONSE_WINDOW_BLOCK() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_RESPONSE_WINDOW_BLOCKCall {}
    ///Container type for the return parameters of the [`TASK_RESPONSE_WINDOW_BLOCK()`](TASK_RESPONSE_WINDOW_BLOCKCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_RESPONSE_WINDOW_BLOCKReturn {
        pub _0: u32,
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
            impl ::core::convert::From<TASK_RESPONSE_WINDOW_BLOCKCall> for UnderlyingRustTuple<'_> {
                fn from(value: TASK_RESPONSE_WINDOW_BLOCKCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TASK_RESPONSE_WINDOW_BLOCKCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<TASK_RESPONSE_WINDOW_BLOCKReturn> for UnderlyingRustTuple<'_> {
                fn from(value: TASK_RESPONSE_WINDOW_BLOCKReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TASK_RESPONSE_WINDOW_BLOCKReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TASK_RESPONSE_WINDOW_BLOCKCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = TASK_RESPONSE_WINDOW_BLOCKReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TASK_RESPONSE_WINDOW_BLOCK()";
            const SELECTOR: [u8; 4] = [26u8, 212u8, 49u8, 137u8];
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
    /**Function with signature `aggregator()` and selector `0x245a7bfc`.
    ```solidity
    function aggregator() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aggregatorCall {}
    ///Container type for the return parameters of the [`aggregator()`](aggregatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aggregatorReturn {
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
            impl ::core::convert::From<aggregatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: aggregatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for aggregatorCall {
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
            impl ::core::convert::From<aggregatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: aggregatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for aggregatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for aggregatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = aggregatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "aggregator()";
            const SELECTOR: [u8; 4] = [36u8, 90u8, 123u8, 252u8];
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
    /**Function with signature `allTaskHashes(uint32)` and selector `0x2d89f6fc`.
    ```solidity
    function allTaskHashes(uint32) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskHashesCall {
        pub _0: u32,
    }
    ///Container type for the return parameters of the [`allTaskHashes(uint32)`](allTaskHashesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskHashesReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<allTaskHashesCall> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskHashesCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskHashesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<allTaskHashesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskHashesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskHashesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allTaskHashesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = allTaskHashesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allTaskHashes(uint32)";
            const SELECTOR: [u8; 4] = [45u8, 137u8, 246u8, 252u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
    /**Function with signature `allTaskResponses(uint32)` and selector `0x2cb223d5`.
    ```solidity
    function allTaskResponses(uint32) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskResponsesCall {
        pub _0: u32,
    }
    ///Container type for the return parameters of the [`allTaskResponses(uint32)`](allTaskResponsesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskResponsesReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<allTaskResponsesCall> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskResponsesCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskResponsesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<allTaskResponsesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskResponsesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskResponsesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allTaskResponsesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = allTaskResponsesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allTaskResponses(uint32)";
            const SELECTOR: [u8; 4] = [44u8, 178u8, 35u8, 213u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
    /**Function with signature `blsApkRegistry()` and selector `0x5df45946`.
    ```solidity
    function blsApkRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blsApkRegistryCall {}
    ///Container type for the return parameters of the [`blsApkRegistry()`](blsApkRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blsApkRegistryReturn {
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
            impl ::core::convert::From<blsApkRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: blsApkRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for blsApkRegistryCall {
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
            impl ::core::convert::From<blsApkRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: blsApkRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for blsApkRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for blsApkRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = blsApkRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "blsApkRegistry()";
            const SELECTOR: [u8; 4] = [93u8, 244u8, 89u8, 70u8];
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
    /**Function with signature `checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x6efb4636`.
    ```solidity
    function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureChecker.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureChecker.QuorumStakeTotals memory, bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkSignaturesCall {
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        pub referenceBlockNumber: u32,
        pub params: <IBLSSignatureChecker::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))`](checkSignaturesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkSignaturesReturn {
        pub _0: <IBLSSignatureChecker::QuorumStakeTotals as alloy::sol_types::SolType>::RustType,
        pub _1: alloy::sol_types::private::FixedBytes<32>,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
                IBLSSignatureChecker::NonSignerStakesAndSignature,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
                u32,
                <IBLSSignatureChecker::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<checkSignaturesCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkSignaturesCall) -> Self {
                    (
                        value.msgHash,
                        value.quorumNumbers,
                        value.referenceBlockNumber,
                        value.params,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkSignaturesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        msgHash: tuple.0,
                        quorumNumbers: tuple.1,
                        referenceBlockNumber: tuple.2,
                        params: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IBLSSignatureChecker::QuorumStakeTotals,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IBLSSignatureChecker::QuorumStakeTotals as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<checkSignaturesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkSignaturesReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkSignaturesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkSignaturesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
                IBLSSignatureChecker::NonSignerStakesAndSignature,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkSignaturesReturn;
            type ReturnTuple<'a> = (
                IBLSSignatureChecker::QuorumStakeTotals,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))";
            const SELECTOR: [u8; 4] = [110u8, 251u8, 70u8, 54u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.msgHash),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceBlockNumber),
                    <IBLSSignatureChecker::NonSignerStakesAndSignature as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `createNewTask(uint256,uint32,bytes)` and selector `0x6b92787e`.
    ```solidity
    function createNewTask(uint256 numberToBeSquared, uint32 quorumThresholdPercentage, bytes memory quorumNumbers) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createNewTaskCall {
        pub numberToBeSquared: alloy::sol_types::private::primitives::aliases::U256,
        pub quorumThresholdPercentage: u32,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`createNewTask(uint256,uint32,bytes)`](createNewTaskCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createNewTaskReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                u32,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<createNewTaskCall> for UnderlyingRustTuple<'_> {
                fn from(value: createNewTaskCall) -> Self {
                    (
                        value.numberToBeSquared,
                        value.quorumThresholdPercentage,
                        value.quorumNumbers,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createNewTaskCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        numberToBeSquared: tuple.0,
                        quorumThresholdPercentage: tuple.1,
                        quorumNumbers: tuple.2,
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
            impl ::core::convert::From<createNewTaskReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createNewTaskReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createNewTaskReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createNewTaskCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createNewTaskReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createNewTask(uint256,uint32,bytes)";
            const SELECTOR: [u8; 4] = [107u8, 146u8, 120u8, 126u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.numberToBeSquared,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumThresholdPercentage,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
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
    /**Function with signature `delegation()` and selector `0xdf5cf723`.
    ```solidity
    function delegation() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationCall {}
    ///Container type for the return parameters of the [`delegation()`](delegationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationReturn {
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
            impl ::core::convert::From<delegationCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationCall {
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
            impl ::core::convert::From<delegationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegation()";
            const SELECTOR: [u8; 4] = [223u8, 92u8, 247u8, 35u8];
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
    /**Function with signature `generator()` and selector `0x7afa1eed`.
    ```solidity
    function generator() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct generatorCall {}
    ///Container type for the return parameters of the [`generator()`](generatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct generatorReturn {
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
            impl ::core::convert::From<generatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: generatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for generatorCall {
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
            impl ::core::convert::From<generatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: generatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for generatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for generatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = generatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "generator()";
            const SELECTOR: [u8; 4] = [122u8, 250u8, 30u8, 237u8];
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
    /**Function with signature `getBatchOperatorFromId(address,bytes32[])` and selector `0x4d2b57fe`.
    ```solidity
    function getBatchOperatorFromId(address registryCoordinator, bytes32[] memory operatorIds) external view returns (address[] memory operators);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorFromIdCall {
        pub registryCoordinator: alloy::sol_types::private::Address,
        pub operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    ///Container type for the return parameters of the [`getBatchOperatorFromId(address,bytes32[])`](getBatchOperatorFromIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorFromIdReturn {
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<getBatchOperatorFromIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorFromIdCall) -> Self {
                    (value.registryCoordinator, value.operatorIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBatchOperatorFromIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        operatorIds: tuple.1,
                    }
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
            impl ::core::convert::From<getBatchOperatorFromIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorFromIdReturn) -> Self {
                    (value.operators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBatchOperatorFromIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operators: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBatchOperatorFromIdCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBatchOperatorFromIdReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBatchOperatorFromId(address,bytes32[])";
            const SELECTOR: [u8; 4] = [77u8, 43u8, 87u8, 254u8];
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
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorIds),
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
    /**Function with signature `getBatchOperatorId(address,address[])` and selector `0x31b36bd9`.
    ```solidity
    function getBatchOperatorId(address registryCoordinator, address[] memory operators) external view returns (bytes32[] memory operatorIds);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorIdCall {
        pub registryCoordinator: alloy::sol_types::private::Address,
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`getBatchOperatorId(address,address[])`](getBatchOperatorIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorIdReturn {
        pub operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<getBatchOperatorIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorIdCall) -> Self {
                    (value.registryCoordinator, value.operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBatchOperatorIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        operators: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,);
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
            impl ::core::convert::From<getBatchOperatorIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorIdReturn) -> Self {
                    (value.operatorIds,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBatchOperatorIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorIds: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBatchOperatorIdCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBatchOperatorIdReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBatchOperatorId(address,address[])";
            const SELECTOR: [u8; 4] = [49u8, 179u8, 107u8, 217u8];
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
                        &self.registryCoordinator,
                    ),
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getCheckSignaturesIndices(address,uint32,bytes,bytes32[])` and selector `0x4f739f74`.
    ```solidity
    function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes memory quorumNumbers, bytes32[] memory nonSignerOperatorIds) external view returns (OperatorStateRetriever.CheckSignaturesIndices memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCheckSignaturesIndicesCall {
        pub registryCoordinator: alloy::sol_types::private::Address,
        pub referenceBlockNumber: u32,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        pub nonSignerOperatorIds:
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    ///Container type for the return parameters of the [`getCheckSignaturesIndices(address,uint32,bytes,bytes32[])`](getCheckSignaturesIndicesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCheckSignaturesIndicesReturn {
        pub _0:
            <OperatorStateRetriever::CheckSignaturesIndices as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                u32,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<getCheckSignaturesIndicesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCheckSignaturesIndicesCall) -> Self {
                    (
                        value.registryCoordinator,
                        value.referenceBlockNumber,
                        value.quorumNumbers,
                        value.nonSignerOperatorIds,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCheckSignaturesIndicesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        referenceBlockNumber: tuple.1,
                        quorumNumbers: tuple.2,
                        nonSignerOperatorIds: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (OperatorStateRetriever::CheckSignaturesIndices,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorStateRetriever::CheckSignaturesIndices as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCheckSignaturesIndicesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCheckSignaturesIndicesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCheckSignaturesIndicesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCheckSignaturesIndicesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCheckSignaturesIndicesReturn;
            type ReturnTuple<'a> = (OperatorStateRetriever::CheckSignaturesIndices,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getCheckSignaturesIndices(address,uint32,bytes,bytes32[])";
            const SELECTOR: [u8; 4] = [79u8, 115u8, 159u8, 116u8];
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
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceBlockNumber),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerOperatorIds),
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
    /**Function with signature `getOperatorState(address,bytes,uint32)` and selector `0x3563b0d1`.
    ```solidity
    function getOperatorState(address registryCoordinator, bytes memory quorumNumbers, uint32 blockNumber) external view returns (OperatorStateRetriever.Operator[][] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorState_0Call {
        pub registryCoordinator: alloy::sol_types::private::Address,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getOperatorState(address,bytes,uint32)`](getOperatorState_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorState_0Return {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<
                <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
            >,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<getOperatorState_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_0Call) -> Self {
                    (
                        value.registryCoordinator,
                        value.quorumNumbers,
                        value.blockNumber,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorState_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        quorumNumbers: tuple.1,
                        blockNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
                    >,
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
            impl ::core::convert::From<getOperatorState_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorState_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorState_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorState_0Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorState(address,bytes,uint32)";
            const SELECTOR: [u8; 4] = [53u8, 99u8, 176u8, 209u8];
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
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.blockNumber,
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
    /**Function with signature `getOperatorState(address,bytes32,uint32)` and selector `0xcefdc1d4`.
    ```solidity
    function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) external view returns (uint256, OperatorStateRetriever.Operator[][] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorState_1Call {
        pub registryCoordinator: alloy::sol_types::private::Address,
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getOperatorState(address,bytes32,uint32)`](getOperatorState_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorState_1Return {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
        pub _1: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<
                <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
            >,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getOperatorState_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_1Call) -> Self {
                    (
                        value.registryCoordinator,
                        value.operatorId,
                        value.blockNumber,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorState_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        operatorId: tuple.1,
                        blockNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
                    >,
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
            impl ::core::convert::From<getOperatorState_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_1Return) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorState_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorState_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorState_1Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorState(address,bytes32,uint32)";
            const SELECTOR: [u8; 4] = [206u8, 253u8, 193u8, 212u8];
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
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
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
    /**Function with signature `getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)` and selector `0x5c155662`.
    ```solidity
    function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] memory operatorIds, uint32 blockNumber) external view returns (uint256[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapsAtBlockNumberCall {
        pub registryCoordinator: alloy::sol_types::private::Address,
        pub operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)`](getQuorumBitmapsAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapsAtBlockNumberReturn {
        pub _0:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<getQuorumBitmapsAtBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapsAtBlockNumberCall) -> Self {
                    (
                        value.registryCoordinator,
                        value.operatorIds,
                        value.blockNumber,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapsAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        operatorIds: tuple.1,
                        blockNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getQuorumBitmapsAtBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapsAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapsAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapsAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapsAtBlockNumberReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)";
            const SELECTOR: [u8; 4] = [92u8, 21u8, 86u8, 98u8];
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
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorIds),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
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
    /**Function with signature `getTaskResponseWindowBlock()` and selector `0xf5c9899d`.
    ```solidity
    function getTaskResponseWindowBlock() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTaskResponseWindowBlockCall {}
    ///Container type for the return parameters of the [`getTaskResponseWindowBlock()`](getTaskResponseWindowBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTaskResponseWindowBlockReturn {
        pub _0: u32,
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
            impl ::core::convert::From<getTaskResponseWindowBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTaskResponseWindowBlockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTaskResponseWindowBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<getTaskResponseWindowBlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTaskResponseWindowBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTaskResponseWindowBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTaskResponseWindowBlockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTaskResponseWindowBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTaskResponseWindowBlock()";
            const SELECTOR: [u8; 4] = [245u8, 201u8, 137u8, 157u8];
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
    /**Function with signature `initialize(address,address,address)` and selector `0xc0c53b8b`.
    ```solidity
    function initialize(address initialOwner, address _aggregator, address _generator) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub initialOwner: alloy::sol_types::private::Address,
        pub _aggregator: alloy::sol_types::private::Address,
        pub _generator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,address,address)`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value.initialOwner, value._aggregator, value._generator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialOwner: tuple.0,
                        _aggregator: tuple.1,
                        _generator: tuple.2,
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
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,address,address)";
            const SELECTOR: [u8; 4] = [192u8, 197u8, 59u8, 139u8];
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
                        &self.initialOwner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._aggregator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._generator,
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
    /**Function with signature `instantSlasher()` and selector `0x9b290e98`.
    ```solidity
    function instantSlasher() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct instantSlasherCall {}
    ///Container type for the return parameters of the [`instantSlasher()`](instantSlasherCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct instantSlasherReturn {
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
            impl ::core::convert::From<instantSlasherCall> for UnderlyingRustTuple<'_> {
                fn from(value: instantSlasherCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for instantSlasherCall {
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
            impl ::core::convert::From<instantSlasherReturn> for UnderlyingRustTuple<'_> {
                fn from(value: instantSlasherReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for instantSlasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for instantSlasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = instantSlasherReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "instantSlasher()";
            const SELECTOR: [u8; 4] = [155u8, 41u8, 14u8, 152u8];
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
    /**Function with signature `latestTaskNum()` and selector `0x8b00ce7c`.
    ```solidity
    function latestTaskNum() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestTaskNumCall {}
    ///Container type for the return parameters of the [`latestTaskNum()`](latestTaskNumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestTaskNumReturn {
        pub _0: u32,
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
            impl ::core::convert::From<latestTaskNumCall> for UnderlyingRustTuple<'_> {
                fn from(value: latestTaskNumCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for latestTaskNumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<latestTaskNumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: latestTaskNumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for latestTaskNumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for latestTaskNumCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = latestTaskNumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "latestTaskNum()";
            const SELECTOR: [u8; 4] = [139u8, 0u8, 206u8, 124u8];
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
    /**Function with signature `pause(uint256)` and selector `0x136439dd`.
    ```solidity
    function pause(uint256 newPausedStatus) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseCall {
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`pause(uint256)`](pauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseReturn {}
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
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newPausedStatus: tuple.0,
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
            impl ::core::convert::From<pauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pause(uint256)";
            const SELECTOR: [u8; 4] = [19u8, 100u8, 57u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newPausedStatus,
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
    /**Function with signature `pauseAll()` and selector `0x595c6a67`.
    ```solidity
    function pauseAll() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllCall {}
    ///Container type for the return parameters of the [`pauseAll()`](pauseAllCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllReturn {}
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
            impl ::core::convert::From<pauseAllCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseAllCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseAllCall {
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
            impl ::core::convert::From<pauseAllReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseAllReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseAllReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseAllCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseAllReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pauseAll()";
            const SELECTOR: [u8; 4] = [89u8, 92u8, 106u8, 103u8];
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
    /**Function with signature `paused(uint8)` and selector `0x5ac86ab7`.
    ```solidity
    function paused(uint8 index) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Call {
        pub index: u8,
    }
    ///Container type for the return parameters of the [`paused(uint8)`](paused_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Return {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
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
            impl ::core::convert::From<paused_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: paused_0Call) -> Self {
                    (value.index,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { index: tuple.0 }
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
            impl ::core::convert::From<paused_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: paused_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paused_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = paused_0Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused(uint8)";
            const SELECTOR: [u8; 4] = [90u8, 200u8, 106u8, 183u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.index,
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
    /**Function with signature `paused()` and selector `0x5c975abb`.
    ```solidity
    function paused() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_1Call {}
    ///Container type for the return parameters of the [`paused()`](paused_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_1Return {
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
            impl ::core::convert::From<paused_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: paused_1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_1Call {
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
            impl ::core::convert::From<paused_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: paused_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paused_1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = paused_1Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
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
    /**Function with signature `pauserRegistry()` and selector `0x886f1195`.
    ```solidity
    function pauserRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauserRegistryCall {}
    ///Container type for the return parameters of the [`pauserRegistry()`](pauserRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauserRegistryReturn {
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
            impl ::core::convert::From<pauserRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauserRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauserRegistryCall {
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
            impl ::core::convert::From<pauserRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauserRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauserRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauserRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauserRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pauserRegistry()";
            const SELECTOR: [u8; 4] = [136u8, 111u8, 17u8, 149u8];
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
    /**Function with signature `raiseAndResolveChallenge((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32,bytes32),(uint256,uint256)[])` and selector `0x6b532e9e`.
    ```solidity
    function raiseAndResolveChallenge(IIncredibleSquaringTaskManager.Task memory task, IIncredibleSquaringTaskManager.TaskResponse memory taskResponse, IIncredibleSquaringTaskManager.TaskResponseMetadata memory taskResponseMetadata, BN254.G1Point[] memory pubkeysOfNonSigningOperators) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct raiseAndResolveChallengeCall {
        pub task: <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
        pub taskResponse: <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
        pub taskResponseMetadata: <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
        pub pubkeysOfNonSigningOperators: alloy::sol_types::private::Vec<
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`raiseAndResolveChallenge((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32,bytes32),(uint256,uint256)[])`](raiseAndResolveChallengeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct raiseAndResolveChallengeReturn {}
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
                IIncredibleSquaringTaskManager::Task,
                IIncredibleSquaringTaskManager::TaskResponse,
                IIncredibleSquaringTaskManager::TaskResponseMetadata,
                alloy::sol_types::sol_data::Array<BN254::G1Point>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
                <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
                <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<raiseAndResolveChallengeCall> for UnderlyingRustTuple<'_> {
                fn from(value: raiseAndResolveChallengeCall) -> Self {
                    (
                        value.task,
                        value.taskResponse,
                        value.taskResponseMetadata,
                        value.pubkeysOfNonSigningOperators,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for raiseAndResolveChallengeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        task: tuple.0,
                        taskResponse: tuple.1,
                        taskResponseMetadata: tuple.2,
                        pubkeysOfNonSigningOperators: tuple.3,
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
            impl ::core::convert::From<raiseAndResolveChallengeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: raiseAndResolveChallengeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for raiseAndResolveChallengeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for raiseAndResolveChallengeCall {
            type Parameters<'a> = (
                IIncredibleSquaringTaskManager::Task,
                IIncredibleSquaringTaskManager::TaskResponse,
                IIncredibleSquaringTaskManager::TaskResponseMetadata,
                alloy::sol_types::sol_data::Array<BN254::G1Point>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = raiseAndResolveChallengeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "raiseAndResolveChallenge((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32,bytes32),(uint256,uint256)[])";
            const SELECTOR: [u8; 4] = [107u8, 83u8, 46u8, 158u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IIncredibleSquaringTaskManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                    <IIncredibleSquaringTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponse,
                    ),
                    <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponseMetadata,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.pubkeysOfNonSigningOperators,
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
    /**Function with signature `registryCoordinator()` and selector `0x6d14a987`.
    ```solidity
    function registryCoordinator() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registryCoordinatorCall {}
    ///Container type for the return parameters of the [`registryCoordinator()`](registryCoordinatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registryCoordinatorReturn {
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
            impl ::core::convert::From<registryCoordinatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registryCoordinatorCall {
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
            impl ::core::convert::From<registryCoordinatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registryCoordinatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registryCoordinatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registryCoordinatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registryCoordinator()";
            const SELECTOR: [u8; 4] = [109u8, 20u8, 169u8, 135u8];
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
    /**Function with signature `respondToTask((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x5baec9a0`.
    ```solidity
    function respondToTask(IIncredibleSquaringTaskManager.Task memory task, IIncredibleSquaringTaskManager.TaskResponse memory taskResponse, IBLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respondToTaskCall {
        pub task: <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
        pub taskResponse: <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
        pub nonSignerStakesAndSignature: <IBLSSignatureChecker::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`respondToTask((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))`](respondToTaskCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respondToTaskReturn {}
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
                IIncredibleSquaringTaskManager::Task,
                IIncredibleSquaringTaskManager::TaskResponse,
                IBLSSignatureChecker::NonSignerStakesAndSignature,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
                <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
                <IBLSSignatureChecker::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<respondToTaskCall> for UnderlyingRustTuple<'_> {
                fn from(value: respondToTaskCall) -> Self {
                    (
                        value.task,
                        value.taskResponse,
                        value.nonSignerStakesAndSignature,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for respondToTaskCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        task: tuple.0,
                        taskResponse: tuple.1,
                        nonSignerStakesAndSignature: tuple.2,
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
            impl ::core::convert::From<respondToTaskReturn> for UnderlyingRustTuple<'_> {
                fn from(value: respondToTaskReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for respondToTaskReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for respondToTaskCall {
            type Parameters<'a> = (
                IIncredibleSquaringTaskManager::Task,
                IIncredibleSquaringTaskManager::TaskResponse,
                IBLSSignatureChecker::NonSignerStakesAndSignature,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = respondToTaskReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "respondToTask((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))";
            const SELECTOR: [u8; 4] = [91u8, 174u8, 201u8, 160u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IIncredibleSquaringTaskManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                    <IIncredibleSquaringTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponse,
                    ),
                    <IBLSSignatureChecker::NonSignerStakesAndSignature as alloy_sol_types::SolType>::tokenize(
                        &self.nonSignerStakesAndSignature,
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
    /**Function with signature `serviceManager()` and selector `0x3998fdd3`.
    ```solidity
    function serviceManager() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceManagerCall {}
    ///Container type for the return parameters of the [`serviceManager()`](serviceManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceManagerReturn {
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
            impl ::core::convert::From<serviceManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: serviceManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for serviceManagerCall {
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
            impl ::core::convert::From<serviceManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: serviceManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for serviceManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for serviceManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = serviceManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "serviceManager()";
            const SELECTOR: [u8; 4] = [57u8, 152u8, 253u8, 211u8];
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
    /**Function with signature `setStaleStakesForbidden(bool)` and selector `0x416c7e5e`.
    ```solidity
    function setStaleStakesForbidden(bool value) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setStaleStakesForbiddenCall {
        pub value: bool,
    }
    ///Container type for the return parameters of the [`setStaleStakesForbidden(bool)`](setStaleStakesForbiddenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setStaleStakesForbiddenReturn {}
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
            impl ::core::convert::From<setStaleStakesForbiddenCall> for UnderlyingRustTuple<'_> {
                fn from(value: setStaleStakesForbiddenCall) -> Self {
                    (value.value,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setStaleStakesForbiddenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { value: tuple.0 }
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
            impl ::core::convert::From<setStaleStakesForbiddenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setStaleStakesForbiddenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setStaleStakesForbiddenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setStaleStakesForbiddenCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bool,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setStaleStakesForbiddenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setStaleStakesForbidden(bool)";
            const SELECTOR: [u8; 4] = [65u8, 108u8, 126u8, 94u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.value,
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
    /**Function with signature `stakeRegistry()` and selector `0x68304835`.
    ```solidity
    function stakeRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryCall {}
    ///Container type for the return parameters of the [`stakeRegistry()`](stakeRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryReturn {
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
            impl ::core::convert::From<stakeRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryCall {
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
            impl ::core::convert::From<stakeRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakeRegistry()";
            const SELECTOR: [u8; 4] = [104u8, 48u8, 72u8, 53u8];
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
    /**Function with signature `staleStakesForbidden()` and selector `0xb98d0908`.
    ```solidity
    function staleStakesForbidden() external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct staleStakesForbiddenCall {}
    ///Container type for the return parameters of the [`staleStakesForbidden()`](staleStakesForbiddenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct staleStakesForbiddenReturn {
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
            impl ::core::convert::From<staleStakesForbiddenCall> for UnderlyingRustTuple<'_> {
                fn from(value: staleStakesForbiddenCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for staleStakesForbiddenCall {
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
            impl ::core::convert::From<staleStakesForbiddenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: staleStakesForbiddenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for staleStakesForbiddenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for staleStakesForbiddenCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = staleStakesForbiddenReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "staleStakesForbidden()";
            const SELECTOR: [u8; 4] = [185u8, 141u8, 9u8, 8u8];
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
    /**Function with signature `taskNumber()` and selector `0x72d18e8d`.
    ```solidity
    function taskNumber() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskNumberCall {}
    ///Container type for the return parameters of the [`taskNumber()`](taskNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskNumberReturn {
        pub _0: u32,
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
            impl ::core::convert::From<taskNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: taskNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<taskNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: taskNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for taskNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = taskNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "taskNumber()";
            const SELECTOR: [u8; 4] = [114u8, 209u8, 142u8, 141u8];
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
    /**Function with signature `taskSuccesfullyChallenged(uint32)` and selector `0x5decc3f5`.
    ```solidity
    function taskSuccesfullyChallenged(uint32) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskSuccesfullyChallengedCall {
        pub _0: u32,
    }
    ///Container type for the return parameters of the [`taskSuccesfullyChallenged(uint32)`](taskSuccesfullyChallengedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskSuccesfullyChallengedReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<taskSuccesfullyChallengedCall> for UnderlyingRustTuple<'_> {
                fn from(value: taskSuccesfullyChallengedCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskSuccesfullyChallengedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<taskSuccesfullyChallengedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: taskSuccesfullyChallengedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskSuccesfullyChallengedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for taskSuccesfullyChallengedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = taskSuccesfullyChallengedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "taskSuccesfullyChallenged(uint32)";
            const SELECTOR: [u8; 4] = [93u8, 236u8, 195u8, 245u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
    /**Function with signature `trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))` and selector `0x171f1d5b`.
    ```solidity
    function trySignatureAndApkVerification(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma) external view returns (bool pairingSuccessful, bool siganatureIsValid);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trySignatureAndApkVerificationCall {
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        pub apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))`](trySignatureAndApkVerificationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trySignatureAndApkVerificationReturn {
        pub pairingSuccessful: bool,
        pub siganatureIsValid: bool,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                BN254::G1Point,
                BN254::G2Point,
                BN254::G1Point,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
                <BN254::G2Point as alloy::sol_types::SolType>::RustType,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<trySignatureAndApkVerificationCall> for UnderlyingRustTuple<'_> {
                fn from(value: trySignatureAndApkVerificationCall) -> Self {
                    (value.msgHash, value.apk, value.apkG2, value.sigma)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for trySignatureAndApkVerificationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        msgHash: tuple.0,
                        apk: tuple.1,
                        apkG2: tuple.2,
                        sigma: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool, bool);
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
            impl ::core::convert::From<trySignatureAndApkVerificationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: trySignatureAndApkVerificationReturn) -> Self {
                    (value.pairingSuccessful, value.siganatureIsValid)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for trySignatureAndApkVerificationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pairingSuccessful: tuple.0,
                        siganatureIsValid: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for trySignatureAndApkVerificationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                BN254::G1Point,
                BN254::G2Point,
                BN254::G1Point,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = trySignatureAndApkVerificationReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))";
            const SELECTOR: [u8; 4] = [23u8, 31u8, 29u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.msgHash),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.apk),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.apkG2),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.sigma),
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
    /**Function with signature `unpause(uint256)` and selector `0xfabc1cbc`.
    ```solidity
    function unpause(uint256 newPausedStatus) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall {
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`unpause(uint256)`](unpauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseReturn {}
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
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newPausedStatus: tuple.0,
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
            impl ::core::convert::From<unpauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unpauseCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpause(uint256)";
            const SELECTOR: [u8; 4] = [250u8, 188u8, 28u8, 188u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newPausedStatus,
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
    ///Container for all the [`IncredibleSquaringTaskManager`](self) function calls.
    pub enum IncredibleSquaringTaskManagerCalls {
        TASK_CHALLENGE_WINDOW_BLOCK(TASK_CHALLENGE_WINDOW_BLOCKCall),
        TASK_RESPONSE_WINDOW_BLOCK(TASK_RESPONSE_WINDOW_BLOCKCall),
        aggregator(aggregatorCall),
        allTaskHashes(allTaskHashesCall),
        allTaskResponses(allTaskResponsesCall),
        allocationManager(allocationManagerCall),
        blsApkRegistry(blsApkRegistryCall),
        checkSignatures(checkSignaturesCall),
        createNewTask(createNewTaskCall),
        delegation(delegationCall),
        generator(generatorCall),
        getBatchOperatorFromId(getBatchOperatorFromIdCall),
        getBatchOperatorId(getBatchOperatorIdCall),
        getCheckSignaturesIndices(getCheckSignaturesIndicesCall),
        getOperatorState_0(getOperatorState_0Call),
        getOperatorState_1(getOperatorState_1Call),
        getQuorumBitmapsAtBlockNumber(getQuorumBitmapsAtBlockNumberCall),
        getTaskResponseWindowBlock(getTaskResponseWindowBlockCall),
        initialize(initializeCall),
        instantSlasher(instantSlasherCall),
        latestTaskNum(latestTaskNumCall),
        owner(ownerCall),
        pause(pauseCall),
        pauseAll(pauseAllCall),
        paused_0(paused_0Call),
        paused_1(paused_1Call),
        pauserRegistry(pauserRegistryCall),
        raiseAndResolveChallenge(raiseAndResolveChallengeCall),
        registryCoordinator(registryCoordinatorCall),
        renounceOwnership(renounceOwnershipCall),
        respondToTask(respondToTaskCall),
        serviceManager(serviceManagerCall),
        setStaleStakesForbidden(setStaleStakesForbiddenCall),
        stakeRegistry(stakeRegistryCall),
        staleStakesForbidden(staleStakesForbiddenCall),
        taskNumber(taskNumberCall),
        taskSuccesfullyChallenged(taskSuccesfullyChallengedCall),
        transferOwnership(transferOwnershipCall),
        trySignatureAndApkVerification(trySignatureAndApkVerificationCall),
        unpause(unpauseCall),
    }
    #[automatically_derived]
    impl IncredibleSquaringTaskManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [19u8, 100u8, 57u8, 221u8],
            [23u8, 31u8, 29u8, 91u8],
            [26u8, 212u8, 49u8, 137u8],
            [36u8, 90u8, 123u8, 252u8],
            [44u8, 178u8, 35u8, 213u8],
            [45u8, 137u8, 246u8, 252u8],
            [49u8, 179u8, 107u8, 217u8],
            [53u8, 99u8, 176u8, 209u8],
            [57u8, 152u8, 253u8, 211u8],
            [65u8, 108u8, 126u8, 94u8],
            [77u8, 43u8, 87u8, 254u8],
            [79u8, 115u8, 159u8, 116u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [91u8, 174u8, 201u8, 160u8],
            [92u8, 21u8, 86u8, 98u8],
            [92u8, 151u8, 90u8, 187u8],
            [93u8, 236u8, 195u8, 245u8],
            [93u8, 244u8, 89u8, 70u8],
            [104u8, 48u8, 72u8, 53u8],
            [107u8, 83u8, 46u8, 158u8],
            [107u8, 146u8, 120u8, 126u8],
            [109u8, 20u8, 169u8, 135u8],
            [110u8, 251u8, 70u8, 54u8],
            [113u8, 80u8, 24u8, 166u8],
            [114u8, 209u8, 142u8, 141u8],
            [122u8, 250u8, 30u8, 237u8],
            [136u8, 111u8, 17u8, 149u8],
            [139u8, 0u8, 206u8, 124u8],
            [141u8, 165u8, 203u8, 91u8],
            [155u8, 41u8, 14u8, 152u8],
            [185u8, 141u8, 9u8, 8u8],
            [192u8, 197u8, 59u8, 139u8],
            [202u8, 138u8, 167u8, 199u8],
            [206u8, 253u8, 193u8, 212u8],
            [223u8, 92u8, 247u8, 35u8],
            [242u8, 253u8, 227u8, 139u8],
            [245u8, 201u8, 137u8, 157u8],
            [246u8, 60u8, 91u8, 171u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IncredibleSquaringTaskManagerCalls {
        const NAME: &'static str = "IncredibleSquaringTaskManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 40usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::TASK_CHALLENGE_WINDOW_BLOCK(_) => {
                    <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::TASK_RESPONSE_WINDOW_BLOCK(_) => {
                    <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::aggregator(_) => <aggregatorCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::allTaskHashes(_) => <allTaskHashesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::allTaskResponses(_) => {
                    <allTaskResponsesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::blsApkRegistry(_) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkSignatures(_) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createNewTask(_) => <createNewTaskCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::delegation(_) => <delegationCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::generator(_) => <generatorCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getBatchOperatorFromId(_) => {
                    <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBatchOperatorId(_) => {
                    <getBatchOperatorIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCheckSignaturesIndices(_) => {
                    <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorState_0(_) => {
                    <getOperatorState_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorState_1(_) => {
                    <getOperatorState_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumBitmapsAtBlockNumber(_) => {
                    <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTaskResponseWindowBlock(_) => {
                    <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => <initializeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::instantSlasher(_) => {
                    <instantSlasherCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestTaskNum(_) => <latestTaskNumCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::raiseAndResolveChallenge(_) => {
                    <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::respondToTask(_) => <respondToTaskCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::serviceManager(_) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setStaleStakesForbidden(_) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::staleStakesForbidden(_) => {
                    <staleStakesForbiddenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::taskNumber(_) => <taskNumberCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::taskSuccesfullyChallenged(_) => {
                    <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::trySignatureAndApkVerification(_) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
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
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<
                IncredibleSquaringTaskManagerCalls,
            >] = &[
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::pause)
                    }
                    pause
                },
                {
                    fn trySignatureAndApkVerification(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerCalls::trySignatureAndApkVerification,
                            )
                    }
                    trySignatureAndApkVerification
                },
                {
                    fn TASK_RESPONSE_WINDOW_BLOCK(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerCalls::TASK_RESPONSE_WINDOW_BLOCK,
                            )
                    }
                    TASK_RESPONSE_WINDOW_BLOCK
                },
                {
                    fn aggregator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <aggregatorCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::aggregator)
                    }
                    aggregator
                },
                {
                    fn allTaskResponses(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::allTaskResponses)
                    }
                    allTaskResponses
                },
                {
                    fn allTaskHashes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <allTaskHashesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::allTaskHashes)
                    }
                    allTaskHashes
                },
                {
                    fn getBatchOperatorId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <getBatchOperatorIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::getBatchOperatorId)
                    }
                    getBatchOperatorId
                },
                {
                    fn getOperatorState_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <getOperatorState_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::getOperatorState_0)
                    }
                    getOperatorState_0
                },
                {
                    fn serviceManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <serviceManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::serviceManager)
                    }
                    serviceManager
                },
                {
                    fn setStaleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::setStaleStakesForbidden)
                    }
                    setStaleStakesForbidden
                },
                {
                    fn getBatchOperatorFromId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::getBatchOperatorFromId)
                    }
                    getBatchOperatorFromId
                },
                {
                    fn getCheckSignaturesIndices(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::getCheckSignaturesIndices)
                    }
                    getCheckSignaturesIndices
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn respondToTask(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <respondToTaskCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::respondToTask)
                    }
                    respondToTask
                },
                {
                    fn getQuorumBitmapsAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerCalls::getQuorumBitmapsAtBlockNumber,
                            )
                    }
                    getQuorumBitmapsAtBlockNumber
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn taskSuccesfullyChallenged(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::taskSuccesfullyChallenged)
                    }
                    taskSuccesfullyChallenged
                },
                {
                    fn blsApkRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::blsApkRegistry)
                    }
                    blsApkRegistry
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn raiseAndResolveChallenge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::raiseAndResolveChallenge)
                    }
                    raiseAndResolveChallenge
                },
                {
                    fn createNewTask(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <createNewTaskCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::createNewTask)
                    }
                    createNewTask
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn checkSignatures(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <checkSignaturesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::checkSignatures)
                    }
                    checkSignatures
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn taskNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <taskNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::taskNumber)
                    }
                    taskNumber
                },
                {
                    fn generator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <generatorCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::generator)
                    }
                    generator
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn latestTaskNum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <latestTaskNumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::latestTaskNum)
                    }
                    latestTaskNum
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn instantSlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <instantSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::instantSlasher)
                    }
                    instantSlasher
                },
                {
                    fn staleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <staleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::staleStakesForbidden)
                    }
                    staleStakesForbidden
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn getOperatorState_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <getOperatorState_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::getOperatorState_1)
                    }
                    getOperatorState_1
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::delegation)
                    }
                    delegation
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn getTaskResponseWindowBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerCalls::getTaskResponseWindowBlock,
                            )
                    }
                    getTaskResponseWindowBlock
                },
                {
                    fn TASK_CHALLENGE_WINDOW_BLOCK(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerCalls::TASK_CHALLENGE_WINDOW_BLOCK,
                            )
                    }
                    TASK_CHALLENGE_WINDOW_BLOCK
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::unpause)
                    }
                    unpause
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::TASK_CHALLENGE_WINDOW_BLOCK(inner) => {
                    <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TASK_RESPONSE_WINDOW_BLOCK(inner) => {
                    <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::aggregator(inner) => {
                    <aggregatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::allTaskHashes(inner) => {
                    <allTaskHashesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allTaskResponses(inner) => {
                    <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::blsApkRegistry(inner) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createNewTask(inner) => {
                    <createNewTaskCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::generator(inner) => {
                    <generatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getBatchOperatorFromId(inner) => {
                    <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBatchOperatorId(inner) => {
                    <getBatchOperatorIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCheckSignaturesIndices(inner) => {
                    <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorState_0(inner) => {
                    <getOperatorState_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorState_1(inner) => {
                    <getOperatorState_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumBitmapsAtBlockNumber(inner) => {
                    <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTaskResponseWindowBlock(inner) => {
                    <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::instantSlasher(inner) => {
                    <instantSlasherCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::latestTaskNum(inner) => {
                    <latestTaskNumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pauseAll(inner) => {
                    <pauseAllCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused_0(inner) => {
                    <paused_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused_1(inner) => {
                    <paused_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pauserRegistry(inner) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::raiseAndResolveChallenge(inner) => {
                    <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::respondToTask(inner) => {
                    <respondToTaskCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::serviceManager(inner) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setStaleStakesForbidden(inner) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::staleStakesForbidden(inner) => {
                    <staleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::taskNumber(inner) => {
                    <taskNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::taskSuccesfullyChallenged(inner) => {
                    <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::trySignatureAndApkVerification(inner) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::TASK_CHALLENGE_WINDOW_BLOCK(inner) => {
                    <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::TASK_RESPONSE_WINDOW_BLOCK(inner) => {
                    <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::aggregator(inner) => {
                    <aggregatorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::allTaskHashes(inner) => {
                    <allTaskHashesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::allTaskResponses(inner) => {
                    <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::blsApkRegistry(inner) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::createNewTask(inner) => {
                    <createNewTaskCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::generator(inner) => {
                    <generatorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getBatchOperatorFromId(inner) => {
                    <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getBatchOperatorId(inner) => {
                    <getBatchOperatorIdCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getCheckSignaturesIndices(inner) => {
                    <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getOperatorState_0(inner) => {
                    <getOperatorState_0Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getOperatorState_1(inner) => {
                    <getOperatorState_1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getQuorumBitmapsAtBlockNumber(inner) => {
                    <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getTaskResponseWindowBlock(inner) => {
                    <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::instantSlasher(inner) => {
                    <instantSlasherCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::latestTaskNum(inner) => {
                    <latestTaskNumCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pauseAll(inner) => {
                    <pauseAllCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused_0(inner) => {
                    <paused_0Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused_1(inner) => {
                    <paused_1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pauserRegistry(inner) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::raiseAndResolveChallenge(inner) => {
                    <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::respondToTask(inner) => {
                    <respondToTaskCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::serviceManager(inner) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setStaleStakesForbidden(inner) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::staleStakesForbidden(inner) => {
                    <staleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::taskNumber(inner) => {
                    <taskNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::taskSuccesfullyChallenged(inner) => {
                    <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::trySignatureAndApkVerification(inner) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`IncredibleSquaringTaskManager`](self) custom errors.
    pub enum IncredibleSquaringTaskManagerErrors {
        CurrentlyPaused(CurrentlyPaused),
        InputAddressZero(InputAddressZero),
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        OnlyPauser(OnlyPauser),
        OnlyUnpauser(OnlyUnpauser),
    }
    #[automatically_derived]
    impl IncredibleSquaringTaskManagerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [132u8, 10u8, 72u8, 213u8],
            [198u8, 29u8, 202u8, 93u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IncredibleSquaringTaskManagerErrors {
        const NAME: &'static str = "IncredibleSquaringTaskManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 5usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNewPausedStatus(_) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyPauser(_) => <OnlyPauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyUnpauser(_) => <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR,
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
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<
                IncredibleSquaringTaskManagerErrors,
            >] = &[
                {
                    fn InputAddressZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`IncredibleSquaringTaskManager`](self) events.
    pub enum IncredibleSquaringTaskManagerEvents {
        Initialized(Initialized),
        NewTaskCreated(NewTaskCreated),
        OwnershipTransferred(OwnershipTransferred),
        Paused(Paused),
        StaleStakesForbiddenUpdate(StaleStakesForbiddenUpdate),
        TaskChallengedSuccessfully(TaskChallengedSuccessfully),
        TaskChallengedUnsuccessfully(TaskChallengedUnsuccessfully),
        TaskCompleted(TaskCompleted),
        TaskResponded(TaskResponded),
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl IncredibleSquaringTaskManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                22u8, 149u8, 184u8, 208u8, 110u8, 200u8, 0u8, 180u8, 97u8, 94u8, 116u8, 92u8,
                251u8, 91u8, 208u8, 12u8, 31u8, 40u8, 117u8, 97u8, 93u8, 66u8, 146u8, 92u8, 59u8,
                90u8, 250u8, 84u8, 59u8, 178u8, 76u8, 72u8,
            ],
            [
                52u8, 156u8, 30u8, 230u8, 14u8, 78u8, 137u8, 114u8, 238u8, 157u8, 186u8, 100u8,
                44u8, 23u8, 116u8, 84u8, 61u8, 92u8, 65u8, 54u8, 135u8, 155u8, 127u8, 76u8, 170u8,
                240u8, 75u8, 248u8, 26u8, 72u8, 122u8, 42u8,
            ],
            [
                53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
            ],
            [
                64u8, 228u8, 237u8, 136u8, 10u8, 41u8, 224u8, 246u8, 221u8, 206u8, 48u8, 116u8,
                87u8, 251u8, 117u8, 205u8, 223u8, 79u8, 238u8, 247u8, 211u8, 236u8, 176u8, 48u8,
                27u8, 253u8, 244u8, 151u8, 106u8, 14u8, 45u8, 252u8,
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
                154u8, 20u8, 79u8, 34u8, 138u8, 147u8, 27u8, 157u8, 13u8, 22u8, 150u8, 251u8,
                205u8, 175u8, 49u8, 11u8, 36u8, 181u8, 210u8, 210u8, 30u8, 121u8, 157u8, 182u8,
                35u8, 252u8, 152u8, 106u8, 15u8, 84u8, 116u8, 48u8,
            ],
            [
                171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8, 188u8,
                152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8, 11u8, 182u8,
                224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
            ],
            [
                194u8, 13u8, 27u8, 176u8, 241u8, 98u8, 54u8, 128u8, 48u8, 107u8, 131u8, 212u8,
                255u8, 75u8, 185u8, 154u8, 43u8, 235u8, 157u8, 134u8, 217u8, 120u8, 50u8, 243u8,
                202u8, 64u8, 253u8, 19u8, 162u8, 157u8, 241u8, 236u8,
            ],
            [
                253u8, 62u8, 38u8, 190u8, 235u8, 89u8, 103u8, 252u8, 90u8, 87u8, 160u8, 68u8,
                105u8, 20u8, 234u8, 188u8, 69u8, 180u8, 170u8, 71u8, 76u8, 103u8, 165u8, 27u8,
                75u8, 81u8, 96u8, 202u8, 198u8, 13u8, 219u8, 5u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for IncredibleSquaringTaskManagerEvents {
        const NAME: &'static str = "IncredibleSquaringTaskManagerEvents";
        const COUNT: usize = 10usize;
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
                Some(<NewTaskCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewTaskCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::NewTaskCreated)
                }
                Some(<OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OwnershipTransferred)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::Paused)
                }
                Some(<StaleStakesForbiddenUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StaleStakesForbiddenUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StaleStakesForbiddenUpdate)
                }
                Some(<TaskChallengedSuccessfully as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskChallengedSuccessfully as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::TaskChallengedSuccessfully)
                }
                Some(
                    <TaskChallengedUnsuccessfully as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <TaskChallengedUnsuccessfully as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data, validate,
                )
                .map(Self::TaskChallengedUnsuccessfully),
                Some(<TaskCompleted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskCompleted as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::TaskCompleted)
                }
                Some(<TaskResponded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskResponded as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::TaskResponded)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::Unpaused)
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
    impl alloy_sol_types::private::IntoLogData for IncredibleSquaringTaskManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewTaskCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::StaleStakesForbiddenUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskChallengedSuccessfully(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskChallengedUnsuccessfully(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskResponded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewTaskCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
                Self::StaleStakesForbiddenUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskChallengedSuccessfully(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskChallengedUnsuccessfully(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskResponded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IncredibleSquaringTaskManager`](self) contract instance.

    See the [wrapper's documentation](`IncredibleSquaringTaskManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IncredibleSquaringTaskManagerInstance<T, P, N> {
        IncredibleSquaringTaskManagerInstance::<T, P, N>::new(address, provider)
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
        _registryCoordinator: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _taskResponseWindowBlock: u32,
        _instantSlasher: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _serviceManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<IncredibleSquaringTaskManagerInstance<T, P, N>>,
    > {
        IncredibleSquaringTaskManagerInstance::<T, P, N>::deploy(
            provider,
            _registryCoordinator,
            _pauserRegistry,
            _taskResponseWindowBlock,
            _instantSlasher,
            _allocationManager,
            _serviceManager,
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
        _registryCoordinator: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _taskResponseWindowBlock: u32,
        _instantSlasher: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _serviceManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        IncredibleSquaringTaskManagerInstance::<T, P, N>::deploy_builder(
            provider,
            _registryCoordinator,
            _pauserRegistry,
            _taskResponseWindowBlock,
            _instantSlasher,
            _allocationManager,
            _serviceManager,
        )
    }
    /**A [`IncredibleSquaringTaskManager`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IncredibleSquaringTaskManager`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IncredibleSquaringTaskManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IncredibleSquaringTaskManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IncredibleSquaringTaskManagerInstance")
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
        > IncredibleSquaringTaskManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IncredibleSquaringTaskManager`](self) contract instance.

        See the [wrapper's documentation](`IncredibleSquaringTaskManagerInstance`) for more details.*/
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
            _registryCoordinator: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _taskResponseWindowBlock: u32,
            _instantSlasher: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _serviceManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<IncredibleSquaringTaskManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _registryCoordinator,
                _pauserRegistry,
                _taskResponseWindowBlock,
                _instantSlasher,
                _allocationManager,
                _serviceManager,
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
            _registryCoordinator: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _taskResponseWindowBlock: u32,
            _instantSlasher: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _serviceManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _registryCoordinator,
                        _pauserRegistry,
                        _taskResponseWindowBlock,
                        _instantSlasher,
                        _allocationManager,
                        _serviceManager,
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
    impl<T, P: ::core::clone::Clone, N> IncredibleSquaringTaskManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IncredibleSquaringTaskManagerInstance<T, P, N> {
            IncredibleSquaringTaskManagerInstance {
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
        > IncredibleSquaringTaskManagerInstance<T, P, N>
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
        ///Creates a new call builder for the [`TASK_CHALLENGE_WINDOW_BLOCK`] function.
        pub fn TASK_CHALLENGE_WINDOW_BLOCK(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, TASK_CHALLENGE_WINDOW_BLOCKCall, N> {
            self.call_builder(&TASK_CHALLENGE_WINDOW_BLOCKCall {})
        }
        ///Creates a new call builder for the [`TASK_RESPONSE_WINDOW_BLOCK`] function.
        pub fn TASK_RESPONSE_WINDOW_BLOCK(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, TASK_RESPONSE_WINDOW_BLOCKCall, N> {
            self.call_builder(&TASK_RESPONSE_WINDOW_BLOCKCall {})
        }
        ///Creates a new call builder for the [`aggregator`] function.
        pub fn aggregator(&self) -> alloy_contract::SolCallBuilder<T, &P, aggregatorCall, N> {
            self.call_builder(&aggregatorCall {})
        }
        ///Creates a new call builder for the [`allTaskHashes`] function.
        pub fn allTaskHashes(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, allTaskHashesCall, N> {
            self.call_builder(&allTaskHashesCall { _0 })
        }
        ///Creates a new call builder for the [`allTaskResponses`] function.
        pub fn allTaskResponses(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, allTaskResponsesCall, N> {
            self.call_builder(&allTaskResponsesCall { _0 })
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
        }
        ///Creates a new call builder for the [`blsApkRegistry`] function.
        pub fn blsApkRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, blsApkRegistryCall, N> {
            self.call_builder(&blsApkRegistryCall {})
        }
        ///Creates a new call builder for the [`checkSignatures`] function.
        pub fn checkSignatures(
            &self,
            msgHash: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
            referenceBlockNumber: u32,
            params: <IBLSSignatureChecker::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkSignaturesCall, N> {
            self.call_builder(&checkSignaturesCall {
                msgHash,
                quorumNumbers,
                referenceBlockNumber,
                params,
            })
        }
        ///Creates a new call builder for the [`createNewTask`] function.
        pub fn createNewTask(
            &self,
            numberToBeSquared: alloy::sol_types::private::primitives::aliases::U256,
            quorumThresholdPercentage: u32,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, createNewTaskCall, N> {
            self.call_builder(&createNewTaskCall {
                numberToBeSquared,
                quorumThresholdPercentage,
                quorumNumbers,
            })
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(&self) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`generator`] function.
        pub fn generator(&self) -> alloy_contract::SolCallBuilder<T, &P, generatorCall, N> {
            self.call_builder(&generatorCall {})
        }
        ///Creates a new call builder for the [`getBatchOperatorFromId`] function.
        pub fn getBatchOperatorFromId(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBatchOperatorFromIdCall, N> {
            self.call_builder(&getBatchOperatorFromIdCall {
                registryCoordinator,
                operatorIds,
            })
        }
        ///Creates a new call builder for the [`getBatchOperatorId`] function.
        pub fn getBatchOperatorId(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBatchOperatorIdCall, N> {
            self.call_builder(&getBatchOperatorIdCall {
                registryCoordinator,
                operators,
            })
        }
        ///Creates a new call builder for the [`getCheckSignaturesIndices`] function.
        pub fn getCheckSignaturesIndices(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            referenceBlockNumber: u32,
            quorumNumbers: alloy::sol_types::private::Bytes,
            nonSignerOperatorIds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCheckSignaturesIndicesCall, N> {
            self.call_builder(&getCheckSignaturesIndicesCall {
                registryCoordinator,
                referenceBlockNumber,
                quorumNumbers,
                nonSignerOperatorIds,
            })
        }
        ///Creates a new call builder for the [`getOperatorState_0`] function.
        pub fn getOperatorState_0(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            quorumNumbers: alloy::sol_types::private::Bytes,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorState_0Call, N> {
            self.call_builder(&getOperatorState_0Call {
                registryCoordinator,
                quorumNumbers,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`getOperatorState_1`] function.
        pub fn getOperatorState_1(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorState_1Call, N> {
            self.call_builder(&getOperatorState_1Call {
                registryCoordinator,
                operatorId,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`getQuorumBitmapsAtBlockNumber`] function.
        pub fn getQuorumBitmapsAtBlockNumber(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapsAtBlockNumberCall, N> {
            self.call_builder(&getQuorumBitmapsAtBlockNumberCall {
                registryCoordinator,
                operatorIds,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`getTaskResponseWindowBlock`] function.
        pub fn getTaskResponseWindowBlock(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTaskResponseWindowBlockCall, N> {
            self.call_builder(&getTaskResponseWindowBlockCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            _aggregator: alloy::sol_types::private::Address,
            _generator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                initialOwner,
                _aggregator,
                _generator,
            })
        }
        ///Creates a new call builder for the [`instantSlasher`] function.
        pub fn instantSlasher(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, instantSlasherCall, N> {
            self.call_builder(&instantSlasherCall {})
        }
        ///Creates a new call builder for the [`latestTaskNum`] function.
        pub fn latestTaskNum(&self) -> alloy_contract::SolCallBuilder<T, &P, latestTaskNumCall, N> {
            self.call_builder(&latestTaskNumCall {})
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauseCall, N> {
            self.call_builder(&pauseCall { newPausedStatus })
        }
        ///Creates a new call builder for the [`pauseAll`] function.
        pub fn pauseAll(&self) -> alloy_contract::SolCallBuilder<T, &P, pauseAllCall, N> {
            self.call_builder(&pauseAllCall {})
        }
        ///Creates a new call builder for the [`paused_0`] function.
        pub fn paused_0(
            &self,
            index: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, paused_0Call, N> {
            self.call_builder(&paused_0Call { index })
        }
        ///Creates a new call builder for the [`paused_1`] function.
        pub fn paused_1(&self) -> alloy_contract::SolCallBuilder<T, &P, paused_1Call, N> {
            self.call_builder(&paused_1Call {})
        }
        ///Creates a new call builder for the [`pauserRegistry`] function.
        pub fn pauserRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauserRegistryCall, N> {
            self.call_builder(&pauserRegistryCall {})
        }
        ///Creates a new call builder for the [`raiseAndResolveChallenge`] function.
        pub fn raiseAndResolveChallenge(
            &self,
            task: <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
            taskResponse: <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
            taskResponseMetadata: <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
            pubkeysOfNonSigningOperators: alloy::sol_types::private::Vec<
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, raiseAndResolveChallengeCall, N> {
            self.call_builder(&raiseAndResolveChallengeCall {
                task,
                taskResponse,
                taskResponseMetadata,
                pubkeysOfNonSigningOperators,
            })
        }
        ///Creates a new call builder for the [`registryCoordinator`] function.
        pub fn registryCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registryCoordinatorCall, N> {
            self.call_builder(&registryCoordinatorCall {})
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`respondToTask`] function.
        pub fn respondToTask(
            &self,
            task: <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
            taskResponse: <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
            nonSignerStakesAndSignature: <IBLSSignatureChecker::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, respondToTaskCall, N> {
            self.call_builder(&respondToTaskCall {
                task,
                taskResponse,
                nonSignerStakesAndSignature,
            })
        }
        ///Creates a new call builder for the [`serviceManager`] function.
        pub fn serviceManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, serviceManagerCall, N> {
            self.call_builder(&serviceManagerCall {})
        }
        ///Creates a new call builder for the [`setStaleStakesForbidden`] function.
        pub fn setStaleStakesForbidden(
            &self,
            value: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, setStaleStakesForbiddenCall, N> {
            self.call_builder(&setStaleStakesForbiddenCall { value })
        }
        ///Creates a new call builder for the [`stakeRegistry`] function.
        pub fn stakeRegistry(&self) -> alloy_contract::SolCallBuilder<T, &P, stakeRegistryCall, N> {
            self.call_builder(&stakeRegistryCall {})
        }
        ///Creates a new call builder for the [`staleStakesForbidden`] function.
        pub fn staleStakesForbidden(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, staleStakesForbiddenCall, N> {
            self.call_builder(&staleStakesForbiddenCall {})
        }
        ///Creates a new call builder for the [`taskNumber`] function.
        pub fn taskNumber(&self) -> alloy_contract::SolCallBuilder<T, &P, taskNumberCall, N> {
            self.call_builder(&taskNumberCall {})
        }
        ///Creates a new call builder for the [`taskSuccesfullyChallenged`] function.
        pub fn taskSuccesfullyChallenged(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, taskSuccesfullyChallengedCall, N> {
            self.call_builder(&taskSuccesfullyChallengedCall { _0 })
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`trySignatureAndApkVerification`] function.
        pub fn trySignatureAndApkVerification(
            &self,
            msgHash: alloy::sol_types::private::FixedBytes<32>,
            apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
            sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, trySignatureAndApkVerificationCall, N> {
            self.call_builder(&trySignatureAndApkVerificationCall {
                msgHash,
                apk,
                apkG2,
                sigma,
            })
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, unpauseCall, N> {
            self.call_builder(&unpauseCall { newPausedStatus })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > IncredibleSquaringTaskManagerInstance<T, P, N>
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
        ///Creates a new event filter for the [`NewTaskCreated`] event.
        pub fn NewTaskCreated_filter(&self) -> alloy_contract::Event<T, &P, NewTaskCreated, N> {
            self.event_filter::<NewTaskCreated>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<T, &P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`StaleStakesForbiddenUpdate`] event.
        pub fn StaleStakesForbiddenUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StaleStakesForbiddenUpdate, N> {
            self.event_filter::<StaleStakesForbiddenUpdate>()
        }
        ///Creates a new event filter for the [`TaskChallengedSuccessfully`] event.
        pub fn TaskChallengedSuccessfully_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TaskChallengedSuccessfully, N> {
            self.event_filter::<TaskChallengedSuccessfully>()
        }
        ///Creates a new event filter for the [`TaskChallengedUnsuccessfully`] event.
        pub fn TaskChallengedUnsuccessfully_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TaskChallengedUnsuccessfully, N> {
            self.event_filter::<TaskChallengedUnsuccessfully>()
        }
        ///Creates a new event filter for the [`TaskCompleted`] event.
        pub fn TaskCompleted_filter(&self) -> alloy_contract::Event<T, &P, TaskCompleted, N> {
            self.event_filter::<TaskCompleted>()
        }
        ///Creates a new event filter for the [`TaskResponded`] event.
        pub fn TaskResponded_filter(&self) -> alloy_contract::Event<T, &P, TaskResponded, N> {
            self.event_filter::<TaskResponded>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
