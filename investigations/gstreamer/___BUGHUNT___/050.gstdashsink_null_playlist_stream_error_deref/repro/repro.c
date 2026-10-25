#include <gst/gst.h>
#include <gio/gio.h>
#include "gstdashsink.h"

/* gstdashdemux.c defines this category; we don't compile that file in
 * (dashsink doesn't need dashdemux), but gstmpdparser.c references it. */
GstDebugCategory *gst_dash_demux_debug;

static GOutputStream *
on_get_playlist_stream (GstDashSink * sink, const gchar * location,
    gpointer user_data)
{
  g_print ("get-playlist-stream handler: returning NULL for %s\n", location);
  return NULL;
}

int
main (int argc, char **argv)
{
  gst_init (&argc, &argv);

  GstElement *sink = g_object_new (gst_dash_sink_get_type (), NULL);
  g_assert (sink != NULL);

  /* "get-playlist-stream" is a G_SIGNAL_RUN_LAST signal whose class closure
   * is the real gst_dash_sink_get_playlist_stream() default handler (opens a
   * real file with g_file_replace()). A plain g_signal_connect() handler
   * runs BEFORE that class closure, so without an accumulator the class
   * handler's real (non-NULL) return would overwrite ours. Connect _after
   * so our NULL is the one left as the signal's final return value - this
   * is exactly what a real application does when it wants to reject a
   * playlist write (e.g. a permission/quota check) via this API. */
  g_signal_connect_after (sink, "get-playlist-stream",
      G_CALLBACK (on_get_playlist_stream), NULL);

  /* gst_dash_sink_write_mpd_file() is `static`, called unconditionally from
   * gst_dash_sink_handle_message() (registered as the real GstBinClass
   * "handle_message" vfunc) whenever a GST_MESSAGE_EOS bubbles up to the
   * bin - so we drive the real vfunc with a real EOS message, exactly as
   * the real bin dispatch machinery would when a genuine child element
   * posts EOS. With zero streams configured, gst_dash_sink_generate_mpd_
   * content()'s first-time setup still produces valid (if minimal) MPD
   * XML, so gst_mpd_client_get_xml_content() succeeds and execution reaches
   * the vulnerable g_output_stream_write_all() call with file_stream==NULL. */
  GstMessage *eos = gst_message_new_eos (GST_OBJECT (sink));
  g_print ("dispatching a real GST_MESSAGE_EOS through the bin's real "
      "handle_message vfunc, with a NULL-returning get-playlist-stream "
      "handler connected...\n");
  GST_BIN_GET_CLASS (sink)->handle_message (GST_BIN (sink), eos);
  g_print ("returned normally (should have crashed before this line if the "
      "bug fired)\n");

  gst_object_unref (sink);
  gst_deinit ();
  return 0;
}
