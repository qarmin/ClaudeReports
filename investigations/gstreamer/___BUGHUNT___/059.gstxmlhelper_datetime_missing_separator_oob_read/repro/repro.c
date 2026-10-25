#include <gst/gst.h>
#include <libxml/parser.h>
#include <libxml/tree.h>
#include "gstxmlhelper.h"

int
main (int argc, char **argv)
{
  gst_init (&argc, &argv);

  /* "availabilityStartTime" has no '-' (or 'T'/':') anywhere in it, so
   * strcspn(str, "-") returns strlen(str) (no match found), and
   * `str += (pos + 1)` advances the pointer ONE PAST the NUL terminator -
   * the very next sscanf() then reads from that out-of-bounds pointer. */
  const char *xml =
      "<MPD availabilityStartTime=\"2024xxxxxxxxxxxxxxxxxxx\"></MPD>";

  xmlDocPtr doc = xmlReadMemory (xml, strlen (xml), "noname.xml", NULL, 0);
  g_assert (doc != NULL);
  xmlNode *root = xmlDocGetRootElement (doc);
  g_assert (root != NULL);

  GstDateTime *result = NULL;
  g_print ("calling gst_xml_helper_get_prop_dateTime() on "
      "availabilityStartTime=\"2024\" (no '-' separator)...\n");
  gboolean ret =
      gst_xml_helper_get_prop_dateTime (root, "availabilityStartTime",
      &result);
  g_print ("gst_xml_helper_get_prop_dateTime returned %d (should have "
      "crashed/been flagged before this line if the bug fired)\n", ret);

  if (result)
    gst_date_time_unref (result);
  xmlFreeDoc (doc);
  xmlCleanupParser ();
  gst_deinit ();
  return 0;
}
