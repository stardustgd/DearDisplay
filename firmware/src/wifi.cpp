#include "wifi.h"
#include <WiFiManager.h>

void setupWifi() {
    WiFiManager wifiManager;
    wifiManager.setConfigPortalTimeout(180);
    wifiManager.autoConnect("E-Paper Display", "happy2years");
}