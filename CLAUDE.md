## 全局优先原则

1. 文档与沟通精简无歧义、人工可维护
2. 使用简体中文（全角标点），不用繁体中文、日文、韩文。文件名用英文
3. 给用户的选项带数字编号，第 1 项为推荐或 All
4. 批量改动用脚本或工具完成；脚本用 Bun + TypeScript
5. SOLID：一个类/模块/函数只负责一项功能；针对接口编程；拆分胖接口；用 Interface/Trait 而非继承
6. DRY：同一逻辑只保留一份，抽取为函数/模块
7. YAGNI：只写当前需要的功能；依次优先复用仓内代码、stdlib、平台原生能力、已装依赖，都不行才写能跑通的最小实现
8. 代码文件（含测试）不超过 512 行、函数不超过 64 行，超限即拆分
9. 不写注释或 docstring，删除已有注释
10. 存储与日志时间用 UTC；面向人展示时标注时区（如 2026-07-17 09:20:09 UTC+8）并支持 TZ 环境变量
11. 新配置文件用 yaml；默认值、数值常量、业务字符串常量从配置读取
12. secret 只经环境变量加载、不设默认值，写法 db_password: "${DB_PASSWORD}"
13. 输出 debug 级结构化日志（request_id、feature、action、req、resp 等），供 AI 分析
14. 测试先行：先写失败的测试再实现；覆盖率先保证 branch 100% 再保证 region 100%
15. 测试与业务代码分文件
16. 不自行新建 git branch 或 worktree
17. 多轮改动合并为一个 git commit；commit message 单行、少于 64 个 UTF-8 字符、不含 Co-Authored-By 或 Claude 签名，格式 [TASK-ID ]type: 业务描述，type ∈ build/chore/ci/docs/feat/fix/perf/refactor/revert/style/test，TASK-ID 由用户提供或取自最近 git log，没有则留空
18. 可复用的规则、原则、坑写进对应领域的 CLAUDE.md（与既有规则去重合并，单个 CLAUDE.md 少于 256 行），不记入 TODO.md 或 MEMORY.md；一次性规则丢弃
19. TODO.md 是任务清单，任务完成即删除
20. Plan 用 subagent 从至少 3 个角度（含 1 个批判角度）推敲
21. Plan 定稿后把 ~/.claude/plans/<name>.md 移到当前项目目录，无需确认（不受第 22 条限制）
22. 新建文档（.md .rst .adoc .txt 等）先征得用户同意

## 文档分工

- CLAUDE.md 给 AI：规则、约定、坑。跨目录或影响非 src 文件的写根文件，仅代码适用的写 src/CLAUDE.md（子目录 CLAUDE.md 仅在读取该目录文件时加载）
- docs/ 给人：业务需求，不写技术细节
- README.md 给用户：仅使用方法
- 改动影响需求或用法时，同步更新 docs/ 或 README.md
- 一次性内容（本机状态、排障过程、已完成的计划）不写入任何文档

## 命令

- 测试：`cargo test`
- 运行：`cargo run`；看 debug 日志：`MACPROBE_LOG=debug cargo run`
- 覆盖率门禁：`bun scripts/coverage.ts`，branch 与 region 均须 100%
- 发布：`cargo +stable build --release`

## 目录地图

- `src/`：业务代码与单元测试，规则见 src/CLAUDE.md
- `config/default.yaml`：全部常量、文案、路径、errno、退出码；经 `include_str!` 编译期嵌入，修改后需重新编译；增删字段须同步 `src/config.rs`
- `tests/cli_test.rs`：真实二进制的集成测试；只断言 feature 序列、debug action 序列、日志级别、退出码范围，不断言授权结果
- `scripts/coverage.ts`：覆盖率门禁；排除 `_test.rs` 与 `tests/`，先校验 branch 再校验 region；cargo-llvm-cov 没有 branch 阈值参数，故由脚本解析 JSON 判定
- `rust-toolchain.toml`：nightly + llvm-tools

## 环境坑

- nightly 仅为 llvm-cov 的 branch 覆盖率；代码不得使用 nightly 特性，保证 stable 发布可编译
- 用户级 hook 拒绝 Bash 命令文本中“点号紧跟 env”的写法；含该字样的检索与写入用 Read/Grep/Edit 工具；配置字段用 `_env` 后缀
- 系统命令禁止用 `--help` 试探，只调用明确的只读子命令：`DevToolsSecurity` 不带参数会直接启用开发者模式；`spctl developer-mode enable-terminal --help` 会直接执行
