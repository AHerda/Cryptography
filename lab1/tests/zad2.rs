use lab1::consts::{
    DIFF_M0, DIFF_M1_1, DIFF_M1_2, EXPECTED_HASH1, EXPECTED_HASH2, M0_1, M0_2, M0_PRIM_1, M0_PRIM_2, M1_1, M1_2, M1_PRIM_1, M1_PRIM_2
};
use lab1::md5::Md5;

#[test]
fn test_collision_one_from_table2_v1() {
    let m0 = M0_1;
    let m0_prim = M0_PRIM_1;
    for i in 0..16 {
        assert_eq!(m0[i] as i64 - m0_prim[i] as i64, DIFF_M0[i], "Error at index: {}", i);
    }
    let m1 = M1_1;
    let m1_prim = M1_PRIM_1;
    for i in 0..16 {
        assert_eq!(m1[i] as i64 - m1_prim[i] as i64, DIFF_M1_1[i], "Error at index: {}", i);
    }
    let expected_hash = EXPECTED_HASH1;
    assert_ne!(m1, m1_prim);

    let iv = Md5::new_raw_block(&m0);
    let iv_prim = Md5::new_raw_block(&m0_prim);

    let h = Md5::new_with_state_raw_block(&m1, iv.get_state());
    let h_prim = Md5::new_with_state_raw_block(&m1_prim, iv_prim.get_state());

    assert_eq!(h.to_str(), h_prim.to_str());
    assert_eq!(h.to_str_be(), expected_hash);
    assert_eq!(h_prim.to_str_be(), expected_hash);
}

#[test]
fn test_collision_one_from_table2_v2() {
    let m0 = M0_2;
    let m0_prim = M0_PRIM_2;
    for i in 0..16 {
        assert_eq!(m0[i] as i64 - m0_prim[i] as i64, DIFF_M0[i], "Error at index: {}", i);
    }
    let m1 = M1_2;
    let m1_prim = M1_PRIM_2;
    for i in 0..16 {
        assert_eq!(m1[i] as i64 - m1_prim[i] as i64, DIFF_M1_2[i], "Error at index: {}", i);
    }
    let expected_hash = EXPECTED_HASH2;
    assert_ne!(m1, m1_prim);

    let iv = Md5::new_raw_block(&m0);
    let iv_prim = Md5::new_raw_block(&m0_prim);

    let h = Md5::new_with_state_raw_block(&m1, iv.get_state());
    let h_prim = Md5::new_with_state_raw_block(&m1_prim, iv_prim.get_state());

    assert_eq!(h.to_str(), h_prim.to_str());
    assert_eq!(h.to_str_be(), expected_hash);
    assert_eq!(h_prim.to_str_be(), expected_hash);
}
