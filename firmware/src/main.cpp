#include "Config.h"
#include "Display.h"
#include "wifi.h"

Display displayController(DISPLAY_WIDTH, DISPLAY_HEIGHT);

void setup() {
  Serial.println(115200);
  delay(1000);

  Serial.println("initializing displayController");
  displayController.init();
  displayController.draw_wifi_screen();

  Serial.println("initializing wifi");
  setupWifi();

  displayController.draw_yui();
}

void loop() {}