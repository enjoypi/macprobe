## 设计约定

- 输出只有 stdout 上的 tracing JSON Lines：每项检测一条 info 结论（`report::outcome`）与一条 debug 细节（`log::debug`）；参数错误一条 error；`--version`/`-V` 一条 info
- 退出码数值取 config `exit` 段，按 `probe::overall` 映射：任一项 unknown 或参数错误为 unknown，否则任一项 denied 为 denied，否则为 granted；版本查询为 ok
- 检测失败（FFI、子进程、IO）归为 unknown，原因写入 detail，不 panic；panic 只用于内嵌配置损坏
- 判定依据优先级：内核实际拦截（如 FDA 文件探测）> 公开 API > 私有 API
- 新增检测项：
  1. 新建 `src/<模块>.rs`：系统调用与纯函数 `classify`/`parse` 分离，实现 `Probe`，debug action 取唯一名
  2. `config/default.yaml` 加一段（至少 `id`、`name`），`src/config.rs` 加结构体与字段
  3. `src/lib.rs` 声明模块，`app::main_entry` 注册；注册顺序即输出顺序
  4. `tests/cli_test.rs` 同步 feature 序列与 debug action 序列

## 测试约定

- 单元测试写在 `src/<模块>_test.rs`，业务文件末尾用 `#[cfg(test)] #[path = "<模块>_test.rs"] mod tests;` 引入；`support_test.rs` 是公共夹具（`cfg()`、`ctx()`、`capture()`）
- 产生 tracing 日志的测试一律经 `capture`（带全局锁），否则并行测试下 callsite interest 缓存竞争导致偶发漏日志
- FFI、子进程的失败分支通过改配置字段注入错误值覆盖（不存在的路径、`/usr/bin/false`、含 NUL 的字符串），不为测试新增 trait，不用 `coverage(off)`
- 依赖宿主权限的分支由纯函数单测覆盖；调用真实系统的测试只断言非 unknown 或与 `classify` 结果一致

## macOS 平台坑

- TCC、FDA、AX 判定的都是负责进程（宿主终端或 IDE），不是本二进制；ssh、launchd 下通常为拒绝；本二进制自身为负责进程时，ad-hoc 签名重编译后 TCC 条目失配
- `DevToolsSecurity -status` 退出码恒为 0，只能整句匹配输出
- `TCCAccessPreflight`（私有 API）对不存在的服务名也返回 1，与拒绝无法区分
- FDA：只读 open 系统 TCC.db，仅 EPERM 为拒绝，其他 errno 为 unknown；须用 `raw_os_error` 区分（`ErrorKind::PermissionDenied` 合并了 EPERM 与 EACCES）；metadata/stat 不受 FDA 拦截；公司设备 EDR 可能对访问 TCC.db 告警
- `AXIsProcessTrusted` 只返回真假（异常也为假）；禁用 `AXIsProcessTrustedWithOptions` 的 prompt 选项（会弹窗改系统状态）
- 公开框架用 `#[link]` 直接链接，私有框架用 dlopen；C 的 `Boolean` 用 `u8` 接收
- `CFStringCreateWithCString` 用 ASCII 转非 ASCII 不会返回 NULL，只有 `kCFStringEncodingInvalidId`（0xFFFFFFFF）会；测试据此覆盖失败分支
