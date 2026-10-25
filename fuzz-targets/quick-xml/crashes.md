# quick-xml - crashes

Source: https://github.com/tafia/quick-xml @ 2eaa844b4b40117fb7ed4d1f512ede918125aeab

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let se_error = SeError::Custom("\u{200b}".to_string());
let string = se_error.to_string();
let namespace_resolver = NamespaceResolver::default();
let mut attributes = Attributes::html(&amp;string, 1usize);
let _ = attributes.has_nil(&amp;namespace_resolver);</pre></td><td valign="top"><pre>thread 'main' panicked at quick-xml/src/events/attributes.rs:604:59:
start byte index 1 is not a char boundary; it is inside '\u{200b}' (bytes 0..3 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;quick_xml::events::attributes::Attributes as core::iter::traits::iterator::Iterator&gt;::next::{closure#0} (quick-xml/src/events/attributes.rs:604:59)
  &lt;quick_xml::events::attributes::Attr&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::map::&lt;&amp;str, &lt;quick_xml::events::attributes::Attributes as core::iter::traits::iterator::Iterator&gt;::next::{closure#0}&gt; (quick-xml/src/events/attributes.rs:780:45)
  &lt;quick_xml::events::attributes::Attributes as core::iter::traits::iterator::Iterator&gt;::next (quick-xml/src/events/attributes.rs:604:38)
  &lt;quick_xml::events::attributes::Attributes as core::iter::traits::iterator::Iterator&gt;::try_fold::&lt;(), core::iter::traits::iterator::Iterator::any::check&lt;core::result::Result&lt;quick_xml::events::attributes::Attribute, quick_xml::events::attributes::AttrError&gt;, &lt;quick_xml::events::attributes::Attribu... (+47 chars)
  &lt;quick_xml::events::attributes::Attributes as core::iter::traits::iterator::Iterator&gt;::any::&lt;&lt;quick_xml::events::attributes::Attributes&gt; (core/src/iter/traits/iterator.rs:2893:14)
  &lt;quick_xml::events::attributes::Attributes&gt; (quick-xml/src/events/attributes.rs:572:14)
  repro_quick_xml_src_events_attributes_rs_604_59_a28d4d::main (src/main.rs:10:24)</pre></td></tr>
</table>
