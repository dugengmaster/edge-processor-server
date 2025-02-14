# 資料庫結構說明文件

本文件描述了 `gateway register modbus device` 系統中使用的資料庫表格結構。

## 資料表

### 1. `gateways` 表

*   **用途:** 儲存閘道資訊。
*   **欄位:**
    *   `id` (INTEGER, 主鍵, 自動遞增)
    *   `macid` (TEXT, 唯一索引)

### 2. `brands` 表

*   **用途:** 儲存品牌資訊。
*   **欄位:**
    *   `id` (INTEGER, 主鍵, 自動遞增)
    *   `name` (TEXT)

### 3. `equipment_types` 表

*   **用途:** 儲存設備類型資訊。
*   **欄位:**
    *   `id` (INTEGER, 主鍵, 自動遞增)
    *   `name` (TEXT)

### 4. `models` 表

*   **用途:** 儲存設備型號資訊。
*   **欄位:**
    *   `id` (INTEGER, 主鍵, 自動遞增)
    *   `name` (TEXT)
    *   `brand_id` (INTEGER, 外鍵, 參考 `brands` 表)
    *   `equipment_type_id` (INTEGER, 外鍵, 參考 `equipment_types` 表)

### 5. `equipment` 表

*   **用途:** 儲存 Modbus 設備資訊。
*   **欄位:**
    *   `index` (INTEGER, 主鍵, 自動遞增)
    *   `modbus_slave_id` (INTEGER)
    *   `model_id` (INTEGER, 外鍵, 參考 `models` 表)
    *   `name`(TEXT)
    *   `horsepower`(INTEGER)

### 6. costomers

* **用途:** 儲存客戶資訊。
* **欄位:**
    *   `id` (INTEGER, 主鍵, 自動遞增)
    *   `name` (TEXT)
    *   `person` (TEXT)
    *   `email` (TEXT)
    *   `phone` (TEXT)
    *   `address` (TEXT)

### 7. `customer_contacts` 表

*   **用途:** 儲存客戶的聯絡方式。
*   **欄位:**
    *   `contact_id` (INTEGER, 主鍵, 自動遞增)
    *   `customer_id` (INTEGER, 外鍵, 參考 `customers` 表)
    *   `contact_type` (TEXT)  # 例如 "email", "phone", "line", "whatsapp"
    *   `contact_value` (TEXT)  # 聯絡方式的值

### 8. `users` 表

*   **用途:** 儲存使用者帳號密碼。
*   **欄位:**
    *   `user_id` (INTEGER, 主鍵, 自動遞增)
    *   `customer_id` (INTEGER, 外鍵, 參考 `customers` 表)
    *   `username` (TEXT, 唯一索引)
    *   `password` (TEXT)
    *   `role` (TEXT)  # 角色，例如 "admin", "user"


## 資料表關係

*   `gateway register modbus device` 表格關聯到 `models` 表格，使用 `model_id` 作為外鍵。
*   `gateway register modbus device` 表格關聯到 `gateways` 表格, 使用 `gateway_id` 作為外鍵。
*   `models` 表格關聯到 `brands` 和 `device_types` 表格，使用 `brand_id` 和 `device_type_id` 作為外鍵。

```sql
-- 客戶與設備的直接關係表
CREATE TABLE customer_equipment (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    customer_id INTEGER,
    equipment_id INTEGER,
    created_at TIMESTAMP,
    FOREIGN KEY (customer_id) REFERENCES customers(id),
    FOREIGN KEY (equipment_id) REFERENCES equipment(index)
);

-- sensors_data 直接關聯到 customer_equipment
CREATE TABLE sensors_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    customer_equipment_id INTEGER,  -- 關聯到客戶的設備
    timestamp TIMESTAMP,
    -- 感測器數據欄位
    FOREIGN KEY (customer_equipment_id) REFERENCES customer_equipment(id)
);

-- gateway 的部分變成純通訊設定
CREATE TABLE gateway_equipment_mapping (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gateway_id INTEGER,
    customer_equipment_id INTEGER,  -- 改為關聯到 customer_equipment
    modbus_slave_id INTEGER,
    created_at TIMESTAMP,
    FOREIGN KEY (gateway_id) REFERENCES gateways(id),
    FOREIGN KEY (customer_equipment_id) REFERENCES customer_equipment(id)
);
```