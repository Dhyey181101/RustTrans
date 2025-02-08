
use std::fmt;

const GEO_S2_NUM_FACES: u64 = 6;
const GEO_S2_POS_BITS: u64 = 2 * GEO_S2_MAX_LEVEL + 1;
const GEO_S2_MAX_LEVEL: u64 = 30;

fn geo_s2_cell_id_string(ci: u64) -> String {
    if !geo_s2_cell_id_is_valid(ci) {
        return format!("Invalid: {:#x}", ci);
    }
    let mut b = String::new();
    b.push(match geo_s2_cell_id_face(ci) {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        _ => unreachable!(),
    });
    b.push('/');
    for level in 1..=geo_s2_cell_id_level(ci) {
        b.push(match geo_s2_cell_id_child_position(ci, level) {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            _ => unreachable!(),
        });
    }
    b
}

fn geo_s2_cell_id_is_valid(ci: u64) -> bool {
    geo_s2_cell_id_face(ci) < GEO_S2_NUM_FACES
        && (geo_s2_cell_id_lsb(ci) & 0x1555555555555555 != 0)
}

fn geo_s2_cell_id_face(ci: u64) -> u64 {
    (ci >> GEO_S2_POS_BITS) as u64
}

fn geo_s2_cell_id_lsb(ci: u64) -> u64 {
    ci & (!ci).wrapping_add(1)
}

fn geo_s2_cell_id_child_position(ci: u64, level: u64) -> u64 {
    ((ci >> (2 * (GEO_S2_MAX_LEVEL - level) + 1)) & 3) as u64
}

fn geo_s2_cell_id_level(ci: u64) -> u64 {
    GEO_S2_MAX_LEVEL - geo_s2_find_lsb_set_non_zero_64(ci) >> 1
}

fn geo_s2_find_lsb_set_non_zero_64(x: u64) -> u64 {
    if x == 0 {
        0
    } else {
        x.trailing_zeros() as u64
    }
}

fn u64_to_string(value: u64) -> String {
    geo_s2_cell_id_string(value)
}
