// Minimal stand-in for core/error/error_macros.cpp's logging backend.
// The real file routes through OS::get_singleton()->print_error(), which drags in
// the whole engine (OS, ScriptServer, Logger...). None of that participates in the
// race being reproduced here, and none of these overloads are actually hit on the
// repro's hot path (they only fire on ERR_FAIL_* branches we never take), so the
// bodies just print the parts that don't require linking the real String/utf8
// implementation (core/string/ustring.cpp, which would pull in Object/OS/Variant).
#include "core/error/error_macros.h"
#include "core/string/ustring.h"
#include <cstdio>

void _err_print_error(const char *p_function, const char *p_file, int p_line, const char *p_error, bool p_editor_notify, ErrorHandlerType p_type) {
	fprintf(stderr, "ERROR: %s:%d (%s): %s\n", p_file, p_line, p_function, p_error);
}
void _err_print_error(const char *p_function, const char *p_file, int p_line, const String &p_error, bool p_editor_notify, ErrorHandlerType p_type) {
	fprintf(stderr, "ERROR: %s:%d (%s): <String error>\n", p_file, p_line, p_function);
}
void _err_print_error(const char *p_function, const char *p_file, int p_line, const char *p_error, const char *p_message, bool p_editor_notify, ErrorHandlerType p_type) {
	fprintf(stderr, "ERROR: %s:%d (%s): %s %s\n", p_file, p_line, p_function, p_error, p_message);
}
void _err_print_error(const char *p_function, const char *p_file, int p_line, const String &p_error, const char *p_message, bool p_editor_notify, ErrorHandlerType p_type) {
	fprintf(stderr, "ERROR: %s:%d (%s): <String error> %s\n", p_file, p_line, p_function, p_message);
}
void _err_print_error(const char *p_function, const char *p_file, int p_line, const char *p_error, const String &p_message, bool p_editor_notify, ErrorHandlerType p_type) {
	fprintf(stderr, "ERROR: %s:%d (%s): %s <String message>\n", p_file, p_line, p_function, p_error);
}
void _err_print_error(const char *p_function, const char *p_file, int p_line, const String &p_error, const String &p_message, bool p_editor_notify, ErrorHandlerType p_type) {
	fprintf(stderr, "ERROR: %s:%d (%s): <String error> <String message>\n", p_file, p_line, p_function);
}
void _err_print_error_asap(const String &p_error, ErrorHandlerType p_type) {
	fprintf(stderr, "ERROR (asap)\n");
}
void _err_flush_stdout() {
	fflush(stdout);
}
