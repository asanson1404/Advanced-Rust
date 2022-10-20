/*
 * Copyright (c) 2017 STMicroelectronics
 *
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/kernel.h>
#include <zephyr/drivers/led.h>
#include <zephyr/drivers/gpio.h>
#include <zephyr/drivers/i2c.h>
#include <stdio.h>
#include <math.h>
#include <zephyr/sys/printk.h>

#define LSM6DSL_NODE DT_NODELABEL(lsm6dsl)


static const struct i2c_dt_spec  lsm6dsl     = I2C_DT_SPEC_GET(LSM6DSL_NODE);
static const struct gpio_dt_spec irq_lsm6dsl = GPIO_DT_SPEC_GET(LSM6DSL_NODE, irq_gpios);

static struct gpio_callback lsm6dsl_cb_data;
struct k_work my_work_q;

static void read_who_am_i() {
	uint8_t reg_addr = 0x0f;
	uint8_t reg_val;
	i2c_reg_read_byte_dt(&lsm6dsl, reg_addr, &reg_val);
	printk("WHO_AM_I = %x\n\n", reg_val);
}

void interrupt_handler(const struct device *dev, struct gpio_callback *cb, uint32_t pins) {
	k_work_submit(&my_work_q);
}

void read_value(struct k_work *item) {
	
	uint8_t addr_reg1 = 0x28;
	int16_t data[3];
	float acc_tot;

	i2c_write_read_dt(&lsm6dsl, &addr_reg1, 1, &data, 6);
	//printk("X = %d, Y = %d, Z = %d\n", data[0], data[1], data[2]);

	// valeur totale de l'accélération, peu importe l'orientation
	acc_tot = sqrt(data[0]*data[0] + data[1]*data[1] + data[2]*data[2]);

	// calcul des angles et conversion en degrés
	float angleX = asinf(data[0]/acc_tot) * 180.0/3.14158;
	float angleY = asinf(data[1]/acc_tot) * 180.0/3.14158;
	float angleZ = acosf(data[2]/acc_tot) * 180.0/3.14158;

	printk("ANGLE_X = %.2f, ANGLE_Y = %.2f, ANGLE_Z = %.2f\n", angleX, angleY, angleZ);

}


void main(void)
{
	int ret;

	float test = cos(0.0);
	printk("TEST DU COSINUS  cos(0) = %f", test);

	k_work_init(&my_work_q, read_value);

	//============================================================
		// Check if the I2C bus is ready
	//============================================================
	if (!device_is_ready(lsm6dsl.bus)) {
		printk("I2C bus : bus %s not ready.\n", lsm6dsl.bus->name);
		return;
	}
	printk ("\nBus I2C %s is ready\n", lsm6dsl.bus->name);

	//============================================================
		// Config sensor registers
	//============================================================
	// Reset the sensor and the register address automatically incremented 
	// during a multiple byte access with a serial interface (I2C or SPI).
	i2c_reg_write_byte_dt(&lsm6dsl, 0x12, (uint8_t) 0x05);
	// XL_HM_MODE = 1 ==> high-performance operating mode disabled
	i2c_reg_write_byte_dt(&lsm6dsl, 0x15, (uint8_t) (1 << 4));
	// CTRL1_XL ==> the sensor performs measures at 1.6Hz 
	i2c_reg_write_byte_dt(&lsm6dsl, 0x10, (uint8_t) (0b1011 << 4));
	// Configure register to raise an interrupt on INT1 pad when Accelerometer Data Ready & Gyroscope Data Ready
	i2c_reg_write_byte_dt(&lsm6dsl, 0x0d, (uint8_t) 0x3);

	// X offset accelerometer
	i2c_reg_write_byte_dt(&lsm6dsl, 0x73, (int) 1);
	// Y offset accelerometer
	i2c_reg_write_byte_dt(&lsm6dsl, 0x74, (int) 18);

	//============================================================
		// Config interrupt
	//============================================================
	// Interrupt line as an input
	ret = gpio_pin_configure_dt(&irq_lsm6dsl, GPIO_INPUT);
	if (ret != 0) {
		printk("Error %d: failed to configure %s pin %d\n", ret, irq_lsm6dsl.port->name, irq_lsm6dsl.pin);
		return;
	}
	// Interruption when there is a raising edge on the interrupt line
	ret = gpio_pin_interrupt_configure_dt(&irq_lsm6dsl, GPIO_INT_EDGE_TO_ACTIVE);
	if (ret != 0) {
		printk("Error %d: failed to configure interrupt on %s pin %d\n", ret, irq_lsm6dsl.port->name, irq_lsm6dsl.pin);
	}
	gpio_init_callback(&lsm6dsl_cb_data, interrupt_handler, BIT(irq_lsm6dsl.pin));
	gpio_add_callback(irq_lsm6dsl.port, &lsm6dsl_cb_data);


	printk("All devices are ready.\nStarting ....... \n\n\n");


	read_who_am_i();
	// Necessary to enter a first time in the interrupt
	read_value(&my_work_q);

}
