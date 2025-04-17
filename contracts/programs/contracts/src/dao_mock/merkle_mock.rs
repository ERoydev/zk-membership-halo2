use rs_merkle::{algorithms::Sha256, Hasher, MerkleProof, MerkleTree};


pub fn generate_merkle_tree_mock() -> [u8; 32] {
    let pubkeys = vec![
        "0x0000000000000000000000000000000000000001",
        "0x0000000000000000000000000000000000000002",
        "0x0000000000000000000000000000000000000003",
        "0x0000000000000000000000000000000000000004",
        "ERWLCA9bXkEtwHV5JMnobCYBWMKLReRmijDS7wkbHQUT", // user1.json Address
    ];
    
    let leaves: Vec<[u8; 32]> = pubkeys
        .iter()
        .map(|x| Sha256::hash(x.as_bytes()))
        .collect();

    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
    let indices_to_prove = vec![3, 4];
    let leaves_to_prove = leaves.get(3..5).ok_or("can't get leaves to prove").unwrap();
    let merkle_proof = merkle_tree.proof(&indices_to_prove);
    let merkle_root = merkle_tree.root().ok_or("couldn't get the merkle root").unwrap();
    // Serialize proof to pass it to the client
    let proof_bytes = merkle_proof.to_bytes();

    // Parse proof back on the client
    let proof = MerkleProof::<Sha256>::try_from(proof_bytes).unwrap();

    assert!(proof.verify(merkle_root, &indices_to_prove, leaves_to_prove, leaves.len()));

    merkle_root
}