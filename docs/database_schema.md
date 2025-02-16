# 資料庫結構說明文件

本文件描述了 `gateway register modbus device` 系統中使用的資料庫表格結構。

## 系統架構總覽

本系統的資料庫設計分為六大核心模組，每個模組負責特定的功能領域：

### 1. 企業與組織管理模組

- **用途：** 管理企業組織架構及其相關資訊
- **核心表格：**
  - enterprises（企業基本資料）
  - enterprise_sites（營運據點）
  - site_facilities（設施分區）
  - enterprise_contacts（聯絡人資訊）
- **主要功能：**
  - 企業資料管理
  - 據點位置管理
  - 設施分區管理
  - 聯絡人管理

### 2. 用戶管理模組

- **用途：** 管理系統使用者及其權限
- **核心表格：**
  - users（帳號基本資訊）
  - user_permissions（功能權限）
  - password_reset_tokens（密碼重置）
  - login_logs（登入記錄）
- **主要功能：**
  - 帳號管理
    - 基本資料維護
    - 密碼重置機制
    - 登入活動追蹤
  - 權限控制
    - 角色基礎權限
    - 功能細部權限
  - 安全機制
    - 登入追蹤
    - 異常偵測

### 3. 設備規格管理模組

- **用途：** 管理設備的基本規格定義
- **核心表格：**
  - brands（品牌資訊）
  - equipment_types（設備類型）
  - model_specs（規格定義）
  - models（設備型號）
- **主要功能：**
  - 品牌管理
  - 設備類型管理
  - 規格定義維護
  - 型號管理

### 4. 設備實例管理模組

- **用途：** 管理實體設備資訊、閘道器版本及其連接狀態，並追蹤維護記錄
- **核心表格：**
  - equipment（實體設備）- 管理設備基本資訊與啟用狀態
  - maintenance_records（維護記錄）- 追蹤設備安裝與維護歷史
  - gateways（閘道器）- 記錄閘道器資訊與版本
  - gateway_update_tasks（閘道器更新任務）- 管理韌體與檔案系統更新排程
  - gateway_equipment_connections（設備連接狀態）- 記錄 Gateway 回報的連接狀態
- **主要功能：**
  - 實體設備資訊管理
  - 設備維護歷程追蹤
  - 閘道器版本控制與更新
  - 設備連接狀態監控
- **關鍵特性：**
  - 支援設備啟用/停用管理
  - 完整的維護記錄追蹤
  - 自動同步 Gateway 連接狀態
  - 閘道器版本更新排程

### 5. 通知系統模組

- **用途：** 管理系統警報及通知機制
- **核心表格：**
  - notification_rule_defaults（預設規則）
  - notification_rules（通知規則）
- **主要功能：**
  - 預設規則管理
  - 客製規則配置

### 6. 監控與數據模組

- **用途：** 全方位管理設備與閘道器的運行狀態、數據收集、遠端控制及通知管理
- **核心表格：**
  - equipment_metrics（設備即時數據）- 儲存設備時間序列數據
  - gateway_status_logs（閘道器狀態）- 追蹤閘道器運行狀態
  - equipment_status_logs（設備狀態）- 追蹤設備運行狀態
  - notification_logs（通知記錄）- 管理警報通知發送
  - gateway_update_logs（閘道器更新）- 記錄更新結果
  - equipment_control_logs（設備控制）- 記錄遠端控制操作
- **主要功能：**
  - 即時數據採集與存儲
  - 閘道器與設備狀態監控
  - 設備運行歷史記錄追蹤
  - 設備遠端控制與追蹤
  - 警報通知管理
  - 閘道器更新追蹤
- **關鍵特性：**
  - 採用時間序列儲存格式
  - 支援 JSONB 格式的動態數據與指令
  - 完整的操作結果追蹤
  - 詳細的時間戳記記錄

## 資料表詳細說明

### 企業與組織管理模組

1. `enterprises` 表

