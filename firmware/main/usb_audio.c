/**
 * USB Audio + Serial Composite Device
 *
 * USB Composite: UAC2 (48kHz 24-bit Mono) + CDC (Serial)
 * Audio: I2S ring buffer <-> USB audio frames
 * Serial: Command parser (PTT, GAIN, MODE, STATUS, INFO)
 */

#include "usb_audio.h"
#include "esp_log.h"

static const char *TAG = "usb_audio";

void usb_audio_init(void)
{
    ESP_LOGI(TAG, "USB Audio init (TODO: TinyUSB UAC2 + CDC composite)");
    // TODO: Initialize TinyUSB with composite descriptor
    // - UAC2: 48kHz, 24-bit, Mono In + Out
    // - CDC: Virtual serial port for commands
}

void usb_audio_process_serial(void)
{
    // TODO: Read from CDC serial, parse commands:
    // PTT ON / PTT OFF
    // STATUS
    // SET RX_GAIN x
    // SET TX_GAIN x
    // MODE REPEATER / DIGIMODE / REMOTE
    // INFO
}
