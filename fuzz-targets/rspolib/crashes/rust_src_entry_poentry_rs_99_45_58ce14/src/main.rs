use rspolib::POEntry;

fn main() {
    let p_o_entry = POEntry { msgid: "".to_string(), msgstr: None, msgid_plural: None, msgstr_plural: vec![], msgctxt: None, obsolete: false, comment: Some("a".to_string()), tcomment: None, occurrences: vec![], flags: vec![], previous_msgid: None, previous_msgid_plural: None, previous_msgctxt: None, linenum: 0usize };
    let _ = p_o_entry.to_string_with_wrapwidth(0usize);
}
