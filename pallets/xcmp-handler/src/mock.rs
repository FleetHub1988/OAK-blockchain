// This file is part of OAK Blockchain.

// Copyright (C) 2022 OAK Network
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate as pallet_xcmp_handler;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	parameter_types,
	traits::{Everything, GenesisBuild},
};
use frame_system as system;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, Convert, IdentityLookup},
	AccountId32, RuntimeDebug,
};
use xcm::latest::prelude::*;
use xcm_builder::LocationInverter;

pub const ALICE: AccountId32 = AccountId32::new([0u8; 32]);

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = AccountId32;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		ParachainInfo: parachain_info::{Pallet, Storage, Config},
		XcmpHandler: pallet_xcmp_handler::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 51;
}

impl system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl parachain_info::Config for Test {}

#[derive(
	Encode,
	Decode,
	Deserialize,
	Eq,
	PartialEq,
	Copy,
	Clone,
	RuntimeDebug,
	PartialOrd,
	Serialize,
	Ord,
	TypeInfo,
	MaxEncodedLen,
)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CurrencyId {
	Native,
	ROC,
	UNIT,
}

pub struct AccountIdToMultiLocation;
impl Convert<AccountId, MultiLocation> for AccountIdToMultiLocation {
	fn convert(account: AccountId) -> MultiLocation {
		X1(Junction::AccountId32 { network: NetworkId::Any, id: account.into() }).into()
	}
}

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = CurrencyId::Native;
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}

impl pallet_xcmp_handler::Config for Test {
	type Event = Event;
	type CurrencyId = CurrencyId;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type SelfParaId = parachain_info::Pallet<Test>;
	type AccountIdToMultiLocation = AccountIdToMultiLocation;
	type LocationInverter = LocationInverter<Ancestry>;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.expect("Frame system builds valid default genesis config");

	GenesisBuild::<Test>::assimilate_storage(
		&parachain_info::GenesisConfig { parachain_id: 2114u32.into() },
		&mut t,
	)
	.expect("Pallet Parachain info can be assimilated");

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}