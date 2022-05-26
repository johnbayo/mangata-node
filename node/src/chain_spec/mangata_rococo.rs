use crate::chain_spec::Extensions;
use artemis_core::{App, AppId};
use codec::Encode;
use cumulus_primitives_core::ParaId;
use hex::FromHex;
use hex_literal::hex;
use mangata_rococo_runtime::{AccountId, AuraId, Signature, VersionedMultiLocation, ROC_TOKEN_ID};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, ByteArray, Pair, Public, H160};
use sp_runtime::traits::{IdentifyAccount, Verify};

pub mod public_testnet_keys {
	pub const ALICE_SR25519: &str =
		"0x76e810a1f116b779fea0962f4102e9acb3d22ba603999ecb08ad163582de192c";
	pub const BOB_SR25519: &str =
		"0x2eec12f5af27c95fd5661323e544dc857155ea50689eaccdfd580af7c1be921c";
	pub const CHARLIE_SR25519: &str =
		"0xbc443b4f7023de2d868f74f9e51159961482dc46f76aa90a1c6ce58efff4be6a";
	pub const SUDO_SR25519: &str =
		"0x249af97c2ab99f229cdf18cc966833b894ae7c4d94c13fa86341209c64c8ec18";
	pub const RELAY_SR25519: &str =
		"0x7481b06f37b3500bb6ec8d569d2cede4ffcb151daee75f7de20c5bda2e22bb13";
}

pub mod kusama_mainnet_keys {
	pub const ALICE_SR25519: &str =
		"0x02d3074216e37c4e96c3a35be89fa27b6022fe08f02051989ed5b94768e69652";
	pub const BOB_SR25519: &str =
		"0xac1d5ec7cf53260c5ea1bb6be0d4fd8b23c50c088fad593de7cb60f76de4fe21";
	pub const CHARLIE_SR25519: &str =
		"0x708dbfb26bdf220b53443ff823da3e28845ddbd0d0aab1babd6074ac99b7b254";
	pub const SUDO_SR25519: &str =
		"0x8080dc038d21840c3139140f0fa982b3882c67fc3e558eae7dec4f5f63d11237";
	pub const RELAY_SR25519: &str =
		"0x2ac2b810caa998b14207a8fc6414a94c833974dc482f11a25a2264508c9dff40";
}

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
	sc_service::GenericChainSpec<mangata_rococo_runtime::GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_public_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_public_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_public_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn mangata_session_keys(keys: AuraId) -> mangata_rococo_runtime::SessionKeys {
	mangata_rococo_runtime::SessionKeys { aura: keys }
}

