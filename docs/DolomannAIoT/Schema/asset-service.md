# **設備規格管理模組 (`asset-service`)**

## **1️⃣ `device_brands`（設備品牌表）**

```sql
CREATE TABLE device_brands (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE UNIQUE INDEX device_brands_unique_active
ON device_brands (name)
WHERE is_active = TRUE;
```

**`device_brands` 表**

- **用途:** 用於儲存設備或產品所屬的品牌資訊。

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER** (主鍵，自動遞增)
    - 說明: 品牌的唯一識別碼。
  - **`name`**
    - 型態: **TEXT** (UNIQUE, NOT NULL)
    - 說明: 品牌名稱，需保證唯一性。
  - **`is_active`**
    - 型態: **BOOLEAN** (UNIQUE, NOT NULL)
    - 說明: 是否啟用，若為 `FALSE` 表示已停用。
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設值: `NOW()`)
    - 說明: 設備品牌的建立時間。

## **2️⃣ `device_types`（設備類型表）**

```sql
CREATE TABLE device_types (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE UNIQUE INDEX device_types_unique_active
ON device_types (name)
WHERE is_active = TRUE;
```

**`device_types` 表**

- **用途:** 用於儲存設備的類型資訊，如 "空壓機"、"水泵" 等。

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER** (主鍵，自動遞增)
    - 說明: 設備類型的唯一識別碼。
  - **`name`**
    - 型態: **TEXT** (UNIQUE, NOT NULL)
    - 說明: 設備類型的名稱，且需保證唯一。
  - **`is_active`**
    - 型態: **BOOLEAN** (UNIQUE, NOT NULL)
    - 說明: 設備類型是否啟用，若為 `FALSE` 表示已停用。
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設值: `NOW()`)
    - 說明: 設備類型的建立時間。

## **3️⃣ `device_specs`（設備規格表）**

```sql
CREATE TABLE device_specs (
    id SERIAL PRIMARY KEY,
    static_specs JSONB NULL,  -- 靜態規格，如馬力、冷卻方式
    dynamic_specs JSONB NULL, -- 動態規格，如壓力、電流範圍
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**`device_specs` 表**

- **用途:** 儲存設備型號的完整規格定義。

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER** (主鍵，自動遞增)
    - 說明: 唯一識別碼。
  - **`static_specs`**
    - 型態: **JSONB** (NOT NULL)
    - 說明: 儲存不會變動的靜態規格，如馬力、冷卻方式等。
    - 範例:
      ```json
      {
        "horsepower": "15HP",
        "cooling_method": "Water Cooled",
        "power_supply": "3-Phase 220V"
      }
      ```
  - **`dynamic_specs`**
    - 型態: **JSONB** (NOT NULL)
    - 說明: 儲存可變動的動態規格，如壓力、電流、啟動 / 停止狀態等。
    - 範例:
      ```json
      {
        "pressure": {
          "value": 0.65,
          "unit": "bar"
        },
        "current": {
          "value": 32,
          "unit": "A"
        }
      }
      ```
  - **`is_active`**
    - 型態: **BOOLEAN** (預設 `TRUE`)
    - 說明: 表示此規格定義是否仍有效。
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設值: `NOW()`)
    - 說明: 規格定義的建立時間。

## **4️⃣ `device_models`（設備型號表）**

**重要注意事項：**
- 此表格作為對外窗口，使用 UUID 作為主鍵。
- `updated_at` 欄位由業務層負責更新，不使用資料庫觸發器。
- 所有對此表格的更新操作都必須包含 `updated_at` 的更新。
- 當規格有更新時，只需要更新 `device_spec_id` 指向新的 `device_specs` 記錄，不需要建立新的 `device_models` 記錄。

```sql
CREATE TABLE device_models (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    brand_id INTEGER NOT NULL REFERENCES device_brands(id),
    device_type_id INTEGER NOT NULL REFERENCES device_types(id),
    device_spec_id INTEGER NOT NULL REFERENCES device_specs(id),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);
