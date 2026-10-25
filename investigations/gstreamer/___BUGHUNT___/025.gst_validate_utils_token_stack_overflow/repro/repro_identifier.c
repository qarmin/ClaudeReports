#include <gst/gst.h>
#include <gst/validate/gst-validate-utils.h>
#include <stdio.h>
#include <string.h>

static int
no_variables (const gchar * name, gdouble * value, gpointer user_data)
{
  (void) name; (void) value; (void) user_data;
  return 0;
}

int
main (int argc, char **argv)
{
  gst_init (&argc, &argv);

  const int n_chars = 4096;
  gchar *expr = g_malloc (n_chars + 1);
  memset (expr, 'a', n_chars);
  expr[n_chars] = '\0';

  fprintf (stderr, "Calling gst_validate_utils_parse_expression() with a "
      "%d-char identifier (token buffer is only 256 bytes)...\n", n_chars);

  gchar *error = NULL;
  gdouble val = gst_validate_utils_parse_expression (expr, no_variables, NULL, &error);

  fprintf (stderr, "Returned without crashing: val=%f error=%s\n", val, error ? error : "(none)");
  g_free (error);
  g_free (expr);
  return 0;
}
