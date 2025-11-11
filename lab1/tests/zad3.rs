use lab1::consts::{
    self, DIFF_M0, DIFF_M1
};
use lab1::my_collision;
use lab1::md5::Md5;

#[test]
fn test_collision_one_from_table2_v1() {
    let m0 = consts::M0_1;
    let m0_prim = consts::M0_PRIM_1;
    for i in 0..16 {
        assert_eq!(
            (m0[i] as i64).wrapping_add(DIFF_M0[i]) % (1 << 32),
            m0_prim[i] as i64,
            "Error at index: {}",
            i
        );
    }
    let m1 = my_collision::M1;
    let m1_prim = my_collision::M1_PRIM;
    for i in 0..16 {
        assert_eq!(
            (m1[i] as i64).wrapping_add(DIFF_M1[i]) % (1 << 32),
            m1_prim[i] as i64,
            "Error at index: {}",
            i
        );
    }
    assert_ne!(m1, m1_prim);

    let iv = Md5::new_raw_block(&m0);
    let iv_prim = Md5::new_raw_block(&m0_prim);

    let h = Md5::new_with_state_raw_block(&m1, iv.get_state());
    let h_prim = Md5::new_with_state_raw_block(&m1_prim, iv_prim.get_state());

    assert_eq!(h.to_str(), h_prim.to_str());
}
