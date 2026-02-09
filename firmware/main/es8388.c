/**
 * ES8388 Codec Driver — I2C configuration, I2S audio
 *
 * Pin mapping (ESP32-S3):
 *   I2S_BCK  = GPIO 4
 *   I2S_WS   = GPIO 5
 *   I2S_DOUT = GPIO 6 (to DAC)
 *   I2S_DIN  = GPIO 7 (from ADC)
 *   I2C_SDA  = GPIO 8
 *   I2C_SCL  = GPIO 9
 */

#include "es8388.h"
#include "esp_log.h"
#include "driver/i2c.h"
#include "driver/i2s_std.h"

static const char *TAG = "es8388";

#define ES8388_ADDR     0x10
#define I2C_PORT        I2C_NUM_0

// I2S pins
#define I2S_BCK_PIN     4
#define I2S_WS_PIN      5
#define I2S_DOUT_PIN    6
#define I2S_DIN_PIN     7

// I2C pins
#define I2C_SDA_PIN     8
#define I2C_SCL_PIN     9

static esp_err_t es8388_write_reg(uint8_t reg, uint8_t val)
{
    uint8_t data[2] = {reg, val};
    return i2c_master_write_to_device(I2C_PORT, ES8388_ADDR, data, 2, pdMS_TO_TICKS(100));
}

void es8388_init(void)
{
    // Initialize I2C
    i2c_config_t i2c_cfg = {
        .mode = I2C_MODE_MASTER,
        .sda_io_num = I2C_SDA_PIN,
        .scl_io_num = I2C_SCL_PIN,
        .sda_pullup_en = GPIO_PULLUP_ENABLE,
        .scl_pullup_en = GPIO_PULLUP_ENABLE,
        .master.clk_speed = 100000,
    };
    i2c_param_config(I2C_PORT, &i2c_cfg);
    i2c_driver_install(I2C_PORT, I2C_MODE_MASTER, 0, 0, 0);

    // ES8388 initialization sequence
    es8388_write_reg(0x08, 0x00);  // Chip control: power on
    es8388_write_reg(0x02, 0xF3);  // Power management: all on
    es8388_write_reg(0x2B, 0x80);  // DAC control: left channel
    es8388_write_reg(0x00, 0x36);  // Master mode, I2S 24-bit
    es8388_write_reg(0x01, 0x72);  // 48kHz sample rate
    es8388_write_reg(0x09, 0x00);  // ADC PGA gain: 0dB
    es8388_write_reg(0x04, 0x00);  // DAC power on

    ESP_LOGI(TAG, "ES8388 initialized (48kHz, 24-bit, Line In/Out)");

    // TODO: Initialize I2S in standard mode
    ESP_LOGI(TAG, "I2S init (TODO: ESP-IDF v5 i2s_std driver)");
}

void es8388_set_rx_gain(int db)
{
    if (db < 0) db = 0;
    if (db > 24) db = 24;
    uint8_t val = db / 3;
    es8388_write_reg(0x09, val);
    ESP_LOGI(TAG, "RX gain: %d dB", db);
}

void es8388_set_tx_volume(int db)
{
    if (db < -96) db = -96;
    if (db > 0) db = 0;
    uint8_t val = (uint8_t)((-db) * 2);
    es8388_write_reg(0x2E, val);
    ESP_LOGI(TAG, "TX volume: %d dB", db);
}
