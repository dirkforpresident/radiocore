/**
 * RadioCore Firmware — ESP32-S3
 *
 * USB Composite Device: UAC2 Audio + CDC Serial
 * I2S <-> ES8388 Audio Codec
 * GPIO: PTT, COS, Status LED
 *
 * Firmware modes (set via serial command or WiFi):
 *   MODE REPEATER  — standalone repeater (COS->PTT, timers, WAV announcements)
 *   MODE DIGIMODE  — USB audio bridge (transparent pass-through)
 *   MODE REMOTE    — WebSocket client to cloud, audio + CAT streaming
 *
 * Serial commands:
 *   PTT ON / PTT OFF          — manual PTT control
 *   SET RX_GAIN <0-24>        — ES8388 input gain (dB)
 *   SET TX_GAIN <0-24>        — ES8388 output volume (dB)
 *   EXP <1-4> ON/OFF          — expansion GPIO control
 *   STATUS                    — JSON status report
 *   MODE REPEATER/DIGIMODE/REMOTE — switch firmware mode
 *
 * Core 0: Audio (I2S + USB Audio, highest priority)
 * Core 1: USB Serial + Control logic + WiFi
 */

#include <stdio.h>
#include <string.h>
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "esp_log.h"
#include "driver/gpio.h"

#include "usb_audio.h"
#include "es8388.h"
#include "ptt.h"

static const char *TAG = "radiocore";

typedef enum {
    MODE_DIGIMODE = 0,   // Default: transparent USB audio bridge
    MODE_REPEATER = 1,   // Standalone repeater controller
    MODE_REMOTE = 2,     // Remote station audio + CAT bridge
} radiocore_mode_t;

static radiocore_mode_t current_mode = MODE_DIGIMODE;

void app_main(void)
{
    ESP_LOGI(TAG, "RadioCore Firmware v0.1.0 starting...");
    ESP_LOGI(TAG, "Mode: %s", current_mode == MODE_REPEATER ? "REPEATER" :
                               current_mode == MODE_REMOTE ? "REMOTE" : "DIGIMODE");

    // Initialize ES8388 Audio Codec via I2C + I2S
    ESP_LOGI(TAG, "Initializing ES8388 codec...");
    es8388_init();

    // Initialize PTT and COS GPIO
    ESP_LOGI(TAG, "Initializing PTT/COS...");
    ptt_init();

    // Initialize USB Composite Device (Audio + Serial)
    ESP_LOGI(TAG, "Initializing USB...");
    usb_audio_init();

    ESP_LOGI(TAG, "RadioCore ready.");

    // Main loop
    while (1) {
        bool cos = ptt_get_cos();

        // In REPEATER mode: auto PTT on COS
        if (current_mode == MODE_REPEATER) {
            ptt_set(cos);
        }

        // Process USB serial commands
        usb_audio_process_serial();

        vTaskDelay(pdMS_TO_TICKS(10));
    }
}
