#include <sys/stat.h>
#include <sys/types.h>
#include <inttypes.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>

static void usage(char *prgname) {
	fprintf(stdout, "Usage: %s [OPTION]... [FILE]...\n", prgname);
	exit(0);
}

int main(int argc, char **argv) {
	int option;
	opterr = 0;
	while ((option = getopt(argc, argv, "h")) != -1) {
		switch (option) {
			case 'h':
				usage(argv[0]);
				break;
			default:
				break;
		}
	}

	if (mkdir(argv[1], 0777) == -1) {
		fprintf(stderr, "%s: cannot create directory %s: %s\n", argv[0], argv[1], strerror(errno));
		exit(1);
	} else {
		return 0;
	}
}

