#include <Arduino.h>
#include <GxEPD2_BW.h>
#include <Fonts/FreeMonoBold9pt7b.h>
#include "bitmap.h"
#include "wifi.h"

#define GxEPD2_DRIVER_CLASS GxEPD2_750_T7  // GDEW075T7   800x480, EK79655 (GD7965), (WFT0583CZ61)

GxEPD2_BW<GxEPD2_DRIVER_CLASS, GxEPD2_DRIVER_CLASS::HEIGHT> display(
  GxEPD2_DRIVER_CLASS(/*CS*/ 5, /*DC*/ 17, /*RST*/ 16, /*BUSY*/ 4)
);

// put function declarations here:
void helloWorld();

void setup() {
  Serial.println("initializing");
  display.init(115200, true, 2, true); // USE THIS for Waveshare boards with "clever" reset circuit, 2ms reset pulse
  
  helloWorld();
  display.hibernate();

  setupWifi();
}

const char HelloWorld[] = "Connect to WiFi!";

void loop() {}

// put function definitions here:
void helloWorld() {
  display.setRotation(0);
  display.setFont(&FreeMonoBold9pt7b);
  display.setTextColor(GxEPD_BLACK);
  int16_t tbx, tby; uint16_t tbw, tbh;
  display.getTextBounds(HelloWorld, 0, 0, &tbx, &tby, &tbw, &tbh);
  // center the bounding box by transposition of the origin:
  uint16_t x = ((display.width() - tbw) / 2) - tbx;
  uint16_t y = ((display.height() - tbh) / 2) - tby;
  display.setFullWindow();
  display.firstPage();
  do
  {
    display.fillScreen(GxEPD_WHITE);
    display.setCursor(x, y + 150);
    display.print(HelloWorld);

    display.drawBitmap(x - 128, y - 128, wifiIcon, 128, 128, GxEPD_BLACK);
  }
  while (display.nextPage());
}