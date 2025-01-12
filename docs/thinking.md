client/factoryA/line1/

## v1

### 流程
`{edge device type}/{macid}/{channel_type}`

#### 1. 訊息接收階段

*   **MQTT 訊息接收：** 從 MQTT Broker 接收訊息。
*   **Topic 與 Payload 解析：** 解析 MQTT 訊息，取得 Topic 和 Payload。

#### 2. 驗證階段

*  **Topic 解析:** 從 Topic 字串中解析出 `edge_devices_type`、`macid` 和 `channel_type`。
*   **裝置註冊表驗證 (第一階段)**
    *   **註冊表查詢：** 根據 `macid` 查詢 **裝置註冊表**。
    *   **驗證失敗處理：** 若查詢失敗 (`macid` 不存在)，直接丟棄該訊息，結束流程。
*   **Payload 解析與資訊提取**
    *   **完整 Payload 解析：** 根據 `channel` 解構 Payload 內的所有數據。
    *   **`SlaveID` 與 `DeviceIdentifier` 提取：** 從 Payload 中取得 `SlaveID` 和 `DeviceIdentifier` (字串形式)。
*   **macid 註冊表驗證 (第二階段)**
    *   **註冊表查詢：** 根據 `macid` 查詢 **macid 註冊表**。
    *   **驗證：** 驗證從 Payload 解析出的 `SlaveID` 和 `DeviceIdentifier` 是否符合註冊表中的預期值。
    *   **驗證失敗處理：** 若驗證失敗 (`SlaveID` 或 `DeviceIdentifier` 不符)，直接丟棄該訊息，結束流程。

#### 3. 頻道分配階段

*   **訊息分配：** 將處理好的 message 依據 channel_type 分配到不同的頻道 (queue)。

#### 4. 數據處理階段

*   **DeviceIdentifier 註冊表查詢：** 根據 **字串** 形式的 `DeviceIdentifier` 查詢 **DeviceIdentifier 註冊表**。
*   **處理邏輯選擇：** 決定如何處理 Payload 內的數據。
*   **查詢失敗處理：** 若查詢失敗 (`DeviceIdentifier` 不存在或無處理邏輯)，直接丟棄該訊息，結束流程。
*   **數據處理執行：** 執行對應的數據處理邏輯。

#### 5. 數據儲存階段

*   **數據儲存：** 將處理後的數據儲存起來。

## v2

### Topic 格式
`{edge device type}/{hash code}/{channel_type}` or `{edge device type}/{macid}/{channel}/{hash code}`
