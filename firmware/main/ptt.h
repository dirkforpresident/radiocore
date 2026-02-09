#pragma once

#include <stdbool.h>
#include <stdint.h>

/**
 * PTT, COS/SQL, and Expansion GPIO control
 *
 * PTT_OUT  = GPIO 10 (Open Collector via MOSFET, Active Low)
 * COS_IN   = GPIO 11 (Internal Pull-Up, Active Low)
 * LED      = GPIO 12 (WS2812B)
 * EXP_OUT1 = GPIO 13 (Expansion Header Pin 5)
 * EXP_OUT2 = GPIO 14 (Expansion Header Pin 6)
 * EXP_OUT3 = GPIO 15 (Expansion Header Pin 7)
 * EXP_OUT4 = GPIO 16 (Expansion Header Pin 8)
 */

void ptt_init(void);
void ptt_set(bool active);
bool ptt_get_cos(void);
void exp_set(uint8_t output, bool active);
