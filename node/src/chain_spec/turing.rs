use cumulus_primitives_core::ParaId;
use sc_service::ChainType;
use sp_core::sr25519;
use sp_runtime::{Perbill, Percent};

use crate::chain_spec::{
	get_account_id_from_seed,
	get_collator_keys_from_seed,
	// inflation_config,
	DummyChainSpec,
	Extensions,
};
use codec::Encode;
use common_runtime::{
	config::orml_asset_registry::{AssetMetadataOf, StringLimit},
	constants::currency::{DOLLAR, TOKEN_DECIMALS},
};
use frame_support::{traits::ConstU32, BoundedVec};
use primitives::{assets::CustomMetadata, AccountId, AuraId, Balance, TokenId};
use turing_runtime::{
	AssetRegistryConfig,
	// ValveConfig, VestingConfig,
	CollatorSelectionConfig,
	CouncilConfig,
	PolkadotXcmConfig,
	TechnicalMembershipConfig,
};
use xcm::{prelude::*, VersionedMultiLocation::V3};

const TOKEN_SYMBOL: &str = "TUR";
const SS_58_FORMAT: u32 = 51;
static RELAY_CHAIN: &str = "rococo-local";
const REGISTERED_PARA_ID: u32 = 2114;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<turing_runtime::GenesisConfig, Extensions>;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: AuraId) -> turing_runtime::SessionKeys {
	turing_runtime::SessionKeys { aura: keys }
}

pub fn turing_development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), TOKEN_SYMBOL.into());
	properties.insert("tokenDecimals".into(), TOKEN_DECIMALS.into());
	properties.insert("ss58Format".into(), SS_58_FORMAT.into());

	ChainSpec::from_genesis(
		// Name
		"Turing Development",
		// ID
		"turing-dev",
		ChainType::Development,
		move || {
			let accounts = vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			];
			const ALLOC_TOKENS_TOTAL: u128 = DOLLAR * 58_000_000;
			let initial_balance: u128 = ALLOC_TOKENS_TOTAL / accounts.len() as u128;
			let endowed_accounts: Vec<(AccountId, Balance)> =
				accounts.iter().cloned().map(|k| (k, initial_balance)).collect();

			testnet_genesis(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				endowed_accounts,
				REGISTERED_PARA_ID.into(),
				vec![],
				vec![],
				vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
				],
				vec![
					(
						1,
						orml_asset_registry::AssetMetadata::<Balance, CustomMetadata, StringLimit>::encode(
							&AssetMetadataOf {
								decimals: 18,
								name:  BoundedVec::truncate_from(b"Mangata Rococo".to_vec()),
								symbol: BoundedVec::truncate_from(b"MGR".to_vec()),
								existential_deposit: Default::default(),
								location: Some(
									MultiLocation::new(
										1,
										X2(
											Parachain(2110),
											GeneralKey { length: 4, data: [0; 32] },
										),
									)
									.into(),
								),
								additional: CustomMetadata {
									fee_per_second: Some(416_000_000_000),
									conversion_rate: None,
								},
							},
						),
					),
					(
						2,
						orml_asset_registry::AssetMetadata::<Balance, CustomMetadata, StringLimit>::encode(
							&AssetMetadataOf {
								decimals: 18,
								name: BoundedVec::truncate_from(b"Rocstar".to_vec()),
								symbol: BoundedVec::truncate_from(b"RSTR".to_vec()),
								existential_deposit: 10_000_000_000_000_000,
								location: Some(MultiLocation::new(1, X1(Parachain(2006))).into()),
								additional: CustomMetadata {
									fee_per_second: Some(416_000_000_000),
									conversion_rate: None,
								},
							},
						),
					),
					(
						3,
						orml_asset_registry::AssetMetadata::<Balance, CustomMetadata, StringLimit>::encode(
							&AssetMetadataOf {
								decimals: 18,
								name: BoundedVec::truncate_from(b"Shiden".to_vec()),
								symbol: BoundedVec::truncate_from(b"SDN".to_vec()),
								existential_deposit: 10_000_000_000_000_000,
								location: Some(MultiLocation::new(1, X1(Parachain(2007))).into()),
								additional: CustomMetadata {
									fee_per_second: Some(416_000_000_000),
									conversion_rate: None,
								},
							},
						),
					),
					(
						4,
						orml_asset_registry::AssetMetadata::<Balance, CustomMetadata, StringLimit>::encode(
							&AssetMetadataOf {
								decimals: 18,
								name: BoundedVec::truncate_from(b"Shibuya".to_vec()),
								symbol: BoundedVec::truncate_from(b"SBY".to_vec()),
								existential_deposit: 10_000_000_000_000_000,
								location: Some(MultiLocation::new(1, X1(Parachain(2000))).into()),
								additional: CustomMetadata {
									fee_per_second: Some(416_000_000_000),
									conversion_rate: None,
								},
							},
						),
					),
					(
						5,
						orml_asset_registry::AssetMetadata::<Balance, CustomMetadata, StringLimit>::encode(
							&AssetMetadataOf {
								decimals: 18,
								name: BoundedVec::truncate_from(b"Moonbase".to_vec()),
								symbol: BoundedVec::truncate_from(b"DEV".to_vec()),
								existential_deposit: 1,
								location: Some(
									MultiLocation::new(1, X2(Parachain(1000), PalletInstance(3)))
										.into(),
								),
								additional: CustomMetadata {
									fee_per_second: Some(10_000_000_000_000_000_000),
									conversion_rate: None,
								},
							},
						),
					),
				],
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions {
			relay_chain: RELAY_CHAIN.into(), // You MUST set this to the correct network!
			para_id: REGISTERED_PARA_ID,
		},
	)
}

