#pragma once

#include <Arduino.h>
#include <GxEPD2_BW.h>

class Display {
public:
    Display(uint16_t width, uint16_t height);

    void init();

    void draw_wifi_screen();

private:
    GxEPD2_BW<GxEPD2_750_T7, GxEPD2_750_T7::HEIGHT> display;
};