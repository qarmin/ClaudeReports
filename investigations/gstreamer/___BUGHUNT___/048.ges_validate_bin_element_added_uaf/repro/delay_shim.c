/*
 * LD_PRELOAD race synchronizer for candidate 048 (ges_validate_bin_element_added_uaf).
 *
 * The bug lives in the *unmodified* project function `bin_element_added()`
 * (subprojects/gst-editing-services/tools/ges-validate.c:168-169), which does, inside a do/while
 * loop walking the ancestor chain of a just-added element:
 *
 *     gst_object_unref (parent);
 *     parent = gst_object_get_parent (parent);
 *
 * If `parent`'s refcount drops to zero on that `gst_object_unref()` call, the following
 * `gst_object_get_parent(parent)` dereferences freed memory. Under single-threaded execution this
 * never happens - every ancestor bin retains an independent owning reference held by its own
 * parent bin, unrelated to the walk's own temporary ref. It is a genuine but narrow *race*: a
 * second, real thread must concurrently drop that ancestor's last other reference (a real
 * `gst_bin_remove()`) inside the handful of instructions between the two calls above.
 *
 * This shim does not change what `bin_element_added()` does or what data flows through it. It
 * only makes the already-real race land reliably, the same way a deliberately inserted sleep is
 * routinely used to force a hard-to-hit interleaving: it intercepts the *one* real
 * `gst_object_unref()` call made, from the designated walker thread and *from code linked into
 * the main executable* (i.e. bin_element_added() itself - libgstvalidate's own signal handlers
 * make their own legitimate, unrelated ref/unref calls on the same object from their own code in
 * libgstvalidate-1.0.so, which this filter excludes), on the designated target pointer. On that
 * one call, it wakes the "remover" thread (via a semaphore) and gives it a short, generous margin
 * to finish a real `gst_bin_remove()` call before letting the real `gst_object_unref()` proceed.
 * Every other `gst_object_unref()` call in the process is completely unaffected.
 */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <execinfo.h>
#include <glib.h>
#include <glib-object.h>
#include <pthread.h>
#include <semaphore.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

typedef void (*gst_object_unref_t) (gpointer object);

static gst_object_unref_t real_gst_object_unref = NULL;
static pthread_once_t g_once = PTHREAD_ONCE_INIT;

static void *g_target = NULL;
static pthread_t g_walker_tid;
static unsigned g_margin_us = 50000;
static volatile int g_armed = 0;
static volatile int g_fired = 0;
static sem_t g_go_sem;

static void
resolve_real (void)
{
  real_gst_object_unref = (gst_object_unref_t) dlsym (RTLD_NEXT, "gst_object_unref");
  if (!real_gst_object_unref) {
    fprintf (stderr, "gst_uaf_race shim: could not resolve real gst_object_unref: %s\n",
        dlerror ());
    _exit (99);
  }
  /* sem_init() is UB if any thread might already be blocked in sem_wait() on it - initialize
   * exactly once, here, instead of every time gst_uaf_race_arm() runs. */
  sem_init (&g_go_sem, 0, 0);
}

/* Called by the harness, on the walker thread, right before it starts the operation
 * (`gst_bin_add()`) whose tracer hook will eventually call `gst_object_unref()` on @target. */
void
gst_uaf_race_arm (void *target, unsigned margin_us)
{
  pthread_once (&g_once, resolve_real);
  g_target = target;
  g_walker_tid = pthread_self ();
  g_margin_us = margin_us;
  g_fired = 0;
  g_armed = 1;
}

/* Called by the remover thread: blocks until the walker thread's tracer hook is about to make
 * its final, fatal `gst_object_unref()` call. */
void
gst_uaf_race_wait_go (void)
{
  pthread_once (&g_once, resolve_real);
  sem_wait (&g_go_sem);
}

/* True if the *direct* caller of the current gst_object_unref() call is code linked into the
 * main executable (bin_element_added(), the only such code this repro links in) rather than some
 * unrelated, legitimate ref/unref pair happening in libgstvalidate's own signal handling (e.g.
 * `_validate_bin_element_added()`'s own temporary `gst_validate_monitor_get_target()` +
 * `gst_object_unref()` pair, which also touches the same object from the same thread, just from a
 * different call site, milliseconds earlier in the very same gst_bin_add() call). */
static gboolean
caller_is_self_exe (void)
{
  /* bt[0]=backtrace() itself, bt[1]=this function, bt[2]=this function's caller
   * (gst_object_unref, in this shim's own .so) - bt[3] is gst_object_unref()'s own caller, what
   * we actually want to identify. */
  void *bt[5];
  int n = backtrace (bt, 5);
  if (n < 4)
    return FALSE;
  Dl_info info;
  if (!dladdr (bt[3], &info) || !info.dli_fname)
    return FALSE;
  return strstr (info.dli_fname, "harness_test") != NULL;
}

void
gst_object_unref (gpointer object)
{
  pthread_once (&g_once, resolve_real);

  if (g_armed && !g_fired && object == g_target && pthread_equal (pthread_self (), g_walker_tid)
      && caller_is_self_exe ()) {
    g_fired = 1;
    fprintf (stderr,
        "[gst_uaf_race] intercepted the walker's gst_object_unref(ancestor) call inside "
        "bin_element_added() - waking the remover thread and giving it %uus to finish a real "
        "gst_bin_remove() before letting the real unref proceed\n", g_margin_us);
    fflush (stderr);
    sem_post (&g_go_sem);
    usleep (g_margin_us);
  }

  real_gst_object_unref (object);
}
