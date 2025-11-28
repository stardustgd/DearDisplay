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
    esp_deep_sleep_start();
  }

  HTTPClient http;
  // TODO: change to actual API url
  const char* API_URL = "https://dummyjson.com/quotes/random";

  http.begin(API_URL);
  int statusCode = http.GET();
  String payload;

  if (statusCode == HTTP_CODE_OK) {
    payload = http.getString();
    Serial.println(payload);
  } else {
    Serial.printf("Error on GET request: %s\n", http.errorToString(statusCode).c_str());
  }

  http.end();

  JsonDocument doc;
  deserializeJson(doc, payload);

  const char* quote = doc["quote"];
  const char* author = doc["author"];

  displayController.drawText(quote);

  delay(5000);

  displayController.drawText(author);

  delay(5000);

  displayController.drawYui();

  esp_deep_sleep_start();
}

void loop() {}