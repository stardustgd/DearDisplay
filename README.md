# DearDisplay

## MVP

### Frontend

- Date display
  - Month, Day, Year
- Weather display
  - List of temperatures throughout the day
  - Forecast for the next 5 days
- Message to display
  - Have a component for a message to be displayed
  - How do we set/fetch this message?

### Backend

- Take in an image, return a BMP compatible with the e-ink display
- Have an endpoint, `/api/display` that returns an image, as well as the time for the ESP32 to sleep

### Hardware

- WiFi Connectivity
  - Check if connected,
  - If not, start AP and show error on e-ink
  - If connected, fetch from the endpoint and display image
