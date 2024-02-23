// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Support code for the runtime.
//!
//! ## Note on Tuple Traits
//!
//! Many of the traits defined in [`traits`] have auto-implementations on tuples as well. Usually,
//! the tuple is a function of number of pallets in the runtime. By default, the traits are
//! implemented for tuples of up to 64 items.
//
// If you have more pallets in your runtime, or for any other reason need more, enabled `tuples-96`
// or the `tuples-128` complication flag. Note that these features *will increase* the compilation
// of this crate.

#![cfg_attr(not(feature = "std"), no_std)]

/// Export ourself as `frame_support` to make tests happy.
#[doc(hidden)]
extern crate self as frame_support;

/// Private exports that are being used by macros.
///
/// The exports are not stable and should not be relied on.
#[doc(hidden)]
pub mod __private {
	pub use codec;
	pub use frame_metadata as metadata;
	pub use log;
	pub use paste;
	pub use scale_info;
	pub use serde;
	pub use sp_core::{Get, OpaqueMetadata, Void};
	pub use sp_crypto_hashing_proc_macro;
	pub use sp_inherents;
	#[cfg(feature = "std")]
	pub use sp_io::TestExternalities;
	pub use sp_io::{self, hashing, storage::root as storage_root};
	pub use sp_metadata_ir as metadata_ir;
	#[cfg(feature = "std")]
	pub use sp_runtime::{bounded_btree_map, bounded_vec};
	pub use sp_runtime::{
		traits::Dispatchable, DispatchError, RuntimeDebug, StateVersion, TransactionOutcome,
	};
	#[cfg(feature = "std")]
	pub use sp_state_machine::BasicExternalities;
	pub use sp_std;
	pub use sp_tracing;
	pub use tt_call::*;
}

#[macro_use]
pub mod dispatch;
pub mod crypto;
pub mod dispatch_context;
mod hash;
pub mod inherent;
pub mod instances;
pub mod migrations;
pub mod storage;
#[cfg(test)]
mod tests;
pub mod traits;
pub mod weights;
#[doc(hidden)]
pub mod unsigned {
	#[doc(hidden)]
	pub use crate::sp_runtime::traits::ValidateUnsigned;
	#[doc(hidden)]
	pub use crate::sp_runtime::transaction_validity::{
		TransactionSource, TransactionValidity, TransactionValidityError, UnknownTransaction,
	};
}

#[cfg(any(feature = "std", feature = "runtime-benchmarks", feature = "try-runtime", test))]
pub use self::storage::storage_noop_guard::StorageNoopGuard;
pub use self::{
	dispatch::{Callable, Parameter},
	hash::{
		Blake2_128, Blake2_128Concat, Blake2_256, Hashable, Identity, ReversibleStorageHasher,
		StorageHasher, Twox128, Twox256, Twox64Concat,
	},
	storage::{
		bounded_btree_map::BoundedBTreeMap,
		bounded_btree_set::BoundedBTreeSet,
		bounded_vec::{BoundedSlice, BoundedVec},
		migration,
		weak_bounded_vec::WeakBoundedVec,
		IterableStorageDoubleMap, IterableStorageMap, IterableStorageNMap, StorageDoubleMap,
		StorageMap, StorageNMap, StoragePrefixedMap, StorageValue,
	},
};
pub use sp_runtime::{
	self, print, traits::Printable, ConsensusEngineId, MAX_MODULE_ERROR_ENCODED_SIZE,
};

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::TypeId;

/// A unified log target for support operations.
pub const LOG_TARGET: &str = "runtime::frame-support";

/// A type that cannot be instantiated.
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
pub enum Never {}

/// A pallet identifier. These are per pallet and should be stored in a registry somewhere.
#[derive(Clone, Copy, Eq, PartialEq, Encode, Decode, TypeInfo)]
pub struct PalletId(pub [u8; 8]);

impl TypeId for PalletId {
	const TYPE_ID: [u8; 4] = *b"modl";
}

/// Generate a [`#[pallet::storage]`](pallet_macros::storage) alias outside of a pallet.
///
/// This storage alias works similarly to the [`#[pallet::storage]`](pallet_macros::storage)
/// attribute macro. It supports [`StorageValue`](storage::types::StorageValue),
/// [`StorageMap`](storage::types::StorageMap),
/// [`StorageDoubleMap`](storage::types::StorageDoubleMap) and
/// [`StorageNMap`](storage::types::StorageNMap). The main difference to the normal
/// [`#[pallet::storage]`](pallet_macros::storage) is the flexibility around declaring the
/// storage prefix to use. The storage prefix determines where to find the value in the
/// storage. [`#[pallet::storage]`](pallet_macros::storage) uses the name of the pallet as
/// declared in [`construct_runtime!`].
///
/// The flexibility around declaring the storage prefix makes this macro very useful for
/// writing migrations etc.
///
/// # Examples
///
/// There are different ways to declare the `prefix` to use. The `prefix` type can either be
/// declared explicetly by passing it to the macro as an attribute or by letting the macro
/// guess on what the `prefix` type is. The `prefix` is always passed as the first generic
/// argument to the type declaration. When using [`#[pallet::storage]`](pallet_macros::storage)
/// this first generic argument is always `_`. Besides declaring the `prefix`, the rest of the
/// type declaration works as with [`#[pallet::storage]`](pallet_macros::storage).
///
/// 1. Use the `verbatim` prefix type. This prefix type uses the given identifier as the
/// `prefix`:
#[doc = docify::embed!("src/tests/storage_alias.rs", verbatim_attribute)]
///
/// 2. Use the `pallet_name` prefix type. This prefix type uses the name of the pallet as
/// configured in    [`construct_runtime!`] as the `prefix`:
#[doc = docify::embed!("src/tests/storage_alias.rs", pallet_name_attribute)]
/// It requires that the given prefix type implements
/// [`PalletInfoAccess`](traits::PalletInfoAccess) (which is always the case for FRAME pallet
/// structs). In the example above, `Pallet<T>` is the prefix type.
///
/// 3. Use the `dynamic` prefix type. This prefix type calls [`Get::get()`](traits::Get::get)
///    to get the `prefix`:
#[doc = docify::embed!("src/tests/storage_alias.rs", dynamic_attribute)]
/// It requires that the given prefix type implements [`Get<'static str>`](traits::Get).
///
/// 4. Let the macro "guess" what kind of prefix type to use. This only supports verbatim or
///    pallet name. The macro uses the presence of generic arguments to the prefix type as an
///    indication that it should use the pallet name as the `prefix`:
#[doc = docify::embed!("src/tests/storage_alias.rs", storage_alias_guess)]
pub use frame_support_procedural::storage_alias;

pub use frame_support_procedural::derive_impl;

/// Experimental macros for defining dynamic params that can be used in pallet configs.
#[cfg(feature = "experimental")]
pub mod dynamic_params {
	pub use frame_support_procedural::{
		dynamic_aggregated_params_internal, dynamic_pallet_params, dynamic_params,
	};
}