pub fn public_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "MGAT".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Mangata Public Testnet",
		// ID
		"mangata_public_testnet",
		ChainType::Live,
		move || {
			mangata_genesis(
				// initial collators.
				vec![
					(
						public_testnet_keys::ALICE_SR25519.parse::<AccountId>().unwrap().into(),
						AuraId::from_slice(
							&<[u8; 32]>::from_hex(
								public_testnet_keys::ALICE_SR25519.strip_prefix("0x").unwrap(),
							)
							.unwrap(),
						)
						.unwrap(),
					),
					(
						public_testnet_keys::BOB_SR25519.parse::<AccountId>().unwrap().into(),
						AuraId::from_slice(
							&<[u8; 32]>::from_hex(
								public_testnet_keys::BOB_SR25519.strip_prefix("0x").unwrap(),
							)
							.unwrap(),
						)
						.unwrap(),
					),
				],
				// Initial relay account
				public_testnet_keys::RELAY_SR25519.parse::<AccountId>().unwrap().into(),
				// Sudo account
				public_testnet_keys::SUDO_SR25519.parse::<AccountId>().unwrap().into(),
				// Ethereum AppId for SnowBridged Assets
				vec![
					(
						App::ETH,
						H160::from_slice(&hex!["6aA07B0e455B393164414380A8A314d7c860CEC8"][..])
							.into(),
					),
					(
						App::ERC20,
						H160::from_slice(&hex!["244691D3822e13e61968322f8d82Dee3B31e0D4a"][..])
							.into(),
					),
				],
				// SnowBridged Assets
				vec![
					(
						b"Mangata".to_vec(),
						b"MGA".to_vec(),
						b"Mangata Asset".to_vec(),
						18u32,
						0u32,
						H160::from_slice(&hex!["C7e3Bda797D2cEb740308eC40142ae235e08144A"][..]),
						300_000_000__000_000_000_000_000_000u128,
						public_testnet_keys::ALICE_SR25519.parse::<AccountId>().unwrap().into(),
					),
					(
						b"Ether".to_vec(),
						b"ETH".to_vec(),
						b"Ethereum Ether".to_vec(),
						18u32,
						1u32,
						H160::zero(),
						0u128,
						public_testnet_keys::ALICE_SR25519.parse::<AccountId>().unwrap().into(),
					),
				],
				// Tokens endowment
				vec![
					(
						0u32,
						400_000_000__000_000_000_000_000_000u128,
						public_testnet_keys::SUDO_SR25519.parse::<AccountId>().unwrap().into(),
					),
					(
						0u32,
						100_000_000__000_000_000_000_000_000u128,
						public_testnet_keys::RELAY_SR25519.parse::<AccountId>().unwrap().into(),
					),
					(
						0u32,
						100_000_000__000_000_000_000_000_000u128,
						public_testnet_keys::BOB_SR25519.parse::<AccountId>().unwrap().into(),
					),
					(
						0u32,
						100_000_000__000_000_000_000_000_000u128,
						public_testnet_keys::CHARLIE_SR25519.parse::<AccountId>().unwrap().into(),
					),
				],
				// Config for Staking
				// Make sure it works with initial-authorities as staking uses both
				vec![
					(
						// Who gets to stake initially
						public_testnet_keys::ALICE_SR25519.parse::<AccountId>().unwrap().into(),
						// Id of MGA token,
						0u32,
						// How much mangata they pool
						10_000__000_000_000_000_000_000u128,
						// Id of the dummy token,
						2u32,
						// How many dummy tokens they pool,
						20_000__000_000_000_000_000_000u128,
						// Id of the liquidity token that is generated
						3u32,
						// How many liquidity tokens they stake,
						10_000__000_000_000_000_000_000u128,
					),
					(
						// Who gets to stake initially
						public_testnet_keys::BOB_SR25519.parse::<AccountId>().unwrap().into(),
						// Id of MGA token,
						0u32,
						// How much mangata they pool
						8_000__000_000_000_000_000_000u128,
						// Id of the dummy token,
						2u32,
						// How many dummy tokens they pool,
						20_000__000_000_000_000_000_000u128,
						// Id of the liquidity token that is generated
						3u32,
						// How many liquidity tokens they stake,
						5_000__000_000_000_000_000_000u128,
					),
				],
				vec![(ROC_TOKEN_ID, None)],
				2000.into(),
			)
		},
		Vec::new(),
		None,
		// Protocol ID
		Some("mangata-public-testnet"),
		// ForkId
		None,
		// Properties
		Some(properties),
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 2000,
		},
	)
}

type BridgedAssetsType = Vec<(Vec<u8>, Vec<u8>, Vec<u8>, u32, u32, H160, u128, AccountId)>;

