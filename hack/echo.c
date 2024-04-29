#include <sys/syscall.h>
#include <unistd.h>

const char message[] =
	"\n"
	"ucrun completed\n"
	"\n";

int main() {
	//write(1, message, sizeof(message) - 1);
	syscall(SYS_write, STDOUT_FILENO, message, sizeof(message) - 1);

	//_exit(0);
	//syscall(SYS_exit, 0);
	return 0;
}
