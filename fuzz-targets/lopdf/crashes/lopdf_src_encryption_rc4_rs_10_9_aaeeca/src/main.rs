use lopdf::encryption::crypt_filters::Rc4CryptFilter;

fn main() {
    let _ = <lopdf::encryption::crypt_filters::Rc4CryptFilter as lopdf::encryption::crypt_filters::CryptFilter>::decrypt(&Rc4CryptFilter, &vec![], &vec![]);
}