- **用途:** 儲存企業基本資訊
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `name` (TEXT) - 企業名稱。
  - `tax_id` (TEXT) - 統一編號。
  - `description` (TEXT) - 企業描述。
  - `is_active` (BOOLEAN) - 企業資訊是否有效。
  - `created_by` (INTEGER, 外鍵, 參考 users 表) - 建立者
  - `created_at`(TIMESTAMP WITH TIME ZONE) - 建立時間。
  - `updated_by` (INTEGER, 外鍵, 參考 users 表) - 最後更新者
  - `updated_at`(TIMESTAMP WITH TIME ZONE) - 最後更新時間。

2. `enterprise_sites` 表

- **用途:** 儲存企業的各個營運據點
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `enterprise_id` (INTEGER, 外鍵) - 關聯的企業 ID。
  - `site_name` (TEXT) - 據點名稱。
  - `site_type` (TEXT) - 據點類型。
  - `location_info` (JSONB) - 位置相關完整資訊，包含：
    - `address`: 完整地址。
    - `coordinate`: 座標相關。
      - lat: 緯度座標。
      - lon: 經度座標。
    - `region`: 行政區域。
      - country: 國家。
      - state: 州/省。
      - city: 城市。
      - district: 區。
      - postal_code: 郵遞區號。
    - `public_contact` (JSONB) - 公共聯絡方式，包含：
      - main_phone: 總機電話。
      - service_phone: 客服電話。
      - emergency_phone: 緊急聯絡電話。
      - fax: 傳真號碼。
      - email: 公共信箱。
      - business_hours: 營業時間設定。
        - regular_hours: 一般營業時間。
          - monday: ["09:00-12:00", "13:00-18:00"]
          - tuesday: ["09:00-12:00", "13:00-18:00"]
          - wednesday: ["09:00-12:00", "13:00-18:00"]
          - thursday: ["09:00-12:00", "13:00-18:00"]
          - friday: ["09:00-12:00", "13:00-18:00"]
          - saturday: ["09:00-12:00"]
          - sunday: []
        - special_dates: 特殊日期設定。
          - holidays: ["2024-01-01", "2024-02-14"]
          - custom_hours: [
            {
            "date": "2024-02-13",
            "hours": ["09:00-12:00"]
            }
            ]
        - emergency_service: 緊急服務時間。
          - available: true
          - hours: ["18:00-09:00"]
  - `is_active` (BOOLEAN) - 是否為現行據點。
  - `created_by` (INTEGER, 外鍵, 參考 users 表) - 建立者
  - `created_at`(TIMESTAMP WITH TIME ZONE) - 建立時間。
  - `updated_by` (INTEGER, 外鍵, 參考 users 表) - 最後更新者
  - `updated_at`(TIMESTAMP WITH TIME ZONE) - 最後更新時間。

3. `site_facilities` 表

- **用途:** 管理據點內的設施分區
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `enterprise_sites_id` (INTEGER, 外鍵, 參考 enterprise_sites 表)
  - `name` (TEXT) - 設施名稱，如：「注塑車間」、「原料倉」
  - `floor` (TEXT) - 樓層
  - `facility_type` (TEXT) - 設施類型（如：生產車間、倉儲、公共設施區等）
  - `description` (TEXT) - 設施描述
  - `is_active` (BOOLEAN) - 此設施是否有效
  - `created_by` (INTEGER, 外鍵, 參考 users 表)
  - `created_at` (TIMESTAMP WITH TIME ZONE)
  - `updated_by` (INTEGER, 外鍵, 參考 users 表)
  - `updated_at` (TIMESTAMP WITH TIME ZONE)

4. `enterprise_contacts` 表

