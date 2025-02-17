# **設備管理模組 (`asset-service`) - 業務層說明文件**

## **1️⃣ 服務概述**
`asset-service` 負責管理設備的靜態規格 (`device`)，以及具體設備實例 (`equipment`)。主要提供 API 來支援設備的註冊、管理、維護記錄等功能。

### **核心業務功能**
1. **品牌 (`device_brands`)**：管理設備的品牌資訊。
2. **設備類型 (`device_types`)**：定義不同類型的設備，如空壓機、水泵等。
3. **設備規格 (`device_specs`)**：儲存設備的靜態與動態規格資訊。
4. **設備型號 (`device_models`)**：關聯品牌、設備類型與規格，形成完整的設備型號。
5. **設備 (`equipment`)**：管理設備的實體實例，包含序號與安裝位置。
6. **設備維護 (`equipment_maintenance`)**：記錄設備的維護與保養歷史。

---

## **2️⃣ 業務邏輯流程**

### **(1) 設備型號 (`device_models`) 建立流程**
1. **建立品牌 (`device_brands`)**（若該品牌不存在）。
2. **建立設備類型 (`device_types`)**（若該類型不存在）。
3. **建立設備規格 (`device_specs`)**，包含 `static_specs`（靜態規格）與 `dynamic_specs`（動態規格）。
4. **綁定 `device_specs` 至 `device_models`**，並指定 `brand_id` 與 `device_type_id`。

### **(2) 設備 (`equipment`) 註冊流程**
1. **選擇設備型號 (`device_models`)**，確保該型號已經存在。
2. **綁定設備至特定場域 (`site_facilities`)**，確保設備被指派至有效的安裝地點。
3. **設定設備的 `serial_number`（唯一序號）**，確保設備的唯一性。
4. **設備 `is_active` 設為 `TRUE`，表示該設備正在使用。**

### **(3) 設備維護 (`equipment_maintenance`) 記錄流程**
1. **查詢設備 (`equipment`)**，確保設備存在。
2. **建立維護紀錄 (`equipment_maintenance`)**，記錄維護時間、維護內容與執行者。
3. **設備可能根據維護結果變更狀態**（如 `is_active = FALSE` 代表停用）。

---

## **3️⃣ API 設計**

### **(1) 設備管理 API**
| 方法  | 路徑                         | 描述               |
|------|-----------------------------|------------------|
| GET  | `/device-brands`            | 取得品牌列表       |
| POST | `/device-brands`            | 新增品牌         |
| GET  | `/device-types`             | 取得設備類型列表   |
| POST | `/device-types`             | 新增設備類型      |
| GET  | `/device-specs`             | 取得設備規格列表   |
| POST | `/device-specs`             | 新增設備規格      |
| GET  | `/device-models`            | 取得設備型號列表   |
| POST | `/device-models`            | 新增設備型號      |

### **(2) 設備實例 API**
| 方法  | 路徑                         | 描述               |
|------|-----------------------------|------------------|
| GET  | `/equipment`                 | 取得設備列表       |
| GET  | `/equipment/{id}`            | 取得特定設備資訊   |
| POST | `/equipment`                 | 新增設備         |
| PUT  | `/equipment/{id}`            | 更新設備資訊      |
| DELETE | `/equipment/{id}`         | 軟刪除設備 (`is_active = FALSE`) |

### **(3) 設備維護 API**
| 方法  | 路徑                                  | 描述           |
|------|--------------------------------------|--------------|
| GET  | `/equipment/{id}/maintenance`       | 取得設備維護紀錄 |
| POST | `/equipment/{id}/maintenance`       | 新增維護紀錄   |

---

## **4️⃣ 權限與存取控制**

- **管理員 (`ADMIN`)** 具備完整管理設備、設備型號與維護紀錄的權限。
- **維護人員 (`MAINTAINER`)** 只能新增與查詢設備維護紀錄。
- **一般使用者 (`USER`)** 只能查詢設備資訊，無法變更資料。

---

## **5️⃣ 事件與日誌管理**

- **審計日誌 (`audit-log-service`)** 記錄所有設備的新增、變更、刪除行為。
- **維護記錄 (`equipment_maintenance`)** 追蹤設備的維修與保養活動。

---

這份業務層說明文件確保 `asset-service` 的 API 設計、業務流程與權限管理清晰明確，方便開發與維護 🚀

