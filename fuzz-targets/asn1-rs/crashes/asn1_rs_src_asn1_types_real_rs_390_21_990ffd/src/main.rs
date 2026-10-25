use asn1_rs::Real;

fn main() {
    let _ = <asn1_rs::Real as asn1_rs::ToDer>::der_content_len(&Real::binary(-0.0f64, 2u32, 0i32));
}
