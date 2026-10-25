#include <gst/gst.h>
#include <gst/analytics/modelinfo.h>
#include <gst/analytics/gsttensor.h>
#include <stdio.h>

int
main (int argc, char **argv)
{
  gst_init (&argc, &argv);

  /* Crafted .modelinfo file: the [modelinfo] header section stray-sets a
   * "type" and "dims" key that happen to match what the caller searches
   * for below. modelinfo_find_tensor_name_by_dims() iterates ALL groups
   * including the header (no GST_MODELINFO_SECTION_NAME exclusion, unlike
   * its sibling modelinfo_find_tensor_name_by_index(), see candidate #005),
   * and modelinfo_check_direction() defaults to TRUE when a group has no
   * "dir" key at all (true for the header) - so the header passes
   * validation and gets returned as if it were a real tensor. */
  const char *content =
      "[modelinfo]\n"
      "version=1.1\n"
      "type=float32\n"
      "dims=224,224\n";

  const char *base = "/tmp/modelinfo_repro_model";
  gchar *filename = g_strconcat (base, ".modelinfo", NULL);
  g_file_set_contents (filename, content, -1, NULL);

  GstAnalyticsModelInfo *info = gst_analytics_modelinfo_load (base);
  g_assert (info != NULL);

  gsize dims[2] = { 224, 224 };
  gchar *tensor_name = gst_analytics_modelinfo_find_tensor_name (info,
      MODELINFO_DIRECTION_INPUT, 999 /* no tensor at this index */ ,
      NULL, GST_TENSOR_DATA_TYPE_FLOAT32, 2, dims);

  g_print ("gst_analytics_modelinfo_find_tensor_name() returned: %s\n",
      tensor_name ? tensor_name : "(NULL)");
  if (tensor_name && g_str_equal (tensor_name, "modelinfo")) {
    g_print ("BUG: the format-version HEADER section was returned as if it "
        "were a real tensor name!\n");
  }

  g_free (tensor_name);
  gst_analytics_modelinfo_free (info);
  g_free (filename);
  gst_deinit ();
  return 0;
}