/// Create new implementations of the [`Get`](crate::traits::Get) trait.
///
/// The so-called parameter type can be created in four different ways:
///
/// - Using `const` to create a parameter type that provides a `const` getter. It is required that
///   the `value` is const.
///
/// - Declare the parameter type without `const` to have more freedom when creating the value.
///
/// - Using `storage` to create a storage parameter type. This type is special as it tries to load
///   the value from the storage under a fixed key. If the value could not be found in the storage,
///   the given default value will be returned. It is required that the value implements
///   [`Encode`](codec::Encode) and [`Decode`](codec::Decode). The key for looking up the value in
///   the storage is built using the following formula:
///
///   `twox_128(":" ++ NAME ++ ":")` where `NAME` is the name that is passed as type name.
///
/// - Using `static` to create a static parameter type. Its value is being provided by a static
///   variable with the equivalent name in `UPPER_SNAKE_CASE`. An additional `set` function is
///   provided in this case to alter the static variable. **This is intended for testing ONLY and is
///   ONLY available when `std` is enabled.**
///
/// # Examples
///
/// ```
/// # use frame_support::traits::Get;
/// # use frame_support::parameter_types;
/// // This function cannot be used in a const context.
/// fn non_const_expression() -> u64 { 99 }
///
/// const FIXED_VALUE: u64 = 10;
/// parameter_types! {
///    pub const Argument: u64 = 42 + FIXED_VALUE;
///    /// Visibility of the type is optional
///    OtherArgument: u64 = non_const_expression();
///    pub storage StorageArgument: u64 = 5;
///    pub static StaticArgument: u32 = 7;
/// }
///
/// trait Config {
///    type Parameter: Get<u64>;
///    type OtherParameter: Get<u64>;
///    type StorageParameter: Get<u64>;
///    type StaticParameter: Get<u32>;
/// }
///
/// struct Runtime;
/// impl Config for Runtime {
///    type Parameter = Argument;
///    type OtherParameter = OtherArgument;
///    type StorageParameter = StorageArgument;
///    type StaticParameter = StaticArgument;
/// }
///
/// // In testing, `StaticArgument` can be altered later: `StaticArgument::set(8)`.
/// ```
///
/// # Invalid example:
///
/// ```compile_fail
/// # use frame_support::traits::Get;
/// # use frame_support::parameter_types;
/// // This function cannot be used in a const context.
/// fn non_const_expression() -> u64 { 99 }
///
/// parameter_types! {
///    pub const Argument: u64 = non_const_expression();
/// }
/// ```
#[macro_export]
macro_rules! parameter_types {
	(
		$( #[ $attr:meta ] )*
		$vis:vis const $name:ident $(< $($ty_params:ident),* >)?: $type:ty = $value:expr;
		$( $rest:tt )*
	) => (
		$( #[ $attr ] )*
		$vis struct $name $(
			< $($ty_params),* >( $($crate::__private::sp_std::marker::PhantomData<$ty_params>),* )
		)?;
		$crate::parameter_types!(IMPL_CONST $name , $type , $value $( $(, $ty_params)* )?);
		$crate::parameter_types!( $( $rest )* );
	);
	(
		$( #[ $attr:meta ] )*
		$vis:vis $name:ident $(< $($ty_params:ident),* >)?: $type:ty = $value:expr;
		$( $rest:tt )*
	) => (
		$( #[ $attr ] )*
		$vis struct $name $(
			< $($ty_params),* >( $($crate::__private::sp_std::marker::PhantomData<$ty_params>),* )
		)?;
		$crate::parameter_types!(IMPL $name, $type, $value $( $(, $ty_params)* )?);
		$crate::parameter_types!( $( $rest )* );
	);
	(
		$( #[ $attr:meta ] )*
		$vis:vis storage $name:ident $(< $($ty_params:ident),* >)?: $type:ty = $value:expr;
		$( $rest:tt )*
	) => (
		$( #[ $attr ] )*
		$vis struct $name $(
			< $($ty_params),* >( $($crate::__private::sp_std::marker::PhantomData<$ty_params>),* )
		)?;
		$crate::parameter_types!(IMPL_STORAGE $name, $type, $value $( $(, $ty_params)* )?);
		$crate::parameter_types!( $( $rest )* );
	);
	() => ();
	(IMPL_CONST $name:ident, $type:ty, $value:expr $(, $ty_params:ident)*) => {
		impl< $($ty_params),* > $name< $($ty_params),* > {
			/// Returns the value of this parameter type.
			pub const fn get() -> $type {
				$value
			}
		}

		impl<_I: From<$type> $(, $ty_params)*> $crate::traits::Get<_I> for $name< $($ty_params),* > {
			fn get() -> _I {
				_I::from(Self::get())
			}
		}

		impl< $($ty_params),* > $crate::traits::TypedGet for $name< $($ty_params),* > {
			type Type = $type;
			fn get() -> $type {
				Self::get()
			}
		}
	};
	(IMPL $name:ident, $type:ty, $value:expr $(, $ty_params:ident)*) => {
		impl< $($ty_params),* > $name< $($ty_params),* > {
			/// Returns the value of this parameter type.
			pub fn get() -> $type {
				$value
			}
		}

		impl<_I: From<$type>, $(, $ty_params)*> $crate::traits::Get<_I> for $name< $($ty_params),* > {
			fn get() -> _I {
				_I::from(Self::get())
			}
		}

		impl< $($ty_params),* > $crate::traits::TypedGet for $name< $($ty_params),* > {
			type Type = $type;
			fn get() -> $type {
				Self::get()
			}
		}
	};
	(IMPL_STORAGE $name:ident, $type:ty, $value:expr $(, $ty_params:ident)*) => {
		#[allow(unused)]
		impl< $($ty_params),* > $name< $($ty_params),* > {
			/// Returns the key for this parameter type.
			pub fn key() -> [u8; 16] {
				$crate::__private::sp_crypto_hashing_proc_macro::twox_128!(b":", $name, b":")
			}

			/// Set the value of this parameter type in the storage.
			///
			/// This needs to be executed in an externalities provided environment.
			pub fn set(value: &$type) {
				$crate::storage::unhashed::put(&Self::key(), value);
			}

			/// Returns the value of this parameter type.
			///
			/// This needs to be executed in an externalities provided environment.
			#[allow(unused)]
			pub fn get() -> $type {
				$crate::storage::unhashed::get(&Self::key()).unwrap_or_else(|| $value)
			}
		}

		impl<_I: From<$type> $(, $ty_params)*> $crate::traits::Get<_I> for $name< $($ty_params),* > {
			fn get() -> _I {
				_I::from(Self::get())
			}
		}

		impl< $($ty_params),* > $crate::traits::TypedGet for $name< $($ty_params),* > {
			type Type = $type;
			fn get() -> $type {
				Self::get()
			}
		}
	};
	(
		$( #[ $attr:meta ] )*
		$vis:vis static $name:ident: $type:ty = $value:expr;
		$( $rest:tt )*
	) => (
		$crate::parameter_types_impl_thread_local!(
			$( #[ $attr ] )*
			$vis static $name: $type = $value;
		);
		$crate::parameter_types!( $( $rest )* );
	);
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! parameter_types_impl_thread_local {
	( $( $any:tt )* ) => {
		compile_error!("static parameter types is only available in std and for testing.");
	};
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! parameter_types_impl_thread_local {
	(
		$(
			$( #[ $attr:meta ] )*
			$vis:vis static $name:ident: $type:ty = $value:expr;
		)*
	) => {
		$crate::parameter_types_impl_thread_local!(
			IMPL_THREAD_LOCAL $( $vis, $name, $type, $value, )*
		);
		$crate::__private::paste::item! {
			$crate::parameter_types!(
				$(
					$( #[ $attr ] )*
					$vis $name: $type = [<$name:snake:upper>].with(|v| v.borrow().clone());
				)*
			);
			$(
				impl $name {
					/// Set the internal value.
					pub fn set(t: $type) {
						[<$name:snake:upper>].with(|v| *v.borrow_mut() = t);
					}

					/// Mutate the internal value in place.
					#[allow(unused)]
					pub fn mutate<R, F: FnOnce(&mut $type) -> R>(mutate: F) -> R{
						let mut current = Self::get();
						let result = mutate(&mut current);
						Self::set(current);
						result
					}

					/// Get current value and replace with initial value of the parameter type.
					#[allow(unused)]
					pub fn take() -> $type {
						let current = Self::get();
						Self::set($value);
						current
					}
				}
			)*
		}
	};
	(IMPL_THREAD_LOCAL $( $vis:vis, $name:ident, $type:ty, $value:expr, )* ) => {
		$crate::__private::paste::item! {
			thread_local! {
				$(
					pub static [<$name:snake:upper>]: std::cell::RefCell<$type> =
						std::cell::RefCell::new($value);
				)*
			}
		}
	};
}

/// Macro for easily creating a new implementation of both the `Get` and `Contains` traits. Use
/// exactly as with `parameter_types`, only the type must be `Ord`.
#[macro_export]
macro_rules! ord_parameter_types {
	(
		$( #[ $attr:meta ] )*
		$vis:vis const $name:ident: $type:ty = $value:expr;
		$( $rest:tt )*
	) => (
		$( #[ $attr ] )*
		$vis struct $name;
		$crate::parameter_types!{IMPL $name , $type , $value}
		$crate::ord_parameter_types!{IMPL $name , $type , $value}
		$crate::ord_parameter_types!{ $( $rest )* }
	);
	() => ();
	(IMPL $name:ident , $type:ty , $value:expr) => {
		impl $crate::traits::SortedMembers<$type> for $name {
			fn contains(t: &$type) -> bool { &$value == t }
			fn sorted_members() -> $crate::__private::sp_std::prelude::Vec<$type> { vec![$value] }
			fn count() -> usize { 1 }
			#[cfg(feature = "runtime-benchmarks")]
			fn add(_: &$type) {}
		}
		impl $crate::traits::Contains<$type> for $name {
			fn contains(t: &$type) -> bool { &$value == t }
		}
	}
}

/// Print out a formatted message.
///
/// # Example
///
/// ```
/// frame_support::runtime_print!("my value is {}", 3);
/// ```
#[macro_export]
macro_rules! runtime_print {
	($($arg:tt)+) => {
		{
			use core::fmt::Write;
			let mut w = $crate::__private::sp_std::Writer::default();
			let _ = core::write!(&mut w, $($arg)+);
			$crate::__private::sp_io::misc::print_utf8(&w.inner())
		}
	}
}

/// Print out the debuggable type.
pub fn debug(data: &impl sp_std::fmt::Debug) {
	runtime_print!("{:?}", data);
}

#[doc(inline)]
pub use frame_support_procedural::{
	construct_runtime, match_and_insert, transactional, PalletError, RuntimeDebugNoBound,
};

#[doc(hidden)]
pub use frame_support_procedural::{__create_tt_macro, __generate_dummy_part_checker};

/// Derive [`Clone`] but do not bound any generic.
///
/// This is useful for type generic over runtime:
/// ```
/// # use frame_support::CloneNoBound;
/// trait Config {
/// 		type C: Clone;
/// }
///
/// // Foo implements [`Clone`] because `C` bounds [`Clone`].
/// // Otherwise compilation will fail with an output telling `c` doesn't implement [`Clone`].
/// #[derive(CloneNoBound)]
/// struct Foo<T: Config> {
/// 		c: T::C,
/// }
/// ```
pub use frame_support_procedural::CloneNoBound;

/// Derive [`Eq`] but do not bound any generic.
///
/// This is useful for type generic over runtime:
/// ```
/// # use frame_support::{EqNoBound, PartialEqNoBound};
/// trait Config {
/// 		type C: Eq;
/// }
///
/// // Foo implements [`Eq`] because `C` bounds [`Eq`].
/// // Otherwise compilation will fail with an output telling `c` doesn't implement [`Eq`].
/// #[derive(PartialEqNoBound, EqNoBound)]
/// struct Foo<T: Config> {
/// 		c: T::C,
/// }
/// ```
pub use frame_support_procedural::EqNoBound;

/// Derive [`PartialEq`] but do not bound any generic.
///
/// This is useful for type generic over runtime:
/// ```
/// # use frame_support::PartialEqNoBound;
/// trait Config {
/// 		type C: PartialEq;
/// }
///
/// // Foo implements [`PartialEq`] because `C` bounds [`PartialEq`].
/// // Otherwise compilation will fail with an output telling `c` doesn't implement [`PartialEq`].
/// #[derive(PartialEqNoBound)]
/// struct Foo<T: Config> {
/// 		c: T::C,
/// }
/// ```
pub use frame_support_procedural::PartialEqNoBound;

/// Derive [`Ord`] but do not bound any generic.
///
/// This is useful for type generic over runtime:
/// ```
/// # use frame_support::{OrdNoBound, PartialOrdNoBound, EqNoBound, PartialEqNoBound};
/// trait Config {
/// 		type C: Ord;
/// }
///
/// // Foo implements [`Ord`] because `C` bounds [`Ord`].
/// // Otherwise compilation will fail with an output telling `c` doesn't implement [`Ord`].
/// #[derive(EqNoBound, OrdNoBound, PartialEqNoBound, PartialOrdNoBound)]
/// struct Foo<T: Config> {
/// 		c: T::C,
/// }
/// ```
pub use frame_support_procedural::OrdNoBound;

/// Derive [`PartialOrd`] but do not bound any generic.
///
/// This is useful for type generic over runtime:
/// ```
/// # use frame_support::{OrdNoBound, PartialOrdNoBound, EqNoBound, PartialEqNoBound};
/// trait Config {
/// 		type C: PartialOrd;
/// }
///
/// // Foo implements [`PartialOrd`] because `C` bounds [`PartialOrd`].
/// // Otherwise compilation will fail with an output telling `c` doesn't implement [`PartialOrd`].
/// #[derive(PartialOrdNoBound, PartialEqNoBound, EqNoBound)]
/// struct Foo<T: Config> {
/// 		c: T::C,
/// }
/// ```
pub use frame_support_procedural::PartialOrdNoBound;

/// Derive [`Debug`] but do not bound any generic.
///
/// This is useful for type generic over runtime:
/// ```
/// # use frame_support::DebugNoBound;
/// # use core::fmt::Debug;
/// trait Config {
/// 		type C: Debug;
/// }
///
/// // Foo implements [`Debug`] because `C` bounds [`Debug`].
/// // Otherwise compilation will fail with an output telling `c` doesn't implement [`Debug`].
/// #[derive(DebugNoBound)]
/// struct Foo<T: Config> {
/// 		c: T::C,
/// }
/// ```
pub use frame_support_procedural::DebugNoBound;

/// Derive [`Default`] but do not bound any generic.
///
/// This is useful for type generic over runtime:
/// ```
/// # use frame_support::DefaultNoBound;
/// # use core::default::Default;
/// trait Config {
/// 	type C: Default;
/// }
///
/// // Foo implements [`Default`] because `C` bounds [`Default`].
/// // Otherwise compilation will fail with an output telling `c` doesn't implement [`Default`].
/// #[derive(DefaultNoBound)]
/// struct Foo<T: Config> {
/// 	c: T::C,
/// }
///
/// // Also works with enums, by specifying the default with #[default]:
/// #[derive(DefaultNoBound)]
/// enum Bar<T: Config> {
/// 	// Bar will implement Default as long as all of the types within Baz also implement default.
/// 	#[default]
/// 	Baz(T::C),
/// 	Quxx,
/// }
/// ```
pub use frame_support_procedural::DefaultNoBound;

/// Assert the annotated function is executed within a storage transaction.
///
/// The assertion is enabled for native execution and when `debug_assertions` are enabled.
///
/// # Example
///
/// ```
/// # use frame_support::{
/// # 	require_transactional, transactional, dispatch::DispatchResult
/// # };
///
/// #[require_transactional]
/// fn update_all(value: u32) -> DispatchResult {
/// 	// Update multiple storages.
/// 	// Return `Err` to indicate should revert.
/// 	Ok(())
/// }
///
/// #[transactional]
/// fn safe_update(value: u32) -> DispatchResult {
/// 	// This is safe
/// 	update_all(value)
/// }
///
/// fn unsafe_update(value: u32) -> DispatchResult {
/// 	// this may panic if unsafe_update is not called within a storage transaction
/// 	update_all(value)
/// }
/// ```
pub use frame_support_procedural::require_transactional;

/// Convert the current crate version into a [`CrateVersion`](crate::traits::CrateVersion).
///
/// It uses the `CARGO_PKG_VERSION_MAJOR`, `CARGO_PKG_VERSION_MINOR` and
/// `CARGO_PKG_VERSION_PATCH` environment variables to fetch the crate version.
/// This means that the [`CrateVersion`](crate::traits::CrateVersion)
/// object will correspond to the version of the crate the macro is called in!
///
/// # Example
///
/// ```
/// # use frame_support::{traits::CrateVersion, crate_to_crate_version};
/// const Version: CrateVersion = crate_to_crate_version!();
/// ```
pub use frame_support_procedural::crate_to_crate_version;

/// Return Err of the expression: `return Err($expression);`.
///
/// Used as `fail!(expression)`.
#[macro_export]
macro_rules! fail {
	( $y:expr ) => {{
		return Err($y.into())
	}};
}

/// Evaluate `$x:expr` and if not true return `Err($y:expr)`.
///
/// Used as `ensure!(expression_to_ensure, expression_to_return_on_false)`.
#[macro_export]
macro_rules! ensure {
	( $x:expr, $y:expr $(,)? ) => {{
		if !$x {
			$crate::fail!($y);
		}
	}};
}

/// Evaluate an expression, assert it returns an expected `Err` value and that
/// runtime storage has not been mutated (i.e. expression is a no-operation).
///
/// Used as `assert_noop(expression_to_assert, expected_error_expression)`.
#[macro_export]
macro_rules! assert_noop {
	(
		$x:expr,
		$y:expr $(,)?
	) => {
		let h = $crate::__private::storage_root($crate::__private::StateVersion::V1);
		$crate::assert_err!($x, $y);
		assert_eq!(
			h,
			$crate::__private::storage_root($crate::__private::StateVersion::V1),
			"storage has been mutated"
		);
	};
}

/// Evaluate any expression and assert that runtime storage has not been mutated
/// (i.e. expression is a storage no-operation).
///
/// Used as `assert_storage_noop(expression_to_assert)`.
#[macro_export]
macro_rules! assert_storage_noop {
	(
		$x:expr
	) => {
		let h = $crate::__private::storage_root($crate::__private::StateVersion::V1);
		$x;
		assert_eq!(h, $crate::__private::storage_root($crate::__private::StateVersion::V1));
	};
}

/// Assert an expression returns an error specified.
///
/// Used as `assert_err!(expression_to_assert, expected_error_expression)`
#[macro_export]
macro_rules! assert_err {
	( $x:expr , $y:expr $(,)? ) => {
		assert_eq!($x, Err($y.into()));
	};
}

/// Assert an expression returns an error specified.
///
/// This can be used on `DispatchResultWithPostInfo` when the post info should
/// be ignored.
#[macro_export]
macro_rules! assert_err_ignore_postinfo {
	( $x:expr , $y:expr $(,)? ) => {
		$crate::assert_err!($x.map(|_| ()).map_err(|e| e.error), $y);
	};
}

/// Assert an expression returns error with the given weight.
#[macro_export]
macro_rules! assert_err_with_weight {
	($call:expr, $err:expr, $weight:expr $(,)? ) => {
		if let Err(dispatch_err_with_post) = $call {
			$crate::assert_err!($call.map(|_| ()).map_err(|e| e.error), $err);
			assert_eq!(dispatch_err_with_post.post_info.actual_weight, $weight);
		} else {
			::core::panic!("expected Err(_), got Ok(_).")
		}
	};
}

/// Panic if an expression doesn't evaluate to `Ok`.
///
/// Used as `assert_ok!(expression_to_assert, expected_ok_expression)`,
/// or `assert_ok!(expression_to_assert)` which would assert against `Ok(())`.
#[macro_export]
macro_rules! assert_ok {
	( $x:expr $(,)? ) => {
		let is = $x;
		match is {
			Ok(_) => (),
			_ => assert!(false, "Expected Ok(_). Got {:#?}", is),
		}
	};
	( $x:expr, $y:expr $(,)? ) => {
		assert_eq!($x, Ok($y));
	};
}

/// Assert that the maximum encoding size does not exceed the value defined in
/// [`MAX_MODULE_ERROR_ENCODED_SIZE`] during compilation.
///
/// This macro is intended to be used in conjunction with `tt_call!`.
#[macro_export]
macro_rules! assert_error_encoded_size {
	{
		path = [{ $($path:ident)::+ }]
		runtime = [{ $runtime:ident }]
		assert_message = [{ $assert_message:literal }]
		error = [{ $error:ident }]
	} => {
		const _: () = assert!(
			<
				$($path::)+$error<$runtime> as $crate::traits::PalletError
			>::MAX_ENCODED_SIZE <= $crate::MAX_MODULE_ERROR_ENCODED_SIZE,
			$assert_message
		);
	};
	{
		path = [{ $($path:ident)::+ }]
		runtime = [{ $runtime:ident }]
		assert_message = [{ $assert_message:literal }]
	} => {};
}

/// Do something hypothetically by rolling back any changes afterwards.
///
/// Returns the original result of the closure.
#[macro_export]
#[cfg(feature = "experimental")]
macro_rules! hypothetically {
	( $e:expr ) => {
		$crate::storage::transactional::with_transaction(|| -> $crate::__private::TransactionOutcome<Result<_, $crate::__private::DispatchError>> {
			$crate::__private::TransactionOutcome::Rollback(Ok($e))
		},
		).expect("Always returning Ok; qed")
	};
}

/// Assert something to be *hypothetically* `Ok`, without actually committing it.
///
/// Reverts any storage changes made by the closure.
#[macro_export]
#[cfg(feature = "experimental")]
macro_rules! hypothetically_ok {
	($e:expr $(, $args:expr)* $(,)?) => {
		$crate::assert_ok!($crate::hypothetically!($e) $(, $args)*);
	};
}

#[doc(hidden)]
pub use serde::{Deserialize, Serialize};

#[doc(hidden)]
#[cfg(not(no_std))]
pub use macro_magic;

/// Prelude to be used for pallet testing, for ease of use.
#[cfg(feature = "std")]
pub mod testing_prelude {
	pub use super::{
		assert_err, assert_err_ignore_postinfo, assert_err_with_weight, assert_error_encoded_size,
		assert_noop, assert_ok, assert_storage_noop, parameter_types, traits::Get,
	};
	pub use sp_arithmetic::assert_eq_error_rate;
	pub use sp_runtime::{bounded_btree_map, bounded_vec};
}

/// Prelude to be used alongside pallet macro, for ease of use.
pub mod pallet_prelude {
	pub use crate::{
		defensive, defensive_assert,
		dispatch::{DispatchClass, DispatchResult, DispatchResultWithPostInfo, Parameter, Pays},
		ensure,
		inherent::{InherentData, InherentIdentifier, ProvideInherent},
		storage,
		storage::{
			bounded_btree_map::BoundedBTreeMap,
			bounded_btree_set::BoundedBTreeSet,
			bounded_vec::BoundedVec,
			types::{
				CountedStorageMap, CountedStorageNMap, Key as NMapKey, OptionQuery, ResultQuery,
				StorageDoubleMap, StorageMap, StorageNMap, StorageValue, ValueQuery,
			},
			weak_bounded_vec::WeakBoundedVec,
			StorageList,
		},
		traits::{
			BuildGenesisConfig, ConstU32, EnsureOrigin, Get, GetDefault, GetStorageVersion, Hooks,
			IsType, PalletInfoAccess, StorageInfoTrait, StorageVersion, Task, TypedGet,
		},
		Blake2_128, Blake2_128Concat, Blake2_256, CloneNoBound, DebugNoBound, EqNoBound, Identity,
		PartialEqNoBound, RuntimeDebugNoBound, Twox128, Twox256, Twox64Concat,
	};
	pub use codec::{Decode, Encode, MaxEncodedLen};
	pub use frame_support::pallet_macros::*;
	/// The optional attribute `#[inject_runtime_type]` can be attached to `RuntimeCall`,
	/// `RuntimeEvent`, `RuntimeOrigin` or `PalletInfo` in an impl statement that has
	/// `#[register_default_impl]` attached to indicate that this item is generated by
	/// `construct_runtime`.
	///
	/// Attaching this attribute to such an item ensures that the combined impl generated via
	/// [`#[derive_impl(..)]`](`macro@super::derive_impl`)  will use the correct type
	/// auto-generated by `construct_runtime!`.
	#[doc = docify::embed!("src/tests/inject_runtime_type.rs", derive_impl_works_with_runtime_type_injection)]
	///
	/// However, if `no_aggregated_types` is specified while using
	/// `[`#[derive_impl(..)]`](`macro@super::derive_impl`)`, then these items are attached
	/// verbatim to the combined impl.
	#[doc = docify::embed!("src/tests/inject_runtime_type.rs", derive_impl_works_with_no_aggregated_types)]
	pub use frame_support_procedural::inject_runtime_type;
	pub use frame_support_procedural::register_default_impl;
	pub use scale_info::TypeInfo;
	pub use sp_inherents::MakeFatalError;
	pub use sp_runtime::{
		traits::{MaybeSerializeDeserialize, Member, ValidateUnsigned},
		transaction_validity::{
			InvalidTransaction, TransactionLongevity, TransactionPriority, TransactionSource,
			TransactionTag, TransactionValidity, TransactionValidityError, UnknownTransaction,
			ValidTransaction,
		},
		DispatchError, RuntimeDebug, MAX_MODULE_ERROR_ENCODED_SIZE,
	};
	pub use sp_std::marker::PhantomData;
	pub use sp_weights::Weight;
}

/// The `pallet` attribute macro defines a pallet that can be used with
/// [`construct_runtime!`]. It must be attached to a module named `pallet` as follows:
///
/// ```ignore
/// #[pallet]
/// pub mod pallet {
/// 	...
/// }
/// ```
///
/// Note that various types can be automatically imported using
/// [`frame_support::pallet_prelude`] and `frame_system::pallet_prelude`:
///
/// ```ignore
/// #[pallet]
/// pub mod pallet {
/// 	use frame_support::pallet_prelude::*;
/// 	use frame_system::pallet_prelude::*;
/// 	...
/// }
/// ```
///
/// # pallet::* Attributes
///
/// The `pallet` macro will parse any items within your `pallet` module that are annotated with
/// `#[pallet::*]` attributes. Some of these attributes are mandatory and some are optional,
/// and they can attach to different types of items within your pallet depending on the
/// attribute in question. The full list of `#[pallet::*]` attributes is shown below in the
/// order in which they are mentioned in this document:
///
/// * [`pallet::pallet`](#pallet-struct-placeholder-palletpallet-mandatory)
/// * [`pallet::config`](#config-trait-palletconfig-mandatory)
/// * [`pallet::constant`](#palletconstant)
/// * [`pallet::disable_frame_system_supertrait_check`](#disable_supertrait_check)
/// * [`pallet::generate_store($vis trait Store)`](#palletgenerate_storevis-trait-store)
/// * [`pallet::storage_version`](#palletstorage_version)
/// * [`pallet::hooks`](#hooks-pallethooks-optional)
/// * [`pallet::call`](#call-palletcall-optional)
/// * [`pallet::weight($expr)`](#palletweightexpr)
/// * [`pallet::compact`](#palletcompact-some_arg-some_type)
/// * [`pallet::call_index($idx)`](#palletcall_indexidx)
/// * [`pallet::extra_constants`](#extra-constants-palletextra_constants-optional)
/// * [`pallet::error`](#error-palleterror-optional)
/// * [`pallet::event`](#event-palletevent-optional)
/// * [`pallet::generate_deposit($visibility fn
///   deposit_event)`](#palletgenerate_depositvisibility-fn-deposit_event)
/// * [`pallet::storage`](#storage-palletstorage-optional)
/// * [`pallet::getter(fn $my_getter_fn_name)`](#palletgetterfn-my_getter_fn_name-optional)
/// * [`pallet::storage_prefix = "SomeName"`](#palletstorage_prefix--somename-optional)
/// * [`pallet::unbounded`](#palletunbounded-optional)
/// * [`pallet::whitelist_storage`](#palletwhitelist_storage-optional)
/// * [`pallet::disable_try_decode_storage`](#palletdisable_try_decode_storage-optional)
/// * [`cfg(..)`](#cfg-for-storage) (on storage items)
/// * [`pallet::type_value`](#type-value-pallettype_value-optional)
/// * [`pallet::genesis_config`](#genesis-config-palletgenesis_config-optional)
/// * [`pallet::genesis_build`](#genesis-build-palletgenesis_build-optional)
/// * [`pallet::inherent`](#inherent-palletinherent-optional)
/// * [`pallet::validate_unsigned`](#validate-unsigned-palletvalidate_unsigned-optional)
/// * [`pallet::origin`](#origin-palletorigin-optional)
/// * [`pallet::composite_enum`](#composite-enum-palletcomposite_enum-optional)
///
/// Note that at compile-time, the `#[pallet]` macro will analyze and expand all of these
/// attributes, ultimately removing their AST nodes before they can be parsed as real
/// attribute macro calls. This means that technically we do not need attribute macro
/// definitions for any of these attributes, however, for consistency and discoverability
/// reasons, we still maintain stub attribute macro definitions for all of these attributes in
/// the [`pallet_macros`] module which is automatically included in all pallets as part of the
/// pallet prelude. The actual "work" for all of these attribute macros can be found in the
/// macro expansion for `#[pallet]`.
///
/// Also note that in this document, pallet attributes are explained using the syntax of
/// non-instantiable pallets. For an example of an instantiable pallet, see [this
/// example](#example-of-an-instantiable-pallet).
///
/// # Dev Mode (`#[pallet(dev_mode)]`)
///
/// Specifying the argument `dev_mode` on the `#[pallet]` or `#[frame_support::pallet]`
/// attribute attached to your pallet module will allow you to enable dev mode for a pallet.
/// The aim of dev mode is to loosen some of the restrictions and requirements placed on
/// production pallets for easy tinkering and development. Dev mode pallets should not be used
/// in production. Enabling dev mode has the following effects:
///
/// * Weights no longer need to be specified on every `#[pallet::call]` declaration. By
///   default, dev mode pallets will assume a weight of zero (`0`) if a weight is not
///   specified. This is equivalent to specifying `#[weight(0)]` on all calls that do not
///   specify a weight.
/// * Call indices no longer need to be specified on every `#[pallet::call]` declaration. By
///   default, dev mode pallets will assume a call index based on the order of the call.
/// * All storages are marked as unbounded, meaning you do not need to implement
///   `MaxEncodedLen` on storage types. This is equivalent to specifying `#[pallet::unbounded]`
///   on all storage type definitions.
/// * Storage hashers no longer need to be specified and can be replaced by `_`. In dev mode,
///   these will be replaced by `Blake2_128Concat`. In case of explicit key-binding, `Hasher`
///   can simply be ignored when in `dev_mode`.
///
/// Note that the `dev_mode` argument can only be supplied to the `#[pallet]` or
/// `#[frame_support::pallet]` attribute macro that encloses your pallet module. This argument
/// cannot be specified anywhere else, including but not limited to the `#[pallet::pallet]`
/// attribute macro.
///
/// <div class="example-wrap" style="display:inline-block"><pre class="compile_fail"
/// style="white-space:normal;font:inherit;">
/// <strong>WARNING</strong>:
/// You should not deploy or use dev mode pallets in production. Doing so can break your chain
/// and therefore should never be done. Once you are done tinkering, you should remove the
/// 'dev_mode' argument from your #[pallet] declaration and fix any compile errors before
/// attempting to use your pallet in a production scenario.
/// </pre></div>
///
/// # Pallet struct placeholder: `#[pallet::pallet]` (mandatory)
///
/// The pallet struct placeholder `#[pallet::pallet]` is mandatory and allows you to specify
/// pallet information.
///
/// The struct must be defined as follows:
/// ```ignore
/// #[pallet::pallet]
/// pub struct Pallet<T>(_);
/// ```
/// I.e. a regular struct definition named `Pallet`, with generic T and no where clause.
///
/// ## Macro expansion:
///
/// The macro adds this attribute to the struct definition:
/// ```ignore
/// #[derive(
/// 	frame_support::CloneNoBound,
/// 	frame_support::EqNoBound,
/// 	frame_support::PartialEqNoBound,
/// 	frame_support::RuntimeDebugNoBound,
/// )]
/// ```
/// and replaces the type `_` with `PhantomData<T>`. It also implements on the pallet:
/// * [`GetStorageVersion`](`traits::GetStorageVersion`)
/// * [`OnGenesis`](`traits::OnGenesis`): contains some logic to write the pallet version into
///   storage.
/// * `PalletErrorTypeInfo`: provides the type information for the pallet error, if defined.
///
/// It declares `type Module` type alias for `Pallet`, used by `construct_runtime`.
///
/// It implements [`PalletInfoAccess`](`traits::PalletInfoAccess') on `Pallet` to ease access
/// to pallet information given by [`frame_support::traits::PalletInfo`]. (The implementation
/// uses the associated type `frame_system::Config::PalletInfo`).
///
/// It implements [`StorageInfoTrait`](`traits::StorageInfoTrait`) on `Pallet` which give
/// information about all storages.
///
/// If the attribute `generate_store` is set then the macro creates the trait `Store` and
/// implements it on `Pallet`.
///
/// If the attribute `set_storage_max_encoded_len` is set then the macro calls
/// [`StorageInfoTrait`](`traits::StorageInfoTrait`) for each storage in the implementation of
/// [`StorageInfoTrait`](`traits::StorageInfoTrait`) for the pallet. Otherwise it implements
/// [`StorageInfoTrait`](`traits::StorageInfoTrait`) for the pallet using the
/// [`PartialStorageInfoTrait`](`traits::PartialStorageInfoTrait`) implementation of storages.
///
/// # Config trait: `#[pallet::config]` (mandatory)
///
/// The mandatory attribute `#[pallet::config]` defines the configurable options for the
/// pallet.
///
/// Item must be defined as:
///
/// ```ignore
/// #[pallet::config]
/// pub trait Config: frame_system::Config + $optionally_some_other_supertraits
/// $optional_where_clause
/// {
/// ...
/// }
/// ```
///
/// I.e. a regular trait definition named `Config`, with the supertrait
/// `frame_system::pallet::Config`, and optionally other supertraits and a where clause.
/// (Specifying other supertraits here is known as [tight
/// coupling](https://docs.substrate.io/reference/how-to-guides/pallet-design/use-tight-coupling/))
///
/// The associated type `RuntimeEvent` is reserved. If defined, it must have the bounds
/// `From<Event>` and `IsType<<Self as frame_system::Config>::RuntimeEvent>`.
///
/// [`pallet::event`](`frame_support::pallet_macros::event`) must be present if `RuntimeEvent`
/// exists as a config item in your `#[pallet::config]`.
///
/// Also see [`pallet::config`](`frame_support::pallet_macros::config`)
///
/// ## `pallet::constant`
///
/// The `#[pallet::constant]` attribute can be used to add an associated type trait bounded by
/// [`Get`](crate::traits::Get) from [`pallet::config`](#palletconfig) into metadata, e.g.:
///
/// ```ignore
/// #[pallet::config]
/// pub trait Config: frame_system::Config {
/// 	#[pallet::constant]
/// 	type Foo: Get<u32>;
/// }
/// ```
///
/// Also see [`pallet::constant`](`frame_support::pallet_macros::constant`)
///
/// ## `pallet::disable_frame_system_supertrait_check`
/// <a name="disable_supertrait_check"></a>
///
/// To bypass the `frame_system::Config` supertrait check, use the attribute
/// `pallet::disable_frame_system_supertrait_check`, e.g.:
///
/// ```ignore
/// #[pallet::config]
/// #[pallet::disable_frame_system_supertrait_check]
/// pub trait Config: pallet_timestamp::Config {}
/// ```
///
/// NOTE: Bypassing the `frame_system::Config` supertrait check is typically desirable when you
/// want to write an alternative to the `frame_system` pallet.
///
/// Also see
/// [`pallet::disable_frame_system_supertrait_check`](`frame_support::pallet_macros::disable_frame_system_supertrait_check`)
///
/// ## Macro expansion:
///
/// The macro expands pallet constant metadata with the information given by
/// `#[pallet::constant]`.
///
/// # `pallet::generate_store($vis trait Store)`
///
/// To generate a `Store` trait associating all storages, annotate your `Pallet` struct with
/// the attribute `#[pallet::generate_store($vis trait Store)]`, e.g.:
///
/// ```ignore
/// #[pallet::pallet]
/// #[pallet::generate_store(pub(super) trait Store)]
/// pub struct Pallet<T>(_);
/// ```
/// More precisely, the `Store` trait contains an associated type for each storage. It is
/// implemented for `Pallet` allowing access to the storage from pallet struct.
///
/// Thus when defining a storage named `Foo`, it can later be accessed from `Pallet` using
/// `<Pallet as Store>::Foo`.
///
/// NOTE: this attribute is only valid when applied _directly_ to your `Pallet` struct
/// definition.
///
/// Also see [`pallet::generate_store`](`frame_support::pallet_macros::generate_store`).
///
/// # `pallet::storage_version`
///
/// Because the [`pallet::pallet`](#pallet-struct-placeholder-palletpallet-mandatory) macro
/// implements [`traits::GetStorageVersion`], the current storage version needs to be
/// communicated to the macro. This can be done by using the `pallet::storage_version`
/// attribute:
///
/// ```ignore
/// const STORAGE_VERSION: StorageVersion = StorageVersion::new(5);
///
/// #[pallet::pallet]
/// #[pallet::storage_version(STORAGE_VERSION)]
/// pub struct Pallet<T>(_);
/// ```
///
/// If not present, the current storage version is set to the default value.
///
/// Also see [`pallet::storage_version`](`frame_support::pallet_macros::storage_version`)
///
/// # Hooks: `#[pallet::hooks]` (optional)
///
/// The `pallet::hooks` attribute allows you to specify a `Hooks` implementation for `Pallet`
/// that specifies pallet-specific logic.
///
/// The item the attribute attaches to must be defined as follows:
/// ```ignore
/// #[pallet::hooks]
/// impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> $optional_where_clause {
///     ...
/// }
/// ```
/// I.e. a regular trait implementation with generic bound: `T: Config`, for the trait
/// `Hooks<BlockNumberFor<T>>` (they are defined in preludes), for the type `Pallet<T>` and
/// with an optional where clause.
///
/// If no `#[pallet::hooks]` exists, then the following default implementation is
/// automatically generated:
/// ```ignore
/// #[pallet::hooks]
/// impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
/// ```
///
/// Also see [`pallet::hooks`](`frame_support::pallet_macros::hooks`)
///
/// # Call: `#[pallet::call]` (optional)
///
/// Implementation of pallet dispatchables.
///
/// Item must be defined as:
/// ```ignore
/// #[pallet::call]
/// impl<T: Config> Pallet<T> {
/// 	/// $some_doc
/// 	#[pallet::weight($ExpressionResultingInWeight)]
/// 	pub fn $fn_name(
/// 		origin: OriginFor<T>,
/// 		$some_arg: $some_type,
/// 		// or with compact attribute: #[pallet::compact] $some_arg: $some_type,
/// 		...
/// 	) -> DispatchResultWithPostInfo { // or `-> DispatchResult`
/// 		...
/// 	}
/// 	...
/// }
/// ```
/// I.e. a regular type implementation, with generic `T: Config`, on type `Pallet<T>`, with
/// an optional where clause.
///
/// ## `#[pallet::weight($expr)]`
///
/// Each dispatchable needs to define a weight with `#[pallet::weight($expr)]` attribute, the
/// first argument must be `origin: OriginFor<T>`.
///
/// Also see [`pallet::weight`](`frame_support::pallet_macros::weight`)
///
/// ### `#[pallet::compact] $some_arg: $some_type`
///
/// Compact encoding for arguments can be achieved via `#[pallet::compact]`. The function must
/// return a `DispatchResultWithPostInfo` or `DispatchResult`.
///
/// Also see [`pallet::compact`](`frame_support::pallet_macros::compact`)
///
/// ## `#[pallet::call_index($idx)]`
///
/// Each dispatchable may also be annotated with the `#[pallet::call_index($idx)]` attribute,
/// which explicitly defines the codec index for the dispatchable function in the `Call` enum.
///
/// All call indexes start from 0, until it encounters a dispatchable function with a defined
/// call index. The dispatchable function that lexically follows the function with a defined
/// call index will have that call index, but incremented by 1, e.g. if there are 3
/// dispatchable functions `fn foo`, `fn bar` and `fn qux` in that order, and only `fn bar`
/// has a call index of 10, then `fn qux` will have an index of 11, instead of 1.
///
/// **WARNING**: modifying dispatchables, changing their order, removing some, etc., must be
/// done with care. Indeed this will change the outer runtime call type (which is an enum with
/// one variant per pallet), this outer runtime call can be stored on-chain (e.g. in
/// `pallet-scheduler`). Thus migration might be needed. To mitigate against some of this, the
/// `#[pallet::call_index($idx)]` attribute can be used to fix the order of the dispatchable so
/// that the `Call` enum encoding does not change after modification. As a general rule of
/// thumb, it is therefore adventageous to always add new calls to the end so you can maintain
/// the existing order of calls.
///
/// Also see [`pallet::call_index`](`frame_support::pallet_macros::call_index`)
///
/// # Extra constants: `#[pallet::extra_constants]` (optional)
///
/// Allows you to define some extra constants to be added into constant metadata.
///
/// Item must be defined as:
///
/// ```ignore
/// #[pallet::extra_constants]
/// impl<T: Config> Pallet<T> where $optional_where_clause {
/// 	/// $some_doc
/// 	$vis fn $fn_name() -> $some_return_type {
/// 		...
/// 	}
/// 	...
/// }
/// ```
/// I.e. a regular rust `impl` block with some optional where clause and functions with 0 args,
/// 0 generics, and some return type.
///
/// ## Macro expansion
///
/// The macro add some extra constants to pallet constant metadata.
///
/// Also see: [`pallet::extra_constants`](`frame_support::pallet_macros::extra_constants`)
///
/// # Error: `#[pallet::error]` (optional)
///
/// The `#[pallet::error]` attribute allows you to define an error enum that will be returned
/// from the dispatchable when an error occurs. The information for this error type is then
/// stored in metadata.
///
/// Item must be defined as:
///
/// ```ignore
/// #[pallet::error]
/// pub enum Error<T> {
/// 	/// $some_optional_doc
/// 	$SomeFieldLessVariant,
/// 	/// $some_more_optional_doc
/// 	$SomeVariantWithOneField(FieldType),
/// 	...
/// }
/// ```
/// I.e. a regular enum named `Error`, with generic `T` and fieldless or multiple-field
/// variants.
///
/// Any field type in the enum variants must implement [`scale_info::TypeInfo`] in order to be
/// properly used in the metadata, and its encoded size should be as small as possible,
/// preferably 1 byte in size in order to reduce storage size. The error enum itself has an
/// absolute maximum encoded size specified by [`MAX_MODULE_ERROR_ENCODED_SIZE`].
///
/// (1 byte can still be 256 different errors. The more specific the error, the easier it is to
/// diagnose problems and give a better experience to the user. Don't skimp on having lots of
/// individual error conditions.)
///
/// Field types in enum variants must also implement [`PalletError`](traits::PalletError),
/// otherwise the pallet will fail to compile. Rust primitive types have already implemented
/// the [`PalletError`](traits::PalletError) trait along with some commonly used stdlib types
/// such as [`Option`] and
/// [`PhantomData`](`frame_support::__private::sp_std::marker::PhantomData`), and hence in most
/// use cases, a manual implementation is not necessary and is discouraged.
///
/// The generic `T` must not bound anything and a `where` clause is not allowed. That said,
/// bounds and/or a where clause should not needed for any use-case.
///
/// Also see: [`pallet::error`](`frame_support::pallet_macros::error`)
///
/// # Event: `#[pallet::event]` (optional)
///
/// Allows you to define pallet events. Pallet events are stored under the `system` / `events`
/// key when the block is applied (and then replaced when the next block writes it's events).
///
/// The Event enum must be defined as follows:
///
/// ```ignore
/// #[pallet::event]
/// #[pallet::generate_deposit($visibility fn deposit_event)] // Optional
/// pub enum Event<$some_generic> $optional_where_clause {
/// 	/// Some doc
/// 	$SomeName($SomeType, $YetanotherType, ...),
/// 	...
/// }
/// ```
///
/// I.e. an enum (with named or unnamed fields variant), named `Event`, with generic: none or
/// `T` or `T: Config`, and optional w here clause.
///
/// Each field must implement [`Clone`], [`Eq`], [`PartialEq`], [`Encode`], [`Decode`], and
/// [`Debug`] (on std only). For ease of use, bound by the trait
/// [`Member`](`frame_support::pallet_prelude::Member`), available in
/// frame_support::pallet_prelude.
///
/// Also see [`pallet::event`](`frame_support::pallet_macros::event`)
///
/// ## `#[pallet::generate_deposit($visibility fn deposit_event)]`
///
/// The attribute `#[pallet::generate_deposit($visibility fn deposit_event)]` generates a
/// helper function on `Pallet` that handles deposit events.
///
/// NOTE: For instantiable pallets, the event must be generic over `T` and `I`.
///
/// Also see [`pallet::generate_deposit`](`frame_support::pallet_macros::generate_deposit`)
///
/// # Storage: `#[pallet::storage]` (optional)
///
/// The `#[pallet::storage]` attribute lets you define some abstract storage inside of runtime
/// storage and also set its metadata. This attribute can be used multiple times.
///
/// Item should be defined as:
///
/// ```ignore
/// #[pallet::storage]
/// #[pallet::getter(fn $getter_name)] // optional
/// $vis type $StorageName<$some_generic> $optional_where_clause
/// 	= $StorageType<$generic_name = $some_generics, $other_name = $some_other, ...>;
/// ```
///
/// or with unnamed generic:
///
/// ```ignore
/// #[pallet::storage]
/// #[pallet::getter(fn $getter_name)] // optional
/// $vis type $StorageName<$some_generic> $optional_where_clause
/// 	= $StorageType<_, $some_generics, ...>;
/// ```
///
/// I.e. it must be a type alias, with generics: `T` or `T: Config`. The aliased type must be
/// one of [`StorageValue`](`pallet_prelude::StorageValue`),
/// [`StorageMap`](`pallet_prelude::StorageMap`) or
/// [`StorageDoubleMap`](`pallet_prelude::StorageDoubleMap`). The generic arguments of the
/// storage type can be given in two manners: named and unnamed. For named generic arguments,
/// the name for each argument should match the name defined for it on the storage struct:
/// * [`StorageValue`](`pallet_prelude::StorageValue`) expects `Value` and optionally
///   `QueryKind` and `OnEmpty`,
/// * [`StorageMap`](`pallet_prelude::StorageMap`) expects `Hasher`, `Key`, `Value` and
///   optionally `QueryKind` and `OnEmpty`,
/// * [`CountedStorageMap`](`pallet_prelude::CountedStorageMap`) expects `Hasher`, `Key`,
///   `Value` and optionally `QueryKind` and `OnEmpty`,
/// * [`StorageDoubleMap`](`pallet_prelude::StorageDoubleMap`) expects `Hasher1`, `Key1`,
///   `Hasher2`, `Key2`, `Value` and optionally `QueryKind` and `OnEmpty`.
///
/// For unnamed generic arguments: Their first generic must be `_` as it is replaced by the
/// macro and other generic must declared as a normal generic type declaration.
///
/// The `Prefix` generic written by the macro is generated using
/// `PalletInfo::name::<Pallet<..>>()` and the name of the storage type. E.g. if runtime names
/// the pallet "MyExample" then the storage `type Foo<T> = ...` should use the prefix:
/// `Twox128(b"MyExample") ++ Twox128(b"Foo")`.
///
/// For the [`CountedStorageMap`](`pallet_prelude::CountedStorageMap`) variant, the `Prefix`
/// also implements
/// [`CountedStorageMapInstance`](`frame_support::storage::types::CountedStorageMapInstance`).
/// It also associates a [`CounterPrefix`](`pallet_prelude::CounterPrefix'), which is
/// implemented the same as above, but the storage prefix is prepend with `"CounterFor"`. E.g.
/// if runtime names the pallet "MyExample" then the storage `type Foo<T> =
/// CountedStorageaMap<...>` will store its counter at the prefix: `Twox128(b"MyExample") ++
/// Twox128(b"CounterForFoo")`.
///
/// E.g:
///
/// ```ignore
/// #[pallet::storage]
/// pub(super) type MyStorage<T> = StorageMap<Hasher = Blake2_128Concat, Key = u32, Value = u32>;
/// ```
///
/// In this case the final prefix used by the map is `Twox128(b"MyExample") ++
/// Twox128(b"OtherName")`.
///
/// Also see [`pallet::storage`](`frame_support::pallet_macros::storage`)
///
/// ## `#[pallet::getter(fn $my_getter_fn_name)]` (optional)
///
/// The optional attribute `#[pallet::getter(fn $my_getter_fn_name)]` allows you to define a
/// getter function on `Pallet`.
///
/// Also see [`pallet::getter`](`frame_support::pallet_macros::getter`)
///
/// ## `#[pallet::storage_prefix = "SomeName"]` (optional)
///
/// The optional attribute `#[pallet::storage_prefix = "SomeName"]` allows you to define the
/// storage prefix to use, see how `Prefix` generic is implemented above. This is helpful if
/// you wish to rename the storage field but don't want to perform a migration.
///
/// E.g:
///
/// ```ignore
/// #[pallet::storage]
/// #[pallet::storage_prefix = "foo"]
/// #[pallet::getter(fn my_storage)]
/// pub(super) type MyStorage<T> = StorageMap<Hasher = Blake2_128Concat, Key = u32, Value = u32>;
/// ```
///
/// or
///
/// ```ignore
/// #[pallet::storage]
/// #[pallet::getter(fn my_storage)]
/// pub(super) type MyStorage<T> = StorageMap<_, Blake2_128Concat, u32, u32>;
/// ```
///
/// Also see [`pallet::storage_prefix`](`frame_support::pallet_macros::storage_prefix`)
///
/// ## `#[pallet::unbounded]` (optional)
///
/// The optional attribute `#[pallet::unbounded]` declares the storage as unbounded. When
/// implementating the storage info (when `#[pallet::generate_storage_info]` is specified on
/// the pallet struct placeholder), the size of the storage will be declared as unbounded. This
/// can be useful for storage which can never go into PoV (Proof of Validity).
///
/// Also see [`pallet::unbounded`](`frame_support::pallet_macros::unbounded`)
///
/// ## `#[pallet::whitelist_storage]` (optional)
///
/// The optional attribute `#[pallet::whitelist_storage]` will declare the storage as
/// whitelisted from benchmarking.
///
/// See
/// [`pallet::whitelist_storage`](frame_support::pallet_macros::whitelist_storage)
/// for more info.
///
/// ## `#[pallet::disable_try_decode_storage]` (optional)
///
/// The optional attribute `#[pallet::disable_try_decode_storage]` will declare the storage as
/// whitelisted state decoding during try-runtime logic.
///
/// See
/// [`pallet::disable_try_decode_storage`](frame_support::pallet_macros::disable_try_decode_storage)
/// for more info.
///
///	## `#[cfg(..)]` (for storage)
/// The optional attributes `#[cfg(..)]` allow conditional compilation for the storage.
///
/// E.g:
///
/// ```ignore
/// #[cfg(feature = "my-feature")]
/// #[pallet::storage]
/// pub(super) type MyStorage<T> = StorageValue<Value = u32>;
/// ```
///
/// All the `cfg` attributes are automatically copied to the items generated for the storage,
/// i.e. the getter, storage prefix, and the metadata element etc.
///
/// Any type placed as the `QueryKind` parameter must implement
/// [`frame_support::storage::types::QueryKindTrait`]. There are 3 implementations of this
/// trait by default:
///
/// 1. [`OptionQuery`](`frame_support::storage::types::OptionQuery`), the default `QueryKind`
///    used when this type parameter is omitted. Specifying this as the `QueryKind` would cause
///    storage map APIs that return a `QueryKind` to instead return an [`Option`], returning
///    `Some` when a value does exist under a specified storage key, and `None` otherwise.
/// 2. [`ValueQuery`](`frame_support::storage::types::ValueQuery`) causes storage map APIs that
///    return a `QueryKind` to instead return the value type. In cases where a value does not
///    exist under a specified storage key, the `OnEmpty` type parameter on `QueryKindTrait` is
///    used to return an appropriate value.
/// 3. [`ResultQuery`](`frame_support::storage::types::ResultQuery`) causes storage map APIs
///    that return a `QueryKind` to instead return a `Result<T, E>`, with `T` being the value
///    type and `E` being the pallet error type specified by the `#[pallet::error]` attribute.
///    In cases where a value does not exist under a specified storage key, an `Err` with the
///    specified pallet error variant is returned.
///
/// NOTE: If the `QueryKind` generic parameter is still generic at this stage or is using some
/// type alias then the generation of the getter might fail. In this case the getter can be
/// implemented manually.
///
/// NOTE: The generic `Hasher` must implement the [`StorageHasher`] trait (or the type is not
/// usable at all). We use [`StorageHasher::METADATA`] for the metadata of the hasher of the
/// storage item. Thus generic hasher is supported.
///
/// ## Macro expansion
///
/// For each storage item the macro generates a struct named
/// `_GeneratedPrefixForStorage$NameOfStorage`, and implements
/// [`StorageInstance`](traits::StorageInstance) on it using the pallet and storage name. It
/// then uses it as the first generic of the aliased type. For
/// [`CountedStorageMap`](`pallet_prelude::CountedStorageMap`),
/// [`CountedStorageMapInstance`](`frame_support::storage::types::CountedStorageMapInstance`)
/// is implemented, and another similar struct is generated.
///
/// For a named generic, the macro will reorder the generics, and remove the names.
///
/// The macro implements the function `storage_metadata` on the `Pallet` implementing the
/// metadata for all storage items based on their kind:
/// * for a storage value, the type of the value is copied into the metadata
/// * for a storage map, the type of the values and the key's type is copied into the metadata
/// * for a storage double map, the type of the values, and the types of `key1` and `key2` are
///   copied into the metadata.
///
/// # Type value: `#[pallet::type_value]` (optional)
///
/// The `#[pallet::type_value]` attribute lets you define a struct implementing the
/// [`Get`](crate::traits::Get) trait to ease use of storage types. This attribute is meant to
/// be used alongside [`#[pallet::storage]`](#storage-palletstorage-optional) to define a
/// storage's default value. This attribute can be used multiple times.
///
/// Item must be defined as:
///
/// ```ignore
/// #[pallet::type_value]
/// fn $MyDefaultName<$some_generic>() -> $default_type $optional_where_clause { $expr }
/// ```
///
/// I.e.: a function definition with generics none or `T: Config` and a returned type.
///
/// E.g.:
///
/// ```ignore
/// #[pallet::type_value]
/// fn MyDefault<T: Config>() -> T::Balance { 3.into() }
/// ```
///
/// Also see [`pallet::type_value`](`frame_support::pallet_macros::type_value`)
///
/// # Genesis config: `#[pallet::genesis_config]` (optional)
///
/// The `#[pallet::genesis_config]` attribute allows you to define the genesis configuration
/// for the pallet.
///
/// Item is defined as either an enum or a struct. It needs to be public and implement the
/// trait [`BuildGenesisConfig`](`traits::BuildGenesisConfig`) with
/// [`#[pallet::genesis_build]`](#genesis-build-palletgenesis_build-optional). The type
/// generics are constrained to be either none, or `T` or `T: Config`.
///
/// E.g:
///
/// ```ignore
/// #[pallet::genesis_config]
/// pub struct GenesisConfig<T: Config> {
/// 	_myfield: BalanceOf<T>,
/// }
/// ```
///
/// Also see [`pallet::genesis_config`](`frame_support::pallet_macros::genesis_config`)
///
/// # Genesis build: `#[pallet::genesis_build]` (optional)
///
/// The `#[pallet::genesis_build]` attribute allows you to define how `genesis_configuration`
/// is built. This takes as input the `GenesisConfig` type (as `self`) and constructs the
/// pallet's initial state.
///
/// The impl must be defined as:
///
/// ```ignore
/// #[pallet::genesis_build]
/// impl<T: Config> GenesisBuild<T> for GenesisConfig<$maybe_generics> {
/// 	fn build(&self) { $expr }
/// }
/// ```
///
/// I.e. a trait implementation with generic `T: Config`, of trait `GenesisBuild<T>` on
/// type `GenesisConfig` with generics none or `T`.
///
/// E.g.:
///
/// ```ignore
/// #[pallet::genesis_build]
/// impl<T: Config> GenesisBuild<T> for GenesisConfig {
/// 	fn build(&self) {}
/// }
/// ```
///
/// Also see [`pallet::genesis_build`](`frame_support::pallet_macros::genesis_build`)
///
/// # Inherent: `#[pallet::inherent]` (optional)
///
/// The `#[pallet::inherent]` attribute allows the pallet to provide some
/// [inherent](https://docs.substrate.io/fundamentals/transaction-types/#inherent-transactions).
/// An inherent is some piece of data that is inserted by a block authoring node at block
/// creation time and can either be accepted or rejected by validators based on whether the
/// data falls within an acceptable range.
///
/// The most common inherent is the `timestamp` that is inserted into every block. Since there
/// is no way to validate timestamps, validators simply check that the timestamp reported by
/// the block authoring node falls within an acceptable range.
///
/// Item must be defined as:
///
/// ```ignore
/// #[pallet::inherent]
/// impl<T: Config> ProvideInherent for Pallet<T> {
/// 	// ... regular trait implementation
/// }
/// ```
///
/// I.e. a trait implementation with bound `T: Config`, of trait
/// [`ProvideInherent`](`pallet_prelude::ProvideInherent`) for type `Pallet<T>`, and some
/// optional where clause.
///
/// Also see [`pallet::inherent`](`frame_support::pallet_macros::inherent`)
///
/// # Validate unsigned: `#[pallet::validate_unsigned]` (optional)
///
/// The `#[pallet::validate_unsigned]` attribute allows the pallet to validate some unsigned
/// transaction:
///
/// Item must be defined as:
///
/// ```ignore
/// #[pallet::validate_unsigned]
/// impl<T: Config> ValidateUnsigned for Pallet<T> {
/// 	// ... regular trait implementation
/// }
/// ```
///
/// I.e. a trait implementation with bound `T: Config`, of trait
/// [`ValidateUnsigned`](`pallet_prelude::ValidateUnsigned`) for type `Pallet<T>`, and some
/// optional where clause.
///
/// NOTE: There is also the [`sp_runtime::traits::SignedExtension`] trait that can be used to
/// add some specific logic for transaction validation.
///
/// Also see [`pallet::validate_unsigned`](`frame_support::pallet_macros::validate_unsigned`)
///
/// # Origin: `#[pallet::origin]` (optional)
///
/// The `#[pallet::origin]` attribute allows you to define some origin for the pallet.
///
/// Item must be either a type alias, an enum, or a struct. It needs to be public.
///
/// E.g.:
///
/// ```ignore
/// #[pallet::origin]
/// pub struct Origin<T>(PhantomData<(T)>);
/// ```
///
/// **WARNING**: modifying origin changes the outer runtime origin. This outer runtime origin
/// can be stored on-chain (e.g. in `pallet-scheduler`), thus any change must be done with care
/// as it might require some migration.
///
/// NOTE: for instantiable pallets, the origin must be generic over `T` and `I`.
///
/// Also see [`pallet::origin`](`frame_support::pallet_macros::origin`)
///
/// # Composite enum `#[pallet::composite_enum]` (optional)
///
/// The `#[pallet::composite_enum]` attribute allows you to define an enum on the pallet which
/// will then instruct `construct_runtime` to amalgamate all similarly-named enums from other
/// pallets into an aggregate enum. This is similar in principle with how the aggregate enum is
/// generated for `#[pallet::event]` or `#[pallet::error]`.
///
/// The item tagged with `#[pallet::composite_enum]` MUST be an enum declaration, and can ONLY
/// be the following identifiers: `FreezeReason`, `HoldReason`, `LockId` or `SlashReason`.
/// Custom identifiers are not supported.
///
/// NOTE: For ease of usage, when no `#[derive]` attributes are detected, the
/// `#[pallet::composite_enum]` attribute will automatically derive the following traits for
/// the enum:
///
/// ```ignore
/// Copy, Clone, Eq, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo, RuntimeDebug
/// ```
///
/// The inverse is also true: if there are any #[derive] attributes present for the enum, then
/// the attribute will not automatically derive any of the traits described above.
///
/// # General notes on instantiable pallets
///
/// An instantiable pallet is one where Config is generic, i.e. `Config<I>`. This allows
/// runtime to implement multiple instances of the pallet, by using different types for the
/// generic. This is the sole purpose of the generic `I`, but because
/// [`PalletInfo`](`traits::PalletInfo`) requires the `Pallet` placeholder to be static, it is
/// important to bound by `'static` whenever [`PalletInfo`](`traits::PalletInfo`) can be used.
/// Additionally, in order to make an instantiable pallet usable as a regular pallet without an
/// instance, it is important to bound by `= ()` on every type.
///
/// Thus impl bound looks like `impl<T: Config<I>, I: 'static>`, and types look like
/// `SomeType<T, I=()>` or `SomeType<T: Config<I>, I: 'static = ()>`.
///
/// # Example of a non-instantiable pallet
///
/// ```
/// pub use pallet::*; // reexport in crate namespace for `construct_runtime!`
///
/// #[frame_support::pallet]
/// // NOTE: The name of the pallet is provided by `construct_runtime` and is used as
/// // the unique identifier for the pallet's storage. It is not defined in the pallet itself.
/// pub mod pallet {
/// 	use frame_support::pallet_prelude::*; // Import various types used in the pallet definition
/// 	use frame_system::pallet_prelude::*; // Import some system helper types.
///
/// 	type BalanceOf<T> = <T as Config>::Balance;
///
/// 	// Define the generic parameter of the pallet
/// 	// The macro parses `#[pallet::constant]` attributes and uses them to generate metadata
/// 	// for the pallet's constants.
/// 	#[pallet::config]
/// 	pub trait Config: frame_system::Config {
/// 		#[pallet::constant] // put the constant in metadata
/// 		type MyGetParam: Get<u32>;
/// 		type Balance: Parameter + MaxEncodedLen + From<u8>;
/// 		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
/// 	}
///
/// 	// Define some additional constant to put into the constant metadata.
/// 	#[pallet::extra_constants]
/// 	impl<T: Config> Pallet<T> {
/// 		/// Some description
/// 		fn exra_constant_name() -> u128 { 4u128 }
/// 	}
///
/// 	// Define the pallet struct placeholder, various pallet function are implemented on it.
/// 	#[pallet::pallet]
/// 	#[pallet::generate_store(pub(super) trait Store)]
/// 	pub struct Pallet<T>(_);
///
/// 	// Implement the pallet hooks.
/// 	#[pallet::hooks]
/// 	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
/// 		fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
/// 			unimplemented!();
/// 		}
///
/// 		// can implement also: on_finalize, on_runtime_upgrade, offchain_worker, ...
/// 		// see `Hooks` trait
/// 	}
///
/// 	// Declare Call struct and implement dispatchables.
/// 	//
/// 	// WARNING: Each parameter used in functions must implement: Clone, Debug, Eq, PartialEq,
/// 	// Codec.
/// 	//
/// 	// The macro parses `#[pallet::compact]` attributes on function arguments and implements
/// 	// the `Call` encoding/decoding accordingly.
/// 	#[pallet::call]
/// 	impl<T: Config> Pallet<T> {
/// 		/// Doc comment put in metadata
/// 		#[pallet::weight(0)] // Defines weight for call (function parameters are in scope)
/// 		pub fn toto(
/// 			origin: OriginFor<T>,
/// 			#[pallet::compact] _foo: u32,
/// 		) -> DispatchResultWithPostInfo {
/// 			let _ = origin;
/// 			unimplemented!();
/// 		}
/// 	}
///
/// 	// Declare the pallet `Error` enum (this is optional).
/// 	// The macro generates error metadata using the doc comment on each variant.
/// 	#[pallet::error]
/// 	pub enum Error<T> {
/// 		/// doc comment put into metadata
/// 		InsufficientProposersBalance,
/// 	}
///
/// 	// Declare pallet Event enum (this is optional).
/// 	//
/// 	// WARNING: Each type used in variants must implement: Clone, Debug, Eq, PartialEq, Codec.
/// 	//
/// 	// The macro generates event metadata, and derive Clone, Debug, Eq, PartialEq and Codec
/// 	#[pallet::event]
/// 	// Generate a funciton on Pallet to deposit an event.
/// 	#[pallet::generate_deposit(pub(super) fn deposit_event)]
/// 	pub enum Event<T: Config> {
/// 		/// doc comment put in metadata
/// 		// `<T as frame_system::Config>::AccountId` is not defined in metadata list, the last
/// 		// Thus the metadata is `<T as frame_system::Config>::AccountId`.
/// 		Proposed(<T as frame_system::Config>::AccountId),
/// 		/// doc
/// 		// here metadata will be `Balance` as define in metadata list
/// 		Spending(BalanceOf<T>),
/// 		// here metadata will be `Other` as define in metadata list
/// 		Something(u32),
/// 	}
///
/// 	// Define a struct which implements `frame_support::traits::Get<T::Balance>` (optional).
/// 	#[pallet::type_value]
/// 	pub(super) fn MyDefault<T: Config>() -> T::Balance { 3.into() }
///
/// 	// Declare a storage item. Any amount of storage items can be declared (optional).
/// 	//
/// 	// Is expected either `StorageValue`, `StorageMap` or `StorageDoubleMap`.
/// 	// The macro generates the prefix type and replaces the first generic `_`.
/// 	//
/// 	// The macro expands the metadata for the storage item with the type used:
/// 	// * for a storage value the type of the value is copied into the metadata
/// 	// * for a storage map the type of the values and the type of the key is copied into the metadata
/// 	// * for a storage double map the types of the values and keys are copied into the
/// 	//   metadata.
/// 	//
/// 	// NOTE: The generic `Hasher` must implement the `StorageHasher` trait (or the type is not
/// 	// usable at all). We use [`StorageHasher::METADATA`] for the metadata of the hasher of the
/// 	// storage item. Thus generic hasher is supported.
/// 	#[pallet::storage]
/// 	pub(super) type MyStorageValue<T: Config> =
/// 		StorageValue<Value = T::Balance, QueryKind = ValueQuery, OnEmpty = MyDefault<T>>;
///
/// 	// Another storage declaration
/// 	#[pallet::storage]
/// 	#[pallet::getter(fn my_storage)]
/// 	#[pallet::storage_prefix = "SomeOtherName"]
/// 	pub(super) type MyStorage<T> =
/// 		StorageMap<Hasher = Blake2_128Concat, Key = u32, Value = u32>;
///
/// 	// Declare the genesis config (optional).
/// 	//
/// 	// The macro accepts either a struct or an enum; it checks that generics are consistent.
/// 	//
/// 	// Type must implement the `Default` trait.
/// 	#[pallet::genesis_config]
/// 	#[derive(frame_support::DefaultNoBound)]
/// 	pub struct GenesisConfig<T: Config> {
/// 	    _config: sp_std::marker::PhantomData<T>,
/// 		_myfield: u32,
/// 	}
///
/// 	// Declare genesis builder. (This is need only if GenesisConfig is declared)
/// 	#[pallet::genesis_build]
/// 	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
/// 		fn build(&self) {}
/// 	}
///
/// 	// Declare a pallet origin (this is optional).
/// 	//
/// 	// The macro accept type alias or struct or enum, it checks generics are consistent.
/// 	#[pallet::origin]
/// 	pub struct Origin<T>(PhantomData<T>);
///
///     // Declare a hold reason (this is optional).
///     //
///     // Creates a hold reason for this pallet that is aggregated by `construct_runtime`.
///     // A similar enum can be defined for `FreezeReason`, `LockId` or `SlashReason`.
///     #[pallet::composite_enum]
/// 	pub enum HoldReason {
/// 		SomeHoldReason
/// 	}
///
/// 	// Declare validate_unsigned implementation (this is optional).
/// 	#[pallet::validate_unsigned]
/// 	impl<T: Config> ValidateUnsigned for Pallet<T> {
/// 		type Call = Call<T>;
/// 		fn validate_unsigned(
/// 			source: TransactionSource,
/// 			call: &Self::Call
/// 		) -> TransactionValidity {
/// 			Err(TransactionValidityError::Invalid(InvalidTransaction::Call))
/// 		}
/// 	}
///
/// 	// Declare inherent provider for pallet (this is optional).
/// 	#[pallet::inherent]
/// 	impl<T: Config> ProvideInherent for Pallet<T> {
/// 		type Call = Call<T>;
/// 		type Error = InherentError;
///
/// 		const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;
///
/// 		fn create_inherent(_data: &InherentData) -> Option<Self::Call> {
/// 			unimplemented!();
/// 		}
///
/// 		fn is_inherent(_call: &Self::Call) -> bool {
/// 			unimplemented!();
/// 		}
/// 	}
///
/// 	// Regular rust code needed for implementing ProvideInherent trait
///
/// 	#[derive(codec::Encode, sp_runtime::RuntimeDebug)]
/// 	#[cfg_attr(feature = "std", derive(codec::Decode))]
/// 	pub enum InherentError {
/// 	}
///
/// 	impl sp_inherents::IsFatalError for InherentError {
/// 		fn is_fatal_error(&self) -> bool {
/// 			unimplemented!();
/// 		}
/// 	}
///
/// 	pub const INHERENT_IDENTIFIER: sp_inherents::InherentIdentifier = *b"testpall";
/// }
/// ```
///
/// # Example of an instantiable pallet
///
/// ```
/// pub use pallet::*;
///
/// #[frame_support::pallet]
/// pub mod pallet {
/// 	use frame_support::pallet_prelude::*;
/// 	use frame_system::pallet_prelude::*;
///
/// 	type BalanceOf<T, I = ()> = <T as Config<I>>::Balance;
///
/// 	#[pallet::config]
/// 	pub trait Config<I: 'static = ()>: frame_system::Config {
/// 		#[pallet::constant]
/// 		type MyGetParam: Get<u32>;
/// 		type Balance: Parameter + MaxEncodedLen + From<u8>;
/// 		type RuntimeEvent: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
/// 	}
///
/// 	#[pallet::extra_constants]
/// 	impl<T: Config<I>, I: 'static> Pallet<T, I> {
/// 		/// Some description
/// 		fn extra_constant_name() -> u128 { 4u128 }
/// 	}
///
/// 	#[pallet::pallet]
/// 	#[pallet::generate_store(pub(super) trait Store)]
/// 	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);
///
/// 	#[pallet::hooks]
/// 	impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {
/// 	}
///
/// 	#[pallet::call]
/// 	impl<T: Config<I>, I: 'static> Pallet<T, I> {
/// 		/// Doc comment put in metadata
/// 		#[pallet::weight(0)]
/// 		pub fn toto(origin: OriginFor<T>, #[pallet::compact] _foo: u32) -> DispatchResultWithPostInfo {
/// 			let _ = origin;
/// 			unimplemented!();
/// 		}
/// 	}
///
/// 	#[pallet::error]
/// 	pub enum Error<T, I = ()> {
/// 		/// doc comment put into metadata
/// 		InsufficientProposersBalance,
/// 	}
///
/// 	#[pallet::event]
/// 	#[pallet::generate_deposit(pub(super) fn deposit_event)]
/// 	pub enum Event<T: Config<I>, I: 'static = ()> {
/// 		/// doc comment put in metadata
/// 		Proposed(<T as frame_system::Config>::AccountId),
/// 		/// doc
/// 		Spending(BalanceOf<T, I>),
/// 		Something(u32),
/// 	}
///
/// 	#[pallet::type_value]
/// 	pub(super) fn MyDefault<T: Config<I>, I: 'static>() -> T::Balance { 3.into() }
///
/// 	#[pallet::storage]
/// 	pub(super) type MyStorageValue<T: Config<I>, I: 'static = ()> =
/// 		StorageValue<Value = T::Balance, QueryKind = ValueQuery, OnEmpty = MyDefault<T, I>>;
///
/// 	#[pallet::storage]
/// 	#[pallet::getter(fn my_storage)]
/// 	#[pallet::storage_prefix = "SomeOtherName"]
/// 	pub(super) type MyStorage<T, I = ()> =
/// 		StorageMap<Hasher = Blake2_128Concat, Key = u32, Value = u32>;
///
/// 	#[pallet::genesis_config]
/// 	#[derive(frame_support::DefaultNoBound)]
/// 	pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
/// 		 _config: sp_std::marker::PhantomData<(T,I)>,
/// 		_myfield: u32,
/// 	}
///
/// 	#[pallet::genesis_build]
/// 	impl<T: Config<I>, I: 'static> BuildGenesisConfig for GenesisConfig<T, I> {
/// 		fn build(&self) {}
/// 	}
///
/// 	#[pallet::origin]
/// 	pub struct Origin<T, I = ()>(PhantomData<(T, I)>);
///
///     #[pallet::composite_enum]
/// 	pub enum HoldReason<I: 'static = ()> {
/// 		SomeHoldReason
/// 	}
///
/// 	#[pallet::validate_unsigned]
/// 	impl<T: Config<I>, I: 'static> ValidateUnsigned for Pallet<T, I> {
/// 		type Call = Call<T, I>;
/// 		fn validate_unsigned(
/// 			source: TransactionSource,
/// 			call: &Self::Call
/// 		) -> TransactionValidity {
/// 			Err(TransactionValidityError::Invalid(InvalidTransaction::Call))
/// 		}
/// 	}
///
/// 	#[pallet::inherent]
/// 	impl<T: Config<I>, I: 'static> ProvideInherent for Pallet<T, I> {
/// 		type Call = Call<T, I>;
/// 		type Error = InherentError;
///
/// 		const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;
///
/// 		fn create_inherent(_data: &InherentData) -> Option<Self::Call> {
/// 			unimplemented!();
/// 		}
///
/// 		fn is_inherent(_call: &Self::Call) -> bool {
/// 			unimplemented!();
/// 		}
/// 	}
///
/// 	// Regular rust code needed for implementing ProvideInherent trait
///
/// 	#[derive(codec::Encode, sp_runtime::RuntimeDebug)]
/// 	#[cfg_attr(feature = "std", derive(codec::Decode))]
/// 	pub enum InherentError {
/// 	}
///
/// 	impl sp_inherents::IsFatalError for InherentError {
/// 		fn is_fatal_error(&self) -> bool {
/// 			unimplemented!();
/// 		}
/// 	}
///
/// 	pub const INHERENT_IDENTIFIER: sp_inherents::InherentIdentifier = *b"testpall";
/// }
/// ```
///
/// # Upgrade guidelines
///
/// 1. Export the metadata of the pallet for later checks
///     - run your node with the pallet active
///     - query the metadata using the `state_getMetadata` RPC and curl, or use `subsee -p
///       <PALLET_NAME> > meta.json`
/// 2. Generate the template upgrade for the pallet provided by `decl_storage` with the
///    environment variable `PRINT_PALLET_UPGRADE`: `PRINT_PALLET_UPGRADE=1 cargo check -p
///    my_pallet`. This template can be used as it contains all information for storages,
///    genesis config and genesis build.
/// 3. Reorganize the pallet to have the trait `Config`, `decl_*` macros,
///    [`ValidateUnsigned`](`pallet_prelude::ValidateUnsigned`),
///    [`ProvideInherent`](`pallet_prelude::ProvideInherent`), and Origin` all together in one
///    file. Suggested order:
///     * `Config`,
///     * `decl_module`,
///     * `decl_event`,
///     * `decl_error`,
///     * `decl_storage`,
///     * `origin`,
///     * `validate_unsigned`,
///     * `provide_inherent`, so far it should compile and all be correct.
/// 4. start writing the new pallet module
/// 	```ignore
/// 	pub use pallet::*;
///
/// 	#[frame_support::pallet]
/// 	pub mod pallet {
/// 		use frame_support::pallet_prelude::*;
/// 		use frame_system::pallet_prelude::*;
/// 		use super::*;
///
/// 		#[pallet::pallet]
/// 		#[pallet::generate_store($visibility_of_trait_store trait Store)]
/// 		// NOTE: if the visibility of trait store is private but you want to make it available
/// 		// in super, then use `pub(super)` or `pub(crate)` to make it available in crate.
/// 		pub struct Pallet<T>(_);
/// 		// pub struct Pallet<T, I = ()>(PhantomData<T>); // for instantiable pallet
/// 	}
/// 	```
/// 5. **migrate Config**: move trait into the module with
///     * all const in `decl_module` to [`#[pallet::constant]`](#palletconstant)
///     * add the bound `IsType<<Self as frame_system::Config>::RuntimeEvent>` to `type
///       RuntimeEvent`
/// 7. **migrate decl_module**: write:
/// 	```ignore
/// 	#[pallet::hooks]
/// 	impl<T: Config> Hooks for Pallet<T> {
/// 	}
/// 	```
///     and write inside `on_initialize`, `on_finalize`, `on_runtime_upgrade`,
///     `offchain_worker`, and `integrity_test`.
///
/// 	then write:
/// 	```ignore
/// 	#[pallet::call]
/// 	impl<T: Config> Pallet<T> {
/// 	}
/// 	```
///     and write inside all the calls in `decl_module` with a few changes in the signature:
///     - origin must now be written completely, e.g. `origin: OriginFor<T>`
///     - result type must be `DispatchResultWithPostInfo`, you need to write it and also you
///    might need to put `Ok(().into())` at the end or the function.
///     - `#[compact]` must now be written
///       [`#[pallet::compact]`](#palletcompact-some_arg-some_type)
///     - `#[weight = ..]` must now be written [`#[pallet::weight(..)]`](#palletweightexpr)
///
/// 7. **migrate event**: rewrite as a simple enum with the attribute
///    [`#[pallet::event]`](#event-palletevent-optional), use [`#[pallet::generate_deposit($vis
///    fn deposit_event)]`](#event-palletevent-optional) to generate `deposit_event`,
/// 8. **migrate error**: rewrite it with attribute
///    [`#[pallet::error]`](#error-palleterror-optional).
/// 9. **migrate storage**: `decl_storage` provide an upgrade template (see 3.). All storages,
///    genesis config, genesis build and default implementation of genesis config can be taken
///    from it directly.
///
///     Otherwise here is the manual process:
///
///     first migrate the genesis logic. write:
/// 	```ignore
/// 	#[pallet::genesis_config]
/// 	struct GenesisConfig {
/// 		// fields of add_extra_genesis
/// 	}
/// 	impl Default for GenesisConfig {
/// 		// type default or default provided for fields
/// 	}
/// 	#[pallet::genesis_build]
/// 	impl<T: Config> GenesisBuild<T> for GenesisConfig {
/// 	// for instantiable pallet:
/// 	// `impl<T: Config, I: 'static> GenesisBuild<T, I> for GenesisConfig {
/// 		fn build() {
/// 			// The add_extra_genesis build logic
/// 		}
/// 	}
/// 	```
///     for each storage, if it contains `config(..)` then add fields, and make it default to
///     the value in `= ..;` or the type default if none, if it contains no build then also add
///     the logic to build the value. for each storage if it contains `build(..)` then add the
///     logic to `genesis_build`.
///
///     NOTE: within `decl_storage`: the individual config is executed first, followed by the
///     build and finally the `add_extra_genesis` build.
///
///     Once this is done you can migrate storages individually, a few notes:
///     - for private storage use `pub(crate) type ` or `pub(super) type` or nothing,
///     - for storages with `get(fn ..)` use [`#[pallet::getter(fn
///       ...)]`](#palletgetterfn-my_getter_fn_name-optional)
///     - for storages with value being `Option<$something>` make generic `Value` being
///       `$something` and generic `QueryKind` being `OptionQuery` (note: this is default).
///       Otherwise make `Value` the complete value type and `QueryKind` being `ValueQuery`.
///     - for storages with default value: `= $expr;` provide some specific `OnEmpty` generic.
///       To do so use of `#[pallet::type_value]` to generate the wanted struct to put.
///       example: `MyStorage: u32 = 3u32` would be written:
///
/// 	  	```ignore
/// 		#[pallet::type_value] fn MyStorageOnEmpty() -> u32 { 3u32 }
/// 		#[pallet::storage]
/// 		pub(super) type MyStorage<T> = StorageValue<_, u32, ValueQuery, MyStorageOnEmpty>;
/// 		```
///
///       NOTE: `decl_storage` also generates the functions `assimilate_storage` and
///       `build_storage` directly on `GenesisConfig`, and these are sometimes used in tests.
///       In order not to break they can be implemented manually, one can implement those
///       functions by calling the `GenesisBuild` implementation.
/// 10. **migrate origin**: move the origin to the pallet module to be under a
///     [`#[pallet::origin]`](#origin-palletorigin-optional) attribute
/// 11. **migrate validate_unsigned**: move the
///     [`ValidateUnsigned`](`pallet_prelude::ValidateUnsigned`) implementation to the pallet
///     module under a
///     [`#[pallet::validate_unsigned]`](#validate-unsigned-palletvalidate_unsigned-optional)
///     attribute
/// 12. **migrate provide_inherent**: move the
///     [`ProvideInherent`](`pallet_prelude::ProvideInherent`) implementation to the pallet
///     module under a [`#[pallet::inherent]`](#inherent-palletinherent-optional) attribute
/// 13. rename the usage of `Module` to `Pallet` inside the crate.
/// 14. migration is done, now double check the migration with the checking migration
///     guidelines shown below.
///
/// # Checking upgrade guidelines:
///
/// * compare metadata. Use [subsee](https://github.com/ascjones/subsee) to fetch the metadata
///   and do a diff of the resulting json before and after migration. This checks for:
/// 		* call, names, signature, docs
///     * event names, docs
///     * error names, docs
///     * storage names, hasher, prefixes, default value
///     * error, error, constant
/// * manually check that:
///     * `Origin` was moved inside the macro under
///       [`#[pallet::origin]`](#origin-palletorigin-optional) if it exists
///     * [`ValidateUnsigned`](`pallet_prelude::ValidateUnsigned`) was moved inside the macro
///       under
/// 	  [`#[pallet::validate_unsigned)]`](#validate-unsigned-palletvalidate_unsigned-optional)
/// 	  if it exists
///     * [`ProvideInherent`](`pallet_prelude::ProvideInherent`) was moved inside the macro
///       under [`#[pallet::inherent)]`](#inherent-palletinherent-optional) if it exists
///     * `on_initialize` / `on_finalize` / `on_runtime_upgrade` / `offchain_worker` were moved
///       to the `Hooks` implementation
///     * storages with `config(..)` were converted to `GenesisConfig` field, and their default
///       is `= $expr;` if the storage has a default value
///     * storages with `build($expr)` or `config(..)` were built in `GenesisBuild::build`
///     * `add_extra_genesis` fields were converted to `GenesisConfig` field with their correct
///       default if specified
///     * `add_extra_genesis` build was written into `GenesisBuild::build`
/// * storage items defined with [`pallet`] use the name of the pallet provided by
///   [`traits::PalletInfo::name`] as `pallet_prefix` (in `decl_storage`, storage items used
///   the `pallet_prefix` given as input of `decl_storage` with the syntax `as Example`). Thus
///   a runtime using the pallet must be careful with this change. To handle this change:
///     * either ensure that the name of the pallet given to `construct_runtime!` is the same
///       as the name the pallet was giving to `decl_storage`,
///     * or do a storage migration from the old prefix used to the new prefix used.
///
/// NOTE: The prefixes used by storage items are in metadata. Thus, ensuring the metadata
/// hasn't changed ensures that the `pallet_prefix`s used by the storage items haven't changed.
///
/// # Notes when macro fails to show proper error message spans:
///
/// Rustc loses span for some macro input. Some tips to fix it:
/// * do not use inner attribute:
/// 	```ignore
/// 	#[pallet]
/// 	pub mod pallet {
/// 		//! This inner attribute will make span fail
/// 		..
/// 	}
/// 	```
/// * use the newest nightly possible.
pub use frame_support_procedural::pallet;

/// Contains macro stubs for all of the pallet:: macros
pub mod pallet_macros {
	pub use frame_support_procedural::{
		composite_enum, config, disable_frame_system_supertrait_check, error, event,
		extra_constants, feeless_if, generate_deposit, generate_store, getter, hooks,
		import_section, inherent, no_default, no_default_bounds, origin, pallet_section,
		storage_prefix, storage_version, type_value, unbounded, validate_unsigned, weight,
		whitelist_storage,
	};

	/// Allows a pallet to declare a set of functions as a *dispatchable extrinsic*. In
	/// slightly simplified terms, this macro declares the set of "transactions" of a pallet.
	///
	/// > The exact definition of **extrinsic** can be found in
	/// > [`sp_runtime::generic::UncheckedExtrinsic`].
	///
	/// A **dispatchable** is a common term in FRAME, referring to process of constructing a
	/// function, and dispatching it with the correct inputs. This is commonly used with
	/// extrinsics, for example "an extrinsic has been dispatched". See
	/// [`sp_runtime::traits::Dispatchable`] and [`crate::traits::UnfilteredDispatchable`].
	///
	/// ## Call Enum
	///
	/// The macro is called `call` (rather than `#[pallet::extrinsics]`) because of the
	/// generation of a `enum Call`. This enum contains only the encoding of the function
	/// arguments of the dispatchable, alongside the information needed to route it to the
	/// correct function.
	///
	/// ```
	/// #[frame_support::pallet(dev_mode)]
	/// pub mod custom_pallet {
	/// #   use frame_support::pallet_prelude::*;
	/// #   use frame_system::pallet_prelude::*;
	/// #   #[pallet::config]
	/// #   pub trait Config: frame_system::Config {}
	/// #   #[pallet::pallet]
	/// #   pub struct Pallet<T>(_);
	/// #   use frame_support::traits::BuildGenesisConfig;
	///     #[pallet::call]
	///     impl<T: Config> Pallet<T> {
	///         pub fn some_dispatchable(_origin: OriginFor<T>, _input: u32) -> DispatchResult {
	///             Ok(())
	///         }
	///         pub fn other(_origin: OriginFor<T>, _input: u64) -> DispatchResult {
	///             Ok(())
	///         }
	///     }
	///
	///     // generates something like:
	///     // enum Call<T: Config> {
	///     //  some_dispatchable { input: u32 }
	///     //  other { input: u64 }
	///     // }
	/// }
	///
	/// fn main() {
	/// #   use frame_support::{derive_impl, construct_runtime};
	/// #   use frame_support::__private::codec::Encode;
	/// #   use frame_support::__private::TestExternalities;
	/// #   use frame_support::traits::UnfilteredDispatchable;
	/// #    impl custom_pallet::Config for Runtime {}
	/// #    #[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
	/// #    impl frame_system::Config for Runtime {
	/// #        type Block = frame_system::mocking::MockBlock<Self>;
	/// #    }
	///     construct_runtime! {
	///         pub enum Runtime {
	///             System: frame_system,
	///             Custom: custom_pallet
	///         }
	///     }
	///
	/// #    TestExternalities::new_empty().execute_with(|| {
	///     let origin: RuntimeOrigin = frame_system::RawOrigin::Signed(10).into();
	///     // calling into a dispatchable from within the runtime is simply a function call.
	///         let _ = custom_pallet::Pallet::<Runtime>::some_dispatchable(origin.clone(), 10);
	///
	///     // calling into a dispatchable from the outer world involves constructing the bytes of
	///     let call = custom_pallet::Call::<Runtime>::some_dispatchable { input: 10 };
	///     let _ = call.clone().dispatch_bypass_filter(origin);
	///
	///     // the routing of a dispatchable is simply done through encoding of the `Call` enum,
	///     // which is the index of the variant, followed by the arguments.
	///     assert_eq!(call.encode(), vec![0u8, 10, 0, 0, 0]);
	///
	///     // notice how in the encoding of the second function, the first byte is different and
	///     // referring to the second variant of `enum Call`.
	///     let call = custom_pallet::Call::<Runtime>::other { input: 10 };
	///     assert_eq!(call.encode(), vec![1u8, 10, 0, 0, 0, 0, 0, 0, 0]);
	///     #    });
	/// }
	/// ```
	///
	/// Further properties of dispatchable functions are as follows:
	///
	/// - Unless if annotated by `dev_mode`, it must contain [`weight`] to denote the
	///   pre-dispatch weight consumed.
	/// - The dispatchable must declare its index via [`call_index`], which can override the
	///   position of a function in `enum Call`.
	/// - The first argument is always an `OriginFor` (or `T::RuntimeOrigin`).
	/// - The return type is always [`crate::dispatch::DispatchResult`] (or
	///   [`crate::dispatch::DispatchResultWithPostInfo`]).
	///
	/// **WARNING**: modifying dispatchables, changing their order (i.e. using [`call_index`]),
	/// removing some, etc., must be done with care. This will change the encoding of the , and
	/// the call can be stored on-chain (e.g. in `pallet-scheduler`). Thus, migration might be
	/// needed. This is why the use of `call_index` is mandatory by default in FRAME.
	///
	/// ## Default Behavior
	///
	/// If no `#[pallet::call]` exists, then a default implementation corresponding to the
	/// following code is automatically generated:
	///
	/// ```ignore
	/// #[pallet::call]
	/// impl<T: Config> Pallet<T> {}
	/// ```
	pub use frame_support_procedural::call;

	/// Enforce the index of a variant in the generated `enum Call`. See [`call`] for more
	/// information.
	///
	/// All call indexes start from 0, until it encounters a dispatchable function with a
	/// defined call index. The dispatchable function that lexically follows the function with
	/// a defined call index will have that call index, but incremented by 1, e.g. if there are
	/// 3 dispatchable functions `fn foo`, `fn bar` and `fn qux` in that order, and only `fn
	/// bar` has a call index of 10, then `fn qux` will have an index of 11, instead of 1.
	pub use frame_support_procedural::call_index;

	/// Declares the arguments of a [`call`] function to be encoded using
	/// [`codec::Compact`]. This will results in smaller extrinsic encoding.
	///
	/// A common example of `compact` is for numeric values that are often times far far away
	/// from their theoretical maximum. For example, in the context of a crypto-currency, the
	/// balance of an individual account is oftentimes way less than what the numeric type
	/// allows. In all such cases, using `compact` is sensible.
	///
	/// ```
	/// #[frame_support::pallet(dev_mode)]
	/// pub mod custom_pallet {
	/// #   use frame_support::pallet_prelude::*;
	/// #   use frame_system::pallet_prelude::*;
	/// #   #[pallet::config]
	/// #   pub trait Config: frame_system::Config {}
	/// #   #[pallet::pallet]
	/// #   pub struct Pallet<T>(_);
	/// #   use frame_support::traits::BuildGenesisConfig;
	///     #[pallet::call]
	///     impl<T: Config> Pallet<T> {
	///         pub fn some_dispatchable(_origin: OriginFor<T>, #[pallet::compact] _input: u32) -> DispatchResult {
	///             Ok(())
	///         }
	///     }
	/// }
	pub use frame_support_procedural::compact;

	/// Allows you to define the genesis configuration for the pallet.
	///
	/// Item is defined as either an enum or a struct. It needs to be public and implement the
	/// trait [`frame_support::traits::BuildGenesisConfig`].
	///
	/// See [`genesis_build`] for an example.
	pub use frame_support_procedural::genesis_config;

	/// Allows you to define how the state of your pallet at genesis is built. This
	/// takes as input the `GenesisConfig` type (as `self`) and constructs the pallet's initial
	/// state.
	///
	/// The fields of the `GenesisConfig` can in turn be populated by the chain-spec.
	///
	/// ## Example
	///
	/// ```
	/// #[frame_support::pallet]
	/// pub mod pallet {
	/// # 	#[pallet::config]
	/// # 	pub trait Config: frame_system::Config {}
	/// # 	#[pallet::pallet]
	/// # 	pub struct Pallet<T>(_);
	/// # 	use frame_support::traits::BuildGenesisConfig;
	///     #[pallet::genesis_config]
	///     #[derive(frame_support::DefaultNoBound)]
	///     pub struct GenesisConfig<T: Config> {
	///         foo: Vec<T::AccountId>
	///     }
	///
	///     #[pallet::genesis_build]
	///     impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
	///         fn build(&self) {
	///             // use &self to access fields.
	///             let foo = &self.foo;
	///             todo!()
	///         }
	///     }
	/// }
	/// ```
	///
	/// ## Former Usage
	///
	/// Prior to <https://github.com/paritytech/substrate/pull/14306>, the following syntax was used.
	/// This is deprecated and will soon be removed.
	///
	/// ```
	/// #[frame_support::pallet]
	/// pub mod pallet {
	/// #     #[pallet::config]
	/// #     pub trait Config: frame_system::Config {}
	/// #     #[pallet::pallet]
	/// #     pub struct Pallet<T>(_);
	/// #     use frame_support::traits::GenesisBuild;
	///     #[pallet::genesis_config]
	///     #[derive(frame_support::DefaultNoBound)]
	///     pub struct GenesisConfig<T: Config> {
	/// 		foo: Vec<T::AccountId>
	/// 	}
	///
	///     #[pallet::genesis_build]
	///     impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
	///         fn build(&self) {
	///             todo!()
	///         }
	///     }
	/// }
	/// ```
	pub use frame_support_procedural::genesis_build;

	/// The `#[pallet::constant]` attribute can be used to add an associated type trait bounded
	/// by [`Get`](frame_support::pallet_prelude::Get) from [`pallet::config`](`macro@config`)
	/// into metadata.
	///
	/// ## Example
	///
	/// ```
	/// #[frame_support::pallet]
	/// mod pallet {
	///     use frame_support::pallet_prelude::*;
	///     # #[pallet::pallet]
	///     # pub struct Pallet<T>(_);
	///     #[pallet::config]
	///     pub trait Config: frame_system::Config {
	/// 		/// This is like a normal `Get` trait, but it will be added into metadata.
	/// 		#[pallet::constant]
	/// 		type Foo: Get<u32>;
	/// 	}
	/// }
	/// ```
	pub use frame_support_procedural::constant;

	/// Declares a type alias as a storage item. Storage items are pointers to data stored
	/// on-chain (the *blockchain state*), under a specific key. The exact key is dependent on
	/// the type of the storage.
	///
	/// > From the perspective of this pallet, the entire blockchain state is abstracted behind
	/// > a key-value api, namely [`sp_io::storage`].
	///
	/// ## Storage Types
	///
	/// The following storage types are supported by the `#[storage]` macro. For specific
	/// information about each storage type, refer to the documentation of the respective type.
	///
	/// * [`StorageValue`](crate::storage::types::StorageValue)
	/// * [`StorageMap`](crate::storage::types::StorageMap)
	/// * [`CountedStorageMap`](crate::storage::types::CountedStorageMap)
	/// * [`StorageDoubleMap`](crate::storage::types::StorageDoubleMap)
	/// * [`StorageNMap`](crate::storage::types::StorageNMap)
	/// * [`CountedStorageNMap`](crate::storage::types::CountedStorageNMap)
	///
	/// ## Storage Type Usage
	///
	/// The following details are relevant to all of the aforementioned storage types.
	/// Depending on the exact storage type, it may require the following generic parameters:
	///
	/// * [`Prefix`](#prefixes) - Used to give the storage item a unique key in the underlying
	///   storage.
	/// * `Key` - Type of the keys used to store the values,
	/// * `Value` - Type of the value being stored,
	/// * [`Hasher`](#hashers) - Used to ensure the keys of a map are uniformly distributed,
	/// * [`QueryKind`](#querykind) - Used to configure how to handle queries to the underlying
	///   storage,
	/// * `OnEmpty` - Used to handle missing values when querying the underlying storage,
	/// * `MaxValues` - _not currently used_.
	///
	/// Each `Key` type requires its own designated `Hasher` declaration, so that
	/// [`StorageDoubleMap`](frame_support::storage::types::StorageDoubleMap) needs two of
	/// each, and [`StorageNMap`](frame_support::storage::types::StorageNMap) needs `N` such
	/// pairs. Since [`StorageValue`](frame_support::storage::types::StorageValue) only stores
	/// a single element, no configuration of hashers is needed.
	///
	/// ### Syntax
	///
	/// Two general syntaxes are supported, as demonstrated below:
	///
	/// 1. Named type parameters, e.g., `type Foo<T> = StorageValue<Value = u32>`.
	/// 2. Positional type parameters, e.g., `type Foo<T> = StorageValue<_, u32>`.
	///
	/// In both instances, declaring the generic parameter `<T>` is mandatory. Optionally, it
	/// can also be explicitly declared as `<T: Config>`. In the compiled code, `T` will
	/// automatically include the trait bound `Config`.
	///
	/// Note that in positional syntax, the first generic type parameter must be `_`.
	///
	/// #### Example
	///
	/// ```
	/// #[frame_support::pallet]
	/// mod pallet {
	///     # use frame_support::pallet_prelude::*;
	///     # #[pallet::config]
	///     # pub trait Config: frame_system::Config {}
	///     # #[pallet::pallet]
	///     # pub struct Pallet<T>(_);
	///     /// Positional syntax, without bounding `T`.
	///     #[pallet::storage]
	///     pub type Foo<T> = StorageValue<_, u32>;
	///
	///     /// Positional syntax, with bounding `T`.
	///     #[pallet::storage]
	///     pub type Bar<T: Config> = StorageValue<_, u32>;
	///
	///     /// Named syntax.
	///     #[pallet::storage]
	///     pub type Baz<T> = StorageMap<Hasher = Blake2_128Concat, Key = u32, Value = u32>;
	/// }
	/// ```
	///
	/// ### QueryKind
	///
	/// Every storage type mentioned above has a generic type called
	/// [`QueryKind`](frame_support::storage::types::QueryKindTrait) that determines its
	/// "query" type. This refers to the kind of value returned when querying the storage, for
	/// instance, through a `::get()` method.
	///
	/// There are three types of queries:
	///
	/// 1. [`OptionQuery`](frame_support::storage::types::OptionQuery): The default query type.
	///    It returns `Some(V)` if the value is present, or `None` if it isn't, where `V` is
	///    the value type.
	/// 2. [`ValueQuery`](frame_support::storage::types::ValueQuery): Returns the value itself
	///    if present; otherwise, it returns `Default::default()`. This behavior can be
	///    adjusted with the `OnEmpty` generic parameter, which defaults to `OnEmpty =
	///    GetDefault`.
	/// 3. [`ResultQuery`](frame_support::storage::types::ResultQuery): Returns `Result<V, E>`,
	///    where `V` is the value type.
	///
	/// See [`QueryKind`](frame_support::storage::types::QueryKindTrait) for further examples.
	///
	/// ### Optimized Appending
	///
	/// All storage items — such as
	/// [`StorageValue`](frame_support::storage::types::StorageValue),
	/// [`StorageMap`](frame_support::storage::types::StorageMap), and their variants—offer an
	/// `::append()` method optimized for collections. Using this method avoids the
	/// inefficiency of decoding and re-encoding entire collections when adding items. For
	/// instance, consider the storage declaration `type MyVal<T> = StorageValue<_, Vec<u8>,
	/// ValueQuery>`. With `MyVal` storing a large list of bytes, `::append()` lets you
	/// directly add bytes to the end in storage without processing the full list. Depending on
	/// the storage type, additional key specifications may be needed.
	///
	/// #### Example
	#[doc = docify::embed!("src/lib.rs", example_storage_value_append)]
	/// Similarly, there also exists a `::try_append()` method, which can be used when handling
	/// types where an append operation might fail, such as a
	/// [`BoundedVec`](frame_support::BoundedVec).
	///
	/// #### Example
	#[doc = docify::embed!("src/lib.rs", example_storage_value_try_append)]
	/// ### Optimized Length Decoding
	///
	/// All storage items — such as
	/// [`StorageValue`](frame_support::storage::types::StorageValue),
	/// [`StorageMap`](frame_support::storage::types::StorageMap), and their counterparts —
	/// incorporate the `::decode_len()` method. This method allows for efficient retrieval of
	/// a collection's length without the necessity of decoding the entire dataset.
	/// #### Example
	#[doc = docify::embed!("src/lib.rs", example_storage_value_decode_len)]
	/// ### Hashers
	///
	/// For all storage types, except
	/// [`StorageValue`](frame_support::storage::types::StorageValue), a set of hashers needs
	/// to be specified. The choice of hashers is crucial, especially in production chains. The
	/// purpose of storage hashers in maps is to ensure the keys of a map are
	/// uniformly distributed. An unbalanced map/trie can lead to inefficient performance.
	///
	/// In general, hashers are categorized as either cryptographically secure or not. The
	/// former is slower than the latter. `Blake2` and `Twox` serve as examples of each,
	/// respectively.
	///
	/// As a rule of thumb:
	///
	/// 1. If the map keys are not controlled by end users, or are cryptographically secure by
	/// definition (e.g., `AccountId`), then the use of cryptographically secure hashers is NOT
	/// required.
	/// 2. If the map keys are controllable by the end users, cryptographically secure hashers
	/// should be used.
	///
	/// For more information, look at the types that implement
	/// [`frame_support::StorageHasher`](frame_support::StorageHasher).
	///
	/// Lastly, it's recommended for hashers with "concat" to have reversible hashes. Refer to
	/// the implementors section of
	/// [`hash::ReversibleStorageHasher`](frame_support::hash::ReversibleStorageHasher).
	///
	/// ### Prefixes
	///
	/// Internally, every storage type generates a "prefix". This prefix serves as the initial
	/// segment of the key utilized to store values in the on-chain state (i.e., the final key
	/// used in [`sp_io::storage`](sp_io::storage)). For all storage types, the following rule
	/// applies:
	///
	/// > The storage prefix begins with `twox128(pallet_prefix) ++ twox128(STORAGE_PREFIX)`,
	/// > where
	/// > `pallet_prefix` is the name assigned to the pallet instance in
	/// > [`frame_support::construct_runtime`](frame_support::construct_runtime), and
	/// > `STORAGE_PREFIX` is the name of the `type` aliased to a particular storage type, such
	/// > as
	/// > `Foo` in `type Foo<T> = StorageValue<..>`.
	///
	/// For [`StorageValue`](frame_support::storage::types::StorageValue), no additional key is
	/// required. For map types, the prefix is extended with one or more keys defined by the
	/// map.
	///
	/// #### Example
	#[doc = docify::embed!("src/lib.rs", example_storage_value_map_prefixes)]
	/// ## Related Macros
	///
	/// The following attribute macros can be used in conjunction with the `#[storage]` macro:
	///
	/// * [`macro@getter`]: Creates a custom getter function.
	/// * [`macro@storage_prefix`]: Overrides the default prefix of the storage item.
	/// * [`macro@unbounded`]: Declares the storage item as unbounded.
	///
	/// #### Example
	/// ```
	/// #[frame_support::pallet]
	/// mod pallet {
	///     # use frame_support::pallet_prelude::*;
	///     # #[pallet::config]
	///     # pub trait Config: frame_system::Config {}
	///     # #[pallet::pallet]
	///     # pub struct Pallet<T>(_);
	/// 	/// A kitchen-sink StorageValue, with all possible additional attributes.
	///     #[pallet::storage]
	/// 	#[pallet::getter(fn foo)]
	/// 	#[pallet::storage_prefix = "OtherFoo"]
	/// 	#[pallet::unbounded]
	///     pub type Foo<T> = StorageValue<_, u32, ValueQuery>;
	/// }
	/// ```
	pub use frame_support_procedural::storage;
	/// This attribute is attached to a function inside an `impl` block annoated with
	/// [`pallet::tasks_experimental`](`tasks_experimental`) to define the conditions for a
	/// given work item to be valid.
	///
	/// It takes a closure as input, which is then used to define the condition. The closure
	/// should have the same signature as the function it is attached to, except that it should
	/// return a `bool` instead.
	pub use frame_support_procedural::task_condition;
	/// This attribute is attached to a function inside an `impl` block annoated with
	/// [`pallet::tasks_experimental`](`tasks_experimental`) to define the index of a given
	/// work item.
	///
	/// It takes an integer literal as input, which is then used to define the index. This
	/// index should be unique for each function in the `impl` block.
	pub use frame_support_procedural::task_index;
	/// This attribute is attached to a function inside an `impl` block annoated with
	/// [`pallet::tasks_experimental`](`tasks_experimental`) to define an iterator over the
	/// available work items for a task.
	///
	/// It takes an iterator as input that yields a tuple with same types as the function
	/// arguments.
	pub use frame_support_procedural::task_list;
	/// This attribute is attached to a function inside an `impl` block annoated with
	/// [`pallet::tasks_experimental`](`tasks_experimental`) define the weight of a given work
	/// item.
	///
	/// It takes a closure as input, which should return a `Weight` value.
	pub use frame_support_procedural::task_weight;
	/// Allows you to define some service work that can be recognized by a script or an
	/// off-chain worker. Such a script can then create and submit all such work items at any
	/// given time.
	///
	/// These work items are defined as instances of the [`Task`](frame_support::traits::Task)
	/// trait. [`pallet:tasks_experimental`](`tasks_experimental`) when attached to an `impl`
	/// block inside a pallet, will generate an enum `Task<T>` whose variants are mapped to
	/// functions inside this `impl` block.
	///
	/// Each such function must have the following set of attributes:
	///
	/// * [`pallet::task_list`](`task_list`)
	/// * [`pallet::task_condition`](`task_condition`)
	/// * [`pallet::task_weight`](`task_weight`)
	/// * [`pallet::task_index`](`task_index`)
	///
	/// All of such Tasks are then aggregated into a `RuntimeTask` by
	/// [`construct_runtime`](frame_support::construct_runtime).
	///
	/// Finally, the `RuntimeTask` can then used by a script or off-chain worker to create and
	/// submit such tasks via an extrinsic defined in `frame_system` called `do_task`.
	///
	/// ## Example
	#[doc = docify::embed!("src/tests/tasks.rs", tasks_example)]
	/// Now, this can be executed as follows:
	#[doc = docify::embed!("src/tests/tasks.rs", tasks_work)]
	pub use frame_support_procedural::tasks_experimental;
}

#[deprecated(note = "Will be removed after July 2023; Use `sp_runtime::traits` directly instead.")]
pub mod error {
	#[doc(hidden)]
	pub use sp_runtime::traits::{BadOrigin, LookupError};
}

#[doc(inline)]
pub use frame_support_procedural::register_default_impl;

// Generate a macro that will enable/disable code based on `std` feature being active.
sp_core::generate_feature_enabled_macro!(std_enabled, feature = "std", $);

// Helper for implementing GenesisBuilder runtime API
pub mod genesis_builder_helper;

#[cfg(test)]
mod test {
	// use super::*;
	use crate::{
		hash::*,
		storage::types::{StorageMap, StorageValue, ValueQuery},
		traits::{ConstU32, StorageInstance},
		BoundedVec,
	};
	use sp_io::{hashing::twox_128, TestExternalities};

	struct Prefix;
	impl StorageInstance for Prefix {
		fn pallet_prefix() -> &'static str {
			"test"
		}
		const STORAGE_PREFIX: &'static str = "foo";
	}

	struct Prefix1;
	impl StorageInstance for Prefix1 {
		fn pallet_prefix() -> &'static str {
			"test"
		}
		const STORAGE_PREFIX: &'static str = "MyVal";
	}
	struct Prefix2;
	impl StorageInstance for Prefix2 {
		fn pallet_prefix() -> &'static str {
			"test"
		}
		const STORAGE_PREFIX: &'static str = "MyMap";
	}

	#[docify::export]
	#[test]
	pub fn example_storage_value_try_append() {
		type MyVal = StorageValue<Prefix, BoundedVec<u8, ConstU32<10>>, ValueQuery>;

		TestExternalities::default().execute_with(|| {
			MyVal::set(BoundedVec::try_from(vec![42, 43]).unwrap());
			assert_eq!(MyVal::get(), vec![42, 43]);
			// Try to append a single u32 to BoundedVec stored in `MyVal`
			assert_ok!(MyVal::try_append(40));
			assert_eq!(MyVal::get(), vec![42, 43, 40]);
		});
	}

	#[docify::export]
	#[test]
	pub fn example_storage_value_append() {
		type MyVal = StorageValue<Prefix, Vec<u8>, ValueQuery>;

		TestExternalities::default().execute_with(|| {
			MyVal::set(vec![42, 43]);
			assert_eq!(MyVal::get(), vec![42, 43]);
			// Append a single u32 to Vec stored in `MyVal`
			MyVal::append(40);
			assert_eq!(MyVal::get(), vec![42, 43, 40]);
		});
	}

	#[docify::export]
	#[test]
	pub fn example_storage_value_decode_len() {
		type MyVal = StorageValue<Prefix, BoundedVec<u8, ConstU32<10>>, ValueQuery>;

		TestExternalities::default().execute_with(|| {
			MyVal::set(BoundedVec::try_from(vec![42, 43]).unwrap());
			assert_eq!(MyVal::decode_len().unwrap(), 2);
		});
	}

	#[docify::export]
	#[test]
	pub fn example_storage_value_map_prefixes() {
		type MyVal = StorageValue<Prefix1, u32, ValueQuery>;
		type MyMap = StorageMap<Prefix2, Blake2_128Concat, u16, u32, ValueQuery>;
		TestExternalities::default().execute_with(|| {
			// This example assumes `pallet_prefix` to be "test"
			// Get storage key for `MyVal` StorageValue
			assert_eq!(
				MyVal::hashed_key().to_vec(),
				[twox_128(b"test"), twox_128(b"MyVal")].concat()
			);
			// Get storage key for `MyMap` StorageMap and `key` = 1
			let mut k: Vec<u8> = vec![];
			k.extend(&twox_128(b"test"));
			k.extend(&twox_128(b"MyMap"));
			k.extend(&1u16.blake2_128_concat());
			assert_eq!(MyMap::hashed_key_for(1).to_vec(), k);
		});
	}
}
