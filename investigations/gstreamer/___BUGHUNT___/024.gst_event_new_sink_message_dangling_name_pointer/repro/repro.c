/*
 * Standalone reproduction for candidate 024: gst_event_new_sink_message() stores its
 * caller-supplied `name` pointer inside the event's GstStructure via
 * gst_structure_new_static_str(), WITHOUT duplicating it, whenever the string is longer
 * than 15 bytes. Nothing in gst_event_new_sink_message()'s own doc comment tells a caller
 * that `name` must be a static/permanently-valid string (unlike the sibling
 * gst_structure_new_static_str(), whose own doc comment explicitly says so) - so a normal
 * caller that builds the name dynamically (heap-allocated, e.g. read off the wire, as the
 * real subprojects/gst-plugins-bad/sys/ipcpipeline/gstipcpipelinecomm.c caller does with a
 * pointer straight into an adapter-mapped buffer that gets unmapped right after the call)
 * ends up with a dangling pointer inside the event the moment that buffer is freed/reused.
 *
 * This exercises the REAL public API end-to-end: gst_event_new_sink_message() (the real
 * affected entry point), followed by gst_event_has_name()/gst_structure_get_name() (the
 * real, documented way to read an event's name back out - used internally by
 * gst_event_has_name() and by GST_EVENT_TYPE debug logging), with a heap-allocated `name`
 * that is freed before that second call - exactly the shape of the real ipcpipeline bug,
 * without needing to actually stand up a second ipcpipeline process for this specific
 * bug: the vulnerable mechanism is entirely inside gst_event_new_sink_message()'s own
 * contract with ANY caller passing a transient string, not something only an IPC peer can
 * trigger. See RESULT.md's "Fully reproducible" section for why this repro shape is used
 * rather than standing up the full two-process ipcpipeline harness.
 */
#include <gst/gst.h>
#include <string.h>
#include <stdio.h>

int
main (int argc, char **argv)
{
  gst_init (&argc, &argv);

  /* Longer than 15 bytes -> stored as a raw, unduplicated pointer
   * (see _gst_id_str_set_static_str_with_len_inline in gstidstr-private.h). */
  const char *orig = "sink-message-name-longer-than-15-bytes";
  gsize len = strlen (orig);
  g_assert (len > 15);

  /* Heap-allocate it, exactly like gst_ipc_pipeline_comm_read_bus_message() hands
   * gst_event_new_sink_message() a pointer into a heap-backed adapter buffer. */
  gchar *name = g_strdup (orig);

  GstStructure *msg_structure = gst_structure_new_empty ("test");
  GstMessage *msg = gst_message_new_element (NULL, msg_structure);

  GstEvent *event = gst_event_new_sink_message (name, msg);
  gst_message_unref (msg);

  /* Simulate the real caller's very next action: gst_ipc_pipeline_comm_read_bus_message()
   * unmaps + flushes the adapter buffer `name` pointed into right after this call returns,
   * freeing/invalidating that memory. A plain heap free() stands in for that here. */
  g_free (name);
  name = NULL;

  fprintf (stderr, "about to read the event's name back out via the public API...\n");

  /* Real, documented way to read the event's name back - this is what dereferences the
   * dangling pointer stored inside the structure. */
  gboolean has_it = gst_event_has_name (event, "irrelevant-probe");
  fprintf (stderr, "gst_event_has_name returned %d (unreachable under ASAN if this crashes)\n",
      has_it);

  gst_event_unref (event);
  return 0;
}
