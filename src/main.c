#include <stdio.h>
#include <zephyr/kernel.h>


#define THREAD_STACK_SIZE 500
#define PRIORITY 5

void my_thread() {
    printf("Hello, world\n");
    while(1){}
}

K_THREAD_STACK_DEFINE(thread_stack_area, THREAD_STACK_SIZE);
struct k_thread my_thread_data;

int main() {

    printf("Hello, world\n");

    k_tid_t tid = k_thread_create (&my_thread_data, thread_stack_area,
                                      K_THREAD_STACK_SIZEOF(thread_stack_area),
                                      my_thread,
                                      NULL, NULL, NULL,
                                      PRIORITY, 0, K_NO_WAIT);

    k_thread_suspend(tid);
    //k_thread_abort(tid);

    return 0;
}
