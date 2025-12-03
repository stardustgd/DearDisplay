#include "bitmap.h"
#include "display.h"
#include <Fonts/FreeMonoBold9pt7b.h>
#include <Fonts/FreeSerif18pt7b.h>

Display::Display()
    : display(EPD_MODEL(EPD_CS, EPD_DC, EPD_RST, EPD_BUSY)) {}

void Display::init() {
  display.init(EPD_SERIAL_BITRATE, EPD_INITIAL, EPD_RESET_DURATION, EPD_PULLDOWN_RST_MODE);
  Serial.println("display initialized");
} 

void Display::drawTitleScreen() {
  prepareDisplay(0, &FreeSerif18pt7b);

  const char* title = "DearDisplay";

  drawText(title);
}

void Display::drawWifiScreen() {
    const char connect_message[] = "Connect to WiFi!";

    prepareDisplay();

    int16_t tbx, tby; uint16_t tbw, tbh;
    display.getTextBounds(connect_message, 0, 0, &tbx, &tby, &tbw, &tbh);

    // center the bounding box by transposition of the origin:
    uint16_t x = ((display.width() - tbw) / 2) - tbx;
    uint16_t y = ((display.height() - tbh) / 2) - tby;
    display.setFullWindow();
    display.firstPage();

    do {
      display.fillScreen(GxEPD_WHITE);
      display.setCursor(x, y + 150);
      display.print(connect_message);

      uint16_t bitmap_x = (display.width() - 128) / 2;
  
      display.drawBitmap(bitmap_x, y - 64, wifiIcon, 128, 128, GxEPD_BLACK);
    } while (display.nextPage());
  
}

void Display::drawText(const char* s) {
  int16_t tbx, tby; uint16_t tbw, tbh;
  display.getTextBounds(s, 0, 0, &tbx, &tby, &tbw, &tbh);

  uint16_t x = (display.width() - tbw) / 2 - tbx;
  uint16_t y = (display.height() - tbh) / 2 - tby;
  display.setFullWindow();
  display.firstPage();

  do {
    display.fillScreen(GxEPD_WHITE);
    display.setCursor(x, y);
    display.print(s);
  } while (display.nextPage());
}

void Display::drawBitmap(int16_t x, int16_t y, const uint8_t* bmp, int16_t w, int16_t h, uint16_t color) {
  display.setFullWindow();
  display.firstPage();

  do {
    display.fillScreen(GxEPD_WHITE);
    display.drawBitmap(x, y, bmp, w, h, color);
  } while (display.nextPage());
}

// Sets rotation, font, color of the display
void Display::prepareDisplay(uint8_t rotation, const GFXfont *f, uint16_t textColor) {
  display.setRotation(rotation);
  display.setFont(f);
  display.setTextColor(textColor);
}
