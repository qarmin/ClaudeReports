# pdf-rs - crashes

Source: https://github.com/pdf-rs/pdf @ 333ec00750b0943f2287e1775c5bdbfb80a6a545

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut no_update = NoUpdate;
let _ = &lt;pdf::object::NoUpdate as pdf::object::Updater&gt;::update_with::&lt;pdf::
    object::LabParams&gt;(&amp;mut no_update,
    &lt;pdf::object::Ref&lt;pdf::object::Stream&lt;()&gt;&gt;&gt;::from_id(0u64).get_inner(),
    LabParams { white_point: vec![], black_point: None, range: None },
    UpdateOptions::default());</pre></td><td valign="top"><pre>thread 'main' panicked at pdf/src/object/mod.rs:156:9:
explicit panic
backtrace:
  &lt;pdf::object::NoUpdate as pdf::object::Updater&gt;::update_with::&lt;pdf::object::color::LabParams&gt; (pdf/src/object/mod.rs:156:9)
  repro_pdf_src_object_mod_rs_156_9_4255e7::main (src/main.rs:7:13)</pre></td></tr>
</table>
