// --- darwinia ---
use crate::{mock::*, *};
use frame_support::{assert_noop, assert_ok};

#[test]
fn test_build_eth_header_from_json_response() {
	let raw_header =
		from_utf8(&SUPPOSED_SHADOW_JSON_RESPONSE[47..SUPPOSED_SHADOW_JSON_RESPONSE.len() - 1])
			.unwrap_or_default();
	let header = EthHeader::from_str_unchecked(raw_header);
	// println!("{:?}", header);
	assert_eq!(header.hash.unwrap(), header.re_compute_hash());

	let mut response = SUPPOSED_SHADOW_JSON_RESPONSE.to_vec();
	EthOffchain::extract_proof(&mut response, true);
	// println!("{:?}", response);
	let double_node_with_proof_list =
		EthOffchain::parse_double_node_with_proof_list_from_json_str(&response[..]).unwrap();
	assert_eq!(1, double_node_with_proof_list.len());
}

#[test]
fn test_build_eth_header_from_scale_response() {
	let scale_decode_header =
		EthOffchain::parse_ethheader_from_scale_str(&SUPPOSED_SHADOW_SCALE_RESPONSE[..]);
	let header = EthHeader::from_str_unchecked(SUPPOSED_ETHHEADER);
	assert_eq!(scale_decode_header, header);

	let mut response = SUPPOSED_SHADOW_SCALE_RESPONSE.to_vec();
	EthOffchain::extract_proof(&mut response, false);
	assert_eq!(260, response.len()); // 260 = (129 + 1) * 2

	let decoded_double_node_with_proof =
		EthOffchain::parse_double_node_with_proof_list_from_scale_str(&response[..]).unwrap();

	assert_eq!(
		vec![DoubleNodeWithMerkleProof::default()],
		decoded_double_node_with_proof
	);
}

#[test]
fn test_should_error_when_best_header_not_set() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(EthOffchain::relay_header(), OffchainError::BestHeaderNE);
	});
}

/// Test offchain worker with different shadow service
/// `set_shadow_service` is unsafe
/// Keep this test run in a single theread
#[test]
fn test_should_handle_different_shadow_service() {
	set_shadow_service(None);
	ExtBuilder::default()
		.set_genesis_header()
		.build()
		.execute_with(|| {
			assert_noop!(EthOffchain::relay_header(), OffchainError::APIRespUnexp);
		});

	set_shadow_service(Some(ShadowService::SCALE));
	ExtBuilder::default()
		.set_genesis_header()
		.build()
		.execute_with(|| {
			assert_ok!(EthOffchain::relay_header());
		});

	set_shadow_service(Some(ShadowService::JSON));
	ExtBuilder::default()
		.set_genesis_header()
		.build()
		.execute_with(|| {
			assert_ok!(EthOffchain::relay_header());
		});
}
