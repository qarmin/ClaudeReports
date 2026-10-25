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
    g_printerr ("could not create element %s\n", elem_name);
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
