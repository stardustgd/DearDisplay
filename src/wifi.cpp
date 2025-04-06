#include "wifi.h"

void setupWifi() {
    WiFiManager wifiManager;
    wifiManager.autoConnect("E-Paper Display", "happy2years");
}