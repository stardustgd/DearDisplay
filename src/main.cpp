#include "Config.h"
#include "Display.h"
#include "wifi.h"

Display displayController(DISPLAY_WIDTH, DISPLAY_HEIGHT);

void setup() {
  Serial.println("initializing");

  displayController.init();
  displayController.draw_wifi_screen();

  setupWifi();
}

void loop() {}