- **用途:** 儲存企業據點的聯絡人資訊。
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `site_id` (INTEGER, 外鍵) - 關聯的據點 ID。
  - `user_id` (INTEGER, 外鍵, 參考 users 表) - 若此聯絡人需要系統存取權限，則關聯到使用者帳號
  - `contact_info` (JSONB) - 聯絡資訊，包含：
    - `basic`: 基本資訊。
      - name: 姓名。
      - title: 職稱。
      - department: 部門。
      - employee_id: 員工編號。
    - `contact`: 聯絡方式。
      - office_phone: 辦公室電話。
      - extension: 分機。
      - fax: 傳真。
      - mobile: 手機。
      - email: 電子郵件。
      - messaging_apps: 通訊軟體的聯絡方式。
        - type: 例如：LINE、WeChat、WhatsApp、Telegram、Skype 等。
        - id: ID 或帳號。
  - `rank_order` (INTEGER) - 職位排序（主要用於高層）。
  - `is_primary` (BOOLEAN) - 是否為主要聯絡人。
  - `is_active` (BOOLEAN) - 是否在職/有效。
  - `created_by` (INTEGER, 外鍵, 參考 users 表) - 建立者
  - `created_at`(TIMESTAMP WITH TIME ZONE) - 建立時間。
  - `updated_by` (INTEGER, 外鍵, 參考 users 表) - 最後更新者
  - `updated_at`(TIMESTAMP WITH TIME ZONE) - 最後更新時間。

### 用戶管理模組

1. `users` 表

- **用途:** 儲存使用者帳號資訊
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `email` (TEXT, UNIQUE) - 登入帳號（使用 email）
  - `password_hash` (TEXT) - 加密後的密碼
  - `role` (ENUM) - 使用者角色，如：
    - SUPER_ADMIN - 系統最高管理者
    - ADMIN - 管理者
    - SITE_ADMIN - 據點管理者
    - SITE_USER - 據點一般使用者
  - `site_id` (INTEGER, 外鍵, 參考 enterprise_sites 表) - 關聯的據點
  - `last_login` (TIMESTAMP WITH TIME ZONE) - 最後登入時間
  - `is_active` (BOOLEAN) - 帳號是否有效
  - `created_at` (TIMESTAMP WITH TIME ZONE) - 建立時間
  - `updated_at` (TIMESTAMP WITH TIME ZONE) - 更新時間

2. `user_permissions` 表

- **用途:** 定義使用者的細部權限
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `user_id` (INTEGER, 外鍵, 參考 users 表)
  - `permission_type` (ENUM) - 權限類型，如：
    - VIEW_DASHBOARD - 查看儀表板
    - EXPORT_DATA - 匯出數據
    - MANAGE_EQUIPMENT - 管理設備
    - MANAGE_USERS - 管理使用者
    - MANAGE_ALERTS - 管理警報設定
  - `created_at` (TIMESTAMP WITH TIME ZONE) - 建立時間

3. `password_reset_tokens` 表

- **用途：** 管理密碼重置請求
- **欄位：**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `user_id` (INTEGER, 外鍵, 參考 users 表)
  - `token` (TEXT) - 重置 token
  - `expired_at` (TIMESTAMP WITH TIME ZONE) - token 過期時間
  - `is_used` (BOOLEAN) - 是否已使用
  - `created_at` (TIMESTAMP WITH TIME ZONE)

4. `login_logs` 表

- **用途：** 記錄使用者登入活動
- **欄位：**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `user_id` (INTEGER, 外鍵, 參考 users 表)
  - `login_at` (TIMESTAMP WITH TIME ZONE) - 登入時間
  - `ip_address` (TEXT) - 來源 IP
  - `user_agent` (TEXT) - 瀏覽器資訊
  - `status` (TEXT) - 登入狀態：'SUCCESS', 'FAILED'
  - `fail_reason` (TEXT, nullable) - 失敗原因

### 設備規格管理模組

1. `brands` 表

- **用途:** 儲存品牌資訊。
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `name` (TEXT) - 品牌名稱，如：JAGUAR、HITACHI、FUSHENG 等。

2. `equipment_types` 表

- **用途:** 儲存設備類型資訊。
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `name` (TEXT, 設備類型名稱) - 如：空壓機、乾燥機、流量計等。

3. `model_specs` 表

- **用途:** 儲存設備型號的完整規格定義。
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `static_specs` (JSONB, 儲存靜態規格定義) - 儲存不會改變的規格，如馬力、冷卻方式等固定參數。
  - `dynamic_specs` (JSONB, 儲存動態規格定義) - 儲存可變動的規格，如排氣壓力、電流、啟動 / 停止等。
  - `is_active` (BOOLEAN) - 表示此規格定義是否仍在使用中。
  - `effective_date` (TIMESTAMP WITH TIME ZONE) - 規格定義生效日期。

