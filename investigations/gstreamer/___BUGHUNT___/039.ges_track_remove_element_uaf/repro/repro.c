#include <ges/ges.h>
#include <gst/gst.h>

int
main (int argc, char **argv)
{
  gst_init (&argc, &argv);
  ges_init ();

  GESTrack *track = GES_TRACK (ges_audio_track_new ());
  GESTrackElement *effect = GES_TRACK_ELEMENT (ges_effect_new ("volume"));

  /* Application keeps its own reference to the element, as is normal when
   * holding on to a track element beyond the track's own ownership. */
  gst_object_ref (effect);

  g_assert (ges_track_add_element (track, effect));

  g_print ("First remove: %d\n", ges_track_remove_element (track, effect));

  /* Second removal of the SAME still-alive element - the public API gives
   * no indication this is invalid; nothing says the object must be added
   * again before it can be "removed" a second time. */
  g_print ("Second remove (triggers UAF): %d\n",
      ges_track_remove_element (track, effect));

  g_print ("Survived (unexpected without ASAN)\n");

  /* Also exercise a third removal, and a legitimate re-add/re-remove cycle,
   * to make sure the fix doesn't just special-case exactly two calls. */
  g_print ("Third remove: %d\n", ges_track_remove_element (track, effect));
  g_print ("Re-add: %d\n", ges_track_add_element (track, effect));
  g_print ("Remove after re-add: %d\n", ges_track_remove_element (track, effect));

  gst_object_unref (effect);
  gst_object_unref (track);

  return 0;
}
