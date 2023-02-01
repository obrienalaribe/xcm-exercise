## Activities

### Build XCM configurations for requirements

Modify different parts of the [parachain runtime](../xcm-related-code/xcm-simulator-for-exercises/src/parachain.rs) configuration to accomplish the following goals:

- Remove free execution from relay and add a trader to charge for fee
- Modify your chain to be a 20 byte account instead of a 32 byte account.
  - **Hint: You can use sp_core::H160 as your account 20 type.**
- Change the configuration to accept teleporting instead of reserve transfer assets.
- Add pallet-assets to the parachain and add an asset transactor for it.

Write some tests with the simulator to prove the aforementioned behavior
