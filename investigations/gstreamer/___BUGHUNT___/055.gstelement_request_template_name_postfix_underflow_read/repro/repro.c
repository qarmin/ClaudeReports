#include <gst/gst.h>
#include <string.h>

static GstPad *
my_request_new_pad (GstElement * element, GstPadTemplate * templ,
    const gchar * name, const GstCaps * caps)
{
  return gst_pad_new_from_template (templ, name);
}

static void
my_element_class_init (GstElementClass * klass)
{
  GstPadTemplate *templ;
  /* Specifier %u immediately followed by a long literal postfix (64 'X's,
   * no separating '_'): templ_postfix_len = 64. */
  gchar templ_name[128];
  g_snprintf (templ_name, sizeof (templ_name), "src_%%u%s",
      "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

  klass->request_new_pad = my_request_new_pad;

  templ = gst_pad_template_new (templ_name, GST_PAD_SRC, GST_PAD_REQUEST,
      gst_caps_new_any ());
  g_assert (templ != NULL);
  gst_element_class_add_pad_template (klass, templ);
}

static void
my_element_init (GTypeInstance * instance, gpointer g_class)
{
}

int
main (int argc, char **argv)
{
  gst_init (&argc, &argv);

  static const GTypeInfo info = {
    sizeof (GstElementClass),
    NULL, NULL,
    (GClassInitFunc) my_element_class_init,
    NULL, NULL,
    sizeof (GstElement),
    0,
    my_element_init,
  };
  GType my_type = g_type_register_static (GST_TYPE_ELEMENT,
      "MyRequestPadElement2", &info, 0);

  GstElement *elem = g_object_new (my_type, NULL);
  g_assert (elem != NULL);

  /* name_postfix_len ("5", 1) << templ_postfix_len (64) - a heap-allocated
   * name string so valgrind can precisely bound-check it. */
  gchar *name = g_strdup ("src_5");
  g_print ("requesting pad \"%s\" (heap ptr %p)...\n", name, (void *) name);
  GstPad *pad = gst_element_request_pad_simple (elem, name);
  g_print ("gst_element_request_pad_simple returned %p\n", (void *) pad);

  if (pad)
    gst_object_unref (pad);
  g_free (name);
  gst_object_unref (elem);
  gst_deinit ();
  return 0;
}
