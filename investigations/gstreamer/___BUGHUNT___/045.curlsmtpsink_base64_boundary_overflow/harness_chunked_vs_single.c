#include <glib.h>
#include <stdio.h>
#include <string.h>

int main(void) {
  guchar buf[97];
  for (gsize i = 0; i < sizeof(buf); i++) buf[i] = (guchar)('A' + (i % 26));

  /* Single call */
  gint state1 = 0, save1 = 0;
  gchar out1[256];
  g_base64_encode_step(buf, 45, TRUE, out1, &state1, &save1);
  gchar close1[16];
  gsize len1 = g_base64_encode_close(TRUE, close1, &state1, &save1);

  /* Chunked: feed 7 bytes at a time up to 45 total, mimicking transfer_chunk()'s
   * repeated small-block_size calls against the same running state/save */
  gint state2 = 0, save2 = 0;
  gchar out2[256];
  gsize off = 0;
  while (off < 45) {
    gsize n = MIN((gsize)7, 45 - off);
    g_base64_encode_step(buf + off, n, TRUE, out2, &state2, &save2);
    off += n;
  }
  gchar close2[16];
  gsize len2 = g_base64_encode_close(TRUE, close2, &state2, &save2);

  printf("single-call close len=%zu, chunked close len=%zu -> %s\n",
         len1, len2, len1 == len2 ? "MATCH" : "MISMATCH");
  return 0;
}
