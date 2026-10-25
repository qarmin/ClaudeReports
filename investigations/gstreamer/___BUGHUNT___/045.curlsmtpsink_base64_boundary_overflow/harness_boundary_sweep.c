#include <glib.h>
#include <stdio.h>
#include <string.h>

/* Exercises exactly the pattern used in gstcurlsmtpsink.c's
 * add_final_boundary_unlocked() / gst_curl_smtp_sink_flush_data_unlocked():
 *   data_out = g_malloc(6);
 *   len = g_base64_encode_close(TRUE, data_out, &state, &save);
 *   data_out[len - 1] = '\r';
 *   data_out[len] = '\n';
 * under ASAN, across every boundary combination of prior encode_step() input
 * length (0 through a run of several 3-byte groups, and 1/2-byte remainders).
 */
static void
run_case (const char *label, const guchar *in, gsize inlen)
{
  gint state = 0, save = 0;
  gchar step_out[256];

  if (inlen > 0)
    g_base64_encode_step (in, inlen, TRUE, step_out, &state, &save);

  gchar *data_out = g_malloc (6);
  gsize len = g_base64_encode_close (TRUE, data_out, &state, &save);

  printf ("%-40s inlen=%2zu -> len=%zu\n", label, inlen, len);

  data_out[len - 1] = '\r';
  data_out[len] = '\n';

  g_free (data_out);
}

int
main (void)
{
  guchar buf[64];
  for (gsize i = 0; i < sizeof (buf); i++)
    buf[i] = (guchar) ('A' + (i % 26));

  /* immediate EOS, no attachment data at all: state=0, save=0 */
  run_case ("immediate EOS, no data", buf, 0);

  /* exact multiples of 3 (the claimed trigger for len==0) */
  for (gsize n = 3; n <= 60; n += 3)
    {
      char label[64];
      snprintf (label, sizeof (label), "multiple-of-3, n=%zu", n);
      run_case (label, buf, n);
    }

  /* 1-byte and 2-byte remainders (non-multiple-of-3) for completeness */
  for (gsize n = 1; n <= 62; n += 1)
    {
      if (n % 3 == 0)
        continue;
      char label[64];
      snprintf (label, sizeof (label), "remainder, n=%zu", n);
      run_case (label, buf, n);
    }

  printf ("All cases completed without ASAN error.\n");
  return 0;
}
