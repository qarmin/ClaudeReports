/* Standalone reproduction for candidate 029.
 *
 * Uses only public GStreamer API: a minimal GstDevice subclass, constructed
 * through the ordinary g_object_new() a real device-provider author would
 * use, and the public gst_device_has_classesv() entry point. No internal
 * struct/field is touched directly - the crash is triggered purely by how a
 * public API call sequence a real caller could make interacts with
 * gst_device_has_classesv()'s own missing NULL check.
 */
#include <gst/gst.h>

#define MY_TYPE_DEVICE (my_device_get_type())
G_DECLARE_FINAL_TYPE (MyDevice, my_device, MY, DEVICE, GstDevice)
struct _MyDevice
{
  GstDevice parent;
};
G_DEFINE_TYPE (MyDevice, my_device, GST_TYPE_DEVICE)

static GstElement *
my_device_create_element (GstDevice * d, const gchar * n)
{
  return NULL;
}

static void
my_device_class_init (MyDeviceClass * klass)
{
  GST_DEVICE_CLASS (klass)->create_element = my_device_create_element;
}

static void
my_device_init (MyDevice * d)
{
}

int
main (void)
{
  gst_init (NULL, NULL);
  GstCaps *caps = gst_caps_new_any ();

  /* A minimal GstDevice subclass constructed via the public g_object_new()
   * API, explicitly passing NULL for the "device-class" construct-only
   * string property (a normal, allowed way to construct any GObject string
   * property - nothing here reaches into GstDevice's private struct). */
  GstDevice *dev = g_object_new (MY_TYPE_DEVICE,
      "display-name", "test", "caps", caps, "device-class", NULL, NULL);

  gchar *classes[] = { "Video/Source", NULL };

  g_print ("calling gst_device_has_classesv() on a device constructed with "
      "device-class=NULL...\n");
  gboolean res = gst_device_has_classesv (dev, classes);
  g_print ("result = %d (no crash)\n", res);

  gst_object_unref (dev);
  gst_caps_unref (caps);
  return 0;
}
