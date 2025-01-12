# 資料庫結構說明文件

本文件描述了 `gateway register modbus device` 系統中使用的資料庫表格結構。

## 資料表

### 1. `gateways` 表

*   **用途:** 儲存閘道資訊。
*   **欄位:**
    *   `gateway_id` (INTEGER, 主鍵, 自動遞增)
    *   `gateway_macid` (TEXT, 唯一索引)

### 2. `brands` 表

*   **用途:** 儲存品牌資訊。
*   **欄位:**
    *   `brand_id` (INTEGER, 主鍵, 自動遞增)
    *   `brand_name` (TEXT)

### 3. `device_types` 表

*   **用途:** 儲存設備類型資訊。
*   **欄位:**
    *   `device_type_id` (INTEGER, 主鍵, 自動遞增)
    *   `device_type_name` (TEXT)

### 4. `models` 表

*   **用途:** 儲存設備型號資訊。
*   **欄位:**
    *   `model_id` (INTEGER, 主鍵, 自動遞增)
    *   `model_name` (TEXT)
    *   `brand_id` (INTEGER, 外鍵, 參考 `brands` 表)
    *   `device_type_id` (INTEGER, 外鍵, 參考 `device_types` 表)

### 5. `gateway register modbus device` 表

*   **用途:** 儲存閘道註冊的 Modbus 設備資訊。
*   **欄位:**
    *   `index` (INTEGER, 主鍵, 自動遞增)
    *   `gateway_id` (INTEGER, 外鍵, 參考 `gateways` 表)
    *   `slaveid` (INTEGER)
    *   `model_id` (INTEGER, 外鍵, 參考 `models` 表)

## 資料表關係

*   `gateway register modbus device` 表格關聯到 `models` 表格，使用 `model_id` 作為外鍵。
*   `gateway register modbus device` 表格關聯到 `gateways` 表格, 使用 `gateway_id` 作為外鍵。
*   `models` 表格關聯到 `brands` 和 `device_types` 表格，使用 `brand_id` 和 `device_type_id` 作為外鍵。
