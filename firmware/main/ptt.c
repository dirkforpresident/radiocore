/**
 * PTT output and COS/SQL input GPIO control
 */

#include "ptt.h"
#include "driver/gpio.h"
#include "esp_log.h"

static const char *TAG = "ptt";

#define PTT_PIN     GPIO_NUM_10
#define COS_PIN     GPIO_NUM_11
#define LED_PIN     GPIO_NUM_12

void ptt_init(void)
{
    gpio_config_t ptt_cfg = {
        .pin_bit_mask = (1ULL << PTT_PIN),
        .mode = GPIO_MODE_OUTPUT,
        .pull_up_en = GPIO_PULLUP_DISABLE,
        .pull_down_en = GPIO_PULLDOWN_DISABLE,
    };
    gpio_config(&ptt_cfg);
    gpio_set_level(PTT_PIN, 1);  // PTT off (active low)

    gpio_config_t cos_cfg = {
        .pin_bit_mask = (1ULL << COS_PIN),
        .mode = GPIO_MODE_INPUT,
        .pull_up_en = GPIO_PULLUP_ENABLE,
        .pull_down_en = GPIO_PULLDOWN_DISABLE,
    };
    gpio_config(&cos_cfg);

    ESP_LOGI(TAG, "PTT (GPIO %d) and COS (GPIO %d) initialized", PTT_PIN, COS_PIN);
}

void ptt_set(bool active)
{
    gpio_set_level(PTT_PIN, active ? 0 : 1);
    ESP_LOGD(TAG, "PTT %s", active ? "ON" : "OFF");
}

bool ptt_get_cos(void)
{
    return gpio_get_level(COS_PIN) == 0;
}
