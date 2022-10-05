#include <stdio.h>
#include <zephyr/drivers/gpio.h>
#include <zephyr/drivers/led.h>
#include <zephyr/device.h>
#include <zephyr/kernel.h>


#define THREAD_STACK_SIZE   500
#define PRIORITY            5
#define LEDS_NODE           DT_PATH(leds)

// Get the struct device pointer to leds at compile time
const struct device * const leds_node = DEVICE_DT_GET(LEDS_NODE);

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

    if (!device_is_ready(leds_node)) {
        return -ENODEV;
    }
    for(;;) {
        led_on(leds_node, 0);
        led_off(leds_node, 1);
        k_sleep(K_SECONDS(1));
        led_on(leds_node, 1);
        led_off(leds_node, 0);
        k_sleep(K_SECONDS(1));
    }

        return 0;
}