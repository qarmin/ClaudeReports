#include <gst/gst.h>
#include <gst/validate/gst-validate-utils.h>
#include <stdio.h>

int
main (int argc, char **argv)
{
  gst_init (&argc, &argv);

  const gchar *exprs[] = {
    "1 + 2 * 3",
    "min(4, 5) + max(1, 2)",
    "-3.5e2 + 1",
    "(1 + 2) * (3 - 4)",
  };

  for (guint i = 0; i < G_N_ELEMENTS (exprs); i++) {
    gchar *error = NULL;
    gdouble val = gst_validate_utils_parse_expression (exprs[i], NULL, NULL, &error);
    fprintf (stderr, "expr=\"%s\" val=%f error=%s\n", exprs[i], val, error ? error : "(none)");
    g_free (error);
  }
  return 0;
}
