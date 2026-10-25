use object::macho::CS_EXECSEG_MAIN_BINARY;
use object::macho::CS_EXEC_SET_KILL;
use object::macho::CS_HASHTYPE_SHA384;
use object::macho::CsVersion;
use object::write::macho::CodeDirectory;
use object::write::macho::CodeSignatureEncoder;

fn main() {
    let code_signature_encoder = CodeSignatureEncoder;
    code_signature_encoder.code_directory::<Vec<u8>>(&mut vec![], &CodeDirectory { length: 0u32, version: CsVersion(1000000000u32), flags: CS_EXEC_SET_KILL, hash_offset: 0u32, ident_offset: 0u32, n_special_slots: 0u32, n_code_slots: 0u32, code_limit: 0u64, hash_size: 0u8, hash_type: CS_HASHTYPE_SHA384, platform: 0u8, page_size: 0u8, scatter_offset: 0u32, team_offset: 0u32, exec_seg_base: 0u64, exec_seg_limit: 0u64, exec_seg_flags: CS_EXECSEG_MAIN_BINARY });
}
