#include "wifi.h"
#include <ArduinoJson.h>
#include <HTTPClient.h>

const char* WIFI_CONFIG_PORTAL_SSID = "DearDisplay";
const char* WIFI_CONFIG_PORTAL_PASSWORD = "happy2years";

bool setupWifi() {
    WiFiManager wifiManager;
    wifiManager.setConnectTimeout(WIFI_CONNECT_TIMEOUT);
    wifiManager.setConfigPortalTimeout(WIFI_CONFIG_PORTAL_TIMEOUT);
    return wifiManager.autoConnect(WIFI_CONFIG_PORTAL_SSID, WIFI_CONFIG_PORTAL_PASSWORD);
}

bool getImage(const char *API_URL, uint8_t *buf) {
    HTTPClient http;

    http.begin(API_URL);
    int statusCode = http.GET();

    if (statusCode != HTTP_CODE_OK) {
        Serial.printf("Error on GET request: %s\n", http.errorToString(statusCode).c_str());
        return false;
    }

    int index = 0;
    int len = http.getSize();
    WiFiClient *stream = http.getStreamPtr();

    while (http.connected() && (index < len)) {
        if (stream->available()) {
            buf[index++] = stream->read();
        }
    }

    http.end();

    return true;
}