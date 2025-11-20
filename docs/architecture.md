## Web Server Design Planning

```
GET /api/display

headers = {
  'ID': 'XX:XX:XX:XX:XX:XX'
}

response:
{
  "status": 200,
  "image_bmp": "link_to_bmp",
  "refresh_rate": 1800
}
```

## Project Structure

```
root/
│
├── server/
│   ├── Cargo.toml
│   └── src/
│
├── web/
│   ├── package.json
│   └── src/
│
├── firmware/
│   ├── platformio.ini
│   └── src/
│
├── shared/                 # Shared code/schema between front + server + firmware
│   ├── protocol/           # (optional) Types or JSON schema definitions
│   │   └── display_message.json
│   ├── rust-types/         # Rust crates for shared types
│   └── ts-types/           # TypeScript generated types
│
├── infra/
│   ├── docker/
│   ├── k8s/                # Optional, in future
│   └── nginx/
│
├── docs/                   # Project docs
│   └── architecture.md
│
└── README.md

```
