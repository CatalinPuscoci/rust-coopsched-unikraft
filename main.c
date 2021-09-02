#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

void *function(void *var) {

    int thread_id = *(int *) var;
    printf("nice from %d\n", thread_id);
}

int main(void) {
    int a = pthread_init();
    pthread_t tid[256];
    printf("pthread_init returned %d, PTE_TRUE is %d\n", a, PTE_TRUE);
    int thread_id[256];
    for (int i = 0; i < 256; i++) {
        thread_id[i] = i;
        pthread_create(&(tid[i]), NULL, function, &(thread_id[i]));
    }
    for (int i = 0; i < 256; i++) {
        pthread_join(tid[i], NULL);
    }
    rust_start();
    return 0;
}
