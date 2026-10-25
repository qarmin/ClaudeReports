use simple_asn1::ASN1Block;

fn main() {
    let a_s_n1_block = ASN1Block::BitString(0usize, 1usize, vec![]);
    let _ = simple_asn1::to_der(&a_s_n1_block);
}
