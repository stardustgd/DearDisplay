#pragma once

#include <Arduino.h>

#define SERIAL_BAUD_RATE 115200
#define WIFI_CONNECT_TIMEOUT 15 // seconds
#define WIFI_CONFIG_PORTAL_TIMEOUT 180 // seconds

#define TIME_TO_SLEEP 30
#define uS_TO_S 1000000

#define MAX_BUFFER_SIZE 48000

#define BASE_URL "http://192.168.1.199:4000/"
#define API_PATH "api/display"
