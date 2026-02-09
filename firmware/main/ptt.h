#pragma once

/**
 * PTT and COS/SQL GPIO control
 *
 * PTT_OUT = GPIO 10 (Open Collector via MOSFET, Active Low)
 * COS_IN  = GPIO 11 (3.3V/5V tolerant, Internal Pull-Up)
 * LED     = GPIO 12 (WS2812B)
 */

void ptt_init(void);
void ptt_set(bool active);
bool ptt_get_cos(void);
