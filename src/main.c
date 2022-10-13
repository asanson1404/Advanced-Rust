/*
 * Copyright (c) 2017 STMicroelectronics
 *
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/kernel.h>
#include <zephyr/device.h>
#include <zephyr/drivers/sensor.h>
#include <zephyr/drivers/gpio.h>
#include <zephyr/drivers/led.h>
#include <stdio.h>
#include <zephyr/sys/printk.h>

#define THREAD_STACK_SIZE 500
#define PRIORITY 1

	
// Preparing the messages queue
char __aligned(4) my_msgq_buffer[1 * sizeof(struct sensor_value)];
struct k_msgq my_msgq;

// Guetting led device
static struct gpio_dt_spec led = GPIO_DT_SPEC_GET_OR(DT_ALIAS(led0), gpios, {0});

// Thread setting the led value
void led_thread() {
	struct sensor_value value;
	double dist;
	gpio_pin_set_dt(&led, 0);
	while(1) {
		k_msgq_get(&my_msgq, &value, K_NO_WAIT);
		dist = sensor_value_to_double(&value);

		gpio_pin_toggle_dt(&led);
		k_msleep(dist*500);
		//printf("distance is %.3fm\n", sensor_value_to_double(&value));
	}
}

// Preparing the thread
K_THREAD_STACK_DEFINE(thread_stack_area, THREAD_STACK_SIZE);
struct k_thread my_thread_data;


void main(void)
{
	const struct device *const dev = DEVICE_DT_GET_ONE(st_vl53l0x);
	struct sensor_value value;
	int ret;

	k_thread_create (&my_thread_data, thread_stack_area,
                     K_THREAD_STACK_SIZEOF(thread_stack_area),
                     led_thread,
                     NULL, NULL, NULL,
                     PRIORITY, 0, K_NO_WAIT
					);


	//============================================================
		// Check if the sensor is ready
	//============================================================
	if (dev == NULL) {
		/* No such node, or the node does not have status "okay". */
		printk("\nError: no device %s found.\n", dev->name);
		return;
	}

	if (!device_is_ready(dev)) {
		printk("sensor : device %s not ready.\n", dev->name);
		return;
	}

	printk ("Found device %s, getting sensor data\n", dev->name);

	//============================================================
		// Check if the LED is ready
	//============================================================
	if (led.port && !device_is_ready(led.port)) {
		printk("Error : LED device %s is not ready; ignoring it\n", led.port->name);
		led.port = NULL;
	}

	if (led.port) {
		ret = gpio_pin_configure_dt(&led, GPIO_OUTPUT);
		if (ret != 0) {
			printk("Error %d: failed to configure LED device %s pin %d\n", ret, led.port->name, led.pin);
			led.port = NULL;
		} else {
			printk("Set up LED at %s pin %d\n\n", led.port->name, led.pin);
		}
	}

	// Initializing the message queue
	k_msgq_init(&my_msgq, my_msgq_buffer, sizeof(struct sensor_value), 1);

	printk("All devices are ready.\n Starting ....... \n");

	// Receiving sensor's datas and sending the message 
	while (1) {

		ret = sensor_sample_fetch(dev);
		if (ret) {
			printk("sensor_sample_fetch failed ret %d\n", ret);
			return;
		}

		sensor_channel_get(dev, SENSOR_CHAN_DISTANCE, &value);

		/* send data to the thread */
		while (k_msgq_put(&my_msgq, &value, K_NO_WAIT) != 0) {
           	/* message queue is full: purge old data & try again */
           	k_msgq_purge(&my_msgq);
    	}

		k_sleep(K_MSEC(100));
	}
}
