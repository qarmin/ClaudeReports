/*
 * Real end-to-end repro for aesdec's g_assert(in_size > GST_AES_BLOCK_SIZE) abort.
 *
 * Builds a real GStreamer pipeline (appsrc ! aesdec ! fakesink), with aesdec configured with
 * serialize-iv=true (the documented mode: read the IV from the first 16 bytes of the first
 * buffer). appsrc then pushes a single, real 16-byte GstBuffer (exactly the AES block size,
 * i.e. an IV with zero bytes of actual ciphertext after it) through the real pad-push / chain
 * function path - exactly what any real upstream element (a network source, a file source
 * producing a short/truncated first read) would do. This exercises the exact same
 * GstBaseTransform::prepare_output_buffer() call that a real pipeline hits, not a hand-built
 * stand-in for it.
 */
#include <gst/gst.h>
#include <gst/app/gstappsrc.h>
#include <stdlib.h>
#include <string.h>

int
main (int argc, char *argv[])
{
  gst_init (&argc, &argv);

  GstElement *pipeline = gst_pipeline_new ("pipeline");
  GstElement *appsrc = gst_element_factory_make ("appsrc", "src");
  GstElement *aesdec = gst_element_factory_make ("aesdec", "dec");
  GstElement *sink = gst_element_factory_make ("fakesink", "sink");

  if (!pipeline || !appsrc || !aesdec || !sink) {
    g_printerr ("Failed to create elements (appsrc=%p aesdec=%p fakesink=%p)\n",
        appsrc, aesdec, sink);
    return 2;
  }

  /* Same key from the aesdec docs example (32 hex chars = 16 bytes, valid AES-128 key);
   * serialize-iv=true means the decoder expects the first buffer to start with the 16-byte IV. */
  g_object_set (aesdec, "key", "1f9423681beb9a79215820f6bda73d0f", NULL);
  g_object_set (aesdec, "serialize-iv", TRUE, NULL);

  gst_bin_add_many (GST_BIN (pipeline), appsrc, aesdec, sink, NULL);
  if (!gst_element_link_many (appsrc, aesdec, sink, NULL)) {
    g_printerr ("Failed to link elements\n");
    return 2;
  }

  g_object_set (appsrc, "format", GST_FORMAT_BYTES, NULL);

  if (gst_element_set_state (pipeline, GST_STATE_PLAYING) ==
      GST_STATE_CHANGE_FAILURE) {
    g_printerr ("Failed to set pipeline to PLAYING\n");
    return 2;
  }

  /* Exactly GST_AES_BLOCK_SIZE (16) bytes: a real, complete serialized IV with zero bytes of
   * ciphertext following it - the minimal, entirely plausible "first buffer" a real upstream
   * element can produce (e.g. a short first read from a socket/file). */
  guint8 iv_only[16];
  memset (iv_only, 0x42, sizeof (iv_only));

  GstBuffer *buf = gst_buffer_new_allocate (NULL, sizeof (iv_only), NULL);
  gst_buffer_fill (buf, 0, iv_only, sizeof (iv_only));

  g_print ("Pushing a real %u-byte buffer (IV only, no ciphertext) into appsrc...\n",
      (guint) sizeof (iv_only));

  GstFlowReturn fret;
  g_signal_emit_by_name (appsrc, "push-buffer", buf, &fret);
  gst_buffer_unref (buf);

  g_print ("push-buffer returned %s\n", gst_flow_get_name (fret));

  g_signal_emit_by_name (appsrc, "end-of-stream", &fret);

  /* Pump the bus briefly so an async error (if the process didn't abort) surfaces. */
  GstBus *bus = gst_element_get_bus (pipeline);
  GstMessage *msg = gst_bus_timed_pop_filtered (bus, 2 * GST_SECOND,
      GST_MESSAGE_ERROR | GST_MESSAGE_EOS);
  if (msg) {
    if (GST_MESSAGE_TYPE (msg) == GST_MESSAGE_ERROR) {
      GError *err = NULL;
      gchar *dbg = NULL;
      gst_message_parse_error (msg, &err, &dbg);
      g_print ("Pipeline posted ERROR: %s (%s)\n", err->message, dbg ? dbg : "");
      g_error_free (err);
      g_free (dbg);
    } else {
      g_print ("Pipeline posted EOS\n");
    }
    gst_message_unref (msg);
  } else {
    g_print ("No ERROR/EOS message within timeout\n");
  }
  gst_object_unref (bus);

  g_print ("Reached end of main() without the process aborting.\n");

  gst_element_set_state (pipeline, GST_STATE_NULL);
  gst_object_unref (pipeline);

  return 0;
}
