#include <gst/gst.h>
#include <string.h>

static gpointer
failing_map (GstMemory * mem, gsize maxsize, GstMapFlags flags)
{
  return NULL;
}

int
main (int argc, char **argv)
{
  gst_init (&argc, &argv);

  GstAllocator *alloc = gst_allocator_find (NULL);
  g_assert (alloc != NULL);
  GstMemoryMapFunction orig_map = alloc->mem_map;

  GstBuffer *buf = gst_buffer_new ();

  /* Fill the buffer to GST_BUFFER_MEM_MAX (16) memory blocks, all via the real,
   * unmodified default allocator - these succeed normally. */
  for (int i = 0; i < 16; i++) {
    GstMemory *mem = gst_allocator_alloc (NULL, 16, NULL);
    g_assert (mem != NULL);
    gst_buffer_append_memory (buf, mem);
  }
  g_print ("memory count before overflow: %u\n", gst_buffer_n_memory (buf));
  g_assert (gst_buffer_n_memory (buf) == 16);

  /* Now break gst_memory_map() for every memory allocated by the default
   * allocator (simulating a real allocation/mapping failure under memory
   * pressure) and push a 17th memory in, forcing _memory_add()'s
   * GST_BUFFER_MEM_MAX overflow path to try to merge the 16 existing memories
   * - which will fail because gst_memory_map(result, GST_MAP_WRITE) inside
   * _actual_merged_memory() now fails too. */
  alloc->mem_map = failing_map;
  GstMemory *mem17 = gst_allocator_alloc (NULL, 16, NULL);
  g_assert (mem17 != NULL);
  gst_buffer_append_memory (buf, mem17);

  /* Restore the real map function so we can genuinely observe what happened,
   * rather than our own fake failure masking it. */
  alloc->mem_map = orig_map;

  g_print ("memory count after overflow: %u (bug: should be 1, is likely 2 "
      "with a dangling freed pointer in slot 0)\n", gst_buffer_n_memory (buf));

  /* Use-after-free: gst_buffer_get_size() reads mem[i]->size for every
   * tracked memory, including the freed slot 0 if the bug fired. */
  gsize size = gst_buffer_get_size (buf);
  g_print ("gst_buffer_get_size() = %" G_GSIZE_FORMAT "\n", size);

  gst_buffer_unref (buf);
  gst_deinit ();
  return 0;
}
