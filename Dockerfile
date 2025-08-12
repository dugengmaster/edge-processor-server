# 使用 Rust 官方映像
FROM rust:1.83

# 設定工作目錄
WORKDIR /app

# 複製所有檔案到容器中
COPY ./ ./

# 建構應用程式
RUN cargo build --release && \
    # 將執行檔移動到 /app 根目錄
    mv /app/target/release/dolomann-edge-server /app/dolomann-edge-server && \
    # 刪除無用的檔案以縮小映像大小
    rm -rf /app/src /app/target

# 設定執行命令
CMD ["./dolomann-edge-server"]
