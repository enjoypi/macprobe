# macprobe

检测当前终端或 IDE 在 macOS 上是否已获得开发者工具、完全磁盘访问、辅助功能与调试授权。各项含义见 [docs/requirements.md](docs/requirements.md)。

## 安装

需要 rustup。在仓库根目录执行：

```sh
cargo +stable build --release
mkdir -p ~/bin && cp target/release/macprobe ~/bin/
```

确保 `~/bin` 在 PATH 中。

## 运行

在要检测的终端或 IDE 中运行：

```sh
macprobe
```

每项输出一行 JSON，例如：

```
{"timestamp":"2026-01-01T00:00:00.000000Z","level":"INFO","message":"辅助功能（按宿主终端或 IDE 判定）：已授予","request_id":"…","user":"…","host":"…","feature":"accessibility","action":"check","status":"granted","resp":"1"}
```

- `message`：可读结论
- `feature`：依次为 `responsible_process`（判定对象）、`developer_tool`、`full_disk_access`、`accessibility`、`debug_authorization`
- `chain`：仅 `responsible_process` 行有，为本进程到 launchd 的启动链，格式 `pid 路径 < pid 路径 …`
- `status`：`granted`、`denied` 或 `unknown`
- `timestamp`：UTC 时间

## 退出码

| 退出码 | 含义 |
|---|---|
| 0 | 全部已授予，或查询版本 |
| 1 | 有未授予，且没有无法判定 |
| 2 | 有无法判定，或参数错误 |

## 调试日志

```sh
MACPROBE_LOG=debug macprobe
```

额外输出每项的原始检测依据（`level` 为 `DEBUG`）。

## 版本

```sh
macprobe --version
macprobe -V
```