fn mangata_genesis(
	initial_authorities: Vec<(AccountId, AuraId)>,
	relay_key: AccountId,
	root_key: AccountId,
	bridged_app_ids: Vec<(App, AppId)>,
	bridged_assets: BridgedAssetsType,
	tokens_endowment: Vec<(u32, u128, AccountId)>,
	staking_accounts: Vec<(AccountId, u32, u128, u32, u128, u32, u128)>,
	xcm_tokens: Vec<(u32, Option<VersionedMultiLocation>)>,
	id: ParaId,
) -> mangata_rococo_runtime::GenesisConfig {
	mangata_rococo_runtime::GenesisConfig {
		system: mangata_rococo_runtime::SystemConfig {
			code: mangata_rococo_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		tokens: mangata_rococo_runtime::TokensConfig {
			tokens_endowment: tokens_endowment
				.iter()
				.cloned()
				.map(|(token_id, amount, account)| (account, token_id, amount))
				.collect(),
			created_tokens_for_staking: {
				let mut created_tokens_for_staking_token_1: Vec<(AccountId, u32, u128)> =
					staking_accounts
						.iter()
						.cloned()
						.map(|x| {
							let (who, _, _, token_id, initial_amount, _, _) = x;
							(who.clone(), token_id, initial_amount)
						})
						.collect();
				let mut created_tokens_for_staking_token_2: Vec<(AccountId, u32, u128)> =
					staking_accounts
						.iter()
						.cloned()
						.map(|x| {
							let (who, token_id, initial_amount, _, _, _, _) = x;
							(who.clone(), token_id, initial_amount)
						})
						.collect();
				created_tokens_for_staking_token_1.append(&mut created_tokens_for_staking_token_2);
				created_tokens_for_staking_token_1
			},
		},
		treasury: Default::default(),
		parachain_info: mangata_rococo_runtime::ParachainInfoConfig { parachain_id: id },
		parachain_staking: mangata_rococo_runtime::ParachainStakingConfig {
			candidates: staking_accounts
				.iter()
				.map(|x| {
					let (account_id, _, _, _, _, liquidity_token_id, liquidity_token_amount) = x;
					(account_id.clone(), *liquidity_token_amount, *liquidity_token_id)
				})
				.collect(),
			delegations: vec![],
		},
		session: mangata_rococo_runtime::SessionConfig {
			keys: initial_authorities
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                // account id
						acc,                        // validator id
						mangata_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		assets_info: mangata_rococo_runtime::AssetsInfoConfig {
			bridged_assets_info: bridged_assets
				.iter()
				.cloned()
				.map(|x| {
					let (name, token, description, decimals, asset_id, ..) = x;
					(Some(name), Some(token), Some(description), Some(decimals), asset_id)
				})
				.collect(),
		},
		xyk: mangata_rococo_runtime::XykConfig {
			created_pools_for_staking: staking_accounts
				.iter()
				.map(|x| {
					let (
						account_id,
						native_token_id,
						native_token_amount,
						pooled_token_id,
						pooled_token_amount,
						liquidity_token_id,
						_,
					) = x;
					(
						account_id.clone(),
						*native_token_id,
						*native_token_amount,
						*pooled_token_id,
						*pooled_token_amount,
						*liquidity_token_id,
					)
				})
				.collect(),
		},
		bridge: mangata_rococo_runtime::BridgeConfig { bridged_app_id_registry: bridged_app_ids },
		bridged_asset: mangata_rococo_runtime::BridgedAssetConfig {
			bridged_assets_links: bridged_assets
				.iter()
				.cloned()
				.map(|x| {
					let (.., asset_id, bridged_asset_id, initial_supply, initial_owner) = x;
					(asset_id, bridged_asset_id, initial_supply, initial_owner)
				})
				.collect(),
		},
		verifier: mangata_rococo_runtime::VerifierConfig { key: relay_key },
		council: Default::default(),
		elections: mangata_rococo_runtime::ElectionsConfig {
			members: tokens_endowment
				.iter()
				.cloned()
				.map(|(_, _, member)| (member, 100 * 100_000_000_000_000))
				.collect(),
		},
		sudo: mangata_rococo_runtime::SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		polkadot_xcm: mangata_rococo_runtime::PolkadotXcmConfig { safe_xcm_version: Some(2) },
		asset_registry: mangata_rococo_runtime::AssetRegistryConfig {
			init_xcm_tokens: xcm_tokens
				.iter()
				.cloned()
				.map(|(x, maybe_y)| {
					if let Some(y) = maybe_y {
						(x, Some(VersionedMultiLocation::encode(&y)))
					} else {
						(x, None)
					}
				})
				.collect(),
		},
	}
}
