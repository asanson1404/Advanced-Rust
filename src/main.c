#include <stdio.h>
#include <zephyr/drivers/gpio.h>
#include <zephyr/device.h>
#include <zephyr/kernel.h>


#define THREAD_STACK_SIZE   500
#define PRIORITY            5
#define GPIOA_NODE          DT_NODELABEL(gpioa)

// Get the struct device pointer to gpioa at compile time
static const struct device * const gpioa_dev = DEVICE_DT_GET(GPIOA_NODE);

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

    if (!device_is_ready(gpioa_dev)) {
        return -ENODEV;  // Device is not ready
    }
    // Configure the led as an output, initially inactive
    gpio_pin_configure(gpioa_dev, 5, GPIO_OUTPUT_INACTIVE);
    // Toggle the led every second
    for (;;) {
        gpio_pin_toggle(gpioa_dev, 5);
        k_sleep(K_SECONDS(1));
    }

    return 0;
}