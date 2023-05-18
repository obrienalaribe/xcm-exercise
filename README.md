# XCM Activities

Repository to hold xcm activities and exercises for PBA.

## Structure

Inside each folder module you will either find a `Module_Activities.md` file describing the activity/coding exercise or you will find a coding exercise itself to be done.

## Using `xcm-simulator`

For most of these exercises you will be using the [`xcm-simulator`](https://github.com/paritytech/polkadot/tree/master/xcm/xcm-simulator).
The `xcm-simulator` allows you to create a test environment in which to test messages received between two runtimes.

You can use the `decl_test_parachain` macro to declare a parachain specifying the `runtime`, `router` and test-externality to use.

You can use the `decl_test_relay_chain` macro to declare a parachain specifying the `runtime`, `xcmConfig` and test-externality to use.

Finally the `decl_test_network` macro allows you to create a network with the parachains and relay declared.

You can take a look at the [main example](xcm-related-code/xcm-simulator-for-exercises/src/lib.rs)
for the exercises to get a sense on how the simulator works.
# xcm-exercise
# xcm-exercise
