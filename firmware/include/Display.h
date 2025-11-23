#pragma once

#include <Arduino.h>
#include <GxEPD2_BW.h>

#define EPD_MODEL GxEPD2_750_T7

const int16_t EPD_CS = 5;
const int16_t EPD_DC = 17;
const int16_t EPD_RST = 16;
const int16_t EPD_BUSY = 4;
const uint32_t EPD_SERIAL_BITRATE = 115200;
const bool EPD_INITIAL= false;
const uint16_t EPD_RESET_DURATION = 2;
const bool EPD_PULLDOWN_RST_MODE = true;

class Display {
public:
    Display();

    void init();

    void draw_wifi_screen();

    void draw_yui();

private:
    GxEPD2_BW<EPD_MODEL, EPD_MODEL::HEIGHT> display;
};
