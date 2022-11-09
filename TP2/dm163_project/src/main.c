#include <zephyr/device.h>
#include <zephyr/drivers/gpio.h>
#include <zephyr/drivers/led.h>
#include <zephyr/kernel.h>

#define DM163_NODE DT_NODELABEL(dm163)
static const struct device *dm163_dev = DEVICE_DT_GET(DM163_NODE);

#define NUM_CHANNELS    (8 * 3 * 8)
#define RGB_MATRIX_NODE DT_NODELABEL(rgb_matrix)
#define FREQ_IM         (1.0 / (8 * 60))

BUILD_ASSERT(DT_PROP_LEN(RGB_MATRIX_NODE, rows_gpios) == 8);

static const struct gpio_dt_spec rows[] = {
    GPIO_DT_SPEC_GET_BY_IDX(RGB_MATRIX_NODE, rows_gpios, 0),
    GPIO_DT_SPEC_GET_BY_IDX(RGB_MATRIX_NODE, rows_gpios, 1),
    GPIO_DT_SPEC_GET_BY_IDX(RGB_MATRIX_NODE, rows_gpios, 2),
    GPIO_DT_SPEC_GET_BY_IDX(RGB_MATRIX_NODE, rows_gpios, 3),
    GPIO_DT_SPEC_GET_BY_IDX(RGB_MATRIX_NODE, rows_gpios, 4),
    GPIO_DT_SPEC_GET_BY_IDX(RGB_MATRIX_NODE, rows_gpios, 5),
    GPIO_DT_SPEC_GET_BY_IDX(RGB_MATRIX_NODE, rows_gpios, 6),
    GPIO_DT_SPEC_GET_BY_IDX(RGB_MATRIX_NODE, rows_gpios, 7),
};

struct k_sem image_sem;
struct k_timer my_timer;

static struct { 
  uint8_t channels[NUM_CHANNELS];
} my_im;

// Give the semaphore every FREQ_IM seconds
static void update_image(struct k_timer *timer)
{
  k_sem_give(&image_sem);
}

int main()
{

  if (!device_is_ready(dm163_dev))
  {
    return -ENODEV;
  }

  // Fill my_im struct (corresponds to the image to print)
  for(int i=0; i < NUM_CHANNELS/2; i++) {
    // Image de rayures verticales de magenta et de vert
    my_im.channels[2 * i] = 255;
  }

  for (int row = 0; row < 8; row++)
    gpio_pin_configure_dt(&rows[row], GPIO_OUTPUT_INACTIVE);
  // Set brightness for all leds
  for (int i = 0; i < 8; i++)
    led_set_brightness(dm163_dev, i, 20);

  k_sem_init(&image_sem, 0, 1);
  k_timer_init(&my_timer, update_image, NULL);
  
  // Call update_image every FREQ_IM seconds
  k_timer_start(&my_timer, K_NO_WAIT, K_SECONDS(FREQ_IM));

  // Print a static image
  while(1) {
    for (int row = 0; row < 8; row++) {
      k_sem_take(&image_sem, K_FOREVER);
      // Desactivates all raws
      for (int i = 0; i < 8; i++) {
        gpio_pin_set_dt(&rows[i], 0);
      }
      led_write_channels(dm163_dev, 0, 3*8, &my_im.channels[3 * 8 * row]);
      gpio_pin_set_dt(&rows[row], 1);
    }
  }

}