pub fn turing_staging() -> Result<DummyChainSpec, String> {
	DummyChainSpec::from_json_bytes(&include_bytes!("../../res/turing-staging.json")[..])
}

pub fn turing_live() -> Result<DummyChainSpec, String> {
	DummyChainSpec::from_json_bytes(&include_bytes!("../../res/turing.json")[..])
}

const NUM_SELECTED_CANDIDATES: u32 = 6;
fn testnet_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<(AccountId, Balance)>,
	para_id: ParaId,
	pallet_gates_closed: Vec<Vec<u8>>,
	vesting_schedule: Vec<(u64, Vec<(AccountId, Balance)>)>,
	general_councils: Vec<AccountId>,
	technical_memberships: Vec<AccountId>,
	additional_assets: Vec<(TokenId, Vec<u8>)>,
) -> turing_runtime::RuntimeGenesisConfig {
	let candidate_stake = turing_runtime::MinCandidateStk::get();

	let assets = [
		vec![(
			0,
			orml_asset_registry::AssetMetadata::<Balance, CustomMetadata, StringLimit>::encode(
				&AssetMetadataOf {
					decimals: TOKEN_DECIMALS,
					name: BoundedVec::truncate_from(b"Native".to_vec()),
					symbol: BoundedVec::truncate_from(TOKEN_SYMBOL.as_bytes().to_vec()),
					existential_deposit: 100_000_000,
					location: Some(V3(MultiLocation { parents: 0, interior: Here })),
					additional: CustomMetadata {
						fee_per_second: Some(416_000_000_000),
						conversion_rate: None,
					},
				},
			),
		)],
		additional_assets,
	]
	.concat();

	let last_asset_id = assets.iter().map(|asset| asset.0).max().expect("At least 1 item!");

	turing_runtime::RuntimeGenesisConfig {
		system: turing_runtime::SystemConfig {
			code: turing_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			..Default::default()
		},
		balances: turing_runtime::BalancesConfig { balances: endowed_accounts },
		parachain_info: turing_runtime::ParachainInfoConfig {
			parachain_id: para_id,
			..Default::default()
		},
		session: turing_runtime::SessionConfig {
			keys: invulnerables
				.iter()
				.cloned()
				.map(|(acc, aura)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
						template_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		// parachain_staking: turing_runtime::ParachainStakingConfig {
		// 	candidates: invulnerables
		// 		.iter()
		// 		.cloned()
		// 		.map(|(acc, _)| (acc, candidate_stake))
		// 		.collect(),
		// 	delegations: vec![],
		// 	inflation_config: inflation_config(600, 5),
		// 	blocks_per_round: 600,
		// 	collator_commission: Perbill::from_percent(20),
		// 	parachain_bond_reserve_percent: Percent::from_percent(30),
		// 	num_selected_candidates: NUM_SELECTED_CANDIDATES,
		// },
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		council: CouncilConfig { members: general_councils, phantom: Default::default() },
		democracy: Default::default(),
		tokens: Default::default(),
		technical_committee: Default::default(),
		technical_membership: TechnicalMembershipConfig {
			members: technical_memberships.try_into().unwrap(),
			phantom: Default::default(),
		},
		parachain_system: Default::default(),
		polkadot_xcm: PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
			..Default::default()
		},
		treasury: Default::default(),
		// valve: ValveConfig { start_with_valve_closed: false, closed_gates: pallet_gates_closed },
		// vesting: VestingConfig { vesting_schedule },
		asset_registry: AssetRegistryConfig { assets, last_asset_id },
		collator_selection: CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: common_runtime::constants::currency::EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::chain_spec::test::{validate_allocation, validate_total_tokens, validate_vesting};
	use common_runtime::constants::currency::EXISTENTIAL_DEPOSIT;

	#[test]
	fn validate_turing_allocation() {
		let allocation_json = &include_bytes!("../../../distribution/turing_alloc.json")[..];
		let initial_allocation: Vec<(AccountId, Balance)> =
			serde_json::from_slice(allocation_json).unwrap();
		const EXPECTED_ALLOC_TOKENS_TOTAL: u128 = DOLLAR * 58_000_000;
		validate_allocation(initial_allocation, EXPECTED_ALLOC_TOKENS_TOTAL, EXISTENTIAL_DEPOSIT);
	}

	#[test]
	fn validate_turing_vesting() {
		let vesting_json = &include_bytes!("../../../distribution/turing_vesting.json")[..];
		let initial_vesting: Vec<(u64, Vec<(AccountId, Balance)>)> =
			serde_json::from_slice(vesting_json).unwrap();

		let vested_tokens = 9_419_999_999_999_999_919;
		let vest_starting_time: u64 = 1651431600;
		let vest_ending_time: u64 = 1743534000;
		validate_vesting(
			initial_vesting,
			vested_tokens,
			EXISTENTIAL_DEPOSIT,
			vest_starting_time,
			vest_ending_time,
		);
	}

	#[test]
	fn validate_total_turing_tokens() {
		let allocation_json = &include_bytes!("../../../distribution/turing_alloc.json")[..];
		let initial_allocation: Vec<(AccountId, Balance)> =
			serde_json::from_slice(allocation_json).unwrap();

		let vesting_json = &include_bytes!("../../../distribution/turing_vesting.json")[..];
		let initial_vesting: Vec<(u64, Vec<(AccountId, Balance)>)> =
			serde_json::from_slice(vesting_json).unwrap();

		let expected_vested_tokens = 9_419_999_999_999_999_919;
		let expected_allocated_tokens = DOLLAR * 58_000_000;
		let expected_total_tokens = expected_vested_tokens + expected_allocated_tokens;
		validate_total_tokens(initial_allocation, initial_vesting, expected_total_tokens);
	}
}
