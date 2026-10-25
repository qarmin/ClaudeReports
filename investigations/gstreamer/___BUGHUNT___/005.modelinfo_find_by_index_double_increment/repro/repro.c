/* Real, public-API reproduction of the modelinfo_find_tensor_name_by_index()
 * double-increment bug: for two output tensors with identical dims/type,
 * asking for the tensor at index 1 (by the public
 * gst_analytics_modelinfo_find_tensor_name() API, exactly as tflite/onnx
 * inference elements call it to resolve which modelinfo section describes
 * which model output) incorrectly returns the tensor at index 0 instead.
 */
#include <stdio.h>
#include <glib.h>
#include <gst/gst.h>
#include <gst/analytics/modelinfo.h>

int
main (void)
{
  const char *modelinfo_path = "/tmp/bughunt_modelinfo_repro.tflite.modelinfo";
  const char *model_path = "/tmp/bughunt_modelinfo_repro.tflite";
  const char *content =
      "[modelinfo]\n"
      "version=1.1\n"
      "\n"
      "[out0]\n"
      "type=float32\n"
      "dims=1,10\n"
      "dir=output\n"
      "\n"
      "[out1]\n"
      "type=float32\n"
      "dims=1,10\n"
      "dir=output\n";
  GstAnalyticsModelInfo *mi;
  gsize dims[2] = { 1, 10 };
  gchar *found0, *found1;
  int rc = 0;

  gst_init (NULL, NULL);

  g_file_set_contents (modelinfo_path, content, -1, NULL);

  mi = gst_analytics_modelinfo_load (model_path);
  if (!mi) {
    fprintf (stderr, "FAIL: could not load modelinfo\n");
    return 1;
  }

  found0 = gst_analytics_modelinfo_find_tensor_name (mi,
      MODELINFO_DIRECTION_OUTPUT, 0, NULL, GST_TENSOR_DATA_TYPE_FLOAT32, 2,
      dims);
  found1 = gst_analytics_modelinfo_find_tensor_name (mi,
      MODELINFO_DIRECTION_OUTPUT, 1, NULL, GST_TENSOR_DATA_TYPE_FLOAT32, 2,
      dims);

  printf ("index 0 -> %s (expected: out0)\n", found0 ? found0 : "(null)");
  printf ("index 1 -> %s (expected: out1)\n", found1 ? found1 : "(null)");

  if (g_strcmp0 (found0, "out0") != 0) {
    printf ("MISMATCH at index 0\n");
    rc = 1;
  }
  if (g_strcmp0 (found1, "out1") != 0) {
    printf ("MISMATCH at index 1: got '%s', expected 'out1'\n",
        found1 ? found1 : "(null)");
    rc = 1;
  }

  g_free (found0);
  g_free (found1);
  gst_analytics_modelinfo_free (mi);
  remove (modelinfo_path);

  return rc;
}
