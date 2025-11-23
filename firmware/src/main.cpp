#include "Config.h"
#include "Display.h"
#include "wifi.h"

Display displayController;

void setup() {
  Serial.begin(SERIAL_BAUD_RATE);
  delay(2000);

  Serial.println("initializing displayController");
  displayController.init();
  displayController.draw_wifi_screen();

  Serial.println("initializing wifi");
  setupWifi();

  displayController.draw_yui();
}

void loop() {}