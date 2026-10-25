/* Standalone repro for candidate 033: ges_title_clip_get_text() (and its 7 siblings)
 * dereference self->priv->track_titles->data without a NULL check. A freshly created
 * GESTitleClip (via the real, public ges_title_clip_new() constructor) has
 * track_titles == NULL until it is added to a timeline with a video track, so calling
 * any of these getters before that crashes with a NULL-pointer read.
 *
 * Public API only, no timeline/pipeline needed - this is exactly the trivial sequence
 * findings.toml's PANIC_8 describes.
 */
#include <ges/ges.h>
#include <stdio.h>

int
main (int argc, char **argv)
{
  GESTitleClip *clip;

  gst_init (&argc, &argv);
  ges_init ();

  clip = ges_title_clip_new ();
  printf ("Created GESTitleClip %p (not added to any timeline)\n", (void *) clip);

  printf ("Calling ges_title_clip_get_text() on it...\n");
  fflush (stdout);

  /* This dereferences self->priv->track_titles->data, and track_titles is NULL here. */
  gchar *text = (gchar *) ges_title_clip_get_text (clip);
  printf ("get_text -> %s\n", text);
  fflush (stdout);
  g_free (text);

  gchar *font_desc = (gchar *) ges_title_clip_get_font_desc (clip);
  printf ("get_font_desc -> %s\n", font_desc);
  fflush (stdout);
  g_free (font_desc);

  printf ("get_halignment -> %d\n", ges_title_clip_get_halignment (clip));
  fflush (stdout);
  printf ("get_valignment -> %d\n", ges_title_clip_get_valignment (clip));
  fflush (stdout);
  printf ("get_text_color -> %u\n", ges_title_clip_get_text_color (clip));
  fflush (stdout);
  printf ("get_background_color -> %u\n",
      ges_title_clip_get_background_color (clip));
  fflush (stdout);
  printf ("get_xpos -> %f\n", ges_title_clip_get_xpos (clip));
  fflush (stdout);
  printf ("get_ypos -> %f\n", ges_title_clip_get_ypos (clip));
  fflush (stdout);

  printf ("All 8 getters returned without crashing.\n");
  fflush (stdout);

  gst_object_unref (clip);
  return 0;
}