4. `models` 表

- **用途:** 儲存設備型號資訊。
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `name` (TEXT) - 型號名稱，如：AC-2023X。
  - `description` (TEXT) - 該型號描述說明。
  - `brand_id` (INTEGER, 外鍵, 參考 `brands` 表) - 關聯到品牌資訊。
  - `equipment_type_id` (INTEGER, 外鍵, 參考 `equipment_types` 表) - 關聯到設備類型。
  - `specs_id` (INTEGER, 外鍵, 參考 `model_specs` 表) - 關聯到當前使用的規格定義

注意：當規格有更新時，只需要更新 specs_id 指向新的 model_specs 記錄，不需要建立新的 models 記錄 🔄。

### 設備實例管理模組

1. `equipment` 表

- **用途:** 儲存實體設備資訊。
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `facility_id` (INTEGER, 外鍵, 參考 site_facilities 表) - 關聯到客戶資訊
  - `model_id` (INTEGER, 外鍵, 參考 `models` 表) - 關聯到設備的型號資訊。
  - `name`(TEXT) - 設備名稱，可自定義，如：「一樓空壓機」。
  - `serial_number` (TEXT) - 設備序號。
  - `purchase_date`(DATE) - 設備購買日期。
  - `is_active` (BOOLEAN) - 設備是否在運作中（用於設備報廢、移除或暫停監控時，可保留歷史數據）。
  - `created_by` (INTEGER, 外鍵, 參考 users 表) - 建立者
  - `created_at`(TIMESTAMP WITH TIME ZONE) - 建立時間。
  - `updated_by` (INTEGER, 外鍵, 參考 users 表) - 最後更新者
  - `updated_at`(TIMESTAMP WITH TIME ZONE) - 最後更新時間。

2. `maintenance_records` 表

- **用途:** 追蹤設備的維護歷史記錄。
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增) - 維護記錄唯一識別碼。
  - `equipment_id` (INTEGER, 外鍵, 參考 `equipment` 表) - 關聯到被維護的設備。
  - `installation_date` (DATE) - 設備安裝日期。
  - `maintenance_date` (DATE) - 維護日期。
  - `maintenance_type` (TEXT) - 維護類型（如：定期保養、緊急維修。
  - `description` (TEXT) - 維護內容描述。
  - `maintained_by` (TEXT) - 維護人員。
  - `created_at` (TIMESTAMP WITH TIME ZONE) - 記錄建立時間。

3. `gateways` 表

- **用途:** 儲存閘道器資訊。
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `macid` (TEXT)
  - `facility_id` (INTEGER, 外鍵, 參考 site_facilities 表) - 所在設施。
  - `firmware_version`(TEXT) - 韌體版本。
  - `FS_version`(TEXT) - 檔案系統版本。
  - `created_by` (INTEGER, 外鍵, 參考 users 表) - 建立者
  - `created_at`(TIMESTAMP WITH TIME ZONE) - 建立時間。
  - `updated_by` (INTEGER, 外鍵, 參考 users 表) - 最後更新者
  - `updated_at`(TIMESTAMP WITH TIME ZONE) - 最後更新時間。

4. `gateway_update_tasks` 表

- **用途：** 管理閘道器的更新任務
- **欄位：**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `gateway_id` (INTEGER, 外鍵, 參考 gateways 表)
  - `update_type` (ENUM) - 更新類型：
    - FIRMWARE - 韌體更新。
    - FILESYSTEM - 檔案系統更新。
  - `target_version` (TEXT) - 目標版本。
  - `file_url` (TEXT) - 更新檔案的下載路徑。
  - `schedule_at` (TIMESTAMP WITH TIME ZONE) - 預定更新時間。
  - `created_by` (INTEGER, 外鍵, 參考 users 表)
  - `created_at` (TIMESTAMP WITH TIME ZONE)

5. `gateway_equipment_connections` 表

