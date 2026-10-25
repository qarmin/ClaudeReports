/* Standalone fuzz driver for gsth266parser.c (VVC/H.266) - no existing ci/fuzzing/ target covers
 * this new codec parser, despite it having the highest density of real, historically-confirmed
 * bugs found anywhere in this codebase this session (see ___BUGHUNT___/018.../RESULT.md). Reads
 * a whole file as an Annex-B byte stream, walks NAL units via the parser's own public API, and
 * calls the generic parse_nal() dispatcher (VPS/SPS/PPS/APS) plus parse_sei() for SEI NALs.
 */
#include <gst/gst.h>
#include <gst/codecparsers/gsth266parser.h>
#include <stdio.h>
#include <stdlib.h>

int
main (int argc, char **argv)
{
  FILE *f;
  long size;
  guint8 *buf;
  GstH266Parser *parser;
  guint offset = 0;

  if (argc < 2)
    return 1;

  f = fopen (argv[1], "rb");
  if (!f)
    return 1;
  fseek (f, 0, SEEK_END);
  size = ftell (f);
  fseek (f, 0, SEEK_SET);
  if (size <= 0) {
    fclose (f);
    return 0;
  }
  buf = malloc (size);
  if (fread (buf, 1, size, f) != (size_t) size) {
    fclose (f);
    free (buf);
    return 0;
  }
  fclose (f);

  gst_init (NULL, NULL);

  parser = gst_h266_parser_new ();

  while (offset < (guint) size) {
    GstH266NalUnit nalu;
    GstH266ParserResult res =
        gst_h266_parser_identify_nalu (parser, buf, offset, (guint) size,
        &nalu);

    if (res != GST_H266_PARSER_OK && res != GST_H266_PARSER_NO_NAL_END)
      break;

    gst_h266_parser_parse_nal (parser, &nalu);

    if (nalu.type == GST_H266_NAL_PREFIX_SEI
        || nalu.type == GST_H266_NAL_SUFFIX_SEI) {
      GArray *messages = NULL;
      if (gst_h266_parser_parse_sei (parser, &nalu, &messages) ==
          GST_H266_PARSER_OK && messages)
        g_array_free (messages, TRUE);
    }

    if (nalu.size == 0)
      break;
    offset = nalu.offset + nalu.size;

    if (res == GST_H266_PARSER_NO_NAL_END)
      break;
  }

  gst_h266_parser_free (parser);
  free (buf);
  return 0;
}
