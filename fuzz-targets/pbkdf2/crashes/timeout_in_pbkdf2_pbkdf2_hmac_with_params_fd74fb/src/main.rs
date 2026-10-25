use pbkdf2::Algorithm;
use pbkdf2::Params;

fn main() {
    let params = Params::recommended_for(Algorithm::default());
    let algorithm = Algorithm::Pbkdf2Sha256;
    pbkdf2::pbkdf2_hmac_with_params(&vec![], &vec![], algorithm, params, &mut include_bytes!("image_0.webp").to_vec());
}
