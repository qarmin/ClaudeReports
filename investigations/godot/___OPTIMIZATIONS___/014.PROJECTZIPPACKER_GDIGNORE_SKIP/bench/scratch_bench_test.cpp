/**************************************************************************/
/*  test_project_zip_packer_scratch.cpp                                  */
/**************************************************************************/
/*                         This file is part of:                          */
/*                             GODOT ENGINE                               */
/*                        https://godotengine.org                         */
/**************************************************************************/

// SCRATCH BENCHMARK for ___OPTIMIZATIONS___/014 - not a real correctness test, temporary only.

#include "tests/test_macros.h"

TEST_FORCE_LINK(test_project_zip_packer_scratch)

#include "core/config/project_settings.h"
#include "core/io/dir_access.h"
#include "core/os/os.h"
#include "editor/export/project_zip_packer.h"
#include "modules/zip/zip_reader.h"

namespace TestProjectZipPackerScratch {

TEST_CASE("[ProjectZIPPacker][Benchmark] SCRATCH pack_project_zip .gdignore skip") {
	const String out_path = OS::get_singleton()->get_cache_path().path_join("scratch_014_out.zip");
	Error setup_err = ProjectSettings::get_singleton()->setup("/tmp/repro_014", String(), false, true);
	REQUIRE(setup_err == OK);
	MESSAGE("SCRATCH_BENCH resource_path=", ProjectSettings::get_singleton()->get_resource_path());

	uint64_t start = OS::get_singleton()->get_ticks_usec();
	ProjectZIPPacker::pack_project_zip(out_path);
	uint64_t elapsed = OS::get_singleton()->get_ticks_usec() - start;
	MESSAGE("SCRATCH_BENCH pack_project_zip total_us=", elapsed);

	Ref<ZIPReader> reader;
	reader.instantiate();
	REQUIRE(reader->open(out_path) == OK);
	PackedStringArray files = reader->get_files();
	reader->close();

	int excluded_count = 0;
	int normal_count = 0;
	for (int i = 0; i < files.size(); i++) {
		if (files[i].contains("big_excluded_dir")) {
			excluded_count++;
		}
		if (files[i].contains("normal_dir")) {
			normal_count++;
		}
	}
	MESSAGE("SCRATCH_BENCH zip_entries=", files.size(), " excluded_dir_entries=", excluded_count, " normal_dir_entries=", normal_count);
	for (int i = 0; i < files.size(); i++) {
		MESSAGE("SCRATCH_BENCH entry: ", files[i]);
	}

	DirAccess::remove_absolute(out_path);
}

} // namespace TestProjectZipPackerScratch
