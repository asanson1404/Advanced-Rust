/*
 * Copyright (c) 2017 STMicroelectronics
 *
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/kernel.h>
#include <zephyr/device.h>
#include <zephyr/drivers/sensor.h>
#include <stdio.h>
#include <zephyr/sys/printk.h>

void main(void)
{
	const struct device *const dev = DEVICE_DT_GET_ONE(st_vl53l0x);
	struct sensor_value value;
	int ret;

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

	while (1) {

		ret = sensor_sample_fetch(dev);
		if (ret) {
			printk("sensor_sample_fetch failed ret %d\n", ret);
			return;
		}

		sensor_channel_get(dev, SENSOR_CHAN_DISTANCE, &value);
		printf("distance is %.3fm\n", sensor_value_to_double(&value));

		k_sleep(K_MSEC(500));
	}
}
