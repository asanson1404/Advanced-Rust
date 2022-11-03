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
#define PI 3.14158
#define GYRO_SPEED 1.0/104.0

static const struct i2c_dt_spec  lsm6dsl     = I2C_DT_SPEC_GET(LSM6DSL_NODE);
static const struct gpio_dt_spec irq_lsm6dsl = GPIO_DT_SPEC_GET(LSM6DSL_NODE, irq_gpios);

static struct gpio_callback lsm6dsl_cb_data;
struct k_work   my_work_q;
struct k_timer  print_timer;
struct k_mutex 	mutex;

typedef struct {
	int16_t			x;
	int16_t			y;
	int16_t			z;
} data_t;

static struct {
	data_t accel;
	data_t gyro;
} imu_data;


static void read_who_am_i() {
	uint8_t reg_addr = 0x0f;
	uint8_t reg_val;
	i2c_reg_read_byte_dt(&lsm6dsl, reg_addr, &reg_val);
	printk("WHO_AM_I = %x\n\n", reg_val);
}

static void interrupt_handler(const struct device *dev, struct gpio_callback *cb, uint32_t pins) {
		k_work_submit(&my_work_q);
}

static void read_value(struct k_work *item) {
	
	uint8_t first_reg_addr = 0x1e;

	struct {
		uint8_t status; 	//  0x1e
		uint8_t _reserved;	//  0x1f
		int16_t _temp;  	//  0x20 & 0x21
		data_t  gyr;		//  0x22 to 0x27
		data_t  acc;		//  0x28 to 0x2d
	} i2c_data;

	// read the I2C bus from register 0x1e to register 0x2d
	i2c_write_read_dt(&lsm6dsl, &first_reg_addr, 1, &i2c_data, sizeof(i2c_data));

	k_mutex_lock(&mutex, K_FOREVER);

	if(!(i2c_data.status & 0x03)){
			printk("No new values of gyroscope and accelerometer........\n");
		}
	
	if (i2c_data.status & 0x01) {
		imu_data.accel = i2c_data.acc;
	}

	if (i2c_data.status & 0x02) {
		imu_data.gyro = i2c_data.gyr;
	}

	k_mutex_unlock(&mutex);
}

// We assume that at the beginning, the board is flat on the table
// angle Z has no sense here
float angleX_g = 0;
float angleY_g = 0;
static void compute_and_print(struct k_timer *timer)
{
	// Données brutes acc
	//printk("ACC  ===>  X = %d, Y = %d, Z = %d\n", imu_data.accel.x, imu_data.accel.y, imu_data.accel.z);

	// valeur totale de l'accélération, peu importe l'orientation
	float acc_tot = sqrt(imu_data.accel.x*imu_data.accel.x + imu_data.accel.y*imu_data.accel.y + imu_data.accel.z*imu_data.accel.z);

	// calcul des angles et conversion en degrés
	float angleX_xl = asinf(imu_data.accel.x/acc_tot) * 180.0/PI;
	float angleY_xl = asinf(imu_data.accel.y/acc_tot) * 180.0/PI;
	float angleZ_xl = acosf(imu_data.accel.z/acc_tot) * 180.0/PI;

	// Données converties en angle de l'acc
	//printk("ACC  ===>  ANGLE_X = %.2f, ANGLE_Y = %.2f, ANGLE_Z = %.2f\n", angleX_xl, angleY_xl, angleZ_xl);

	// On converti les données brutes du gyro en dsp et on en déduit l'angle d'inclinaison (fréquence du gyro 104Hz)
	angleX_g = angleX_g + imu_data.gyro.y * 250.0/(1<<15) * GYRO_SPEED; 
	angleY_g = angleY_g + imu_data.gyro.x * 250.0/(1<<15) * GYRO_SPEED;

	// données brutes gyro
	//printk("GYR  ===>  X = %d, Y = %d, Z = %d\n", imu_data.gyro.x, imu_data.gyro.y, imu_data.gyro.z);

	// Données converties en angle du gyro
	printk("GYR  ===>  ANGLE_X = %.2f, ANGLE_Y = %.2f\n", angleX_g, angleY_g);
}


void main(void)
{
	int ret;

	k_work_init(&my_work_q, read_value);
	k_timer_init(&print_timer, compute_and_print, NULL);
	k_mutex_init(&mutex);

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
	// G_HM_MODE = 1 ==> high-performance operating mode disabled
	i2c_reg_write_byte_dt(&lsm6dsl, 0x16, (uint8_t) (1 << 7));
	// CTRL1_XL ==> the sensor performs measures at 104Hz 
	i2c_reg_write_byte_dt(&lsm6dsl, 0x10, (uint8_t) (0b0100 << 4));
	// CTRL2_G ==> the sensor performs measures at 104Hz
	i2c_reg_write_byte_dt(&lsm6dsl, 0x11, (uint8_t) (0b0100 << 4));
	// Configure register to raise an interrupt on INT1 pad when Accelerometer & Gyroscope Data Ready
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

	// We print values every 10ms (100Hz)
	k_timer_start(&print_timer, K_SECONDS(1), K_MSEC(10));
}
