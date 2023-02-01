// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

#[cfg(test)]
mod tests {
	use codec::Encode;
	use frame_support::assert_ok;
	use xcm::{latest::prelude::*, VersionedXcm};
	use xcm_simulator_for_exercises::{
		parachain, parachain_xcm_executed_successfully, MockNet, ParaA, ParachainPalletBalances,
		ParachainPalletXcm, TestExt, ALICE, BOB, INITIAL_BALANCE,
	};

	#[test]
	fn execute_withdraw_asset() {
		// Execute a new testnet
		MockNet::reset();

		// Asset MultiLocation to use:
		// Parachain Pallet Balances tied to: MultiLocation { parents: 1, interior: Here }
		let withdraw_amount = 100u128;
		let asset_location = MultiLocation { parents: 1, interior: Here };

		ParaA::execute_with(|| {
			// The message we will execute
			let message: Xcm<parachain::RuntimeCall> =
				Xcm(vec![WithdrawAsset((asset_location, withdraw_amount).into())]);

			// Execute the message
			// First parameter is origin
			// Second parameter is versioned message (you can use V2 for now)
			// Third parameter is the max weight (you can hardcode it to 100_000_000_000 for now)
			assert_ok!(ParachainPalletXcm::execute(
				parachain::RuntimeOrigin::signed(ALICE),
				Box::new(VersionedXcm::V2(message.into())),
				100_000_000_000
			));

			// Assert XCM message executed successfully
			assert!(parachain_xcm_executed_successfully());

			// Ensure the balance has been withdrawn
			assert_eq!(
				ParachainPalletBalances::free_balance(ALICE),
				INITIAL_BALANCE - withdraw_amount
			);
		});
	}

	#[test]
	fn execute_buy_execution() {
		MockNet::reset();

		// Task
		// Create a BuyExecution Instruction that buys 1e12 amount of weight
		// Hint: Our chain charges 1 token per 1e12 amount of weight
		// 1.) Who is buying the execution?
		// 2.) How to verify we executed this instruction correctly?
		let weight = 1_000_000_000_000u64;

		ParaA::execute_with(|| {
			// Insert here the appropriate code to execute the XCM message asked for.
		});
	}

	#[test]
	fn execute_send_funds_to_bob() {
		MockNet::reset();

		// Task
		// Send 100 from Alice to Bob from a parachain locally and verify it.
		// 1.) Where to send it from?
		// 2.) how to verify Bob received the funds?

		ParaA::execute_with(|| {
			// Insert here the appropriate code to execute the XCM message asked for.
		});
	}

	#[test]
	fn execute_transact_message() {
		MockNet::reset();

		// Task
		// Create a call which we can dispatch locally
		// 1.) What kinds of calls do we have to choose from?
		//      (Hint system pallet might be useful here but any call can do(Which we can verify))
		// Hint for encoding: https://github.com/paritytech/substrate/blob/master/frame/democracy/src/tests.rs#L223
		// 2.) How can we verify the call was dispatched correctly?(Hint events could be a way :)

		ParaA::execute_with(|| {
			// Insert Here the appropriate code to execute the XCM message asked for.
		});
	}

	#[test]
	fn execute_origin_mutating_instruction() {
		MockNet::reset();

		// Task
		// 1.) Modify the origin register with the DescendOrigin instruction
		// 2.) Clear the origin of the register
		// 3.) To evaluate: Does it work if you append a withdrawAsset after origin mutations?

		ParaA::execute_with(|| {
			// Insert Here the appropriate code to execute the XCM message asked for.
		});
	}

	#[test]
	fn execute_transact_on_own_dispatchable() {
		MockNet::reset();

		// Task
		// 1.) Add your own dispatchable in the `parachain` module via your own custom pallet or in
		// an      existing pallet
		// 2.) Construct the call for your new dispatchable
		// 3.) Send a Transact instruction for your new dispatchable
		// 4.) Verify the call was dispatched

		ParaA::execute_with(|| {
			// Insert Here the appropriate code to execute the XCM message asked for.
		});
	}
}
