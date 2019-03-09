#include <sys/types.h>
#include <sys/stat.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <dirent.h>

int main(int argc, char **argv) {
	DIR *dirp;
	struct dirent *dirent;
	struct stat statbuf;

	char buf[1024];
	dirp = opendir(argv[1]);
	while ((dirent=readdir(dirp)) != NULL) {
		sprintf(buf, "%s/%s", argv[1], dirent->d_name);
		stat(buf, &statbuf);
		printf("%zu",statbuf.st_size);
		printf(" %s\n", dirent->d_name);
	}
	closedir(dirp);
}

