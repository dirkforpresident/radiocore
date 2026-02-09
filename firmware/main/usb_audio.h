#pragma once

/**
 * USB Audio Class 2.0 + CDC Serial Composite Device
 *
 * Registers as:
 *   - USB Audio: 48kHz, 24-bit, Mono In + Mono Out
 *   - USB CDC: Virtual serial port for PTT and config
 */

void usb_audio_init(void);
void usb_audio_process_serial(void);
