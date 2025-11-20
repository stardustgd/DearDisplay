#include "bitmap.h"
#include "Display.h"
#include <Fonts/FreeMonoBold9pt7b.h>

Display::Display(uint16_t width, uint16_t height)
    : display(GxEPD2_750_T7(5, 17, 16, 4)) {}

void Display::init() {
  display.init(115200, true, 2, true);
} 

void Display::draw_wifi_screen() {
    const char connect_message[] = "Connect to WiFi!";
    display.setRotation(0);
    display.setFont(&FreeMonoBold9pt7b);
    display.setTextColor(GxEPD_BLACK);
    int16_t tbx, tby; uint16_t tbw, tbh;
    display.getTextBounds(connect_message, 0, 0, &tbx, &tby, &tbw, &tbh);

    // center the bounding box by transposition of the origin:
    uint16_t x = ((display.width() - tbw) / 2) - tbx;
    uint16_t y = ((display.height() - tbh) / 2) - tby;
    display.setFullWindow();
    display.firstPage();

    do
    {
      display.fillScreen(GxEPD_WHITE);
      display.setCursor(x, y + 150);
      display.print("Connect to WiFi!");

      uint16_t bitmap_x = (display.width() - 128) / 2;
  
      display.drawBitmap(bitmap_x, y - 64, wifiIcon, 128, 128, GxEPD_BLACK);
    }
    while (display.nextPage());
  
}

void Display::draw_yui() {
    const char connect_message[] = "Connect to WiFi!";
    display.setRotation(0);
    display.setFont(&FreeMonoBold9pt7b);
    display.setTextColor(GxEPD_BLACK);
    int16_t tbx, tby; uint16_t tbw, tbh;
    display.getTextBounds(connect_message, 0, 0, &tbx, &tby, &tbw, &tbh);

    // center the bounding box by transposition of the origin:
    uint16_t x = ((display.width() - tbw) / 2) - tbx;
    uint16_t y = ((display.height() - tbh) / 2) - tby;
    display.setFullWindow();
    display.firstPage();

    do
    {
      display.fillScreen(GxEPD_WHITE);
      display.setCursor(x, y + 150);
      display.print("hello world!");

      uint16_t bitmap_x = (display.width() - 128) / 2;
  
      display.drawBitmap(0, 0, epd_bitmap_Yui_Hirasawa_new_mugshot, 800, 480, GxEPD_BLACK);
    }
    while (display.nextPage());
  
}