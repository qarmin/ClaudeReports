/*
 * The real, unmodified ges-validate.c.o object (linked in as-is - see justfile) references a
 * couple of symbols that only exist in ges-launcher.c (the rest of the ges-launch-1.0 tool, not
 * pulled in here since it needs a lot more scaffolding this repro doesn't exercise). Both are only
 * reachable from `ges_validate_activate()`'s `opts->scenario`-with-metadata branch, which this
 * repro's harness.c never takes (it passes scenario="none", so
 * `GST_VALIDATE_BIN_MONITOR(monitor)->scenario` is NULL and that whole block is skipped) - these
 * stubs exist purely to satisfy the linker for dead code, they are never actually called.
 */
#include <glib.h>
#include <gst/gst.h>

gboolean
ges_launcher_parse_options (gpointer self, gchar *** arguments, gint * argc,
    gpointer ctx, GError ** error)
{
  g_assert_not_reached ();
  return FALSE;
}
