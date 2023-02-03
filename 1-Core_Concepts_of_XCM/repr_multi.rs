// 1
// Relay Chain's location 
// Relay chain pov
MultiLocation { 
    parent: 0, 
    interior: Junctions::Here 
};
// Para1000 pov 
MultiLocation {
    parent: 1, 
    interior: Junctions::Here
}
// Para1001 pov
MultiLocation {
    parent: 1,
    interior: Junctions::Here
}

// 2
// Para1000's location
// Relay chain pov
MultiLocaiton {
    parent: 0,
    interior: Junctions::X1(Junction::Parachain(1000))
}
// Para1000 pov
MultiLocation {
    parent: 0,
    interior: Junctions::Here
}
// Para1001 pov
MultiLocation {
    parent: 1,
    interior: Junctions::X1(Junction::Parachain(1000))
}

// 3
// 32 byte in in relay chain
// Relay chain pov
MultiLocation {
    parent: 0,
    interior: Junctions::X1(Junction::AccountId32 { network: NetworkId::Polkadot, id: vec![] })
}
// Para1000 pov
MultiLocaiton {
    parent: 1,
    interior: Juntions::X1(Junction::AccountId32 { network: , id: })
}
// Para1001 pov
MultiLocaiton {
    parent: 1,
    interior: Juntions::X1(Junction::AccountId32 { network: , id: })
}

// 4
// 20 byte smart contract address in parachain 1000
// Relay chain pov
MultiLocation {
    parent: 0,
    interior: Junctions::X2(Junction::Parachain(1000), Junction::AccountKey20 { network: NetworkId::Any, id: vec![]})
}
// Para1000 pov
MultiLocation {
    parent: 0,
    interior: Junctions::X1(Junction::AccountKey20 { network: , id: })
}
// Para1001 pov
MultiLocation {
    parent: 1,
    interior: Junctions::X2(Junction::Parachain(1000), Juntion::AccountIdKey20 { network: ,id: })
}

// 5 
// An asset whose Id is 1 from pallet instance 50 in parachain 1001
// Relay Chain pov
MultiLocation {
    parent: 0,
    interior: Junctions::X3(Junction::Parachain(1001), Junction::PalletInstance(50), Junction::GeneralIndex(1))
}
// Para1000 pov
MultiLocation {
    parent: 1,
    interior: Junctions::X3(Junction::Parachain(1001), Junction::PalletIndex(50), Junction::GeneralIndex(1))
}
// Para1001 pov
MultiLocation {
    parent: 0,
    interior: Junctions::X2(Junction::PalletIndex(50), Junction::GeneralIndex(1))
}