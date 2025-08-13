# IoT Data Processor

A Rust-based IoT data processing service designed for sensor data transformation and MQTT message handling using the Actor pattern.

## 🚀 Features

- **MQTT Data Processing**: Receives, processes, and republishes sensor data
- **Real-time Data Transformation**: Converts raw sensor data into structured JSON format
- **Device Registration Validation**: Ensures only registered devices can send data
- **Multi-language Support**: Chinese and English descriptions with units
- **Actor-based Architecture**: Concurrent message processing using Ractor
- **JSON Configuration**: External device database configuration
- **Environment-based Security**: Sensitive credentials stored in `.env` files

## 📋 Prerequisites

- Rust 1.70+ 
- MQTT1 Broker and MQTT2 Broker accessible
- Windows: Visual Studio Build Tools or GNU toolchain

## 🛠️ Installation

1. **Clone the repository**
```bash
git clone <repository-url>
cd dolomann-edge-server
```

2. **Install Rust dependencies**
```bash
cargo build
```

3. **Set up environment variables**
Copy the example environment file and customize it:
```bash
cp .env.example .env
```
Then edit `.env` with your actual configuration values.

4. **Configure device database**
Copy the example database file and customize it:
```bash
cp database_example.json database.json
```
Then edit `database.json` with your actual device information. See [Database Configuration](#-database-configuration) for format details.

## 🏃‍♂️ Running

```bash
cargo run
```

## 📊 System Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   IoT Devices   │───▶│   MQTT1 Broker   │───▶│  Edge Server    │
│                 │    │  (Input Channel) │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                                        │
                                                        ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Data Consumer │◀───│   MQTT2 Broker   │◀───│  Data Processor │
│                 │    │ (Output Channel) │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### Data Flow

1. **Ingestion**: Raw sensor data received via MQTT1 Broker
2. **Validation**: Device registration check against database
3. **Processing**: Data transformation and enrichment
4. **Publishing**: Structured JSON published via MQTT2 Broker

## 🏗️ Actor System

- **RouterActor**: Routes messages based on channel type (Data/OTA)
- **DataActor**: Processes sensor data and applies transformations
- **PublishActor**: Publishes processed data to output MQTT broker

## 📁 Database Configuration

The `database.json` file defines your device topology and sensor mappings. Use the provided `database_example.json` as a template.

```json
{
  "gateways": [
    {
      "gateway_id": 1,
      "gateway_macid": "AA:BB:CC:DD:EE:FF"
    }
  ],
  "brands": [
    {
      "brand_id": 1,
      "brand_name": "BRAND_A"
    }
  ],
  "device_types": [
    {
      "device_type_id": 1,
      "device_type_name": "DeviceType1"
    }
  ],
  "models": [
    {
      "model_id": 1,
      "model_name": "MODEL_X1",
      "brand_id": 1,
      "device_type_id": 1
    }
  ],
  "devices": [
    {
      "index": 1,
      "gateway_id": 1,
      "slaveid": 1,
      "model_id": 1
    }
  ],
  "data_mapping": [
    {
      "data_key": "param_01",
      "chinese_description": "參數一",
      "eng_description": "Parameter One",
      "unit": "V"
    }
  ]
}
```

## 📡 Message Format

### Input (Raw Sensor Data)
```json
{
  "slaveID": 1,
  "model": "MODEL_X1",
  "timestamp": "2024-01-01T12:00:00Z",
  "data": {
    "param_01": 0.8,
    "param_02": 85.5,
    "param_03": 1000.0
  }
}
```

### Output (Processed Data)
```json
[
  {
    "sensor_name_chinese": "參數一",
    "sensor_name_english": "Parameter One",
    "value": 0.8,
    "timestamp": "2024-01-01T12:00:00Z",
    "unit": "V"
  }
]
```

## 🔧 Development

### Project Structure
```
src/
├── main.rs                 # Application entry point
├── mqtt_client/           # MQTT client implementations
├── v0/
│   ├── actor/             # Actor implementations
│   │   ├── data_actor.rs  # Data processing actor
│   │   ├── publish_actor.rs # Publishing actor
│   │   └── mod.rs         # Router actor
│   ├── message_processor/ # Message parsing and validation
│   ├── gateway_modbus_device.rs # Database models
│   └── device_model.rs    # Device query interface
├── .env.example           # Environment variables template
└── database_example.json  # Database configuration template
```

### Key Dependencies
- `rumqttc`: MQTT client library
- `ractor`: Actor framework
- `serde`: Serialization framework
- `tokio`: Async runtime
- `dotenvy`: Environment variable loading