```

**`device_models` 表**

- **用途:** 儲存設備的型號資訊，以品牌、設備類型與規格之間的關係為主。

- **欄位說明:**
  - **`id`**
    - 型態: **UUID** (主鍵)
    - 說明: 該設備型號的全局唯一識別碼，用於跨服務引用。
  - **`name`**
    - 型態: **TEXT** (UNIQUE, NOT NULL)
    - 說明: 型號名稱，需保證唯一。
  - **`description`**
    - 型態: **TEXT**
    - 說明: 關於此型號的描述，例如功能、特色等。
  - **`brand_id`**
    - 型態: **INTEGER** (外鍵，參考 `brands.id`，NOT NULL)
    - 說明: 指定此型號所屬的品牌。
  - **`device_type_id`**
    - 型態: **INTEGER** (外鍵，參考 `device_types.id`，NOT NULL)
    - 說明: 指定此型號的設備類型。
  - **`device_spec_id`**
    - 型態: **INTEGER** (外鍵，參考 `device_specs.id`，NOT NULL)
    - 說明: 指向與此型號關聯的設備規格。
  - **`is_active`**
    - 型態: **BOOLEAN** (UNIQUE, NOT NULL)
    - 說明: 設備型號資訊是否啟用，若為 `FALSE` 表示已停用
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設值: `NOW()`)
    - 說明: 規格定義的建立時間。
  - **`updated_at`**
    - 型態: **TIMESTAMPTZ**
    - 說明: 最後更新時間，。

## **1️⃣ `equipment`（設備表）**

```sql
CREATE TABLE equipment (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    facility_id UUID NOT NULL REFERENCES site_facilities(id),
    model_id INTEGER NOT NULL REFERENCES device_models(id),
    name TEXT NOT NULL,
    serial_number TEXT UNIQUE NOT NULL,
    purchase_date DATE,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);
```

**`equipment` 表**

- **用途:** 用於儲存每個實際設備的資訊，包含其所屬地點、型號等

- **欄位說明:**
  - **`id`**
    - 型態: **UUID** (主鍵，預設使用 `uuid_generate_v4()` 產生)
    - 說明: 設備的唯一識別碼。
  - **`facility_id`**
    - 型態: **UUID** (外鍵，參考 `site_facilities.id`)
    - 說明: 該設備所屬的設施位置 ID。
  - **`model_id`**
    - 型態: **INTEGER** (外鍵，參考 `device_models.id`)
    - 說明: 指向該設備所使用的型號（含品牌、類型、規格）。
  - **`name`**
    - 型態: **TEXT** (NOT NULL)
    - 說明: 自訂的設備名稱或代號。
  - **`serial_number`**
    - 型態: **TEXT** (UNIQUE, NOT NULL)
    - 說明: 設備序號，須保證唯一。
  - **`purchase_date`**
    - 型態: **DATE**
    - 說明: 購買或取得日期。
  - **`is_active`**
    - 型態: **BOOLEAN** (預設 `TRUE`)
    - 說明: 用來標記設備是否仍在使用。
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 建立時間。
  - **`updated_at`**
    - 型態: **TIMESTAMPTZ**
    - 說明: 最後更新時間。

## **2️⃣ `maintenance_records`（設備維護紀錄）**

```sql
CREATE TABLE maintenance_records (
    id SERIAL PRIMARY KEY,
    equipment_id UUID NOT NULL REFERENCES equipment(id),
    installation_date DATE,
    maintenance_date DATE,
    maintenance_type TEXT,
    description TEXT,
    maintained_by TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**`maintenance_records` 表**

- **用途:** 紀錄設備的安裝與維護歷史，以便追蹤與維護管理。

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER** (主鍵，自動遞增)
    - 說明: 維護紀錄的唯一識別碼。
  - **`equipment_id`**
    - 型態: **UUID** (外鍵，參考 `equipment.id`，NOT NULL)
    - 說明: 該維護紀錄所屬的設備 ID。
  - **`installation_date`**
    - 型態: **DATE**
    - 說明: 設備安裝日期 (若適用)。
  - **`maintenance_date`**
    - 型態: **DATE**
    - 說明: 此次維護的執行日期。
  - **`maintenance_type`**
    - 型態: **TEXT**
    - 說明: 維護類型 (如定期保養、故障維修等)。
  - **`description`**
    - 型態: **TEXT**
    - 說明: 對此次維護的詳述說明。
  - **`maintained_by`**
    - 型態: **TEXT**
    - 說明: 執行維護的人員或單位。
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 紀錄建立時間。
