#pragma once

#include <Arduino.h>
#include <GxEPD2_BW.h>
#include <Fonts/FreeMonoBold9pt7b.h>

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

    void drawTitleScreen();

    void drawWifiScreen();

    void drawYui();

    void drawText(const char* s);

    void drawBitmap(int16_t x, int16_t y, const uint8_t* bmp, int16_t w, int16_t h, uint16_t color = GxEPD_BLACK);

private:
    void prepareDisplay(uint8_t rotation = 0, const GFXfont *f = &FreeMonoBold9pt7b, uint16_t textColor = GxEPD_BLACK);
    GxEPD2_BW<EPD_MODEL, EPD_MODEL::HEIGHT> display;
};
