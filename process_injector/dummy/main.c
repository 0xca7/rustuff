#include <stdio.h>
#include <unistd.h>

int
main()
{
    unsigned long i = 0;
    while(1)
    {
        sleep(1);
        printf("[%d] pid: %d\n", i, getpid());
        ++i;
    }
    return 0;
}