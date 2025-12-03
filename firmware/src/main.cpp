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
  displayController.drawTitleScreen();

  Serial.println("initializing wifi");

  if (!setupWifi()) {
    Serial.println("wifi not connected; timed out");

    // TODO: Draw a "network not available" screen
    displayController.drawWifiScreen();

    Serial.println("entering deep sleep");
    
    esp_sleep_enable_timer_wakeup(TIME_TO_SLEEP * uS_TO_S);
    esp_deep_sleep_start();
  }

  HTTPClient http;

  // TODO: change to actual API url (must contain bin file of bmp to display)
  const char* API_URL = "http://tmpfiles.org/dl/13395127/output.bin";

  http.begin(API_URL);
  int statusCode = http.GET();

  if (statusCode != HTTP_CODE_OK) {
    Serial.printf("Error on GET request: %s\n", http.errorToString(statusCode).c_str());
  }

  int len = http.getSize();
  WiFiClient *stream = http.getStreamPtr();
  uint8_t* buf = (uint8_t*)malloc(len);

  if (!buf) {
    Serial.println("malloc failed");
    http.end();

    esp_sleep_enable_timer_wakeup(TIME_TO_SLEEP * uS_TO_S);
    esp_deep_sleep_start();
  }

  int index = 0;

  while (http.connected() && (index < len)) {
    if (stream->available()) {
      buf[index++] = stream->read();
    }
  }

  http.end();
  displayController.drawBitmap(0, 0, buf, 800, 480, GxEPD_BLACK);

  esp_sleep_enable_timer_wakeup(TIME_TO_SLEEP * uS_TO_S);
  esp_deep_sleep_start();
}

void loop() {}