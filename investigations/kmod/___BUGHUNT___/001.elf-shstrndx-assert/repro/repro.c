// SPDX-License-Identifier: LGPL-2.1-or-later
// Repro: a module file whose ELF header has an out-of-range e_shstrndx aborts any libkmod
// caller that parses the ELF (here: the public kmod_module_get_info(), i.e. what modinfo does).
//
//   repro <outdir>
//
// Writes <outdir>/shstrndx-{oob,eq-count,undef,valid}.ko (minimal ELF64 relocatable files) and,
// for each, forks a child that runs kmod_module_new_from_path() + kmod_module_get_info().
// Exit 0 = every child returned normally (bug absent), 1 = at least one child died on a signal.
#include <elf.h>
#include <errno.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

#include <libkmod/libkmod.h>

static const char shstrtab[] = "\0.shstrtab\0";

static int write_elf(const char *path, uint16_t shstrndx)
{
	struct {
		Elf64_Ehdr ehdr;
		Elf64_Shdr shdr[2];
		char strings[sizeof(shstrtab)];
	} f;
	FILE *fp;

	memset(&f, 0, sizeof(f));
	memcpy(f.ehdr.e_ident, ELFMAG, SELFMAG);
	f.ehdr.e_ident[EI_CLASS] = ELFCLASS64;
	f.ehdr.e_ident[EI_DATA] = ELFDATA2LSB;
	f.ehdr.e_ident[EI_VERSION] = EV_CURRENT;
	f.ehdr.e_type = ET_REL;
	f.ehdr.e_machine = EM_X86_64;
	f.ehdr.e_version = EV_CURRENT;
	f.ehdr.e_ehsize = sizeof(Elf64_Ehdr);
	f.ehdr.e_shoff = sizeof(Elf64_Ehdr);
	f.ehdr.e_shentsize = sizeof(Elf64_Shdr);
	f.ehdr.e_shnum = 2;
	f.ehdr.e_shstrndx = shstrndx;

	f.shdr[1].sh_name = 1;
	f.shdr[1].sh_type = SHT_STRTAB;
	f.shdr[1].sh_offset = offsetof(typeof(f), strings);
	f.shdr[1].sh_size = sizeof(shstrtab);
	memcpy(f.strings, shstrtab, sizeof(shstrtab));

	fp = fopen(path, "wb");
	if (fp == NULL)
		return -errno;
	if (fwrite(&f, sizeof(f), 1, fp) != 1) {
		fclose(fp);
		return -EIO;
	}
	return fclose(fp) == 0 ? 0 : -errno;
}

static int get_info(const char *path)
{
	struct kmod_ctx *ctx;
	struct kmod_module *mod;
	struct kmod_list *list = NULL;
	int err;

	ctx = kmod_new(NULL, NULL);
	if (ctx == NULL)
		return -ENOMEM;
	err = kmod_module_new_from_path(ctx, path, &mod);
	if (err < 0) {
		kmod_unref(ctx);
		return err;
	}
	err = kmod_module_get_info(mod, &list);
	kmod_module_info_free_list(list);
	kmod_module_unref(mod);
	kmod_unref(ctx);
	return err;
}

int main(int argc, char *argv[])
{
	static const struct {
		const char *name;
		uint16_t shstrndx;
	} cases[] = {
		{ "shstrndx-oob", 0x5f29 },
		{ "shstrndx-eq-count", 2 },
		{ "shstrndx-undef", SHN_UNDEF },
		{ "shstrndx-valid", 1 },
	};
	int crashed = 0;

	if (argc != 2) {
		fprintf(stderr, "usage: %s <outdir>\n", argv[0]);
		return 2;
	}

	for (size_t i = 0; i < sizeof(cases) / sizeof(cases[0]); i++) {
		char path[4096];
		int status, err;
		pid_t pid;

		snprintf(path, sizeof(path), "%s/%s.ko", argv[1], cases[i].name);
		err = write_elf(path, cases[i].shstrndx);
		if (err < 0) {
			fprintf(stderr, "write %s: %s\n", path, strerror(-err));
			return 2;
		}

		fflush(NULL);
		pid = fork();
		if (pid < 0) {
			perror("fork");
			return 2;
		}
		if (pid == 0) {
			err = get_info(path);
			printf("%-18s e_shstrndx=%-5u kmod_module_get_info() = %d (%s)\n",
			       cases[i].name, cases[i].shstrndx, err,
			       err < 0 ? strerror(-err) : "ok");
			fflush(stdout);
			_exit(0);
		}
		if (waitpid(pid, &status, 0) < 0) {
			perror("waitpid");
			return 2;
		}
		if (WIFSIGNALED(status)) {
			printf("%-18s e_shstrndx=%-5u CHILD KILLED BY SIGNAL %d (%s)\n",
			       cases[i].name, cases[i].shstrndx, WTERMSIG(status),
			       strsignal(WTERMSIG(status)));
			crashed = 1;
		} else if (WEXITSTATUS(status) != 0) {
			printf("%-18s child exited %d\n", cases[i].name, WEXITSTATUS(status));
			crashed = 1;
		}
	}

	printf("%s\n", crashed ? "RESULT: bug reproduced" : "RESULT: bug not present");
	return crashed;
}
