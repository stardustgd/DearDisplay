#include "config.h"
#include "display.h"
#include "wifi.h"
#include <ArduinoJson.h>
#include <HTTPClient.h>

Display displayController;

void setup() {
  Serial.begin(SERIAL_BAUD_RATE);
  delay(2000);

  Serial.println("initializing displayController");
  displayController.init();

  Serial.println("initializing wifi");

  if (!setupWifi()) {
    Serial.println("wifi not connected; timed out");

    // TODO: Draw a "network not available" screen
    displayController.drawWifiScreen();

    Serial.println("entering deep sleep");
    
    esp_sleep_enable_timer_wakeup(TIME_TO_SLEEP * uS_TO_S);
    esp_deep_sleep_start();
  }

  uint8_t* buf = new uint8_t[MAX_BUFFER_SIZE];

  const char* API_URL = BASE_URL API_PATH;

  if (getImage(API_URL, buf)) {
    displayController.drawBitmap(0, 0, buf, 800, 480, GxEPD_BLACK);
  }

  esp_sleep_enable_timer_wakeup(TIME_TO_SLEEP * uS_TO_S);
  esp_deep_sleep_start();
}

void loop() {}
