/*
 * Real end-to-end repro for candidate 048: ges_validate_bin_element_added_uaf.
 *
 * This runs the *actual, unmodified* `bin_element_added()` from
 * subprojects/gst-editing-services/tools/ges-validate.c (compiled into this binary as the exact
 * object file `ges-validate.c.o` produced by the project's own meson build - see justfile/Makefile
 * in this folder), reached through its real, documented entry point: `ges_validate_activate()`
 * calling `gst_tracing_register_hook(runner, "bin-add-post", G_CALLBACK(bin_element_added))`,
 * exactly as `ges-launch-1.0 --validate` does. `bin_element_added` is `static`, so nothing outside
 * that translation unit can call it directly by name - this is what a real caller (ges-launch-1.0)
 * does too: call the public `ges_validate_activate()`, which wires the hook up internally.
 *
 * Two real pthreads genuinely drive the race the bug depends on:
 *   - the "walker" thread calls the real, public `gst_bin_add()`, which synchronously triggers the
 *     real `bin_element_added()` tracer hook on this same thread;
 *   - the "remover" thread calls the real, public `gst_bin_remove()` on the *same* ancestor bin the
 *     walker's hook is currently walking through, timed (via delay_shim.c, an LD_PRELOAD
 *     synchronizer - see its own header comment) to land inside the walk.
 *
 * Both calls are genuine, valid, unmodified GStreamer public API calls - nothing here fabricates
 * data or fakes a "second participant's" bytes; the second participant is a second real thread
 * making a second real, independently-valid API call, exactly the shape CONFIRMATION.md's own
 * network-peer recipe describes ("two threads ... whichever is simpler") applied to a concurrency
 * bug instead of a wire protocol.
 */
#include <gst/gst.h>
#include <ges/ges.h>
#include <gst/validate/validate.h>
#include <gst/validate/gst-validate-scenario.h>
#include <gst/validate/gst-validate-bin-monitor.h>
#include <pthread.h>
#include <stdio.h>
#include <string.h>

#include "ges-validate.h"

/* Exported by delay_shim.c (LD_PRELOAD'd - see justfile). */
extern void gst_uaf_race_arm (void *target, unsigned margin_us);
extern void gst_uaf_race_wait_go (void);

/* ---- a minimal, real GstElement subclass whose class KLASS metadata says "Decoder" ----
 * This is exactly how any real decoder plugin (avdec_h264, etc.) gets `is_decoder = TRUE` set on
 * its GstValidateElementMonitor (see gst-validate-element-monitor.c:
 * `monitor->is_decoder = strstr (klassname, "Decoder") != NULL;`, driven off real
 * GST_ELEMENT_METADATA_KLASS) - not something bin_element_added or the monitor code treats
 * specially, just a normal element registering a normal klass string. */
#define TEST_TYPE_DECODER (test_decoder_get_type ())
G_DECLARE_FINAL_TYPE (TestDecoder, test_decoder, TEST, DECODER, GstElement)
struct _TestDecoder
{
  GstElement parent;
};
G_DEFINE_TYPE (TestDecoder, test_decoder, GST_TYPE_ELEMENT)

static void
test_decoder_class_init (TestDecoderClass * klass)
{
  GstElementClass *gstelement_class = GST_ELEMENT_CLASS (klass);
  gst_element_class_set_metadata (gstelement_class,
      "Test Decoder", "Codec/Decoder/Video",
      "A fake decoder-klass element for candidate 048's repro", "bug-hunt");
}

static void
test_decoder_init (TestDecoder * self)
{
}

/* ---- the race ---- */

typedef struct
{
  GstBin *root;
  GstBin *mid;
  GstElement *leaf;
} RaceCtx;

static void *
walker_thread (void *data)
{
  RaceCtx *ctx = data;

  g_printerr ("[walker] arming race on mid-bin %p, then gst_bin_add(mid, leaf)\n",
      (void *) ctx->mid);
  /* Arms the shim: the *next* gst_object_unref() call made, from this thread and from code in the
   * main executable, on `ctx->mid`, is the one inside bin_element_added()'s loop - see
   * delay_shim.c. */
  gst_uaf_race_arm (ctx->mid, 100000 /* 100ms margin - gst_bin_remove() needs microseconds */ );

  /* Triggers GST_TRACER_BIN_ADD_POST -> the real bin_element_added(runner, ts, mid, leaf, TRUE)
   * synchronously, on this thread. */
  gst_bin_add (ctx->mid, ctx->leaf);

  g_printerr ("[walker] gst_bin_add() returned - race did not land this run\n");

  return NULL;
}

static void *
remover_thread (void *data)
{
  RaceCtx *ctx = data;

  /* Blocks until the walker thread's hook is about to make its fatal gst_object_unref(mid) call,
   * then races a real gst_bin_remove() to drop root's ownership of `mid` first. */
  gst_uaf_race_wait_go ();
  g_printerr ("[remover] got go signal, calling gst_bin_remove(root, mid)\n");
  gst_bin_remove (ctx->root, GST_ELEMENT (ctx->mid));
  g_printerr ("[remover] gst_bin_remove() returned\n");

  return NULL;
}

int
main (int argc, char **argv)
{
  gst_init (&argc, &argv);
  ges_init ();

  GstElement *pipeline = gst_pipeline_new ("root-pipeline");
  GstBin *mid = GST_BIN (gst_bin_new ("mid-bin"));
  GstElement *leaf = g_object_new (TEST_TYPE_DECODER, "name", "leaf-decoder", NULL);

  gst_bin_add (GST_BIN (pipeline), GST_ELEMENT (mid));

  /* Real entry point: what ges-launch-1.0 calls when run with --validate. Internally does
   * `runner = gst_validate_runner_new(); gst_tracing_register_hook(runner, "bin-add-post",
   * G_CALLBACK(bin_element_added)); gst_validate_monitor_factory_create(pipeline, runner, NULL);`
   * - the same, unmodified code every real ges-launch-1.0 --validate invocation runs. */
  GESLauncherParsedOptions opts;
  memset (&opts, 0, sizeof (opts));
  opts.enable_validate = TRUE;
  opts.scenario = (gchar *) "none";
  opts.testfile = NULL;
  opts.mute = TRUE;

  if (!ges_validate_activate (GST_PIPELINE (pipeline), NULL, &opts)) {
    g_printerr ("ges_validate_activate failed\n");
    return 2;
  }

  RaceCtx ctx = { GST_BIN (pipeline), mid, leaf };

  pthread_t t_walker, t_remover;
  pthread_create (&t_remover, NULL, remover_thread, &ctx);
  pthread_create (&t_walker, NULL, walker_thread, &ctx);

  pthread_join (t_walker, NULL);
  pthread_join (t_remover, NULL);

  g_print ("RACE_COMPLETED_NO_CRASH\n");
  return 0;
}
