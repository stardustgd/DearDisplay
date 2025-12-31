#pragma once

#include "config.h"
#include <WiFiManager.h>

bool setupWifi();
bool getImage(const char *API_URL, uint8_t *buf);