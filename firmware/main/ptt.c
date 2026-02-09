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

// Expansion Header GPIOs (active high, accent external MOSFETs for relays)
#define EXP_OUT1    GPIO_NUM_13
#define EXP_OUT2    GPIO_NUM_14
#define EXP_OUT3    GPIO_NUM_15
#define EXP_OUT4    GPIO_NUM_16
#define EXP_GPIO_MASK ((1ULL << EXP_OUT1) | (1ULL << EXP_OUT2) | (1ULL << EXP_OUT3) | (1ULL << EXP_OUT4))

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

    // Expansion GPIO outputs (active high, accent external MOSFETs)
    gpio_config_t exp_cfg = {
        .pin_bit_mask = EXP_GPIO_MASK,
        .mode = GPIO_MODE_OUTPUT,
        .pull_up_en = GPIO_PULLUP_DISABLE,
        .pull_down_en = GPIO_PULLDOWN_DISABLE,
    };
    gpio_config(&exp_cfg);
    gpio_set_level(EXP_OUT1, 0);
    gpio_set_level(EXP_OUT2, 0);
    gpio_set_level(EXP_OUT3, 0);
    gpio_set_level(EXP_OUT4, 0);

    ESP_LOGI(TAG, "PTT (GPIO %d), COS (GPIO %d), EXP (GPIO %d-%d) initialized",
             PTT_PIN, COS_PIN, EXP_OUT1, EXP_OUT4);
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

void exp_set(uint8_t output, bool active)
{
    gpio_num_t pin;
    switch (output) {
        case 1: pin = EXP_OUT1; break;
        case 2: pin = EXP_OUT2; break;
        case 3: pin = EXP_OUT3; break;
        case 4: pin = EXP_OUT4; break;
        default: return;
    }
    gpio_set_level(pin, active ? 1 : 0);
    ESP_LOGD(TAG, "EXP%d %s", output, active ? "ON" : "OFF");
}
