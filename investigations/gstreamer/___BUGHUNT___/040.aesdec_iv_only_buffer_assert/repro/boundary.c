/* Boundary-value + regression check for the aesdec fix (candidate 040):
 *   - 15-byte first buffer (serialize-iv): must still be gracefully rejected
 *     (GST_ELEMENT_ERROR "Cipher text too short"), not crash - this path was
 *     never broken, just re-verifying the fix didn't touch it.
 *   - 16-byte first buffer (IV only, zero ciphertext): must now succeed with
 *     EOS and zero-length output, not crash (the actual bug fixed here).
 *   - Full aesenc|aesdec round trip on ordinary, non-block-aligned data: must
 *     decrypt back to the exact original plaintext (functional regression
 *     check for the per_buffer_padding && plaintext_len > 0 guard).
 */
#include <gst/gst.h>
#include <gst/app/gstappsrc.h>
#include <gst/app/gstappsink.h>
#include <stdlib.h>
#include <string.h>

static const char *KEY = "1f9423681beb9a79215820f6bda73d0f";

static int
run_short_first_buffer (guint size, gboolean expect_error)
{
  GstElement *pipeline = gst_pipeline_new (NULL);
  GstElement *appsrc = gst_element_factory_make ("appsrc", NULL);
  GstElement *aesdec = gst_element_factory_make ("aesdec", NULL);
  GstElement *sink = gst_element_factory_make ("fakesink", NULL);

  g_object_set (aesdec, "key", KEY, NULL);
  g_object_set (aesdec, "serialize-iv", TRUE, NULL);
  g_object_set (appsrc, "format", GST_FORMAT_BYTES, NULL);

  gst_bin_add_many (GST_BIN (pipeline), appsrc, aesdec, sink, NULL);
  gst_element_link_many (appsrc, aesdec, sink, NULL);
  gst_element_set_state (pipeline, GST_STATE_PLAYING);

  guint8 *data = g_malloc0 (size);
  GstBuffer *buf = gst_buffer_new_allocate (NULL, size, NULL);
  gst_buffer_fill (buf, 0, data, size);
  g_free (data);

  GstFlowReturn fret;
  g_signal_emit_by_name (appsrc, "push-buffer", buf, &fret);
  gst_buffer_unref (buf);
  g_signal_emit_by_name (appsrc, "end-of-stream", &fret);

  GstBus *bus = gst_element_get_bus (pipeline);
  GstMessage *msg = gst_bus_timed_pop_filtered (bus, 2 * GST_SECOND,
      GST_MESSAGE_ERROR | GST_MESSAGE_EOS);
  gboolean got_error = FALSE, got_eos = FALSE;
  if (msg) {
    if (GST_MESSAGE_TYPE (msg) == GST_MESSAGE_ERROR) {
      GError *err = NULL;
      gst_message_parse_error (msg, &err, NULL);
      g_print ("  size=%u: ERROR: %s\n", size, err->message);
      g_error_free (err);
      got_error = TRUE;
    } else {
      g_print ("  size=%u: EOS\n", size);
      got_eos = TRUE;
    }
    gst_message_unref (msg);
  } else {
    g_print ("  size=%u: no message within timeout\n", size);
  }
  gst_object_unref (bus);
  gst_element_set_state (pipeline, GST_STATE_NULL);
  gst_object_unref (pipeline);

  gboolean ok = expect_error ? got_error : got_eos;
  g_print ("  size=%u: %s (expected %s)\n", size, ok ? "PASS" : "FAIL",
      expect_error ? "graceful ERROR" : "EOS, no crash");
  return ok ? 0 : 1;
}

static GstFlowReturn
on_new_sample (GstElement *sink, gpointer user_data)
{
  GByteArray *out = user_data;
  GstSample *sample;
  g_signal_emit_by_name (sink, "pull-sample", &sample);
  if (sample) {
    GstBuffer *buf = gst_sample_get_buffer (sample);
    GstMapInfo map;
    gst_buffer_map (buf, &map, GST_MAP_READ);
    g_byte_array_append (out, map.data, map.size);
    gst_buffer_unmap (buf, &map);
    gst_sample_unref (sample);
  }
  return GST_FLOW_OK;
}

static int
run_round_trip (void)
{
  const char *plaintext = "The quick brown fox jumps over the lazy dog. 1234567890!!";
  gsize plen = strlen (plaintext);

  GstElement *pipeline = gst_pipeline_new (NULL);
  GstElement *appsrc = gst_element_factory_make ("appsrc", NULL);
  GstElement *aesenc = gst_element_factory_make ("aesenc", NULL);
  GstElement *aesdec = gst_element_factory_make ("aesdec", NULL);
  GstElement *appsink = gst_element_factory_make ("appsink", NULL);

  g_object_set (aesenc, "key", KEY, NULL);
  g_object_set (aesenc, "serialize-iv", TRUE, NULL);
  g_object_set (aesdec, "key", KEY, NULL);
  g_object_set (aesdec, "serialize-iv", TRUE, NULL);
  g_object_set (appsrc, "format", GST_FORMAT_BYTES, NULL);
  g_object_set (appsink, "emit-signals", TRUE, "sync", FALSE, NULL);

  GByteArray *out = g_byte_array_new ();
  g_signal_connect (appsink, "new-sample", G_CALLBACK (on_new_sample), out);

  gst_bin_add_many (GST_BIN (pipeline), appsrc, aesenc, aesdec, appsink, NULL);
  gst_element_link_many (appsrc, aesenc, aesdec, appsink, NULL);
  gst_element_set_state (pipeline, GST_STATE_PLAYING);

  GstBuffer *buf = gst_buffer_new_allocate (NULL, plen, NULL);
  gst_buffer_fill (buf, 0, plaintext, plen);
  GstFlowReturn fret;
  g_signal_emit_by_name (appsrc, "push-buffer", buf, &fret);
  gst_buffer_unref (buf);
  g_signal_emit_by_name (appsrc, "end-of-stream", &fret);

  GstBus *bus = gst_element_get_bus (pipeline);
  GstMessage *msg = gst_bus_timed_pop_filtered (bus, 2 * GST_SECOND,
      GST_MESSAGE_ERROR | GST_MESSAGE_EOS);
  if (msg)
    gst_message_unref (msg);
  gst_object_unref (bus);
  gst_element_set_state (pipeline, GST_STATE_NULL);
  gst_object_unref (pipeline);

  gboolean ok = (out->len == plen && memcmp (out->data, plaintext, plen) == 0);
  g_print ("  round-trip: got %u bytes back, expected %zu: %s\n", out->len, plen,
      ok ? "PASS (matches original plaintext)" : "FAIL (mismatch)");
  g_byte_array_free (out, TRUE);
  return ok ? 0 : 1;
}

int
main (int argc, char *argv[])
{
  gst_init (&argc, &argv);
  int failures = 0;

  g_print ("[1] 15-byte first buffer (too short, must be a graceful error):\n");
  failures += run_short_first_buffer (15, TRUE);

  g_print ("[2] 16-byte first buffer (IV only, the fixed bug):\n");
  failures += run_short_first_buffer (16, FALSE);

  g_print ("[3] aesenc|aesdec round trip on ordinary non-block-aligned data:\n");
  failures += run_round_trip ();

  g_print ("%s\n", failures == 0 ? "ALL CHECKS PASSED" : "SOME CHECKS FAILED");
  return failures == 0 ? 0 : 1;
}
