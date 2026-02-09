#pragma once

/**
 * ES8388 Audio Codec Driver
 *
 * I2C address: 0x10
 * I2S: 48kHz, 24-bit, Mono (Left channel)
 * ADC: Line-In (RX from Transceiver)
 * DAC: Line-Out (TX to Transceiver)
 */

void es8388_init(void);
void es8388_set_rx_gain(int db);   // 0-24 dB
void es8388_set_tx_volume(int db); // -96 to 0 dB
