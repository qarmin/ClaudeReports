use object::macho::CS_EXECSEG_SKIP_LV;
use object::macho::CS_RESTRICT;
use object::macho::CS_SUPPORTSSCATTER;
use object::write::macho::CodeDirectory;
use object::write::macho::CodeSignatureEncoder;

fn main() {
    let code_signature_encoder = CodeSignatureEncoder;
    code_signature_encoder.code_directory::<Vec<u8>>(&mut vec![], &CodeDirectory { length: 0u32, version: CS_SUPPORTSSCATTER, flags: CS_RESTRICT, hash_offset: 0u32, ident_offset: 0u32, n_special_slots: 0u32, n_code_slots: 0u32, code_limit: u64::MAX, hash_size: 255u8, hash_type: <object::macho::CsHashType as object::Wrap>::from_inner(72u8), platform: 1u8, page_size: 191u8, scatter_offset: 4077522095u32, team_offset: 32u32, exec_seg_base: 0u64, exec_seg_limit: 4294967295u64, exec_seg_flags: CS_EXECSEG_SKIP_LV });
}
