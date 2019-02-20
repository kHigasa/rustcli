#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <inttypes.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>

static void usage(char *prgname) {
	fprintf(stdout, "Usage: %s [OPTION]... [FILE]...\n", prgname);
	exit(0);
}

int main(int argc, char **argv) {
	uint8_t buf[4096];
	ssize_t nread;
	int fd = 0; //STDIN_FILENO

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

	if (argc > 1) {
		for (int i = 1; i < argc; i++) {
			fd = open(argv[i], O_RDONLY);
			if (fd < 0) {
				fprintf(stderr, "cannot open %s: %s\n", argv[i], strerror(errno));
				exit(1);
			}
			while ((nread = read(fd, buf, sizeof(buf))) > 0)
				write (1, buf, nread);
		}
	}

	while ((nread = read(fd, buf, sizeof(buf))) > 0)
		write (1, buf, nread);

	return 0;
}