- **用途:** 管理閘道器和設備的連接關係。
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `gateway_id` (INTEGER, 外鍵, 參考 gateways 表)
  - `equipment_id` (INTEGER, 外鍵, 參考 equipment 表)
  - `protocol` (ENUM) - 通訊協議：
    - MODBUS_TCP
    - MODBUS_RTU
    - OPC_UA
    - CAN
  - `device_address` (TEXT) - 設備通訊位址（如：Slave ID 或 IP）。
  - `polling_interval` (INTEGER) - 數據採集間隔（秒）。
  - `is_active` (BOOLEAN) - 此連接是否啟用中。
  > **Notice:**
  > 此表格資料由 Gateway 本地設定後自動回報更新，不提供後台手動設定功能。
  > - 連接狀態（`is_active`）反映 Gateway 回報的實際連接狀況
  > - 通訊協議和位址設定需在 Gateway 介面上完成
  > - 即時連線狀態請參考 `gateway_status_logs` 表

### 通知系統模組

1. `notification_rule_defaults` 表

- **用途:** 儲存設備型號的預設通知規則
- **欄位:**
  - `id` (INTEGER, 主鍵)
  - `model_id` (INTEGER, 外鍵, 參考 models 表)
  - `name` (TEXT)
  - `conditions` (JSONB) - 觸發條件，根據 model_specs 的 dynamic_specs 定義，格式如：
    ```json
    {
      "metric": "pressure", // 對應 dynamic_specs 中的參數
      "operator": ">", // 運算符：>, <, =, >=, <=, !=
      "value": 8.5, // 閾值
      "duration": 300 // 持續時間（秒）
    }
    ```
  - `severity` (ENUM) - 嚴重程度：
    - CRITICAL - 緊急
    - WARNING - 警告
    - INFO - 提醒
  - `is_active` (BOOLEAN)
  - `created_by` (INTEGER, 外鍵, 參考 users 表) - 建立者
  - `created_at`(TIMESTAMP WITH TIME ZONE) - 建立時間。
  - `updated_by` (INTEGER, 外鍵, 參考 users 表) - 最後更新者
  - `updated_at`(TIMESTAMP WITH TIME ZONE) - 最後更新時間。

2. `notification_rules` 表

- **用途:** 儲存設備的通知規則設定
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `equipment_id` (INTEGER, 外鍵, 參考 equipment 表)
  - `name` (TEXT) - 規則名稱
  - `rule_type` (ENUM) - 規則類型，如：
    - THRESHOLD - 閾值警報
    - STATUS_CHANGE - 狀態變更
    - MAINTENANCE - 保養提醒
  - `conditions` (JSONB) - 觸發條件，根據 model_specs 的 dynamic_specs 定義，格式如：
    ```json
    {
      "metric": "pressure", // 對應 dynamic_specs 中的參數
      "operator": ">", // 運算符：>, <, =, >=, <=, !=
      "value": 8.5, // 閾值
      "duration": 300 // 持續時間（秒）
    }
    ```
  - `severity` (ENUM) - 嚴重程度：
    - CRITICAL - 緊急
    - WARNING - 警告
    - INFO - 提醒
  - `notification_methods` (JSONB) - 通知方式設定：
    ```json
    {
      "email": true,
      "sms": false,
      "line": true,
      "webhook": false
    }
    ```
  - `recipients` (JSONB) - 通知對象設定，格式如：
    ```json
    {
      "users": [1, 2, 3], // user_id 列表
      "contacts": [5, 8], // enterprise_contacts.id 列表
      "webhook_urls": ["https://..."] // 若有 webhook 則記錄目標 URL
    }
    ```
  - `is_active` (BOOLEAN) - 規則是否啟用
  - `created_by` (INTEGER, 外鍵, 參考 users 表) - 建立者
  - `created_at`(TIMESTAMP WITH TIME ZONE) - 建立時間。
  - `updated_by` (INTEGER, 外鍵, 參考 users 表) - 最後更新者
  - `updated_at`(TIMESTAMP WITH TIME ZONE) - 最後更新時間。

### 監控與數據模組

1. `gateway_status_logs` 表

