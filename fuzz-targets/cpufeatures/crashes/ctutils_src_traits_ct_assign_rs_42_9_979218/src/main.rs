use ctutils::Choice;

fn main() {
    <ctutils::Choice as ctutils::CtAssignSlice>::ct_assign_slice(&mut vec![Choice::from_u32_lsb(1u32)], &[], Choice::FALSE);
}
