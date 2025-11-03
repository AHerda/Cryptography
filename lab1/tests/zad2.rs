use lab1::md5::Md5;

#[test]
fn test_collision_one_from_table2_v1() {
    let m0 = vec![
        0x2dd31d1, 0xc4eee6c5, 0x69a3d69, 0x5cf9af98,
        0x87b5ca2f, 0xab7e4612, 0x3e580440, 0x897ffbb8,
        0x634ad55, 0x2b3f409, 0x8388e483, 0x5a417125,
        0xe8255108, 0x9fc9cdf7, 0xf2bd1dd9, 0x5b3c3780,
    ];
    let m0_prim = vec![
        0x2dd31d1, 0xc4eee6c5, 0x69a3d69, 0x5cf9af98,
        0x7b5ca2f, 0xab7e4612, 0x3e580440, 0x897ffbb8,
        0x634ad55, 0x2b3f409, 0x8388e483, 0x5a41f125,
        0xe8255108, 0x9fc9cdf7, 0x72bd1dd9, 0x5b3c3780,
    ];
    let m1 = vec![
        0xd11d0b96, 0x9c7b41dc, 0xf497d8e4, 0xd555655a,
        0xc79a7335, 0xcfdebf0, 0x66f12930, 0x8fb109d1,
        0x797f2775, 0xeb5cd530, 0xbaade822, 0x5c15cc79,
        0xddcb74ed, 0x6dd3c55f, 0xd80a9bb1, 0xe3a7cc35,
    ];
    let m1_prim = vec![
        0xd11d0b96, 0x9c7b41dc, 0xf497d8e4, 0xd555655a,
        0x479a7335, 0xcfdebf0, 0x66f12930, 0x8fb109d1,
        0x797f2775, 0xeb5cd530, 0xbaade822, 0x5c154c79,
        0xddcb74ed, 0x6dd3c55f, 0x580a9bb1, 0xe3a7cc35,
    ];
    assert_ne!(m1, m1_prim);

    let iv = Md5::new_raw_block(m0);
    let iv_prim = Md5::new_raw_block(m0_prim);

    let h = Md5::new_with_state_raw_block(m1, iv.get_state());
    let h_prim = Md5::new_with_state_raw_block(m1_prim, iv_prim.get_state());

    let expected_hash = "9603161f a30f9dbf 9f65ffbc f41fc7ef".replace(" ", "");
    
    assert_eq!(h.to_str(), h_prim.to_str());
    assert_eq!(h.to_str_be(), expected_hash);
    assert_eq!(h_prim.to_str_be(), expected_hash);
}

#[test]
fn test_collision_one_from_table2_v2() {
    let m0 = vec![
        0x2dd31d1, 0xc4eee6c5, 0x69a3d69, 0x5cf9af98,
        0x87b5ca2f, 0xab7e4612, 0x3e580440, 0x897ffbb8,
        0x634ad55, 0x2b3f409, 0x8388e483, 0x5a417125,
        0xe8255108, 0x9fc9cdf7, 0xf2bd1dd9, 0x5b3c3780,
    ];
    let m0_prim = vec![
        0x2dd31d1, 0xc4eee6c5, 0x69a3d69, 0x5cf9af98,
        0x7b5ca2f, 0xab7e4612, 0x3e580440, 0x897ffbb8,
        0x634ad55, 0x2b3f409, 0x8388e483, 0x5a41f125,
        0xe8255108, 0x9fc9cdf7, 0x72bd1dd9, 0x5b3c3780,
    ];
    let m1 = vec![
        0x313e82d8, 0x5b8f3456, 0xd4ac6dae, 0xc619c936,
        0xb4e253dd, 0xfd03da87, 0x6633902, 0xa0cd48d2,
        0x42339fe9, 0xe87e570f, 0x70b654ce, 0x1e0da880,
        0xbc2198c6, 0x9383a8b6, 0x2b65f996, 0x702af76f,
    ];
    let m1_prim = vec![
        0x313e82d8, 0x5b8f3456, 0xd4ac6dae, 0xc619c936,
        0x34e253dd, 0xfd03da87, 0x6633902, 0xa0cd48d2,
        0x42339fe9, 0xe87e570f, 0x70b654ce, 0x1e0d2880,
        0xbc2198c6, 0x9383a8b6, 0xab65f996, 0x702af76f,
    ];
    assert_ne!(m1, m1_prim);

    let iv = Md5::new_raw_block(m0);
    let iv_prim = Md5::new_raw_block(m0_prim);

    let h = Md5::new_with_state_raw_block(m1, iv.get_state());
    let h_prim = Md5::new_with_state_raw_block(m1_prim, iv_prim.get_state());

    let expected_hash = "8d5e7019 61804e08 715d6b58 6324c015".replace(" ", "");

    assert_eq!(h.to_str(), h_prim.to_str());
    assert_eq!(h.to_str_be(), expected_hash);
    assert_eq!(h_prim.to_str_be(), expected_hash);
}