- **用途:** 追蹤閘道器狀態變化。
- **特性:**
  - 使用時間序列儲存格式。
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `gateway_id` (INTEGER, 外鍵, 參考 `gateways` 表) - 關聯到特定閘道器。
  - `status` (ENUM) - 閘道器狀態，如：
    - ONLINE - 在線。
    - OFFLINE - 離線。
    - ERROR - 錯誤。
    - MAINTENANCE - 維護中。
  - `recorded_at` (TIMESTAMP WITH TIME ZONE) - 狀態記錄時間。

2. `equipment_status_logs` 表

- **用途:** 追蹤設備狀態變化。
- **特性:**
  - 使用時間序列儲存格式。
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `equipment_id` (INTEGER, 外鍵, 參考 `equipment` 表) - 關聯到特定設備。
  - `status` (ENUM) - 設備狀態，如：
    - RUNNING - 運轉中。
    - STOPPED - 停止。
    - ERROR - 異常。
    - MAINTENANCE - 維護中。
    - STANDBY - 待機。
  - `recorded_at` (TIMESTAMP WITH TIME ZONE) - 狀態記錄時間。

3. `equipment_metrics` 表

- **用途:** 儲存從設備採集的即時數據（時間序列資料）
- **特性:**
  - 使用時間序列儲存格式
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `mapping_id` (INTEGER, 外鍵, 參考 gateway_equipment_mappings 表)
  - `metrics_data` (JSONB) - 動態數據內容，對應 model_specs 表中的 dynamic_specs 定義
  - `recorded_at` (TIMESTAMP WITH TIME ZONE) - 數據採集時間戳記

4. `notification_logs` 表

- **用途:** 記錄通知發送歷史
- **欄位:**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `rule_id` (INTEGER, 外鍵, 參考 notification_rules 表)
  - `equipment_id` (INTEGER, 外鍵, 參考 equipment 表)
  - `triggered_value` (JSONB) - 觸發時的數值
  - `message` (TEXT) - 通知訊息內容
  - `status` (ENUM) - 發送狀態：
    - PENDING - 待發送
    - SENT - 已發送
    - FAILED - 發送失敗
  - `sent_at` (TIMESTAMP WITH TIME ZONE) - 發送時間
  - `created_at` (TIMESTAMP WITH TIME ZONE) - 記錄建立時間

5. `gateway_update_logs` 表

- **用途：** 記錄更新過程的詳細狀態變化
- **欄位：**
  - `id` (INTEGER, 主鍵, 自動遞增)
  - `task_id` (INTEGER, 外鍵, 參考 gateway_update_tasks 表)
  - `status` (ENUM) - 當前狀態
    - COMPLETED - 完成
    - FAILED - 失敗
  - `error_message` (TEXT, nullable) - 錯誤訊息
  - `completed_at` (TIMESTAMP WITH TIME ZONE, nullable) - 完成時間
  - `created_at` (TIMESTAMP WITH TIME ZONE)

6. `equipment_control_logs` 表
- **用途:** 記錄設備遠端控制操作
- **欄位:**
  - `id` (INTEGER, 主鍵)
  - `mapping_id` (INTEGER, 外鍵, 參考 gateway_equipment_mappings 表)
  - `command` (JSONB) - 控制指令內容
  - `result` (ENUM) - 執行結果：
    - SUCCESS - 成功
    - FAILED - 失敗
  - `error_message` (TEXT, nullable) - 錯誤訊息
  - `created_by` (INTEGER, 外鍵, 參考 users 表)
  - `created_at` (TIMESTAMP WITH TIME ZONE)

## 未來功能參考 (Future Features)

### 操作日誌系統 (Operation Logging System)

- **優先級：** 中等
- **目的：** 追蹤系統重要操作，提供稽核功能
- **建議實作範圍：**
  1. 設備生命週期管理（新增/刪除/停用）
  2. 閘道器配置變更
  3. 使用者權限異動
  4. 重要參數調整

#### 資料表參考設計

```sql
CREATE TABLE operation_logs (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    action TEXT,           -- 操作類型
    description TEXT,      -- 操作描述
    resource_type TEXT,    -- 資源類型
    resource_id INTEGER,   -- 資源 ID
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```
