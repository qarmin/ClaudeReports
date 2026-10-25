use object::write::macho::CodeSignatureEncoder;

fn main() {
    let code_signature_encoder = CodeSignatureEncoder;
    let _ = code_signature_encoder.code_directory_size(<object::macho::CsVersion as object::Wrap>::from_inner(1000000000u32));
}
