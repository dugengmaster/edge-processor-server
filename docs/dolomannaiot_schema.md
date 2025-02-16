## **目錄**

## 資料表詳細說明

### **企業與組織管理模組 (`org-service`)**

#### **1️⃣ `enterprises`（企業表）**

```sql
CREATE TABLE enterprises (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    tax_id TEXT NOT NULL,
    description TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_by INTEGER REFERENCES users(id),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 確保 name 和 tax_id 在 is_active = TRUE 時唯一
CREATE UNIQUE INDEX enterprises_unique_active
ON enterprises (name, tax_id)
WHERE is_active = TRUE;
```

**`enterprises` 表**

- **用途:** 儲存企業相關的核心資訊

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER**
    - 說明: 企業的唯一識別碼，自動遞增
  - **`name`**
    - 型態: **TEXT (NOT NULL)**
    - 說明: 企業名稱
  - **`tax_id`**
    - 型態: **TEXT (NOT NULL)**
    - 說明: 統一編號
  - **`description`**
    - 型態: **TEXT**
    - 說明: 企業描述或相關備註
  - **`is_active`**
    - 型態: **BOOLEAN (預設 TRUE)**
    - 說明: 標記企業是否為現行有效
  - **`created_by`**
    - 型態: **INTEGER** (外鍵，參考 `users.id`)
    - 說明: 建立該筆記錄的使用者 ID
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 建立時間
  - **`updated_by`**
    - 型態: **INTEGER** (外鍵，參考 `users.id`)
    - 說明: 最後更新該筆記錄的使用者 ID
  - **`updated_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 最後更新時間

#### **2️⃣ `enterprise_sites`（企業據點表）**

```sql
CREATE TABLE enterprise_sites (
    id SERIAL PRIMARY KEY,
    enterprise_id INTEGER NOT NULL REFERENCES enterprises(id),
    name TEXT NOT NULL,
    type TEXT,
    location_info JSONB NOT NULL,
    public_contact JSONB NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_by INTEGER REFERENCES users(id),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

**`enterprise_sites` 表**

- **用途:** 儲存企業的各個營運據點資訊

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER**
    - 說明: 主鍵，自動遞增
  - **`enterprise_id`**
    - 型態: **INTEGER** (外鍵，參考 `enterprises.id`)
    - 說明: 關聯的企業 ID
  - **`name`**
    - 型態: **TEXT**
    - 說明: 據點名稱
  - **`type`**
    - 型態: **TEXT**
    - 說明: 據點類型 (例如：總公司、分公司、倉庫、廠房)
  - **`location_info`**
    - 型態: **JSONB**
    - 說明: 據點的詳細位置資訊
    - 範例內容:
      ```json
      {
        "address": "No. 42, Example Street, Example City",
        "coordinate": {
          "lat": 25.034,
          "lon": 121.5645
        },
        "region": {
          "country": "Taiwan",
          "state": "Taipei City",
          "city": "Taipei",
          "district": "Zhongzheng District",
          "postal_code": "100"
        }
      }
      ```
    - 包含欄位:
      - `address`: 完整地址
      - `coordinate`: 座標（`lat`、`lon`）
      - `region`: 行政區域 (如 `country`, `state`, `city`, `district`, `postal_code`)
  - **`public_contact`**
    - 型態: **JSONB**
    - 說明: 對外公開的聯絡資訊
    - 範例內容:
      ```json
      {
        "main_phone": "02-1234-5678",
        "service_phone": "0800-123-456",
        "emergency_phone": "0912-345-678",
        "fax": "02-8765-4321",
        "email": "info@example.com"
      }
      ```
    - 包含欄位:
      - `main_phone`: 總機電話
      - `service_phone`: 客服電話
      - `emergency_phone`: 緊急聯絡電話
      - `fax`: 傳真號碼
      - `email`: 公共信箱
  - **`is_active`**
    - 型態: **BOOLEAN** (預設 `TRUE`)
    - 說明: 用來標記此據點是否仍在使用
  - **`created_by`**
    - 型態: **INTEGER** (外鍵，參考 `users.id`)
    - 說明: 建立此紀錄的使用者 ID
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 建立時間
  - **`updated_by`**
    - 型態: **INTEGER** (外鍵，參考 `users.id`)
    - 說明: 最後更新此紀錄的使用者 ID
  - **`updated_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 最後更新時間

#### **3️⃣ `site_facilities`（設施分區表）**

```sql
CREATE TABLE site_facilities (
    id SERIAL PRIMARY KEY,
    site_id INTEGER NOT NULL REFERENCES enterprise_sites(id),
    name TEXT NOT NULL,
    floor TEXT,
    facility_type TEXT,
    description TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_by INTEGER REFERENCES users(id),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

**`site_facilities` 表**

- **用途:** 管理據點內的設施分區

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER**
    - 說明: 主鍵，自動遞增
  - **`site_id`**
    - 型態: **INTEGER** (外鍵，參考 `enterprise_sites.id`)
    - 說明: 關聯的據點 ID
  - **`name`**
    - 型態: **TEXT**
    - 說明: 設施名稱，如「注塑車間」、「原料倉」
  - **`floor`**
    - 型態: **TEXT**
    - 說明: 樓層資訊
  - **`facility_type`**
    - 型態: **TEXT**
    - 說明: 設施類型（例如：生產車間、倉儲、公共設施區等）
  - **`description`**
    - 型態: **TEXT**
    - 說明: 設施描述
  - **`is_active`**
    - 型態: **BOOLEAN** (預設 `TRUE`)
    - 說明: 標記此設施是否仍在使用
  - **`created_by`**
    - 型態: **INTEGER** (外鍵，參考 `users.id`)
    - 說明: 建立此紀錄的使用者 ID
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 建立時間
  - **`updated_by`**
    - 型態: **INTEGER** (外鍵，參考 `users.id`)
    - 說明: 最後更新此紀錄的使用者 ID
  - **`updated_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 最後更新時間

#### **4️⃣ `enterprise_contacts`（企業聯絡人表）**

```sql
CREATE TABLE enterprise_contacts (
    id SERIAL PRIMARY KEY,
    site_id INTEGER NOT NULL REFERENCES enterprise_sites(id),
    name TEXT NOT NULL,
    title TEXT,
    contact_info JSONB NOT NULL,
    rank_order INTEGER,
    is_primary BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_by INTEGER REFERENCES users(id),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

**`enterprise_contacts` 表**

- **用途:** 儲存企業據點的聯絡人資訊

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER**
    - 說明: 主鍵，自動遞增
  - **`site_id`**
    - 型態: **INTEGER** (外鍵，參考 `enterprise_sites.id`)
    - 說明: 關聯的據點 ID
  - **`name`**
    - 型態: **TEXT**
    - 說明: 聯絡人姓名
  - **`title`**
    - 型態: **TEXT**
    - 說明: 職稱
  - **`contact_info`**
    - 型態: **JSONB**
    - 說明: 聯絡方式資訊
    - 範例內容:
      ```json
      {
        "contact": {
          "office_phone": "02-1234-5678",
          "extension": "123",
          "mobile": "0912-345-678",
          "fax": "02-8765-4321",
          "email": "jane.doe@example.com",
          "messaging_apps": [
            {
              "type": "LINE",
              "id": "janedoe_line"
            },
            {
              "type": "Telegram",
              "id": "@janedoe"
            }
          ]
        }
      }
      ```
    - 包含欄位:
      - **`contact`**: 聯絡方式 (物件)
        - `office_phone`: 辦公室電話
        - `extension`: 分機
        - `mobile`: 手機
        - `fax`: 傳真
        - `email`: 電子郵件
        - `messaging_apps`: 通訊軟體的聯絡方式 (陣列)
          - `type`: 例如：LINE、WeChat、WhatsApp、Telegram、Skype 等
          - `id`: 對應的 ID 或帳號
  - **`rank_order`**
    - 型態: **INTEGER**
    - 說明: 職位排序（主要用於高層
  - **`is_primary`**
    - 型態: **BOOLEAN** (預設 `FALSE`)
    - 說明: 是否為主要聯絡人
  - **`is_active`**
    - 型態: **BOOLEAN** (預設 `TRUE`)
    - 說明: 此聯絡人是否在職/有效
  - **`created_by`**
    - 型態: **INTEGER** (外鍵，參考 `users.id`)
    - 說明: 建立紀錄的使用者 ID
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 建立時間
  - **`updated_by`**
    - 型態: **INTEGER** (外鍵，參考 `users.id`)
    - 說明: 最後更新紀錄的使用者 ID
  - **`updated_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 最後更新時間

### **用戶管理模組 (`user-service`)**

#### **1️⃣ `users`（使用者帳號表）**

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT CHECK (role IN ('SUPER_ADMIN', 'ADMIN', 'SITE_ADMIN', 'SITE_USER')) NOT NULL,
    site_id INTEGER REFERENCES enterprise_sites(id),
    last_login TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

**`users` 表**

- **用途:** 紀錄並管理使用者帳號與權限資訊

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER** (主鍵，自動遞增)
    - 說明: 使用者的唯一識別碼
  - **`email`**
    - 型態: **TEXT (UNIQUE, NOT NULL)**
    - 說明: 使用者的電子郵件，並作為登入帳號
  - **`password_hash`**
    - 型態: **TEXT (NOT NULL)**
    - 說明: 使用者密碼的雜湊值（Hash）
  - **`role`**
    - 型態: **TEXT** (符合 `('SUPER_ADMIN', 'ADMIN', 'SITE_ADMIN', 'SITE_USER')`)
    - 說明: 帳號角色類型
      - **SUPER_ADMIN**: 系統最高管理者
      - **ADMIN**: 全域管理者
      - **SITE_ADMIN**: 特定據點管理者
      - **SITE_USER**: 一般使用者
  - **`site_id`**
    - 型態: **INTEGER** (外鍵，參考 `enterprise_sites.id`)
    - 說明: 若該使用者僅限於某個據點管理，可以關聯到該據點
  - **`last_login`**
    - 型態: **TIMESTAMPTZ**
    - 說明: 使用者最後一次登入時間
  - **`is_active`**
    - 型態: **BOOLEAN** (預設 `TRUE`)
    - 說明: 帳號是否有效，若為 `FALSE` 表示已停用
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 帳號建立時間
  - **`updated_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 最後更新時間

#### **2️⃣ `user_permissions`（使用者權限表）**

```sql
CREATE TABLE user_permissions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    permission_type TEXT CHECK (permission_type IN ('VIEW_DASHBOARD', 'EXPORT_DATA', 'MANAGE_EQUIPMENT', 'MANAGE_USERS', 'MANAGE_ALERTS')) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**`user_permissions` 表**

- **用途:** 用於記錄使用者擁有的特定權限，透過 `user_id` 與 `users` 表關聯

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER**
    - 說明: 主鍵，自動遞增
  - **`user_id`**
    - 型態: **INTEGER** (外鍵，參考 `users.id`)
    - 說明: 權限所歸屬的使用者 ID
  - **`permission_type`**
    - 型態: **TEXT** (符合 `('VIEW_DASHBOARD', 'EXPORT_DATA', 'MANAGE_EQUIPMENT', 'MANAGE_USERS', 'MANAGE_ALERTS')`)
    - 說明: 權限類型
      - **VIEW_DASHBOARD**: 可檢視儀表板
      - **EXPORT_DATA**: 可匯出資料
      - **MANAGE_EQUIPMENT**: 可管理設備
      - **MANAGE_USERS**: 可管理使用者
      - **MANAGE_ALERTS**: 可管理警報/通知
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 權限建立時間

#### **3️⃣ `password_reset_tokens`（密碼重置令牌表）**

```sql
CREATE TABLE password_reset_tokens (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    token TEXT NOT NULL,
    expired_at TIMESTAMPTZ NOT NULL,
    is_used BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 確保 user_id 只能有一個未使用的 token
CREATE UNIQUE INDEX unique_active_tokens
ON password_reset_tokens (user_id, token)
WHERE is_used = FALSE;
```

**`password_reset_tokens` 表**

- **用途:** 儲存使用者的密碼重置令牌資訊，用於帳號密碼重設流程

- **欄位說明:**

  - **`id`**
    - 型態: **INTEGER** (主鍵，自動遞增)
    - 說明: 唯一識別碼
  - **`user_id`**
    - 型態: **INTEGER** (外鍵，參考 `users.id`)
    - 說明: 關聯到欲重置密碼的使用者
  - **`token`**
    - 型態: **TEXT** (NOT NULL)
    - 說明: 用於身份驗證的重置令牌
  - **`expired_at`**
    - 型態: **TIMESTAMPTZ** (NOT NULL)
    - 說明: 此令牌的到期時間，超過後便無法使用
  - **`is_used`**
    - 型態: **BOOLEAN** (預設 `FALSE`)
    - 說明: 標記此令牌是否已被使用完成
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 建立時間

#### **4️⃣ `login_logs`（登入紀錄表）**

```sql
CREATE TABLE login_logs (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    login_at TIMESTAMPTZ DEFAULT NOW(),
    ip_address TEXT NOT NULL,
    user_agent TEXT,
    status TEXT CHECK (status IN ('SUCCESS', 'FAILED')) NOT NULL,
    fail_reason TEXT
);
```

**`login_logs` 表**

- **用途:** 紀錄使用者的登入紀錄，包括成功與失敗的嘗試，以便後續稽核與檢查

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER** (主鍵，自動遞增)
    - 說明: 該登入紀錄的唯一識別碼
  - **`user_id`**
    - 型態: **INTEGER** (外鍵，參考 `users.id`)
    - 說明: 進行登入動作的使用者 ID
  - **`login_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 紀錄此登入操作的發生時間
  - **`ip_address`**
    - 型態: **TEXT** (NOT NULL)
    - 說明: 來源 IP 位址
  - **`user_agent`**
    - 型態: **TEXT**
    - 說明: 用戶端的瀏覽器或裝置資訊 (User-Agent)
  - **`status`**
    - 型態: **TEXT** (符合 `('SUCCESS', 'FAILED')`, NOT NULL)
    - 說明: 此次登入的結果
      - **SUCCESS**: 登入成功
      - **FAILED**: 登入失敗
  - **`fail_reason`**
    - 型態: **TEXT**
    - 說明: 登入失敗的原因，若登入成功可為 NULL

### **設備規格管理模組 (`device-service`)**

#### **1️⃣ `brands`（品牌表）**

```sql
CREATE TABLE brands (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);
```

**`brands` 表**

- **用途:** 用於儲存設備或產品所屬的品牌資訊

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER** (主鍵，自動遞增)
    - 說明: 品牌的唯一識別碼
  - **`name`**
    - 型態: **TEXT** (UNIQUE, NOT NULL)
    - 說明: 品牌名稱，需保證唯一性

#### **2️⃣ `device_types`（設備類型表）**

```sql
CREATE TABLE device_types (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);
```

**`device_types` 表**

- **用途:** 用於儲存設備的類型資訊，如 "空壓機"、"水泵" 等。

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER** (主鍵，自動遞增)
    - 說明: 設備類型的唯一識別碼
  - **`name`**
    - 型態: **TEXT** (UNIQUE, NOT NULL)
    - 說明: 設備類型的名稱，且需保證唯一

#### **3️⃣ `device_specs`（設備規格表）**

```sql
CREATE TABLE device_specs (
    id SERIAL PRIMARY KEY,
    static_specs JSONB NOT NULL,  -- 靜態規格，如馬力、冷卻方式
    dynamic_specs JSONB NOT NULL, -- 動態規格，如壓力、電流範圍
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL
);
```

**`device_specs` 表**

- **用途:** 儲存設備型號的完整規格定義

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER** (主鍵，自動遞增)
    - 說明: 唯一識別碼
  - **`static_specs`**
    - 型態: **JSONB** (NOT NULL)
    - 說明: 儲存不會變動的靜態規格，如馬力、冷卻方式等
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
    - 說明: 儲存可變動的動態規格，如壓力、電流、啟動 / 停止狀態等
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
    - 說明: 表示此規格定義是否仍有效
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (NOT NULL)
    - 說明: 此規格定義的建立時間

#### **4️⃣ `device_models`（設備型號表）**

```sql
CREATE TABLE device_models (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    brand_id INTEGER NOT NULL REFERENCES brands(id),
    device_type_id INTEGER NOT NULL REFERENCES device_types(id),
    device_spec_id INTEGER NOT NULL REFERENCES device_specs(id)
);
```

> **注意：** 當規格有更新時，只需要更新 `device_spec_id` 指向新的 `device_specs` 記錄，不需要建立新的 `device_models` 記錄。

**`device_models` 表**

- **用途:** 儲存設備的型號資訊，以品牌、設備類型與規格之間的關係為主。

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER** (主鍵，自動遞增)
    - 說明: 該設備型號的唯一識別碼。
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
- **注意:** 若規格更新，僅需更新 `device_spec_id` 指向新的 `device_specs` 記錄即可。

### **設備實例管理模組 (`equipment-service`)**

#### **1️⃣ `equipment`（設備表）**

```sql
CREATE TABLE equipment (
    id SERIAL PRIMARY KEY,
    facility_id INTEGER NOT NULL REFERENCES site_facilities(id),
    model_id INTEGER NOT NULL REFERENCES device_models(id),
    name TEXT NOT NULL,
    serial_number TEXT UNIQUE NOT NULL,
    purchase_date DATE,
    is_active BOOLEAN DEFAULT TRUE,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_by INTEGER REFERENCES users(id),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

**`equipment` 表**

- **用途:** 用於儲存每個實際設備的資訊，包含其所屬地點、型號等

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER** (主鍵，自動遞增)
    - 說明: 設備的唯一識別碼
  - **`facility_id`**
    - 型態: **INTEGER** (外鍵，參考 `site_facilities.id`)
    - 說明: 該設備所屬的設施位置 ID
  - **`model_id`**
    - 型態: **INTEGER** (外鍵，參考 `device_models.id`)
    - 說明: 指向該設備所使用的型號（含品牌、類型、規格）
  - **`name`**
    - 型態: **TEXT** (NOT NULL)
    - 說明: 自訂的設備名稱或代號
  - **`serial_number`**
    - 型態: **TEXT** (UNIQUE, NOT NULL)
    - 說明: 設備序號，須保證唯一
  - **`purchase_date`**
    - 型態: **DATE**
    - 說明: 購買或取得日期
  - **`is_active`**
    - 型態: **BOOLEAN** (預設 `TRUE`)
    - 說明: 用來標記設備是否仍在使用
  - **`created_by`**
    - 型態: **INTEGER** (外鍵，參考 `users.id`)
    - 說明: 建立此紀錄的使用者 ID
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 建立時間
  - **`updated_by`**
    - 型態: **INTEGER** (外鍵，參考 `users.id`)
    - 說明: 最後更新此紀錄的使用者 ID
  - **`updated_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 最後更新時間

#### **2️⃣ `maintenance_records`（設備維護紀錄）**

```sql
CREATE TABLE maintenance_records (
    id SERIAL PRIMARY KEY,
    equipment_id INTEGER NOT NULL REFERENCES equipment(id),
    installation_date DATE,
    maintenance_date DATE,
    maintenance_type TEXT,
    description TEXT,
    maintained_by TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**`maintenance_records` 表**

- **用途:** 紀錄設備的安裝與維護歷史，以便追蹤與維護管理

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER** (主鍵，自動遞增)
    - 說明: 維護紀錄的唯一識別碼
  - **`equipment_id`**
    - 型態: **INTEGER** (外鍵，參考 `equipment.id`，NOT NULL)
    - 說明: 該維護紀錄所屬的設備 ID
  - **`installation_date`**
    - 型態: **DATE**
    - 說明: 設備安裝日期 (若適用)
  - **`maintenance_date`**
    - 型態: **DATE**
    - 說明: 此次維護的執行日期
  - **`maintenance_type`**
    - 型態: **TEXT**
    - 說明: 維護類型 (如定期保養、故障維修等)
  - **`description`**
    - 型態: **TEXT**
    - 說明: 對此次維護的詳述說明
  - **`maintained_by`**
    - 型態: **TEXT**
    - 說明: 執行維護的人員或單位
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 紀錄建立時間

### **閘道器管理模組 (`gateway-service`)**

### **閘道器管理模組 (`gateway-service`)**

#### **1️⃣ `gateways`（閘道器表）**

```sql
CREATE TABLE gateways (
    id SERIAL PRIMARY KEY,
    macid TEXT UNIQUE NOT NULL,
    facility_id INTEGER NOT NULL REFERENCES site_facilities(id),
    firmware_version TEXT,
    fs_version TEXT,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_by INTEGER REFERENCES users(id),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

**`gateways` 表**

- **用途:** 用於儲存閘道器的基本資訊，包含硬體、韌體版本等。

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER** (主鍵，自動遞增)
    - 說明: 閘道器紀錄的唯一識別碼。
  - **`macid`**
    - 型態: **TEXT** (UNIQUE, NOT NULL)
    - 說明: 閘道器的 MAC 位址 (或類似編號)，需保證唯一。
  - **`facility_id`**
    - 型態: **INTEGER** (外鍵，參考 `site_facilities.id`，NOT NULL)
    - 說明: 指向閘道器所屬的設施 ID。
  - **`firmware_version`**
    - 型態: **TEXT**
    - 說明: 閘道器所執行的韌體版本。
  - **`fs_version`**
    - 型態: **TEXT**
    - 說明: 閘道器的檔案系統版本。
  - **`created_by`**
    - 型態: **INTEGER** (外鍵，參考 `users.id`)
    - 說明: 建立此紀錄的使用者。
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 建立時間。
  - **`updated_by`**
    - 型態: **INTEGER** (外鍵，參考 `users.id`)
    - 說明: 最後更新此紀錄的使用者。
  - **`updated_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 最後更新時間。

#### **2️⃣ `gateway_update_tasks`（閘道器更新任務）**

```sql
CREATE TABLE gateway_update_tasks (
    id SERIAL PRIMARY KEY,
    gateway_id INTEGER NOT NULL REFERENCES gateways(id),
    update_type TEXT CHECK (update_type IN ('FIRMWARE', 'FILESYSTEM')),
    target_version TEXT,
    file_url TEXT,
    schedule_at TIMESTAMPTZ,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**`gateway_update_tasks` 表**

- **用途:** 用於排程並記錄對閘道器進行更新的任務，如韌體更新或檔案系統更新

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER** (主鍵，自動遞增)
    - 說明: 更新任務的唯一識別碼。
  - **`gateway_id`**
    - 型態: **INTEGER** (外鍵，參考 `gateways.id`，NOT NULL)
    - 說明: 指向需更新之閘道器的 ID。
  - **`update_type`**
    - 型態: **TEXT** (符合 `('FIRMWARE', 'FILESYSTEM')`)
    - 說明: 更新類型。
      - **FIRMWARE**: 韌體更新
      - **FILESYSTEM**: 檔案系統更新
  - **`target_version`**
    - 型態: **TEXT**
    - 說明: 目標版本號。
  - **`file_url`**
    - 型態: **TEXT**
    - 說明: 提供更新檔案的下載連結。
  - **`schedule_at`**
    - 型態: **TIMESTAMPTZ**
    - 說明: 預定執行更新的時間，若為 NULL，則表示立即執行或手動觸發。
  - **`created_by`**
    - 型態: **INTEGER** (外鍵，參考 `users.id`)
    - 說明: 建立該更新任務的使用者。
  - **`created_at`**
    - 型態: **TIMESTAMPTZ** (預設 `NOW()`)
    - 說明: 建立時間。

#### **3️⃣ `gateway_equipment_connections`（設備與閘道器的關聯）**

```sql
CREATE TABLE gateway_equipment_connections (
    id SERIAL PRIMARY KEY,
    gateway_id INTEGER NOT NULL REFERENCES gateways(id),
    equipment_id INTEGER NOT NULL REFERENCES equipment(id),
    protocol TEXT CHECK (protocol IN ('MODBUS_TCP', 'MODBUS_RTU', 'OPC_UA', 'CAN')),
    device_address TEXT NOT NULL,
    polling_interval INTEGER NOT NULL,
    is_active BOOLEAN DEFAULT TRUE
);
```

**`gateway_equipment_connections` 表**

- **用途:** 用於紀錄閘道器（`gateways`）與實際設備（`equipment`）間的通訊協定、位址與輪詢頻率等資訊

- **欄位說明:**
  - **`id`**
    - 型態: **INTEGER** (主鍵，自動遞增)
    - 說明: 該關聯紀錄的唯一識別碼
  - **`gateway_id`**
    - 型態: **INTEGER** (外鍵，參考 `gateways.id`，NOT NULL)
    - 說明: 指向所屬的閘道器 ID
  - **`equipment_id`**
    - 型態: **INTEGER** (外鍵，參考 `equipment.id`，NOT NULL)
    - 說明: 指向所屬的設備 ID
  - **`protocol`**
    - 型態: **TEXT** (符合 `('MODBUS_TCP', 'MODBUS_RTU', 'OPC_UA', 'CAN')`)
    - 說明: 使用的通訊協定，例如 MODBUS 或 OPC_UA 等
  - **`device_address`**
    - 型態: **TEXT** (NOT NULL)
    - 說明: 設備位址資訊，視協定而定（如 MODBUS 位址、OPC_UA Node ID 等）
  - **`polling_interval`**
    - 型態: **INTEGER** (NOT NULL)
    - 說明: 設備輪詢頻率（毫秒、秒或其他單位），定期向設備讀取資料
  - **`is_active`**
    - 型態: **BOOLEAN** (預設 `TRUE`)
    - 說明: 此連線設定是否啟用中

### **通知系統模組 (`notification-service`)**

#### **1️⃣ `notification_rule`（預設通知規則表）**

```sql
CREATE TABLE notification_rule (
    id SERIAL PRIMARY KEY,
    device_model_id INTEGER NOT NULL REFERENCES device_models(id),
    name TEXT NOT NULL,
    rule_type TEXT CHECK (rule_type IN ('THRESHOLD', 'STATUS_CHANGE', 'MAINTENANCE')) NOT NULL,
    conditions JSONB NOT NULL,
    severity TEXT CHECK (severity IN ('CRITICAL', 'WARNING', 'INFO')) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_by INTEGER REFERENCES users(id),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

#### **2️⃣ `notification_rules`（設備通知規則表）**

```sql
CREATE TABLE notification_rules (
    id SERIAL PRIMARY KEY,
    equipment_id INTEGER NOT NULL REFERENCES equipment(id),
    name TEXT NOT NULL,
    conditions JSONB NOT NULL,
    severity TEXT CHECK (severity IN ('CRITICAL', 'WARNING', 'INFO')) NOT NULL,
    notification_methods JSONB NOT NULL, 
    recipients JSONB NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_by INTEGER REFERENCES users(id),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### **監控與數據模組 (`monitoring-service`)**

#### **1️⃣ `gateway_status_logs`（閘道器狀態紀錄表）**

```sql
CREATE TABLE gateway_status_logs (
    id SERIAL PRIMARY KEY,
    gateway_id INTEGER NOT NULL REFERENCES gateways(id),
    status TEXT CHECK (status IN ('ONLINE', 'OFFLINE', 'ERROR', 'MAINTENANCE')) NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL
);
```

#### **2️⃣ `equipment_status_logs`（設備狀態紀錄表）**

```sql
CREATE TABLE equipment_status_logs (
    id SERIAL PRIMARY KEY,
    equipment_id INTEGER NOT NULL REFERENCES equipment(id),
    status TEXT CHECK (status IN ('RUNNING', 'STOPPED', 'ERROR', 'MAINTENANCE', 'STANDBY')) NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL
);
```

#### **3️⃣ `equipment_metrics`（設備即時數據表）**

```sql
CREATE TABLE equipment_metrics (
    id SERIAL PRIMARY KEY,
    gateway_equipment_connection_id INTEGER NOT NULL REFERENCES gateway_equipment_connections(id),
    metrics_data JSONB NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL
);
```

#### **4️⃣ `notification_logs`（通知發送歷史表）**

```sql
CREATE TABLE notification_logs (
    id SERIAL PRIMARY KEY,
    rule_id INTEGER NOT NULL REFERENCES notification_rules(id),
    equipment_id INTEGER NOT NULL REFERENCES equipment(id),
    triggered_value JSONB NOT NULL,
    message TEXT NOT NULL,
    status TEXT CHECK (status IN ('PENDING', 'SENT', 'FAILED')) NOT NULL,
    sent_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

#### **5️⃣ `gateway_update_logs`（閘道器更新紀錄表）**

```sql
CREATE TABLE gateway_update_logs (
    id SERIAL PRIMARY KEY,
    task_id INTEGER NOT NULL REFERENCES gateway_update_tasks(id),
    status TEXT CHECK (status IN ('COMPLETED', 'FAILED')) NOT NULL,
    error_message TEXT,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

#### **6️⃣ `equipment_control_logs`（設備遠端控制紀錄表）**

```sql
CREATE TABLE equipment_control_logs (
    id SERIAL PRIMARY KEY,
    gateway_equipment_connection_id INTEGER NOT NULL REFERENCES gateway_equipment_connections(id),
    command JSONB NOT NULL,
    result TEXT CHECK (result IN ('SUCCESS', 'FAILED')) NOT NULL,
    error_message TEXT,
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```
