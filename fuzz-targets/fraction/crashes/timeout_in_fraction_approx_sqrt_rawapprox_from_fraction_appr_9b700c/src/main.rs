use fraction::approx::sqrt::RawApprox;

fn main() {
    let _ = fraction::generic::read_generic_integer::<fraction::BigUint, u16>(22u16);
    let _ = RawApprox::from(<fraction::prelude::GenericFraction<u16> as fraction::Zero>::zero().sqrt_abs_raw(4080515371u32));
}
