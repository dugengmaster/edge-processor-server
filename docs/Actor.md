# Actor
1. Actor Model 的起源與發展：
- 1973年由 Hewitt、Bishop 和 Steiger 首次提出
- 受到物理學（包括相對論和量子力學）的啟發
- 也受到 Lisp、Simula 等程式語言的影響 🌱

2. Actor Model 的核心概念：
- Actor 是並發計算的基本單位
- 每個 Actor 都有自己的私有狀態
- Actor 之間只能通過消息傳遞進行通信
- 不需要基於鎖的同步機制 🔄

3. Actor Model 的主要特點：
- 響應消息時，Actor 可以：
  * 做出本地決策
  * 創建更多 Actor
  * 發送更多消息
  * 決定如何處理下一個消息
- 完全異步的通信模型
- 天然支持分布式系統 💫

1. process
2. storage
3. communication

1. create more actor
2. send message to actor it know
3. 