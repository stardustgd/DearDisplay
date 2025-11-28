#include "wifi.h"

const char* WIFI_CONFIG_PORTAL_SSID = "DearDisplay";
const char* WIFI_CONFIG_PORTAL_PASSWORD = "happy2years";

bool setupWifi() {
    WiFiManager wifiManager;
    wifiManager.setConnectTimeout(WIFI_CONNECT_TIMEOUT);
    wifiManager.setConfigPortalTimeout(WIFI_CONFIG_PORTAL_TIMEOUT);
    return wifiManager.autoConnect(WIFI_CONFIG_PORTAL_SSID, WIFI_CONFIG_PORTAL_PASSWORD);
}