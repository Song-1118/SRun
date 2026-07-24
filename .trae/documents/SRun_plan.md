# SRun 项目开发流程计划

> 基于 PROJECT.md、ARCHITECTURE.md、ACCEPTANCE_CRITERIA.md 文档生成

---

## 一、开发阶段总览

| 阶段 | 名称 | 核心产出 | 依赖阶段 |
|------|------|----------|----------|
| **Phase 1** | 项目脚手架搭建 | Tauri + React + Rust 项目骨架 | 无 |
| **Phase 2** | 共享类型定义 | 前后端数据模型（Rust + TS） | Phase 1 |
| **Phase 3** | 数据库层开发 | SQLite 连接池、迁移、仓储接口 | Phase 2 |
| **Phase 4** | 系统检测模块 | 跨平台硬件检测（CPU/GPU/内存/存储/OS） | Phase 2 |
| **Phase 5** | 硬件跑分模块 | 离线跑分数据库、名称归一化、百分比计算 | Phase 2 |
| **Phase 6** | 配置对比模块 | 对比引擎、策略实现、报告生成 | Phase 4 + Phase 5 |
| **Phase 7** | 运行建议模块 | 建议生成器、优化/升级建议 | Phase 6 |
| **Phase 8** | IPC 命令绑定 | Tauri 命令注册、事件分发 | Phase 3-7 |
| **Phase 9** | 前端页面开发 | React 组件、页面、状态管理 | Phase 8 |
| **Phase 10** | 集成测试优化 | E2E 测试、性能优化、Bug 修复 | Phase 1-9 |

---

## 二、Phase 1：项目脚手架搭建

### 1.1 前置条件
- Rust 2024 环境已安装
- Node.js + npm/yarn 已安装
- Tauri CLI 已安装（`cargo install tauri-cli`）

### 1.2 步骤

#### 1.2.1 初始化 Tauri 项目
```
tauri init --app-name SRun --bundle-identifier com.srun.app
```

#### 1.2.2 配置项目结构
```
SRun/
├── src-tauri/                    # Rust 后端
│   ├── Cargo.toml                # Rust 依赖配置
│   ├── src/
│   │   ├── main.rs               # Tauri 入口
│   │   ├── lib.rs                # 库入口（导出命令）
│   │   └── modules/              # 业务模块目录
│   └── tauri.conf.json           # Tauri 配置
├── src/                          # React 前端
│   ├── main.tsx                  # React 入口
│   ├── App.tsx                   # 根组件
│   ├── index.css                 # 全局样式
│   └── components/               # 组件目录
├── index.html                    # HTML 模板
├── package.json                  # 前端依赖配置
└── tsconfig.json                 # TypeScript 配置
```

#### 1.2.3 安装核心依赖

**Rust 依赖**（`src-tauri/Cargo.toml`）：
- `tauri` - Tauri 核心
- `serde` + `serde_json` - 序列化
- `rusqlite` + `r2d2` + `r2d2_rusqlite` - SQLite 数据库
- `tokio` - 异步运行时
- `chrono` - 日期时间
- `regex` - 正则表达式（硬件名称归一化）

**前端依赖**（`package.json`）：
- `react` + `react-dom` + `react-router-dom` - UI 框架
- `typescript` - 类型支持
- `tailwindcss` + `postcss` + `autoprefixer` - 样式
- `@tanstack/react-query` - 状态管理
- `@tauri-apps/api` - Tauri IPC 客户端

#### 1.2.4 配置 Tauri

- 设置窗口标题、尺寸、图标
- 配置 IPC 命令白名单
- 配置 macOS 权限声明

### 1.3 验收标准
- 项目能正常构建（`cargo build` + `npm run build`）
- Tauri 开发服务器能启动（`tauri dev`）
- 基础窗口能正常显示

---

## 三、Phase 2：共享类型定义

### 2.1 步骤

#### 2.1.1 定义 Rust 数据模型（`src-tauri/src/modules/types.rs`）

按 ARCHITECTURE.md 7.1-7.3 节定义：

| 模型 | 说明 | 优先级 |
|------|------|--------|
| `CpuInfo` | CPU 信息 | P0 |
| `GpuInfo` | GPU 信息 | P0 |
| `MemoryInfo` | 内存信息 | P0 |
| `StorageInfo` + `DiskType` | 存储信息 | P0 |
| `OsInfo` | 操作系统信息 | P0 |
| `SystemInfo` | 系统综合信息 | P0 |
| `GameInfo` | 游戏基本信息 | P0 |
| `GameRequirements` + `Requirement` | 游戏配置要求 | P0 |
| `ComparisonResult` + `MatchLevel` | 对比结果 | P0 |
| `ComponentResult` | 组件对比结果（含跑分百分比） | P0 |
| `BenchmarkData` + `BenchmarkType` | 跑分数据 | P0 |
| `Suggestion` + `SuggestionType` + `Priority` | 建议 | P1 |

#### 2.1.2 定义 TypeScript 类型（`src/types/index.ts`）

通过 `tauri` 的 `serde` 自动生成或手动同步：
- 所有 Rust 模型的 TS 等价类型
- API 响应类型（`GameListResponse` 等）
- 组件 Props 类型

#### 2.1.3 实现序列化

- 所有 Rust 结构体实现 `Serialize` + `Deserialize` trait
- 枚举类型使用 `#[serde(tag = "type", content = "value")]` 或 `#[serde(rename_all = "snake_case")]`

### 2.2 验收标准
- 所有数据模型编译通过
- TypeScript 类型与 Rust 类型保持一致
- 基础 JSON 序列化/反序列化测试通过

---

## 四、Phase 3：数据库层开发

### 4.1 步骤

#### 4.1.1 数据库连接管理（`src-tauri/src/modules/database/connection.rs`）
- 使用 `r2d2` 创建 SQLite 连接池
- 配置连接池大小（最小 1，最大 5）
- 实现连接获取/释放逻辑

#### 4.1.2 数据库迁移（`src-tauri/src/modules/database/migrations.rs`）
- 设计初始 schema：
  - `games` 表（游戏基本信息）
  - `game_requirements` 表（配置要求）
  - `detection_history` 表（检测历史）
- 使用 `rusqlite_migration` 管理迁移版本

#### 4.1.3 仓储接口定义（`src-tauri/src/modules/database/repository.rs`）
- `GameRepository` trait：获取游戏列表、搜索、获取详情
- `GameRequirementsRepository` trait：获取配置要求
- `HistoryRepository` trait：保存/获取/删除历史记录

#### 4.1.4 仓储实现（`game_config.rs` + `history.rs`）
- 实现游戏配置的 CRUD 操作
- 实现历史记录的 CRUD 操作
- 添加索引优化搜索性能

#### 4.1.5 数据初始化（`src-tauri/src/modules/database/seeder.rs`）
- 预置 ≥ 100 款主流游戏数据（验收标准 F-08）
- 包含最低配置和推荐配置要求
- 首次启动时自动初始化数据库

### 4.2 验收标准
- 数据库连接池正常工作
- 迁移脚本正确执行
- 游戏数据正确初始化（≥ 100 款）
- 仓储接口测试通过

---

## 五、Phase 4：系统检测模块

### 5.1 步骤

#### 5.1.1 定义检测 Trait（`src-tauri/src/modules/detection/traits.rs`）

```rust
pub trait HardwareDetector {
    fn detect_cpu(&self) -> Result<CpuInfo, DetectionError>;
    fn detect_gpu(&self) -> Result<Vec<GpuInfo>, DetectionError>;
    fn detect_memory(&self) -> Result<MemoryInfo, DetectionError>;
    fn detect_storage(&self) -> Result<Vec<StorageInfo>, DetectionError>;
    fn detect_os(&self) -> Result<OsInfo, DetectionError>;
    async fn detect_all(&self) -> Result<SystemInfo, DetectionError>;
}
```

#### 5.1.2 实现平台检测（优先实现 Windows）

**Windows 平台**（`src-tauri/src/modules/detection/platform/windows.rs`）：
- CPU：WMI `Win32_Processor`
- GPU：DXGI + WMI `Win32_VideoController` fallback
- 内存：`GlobalMemoryStatusEx` API
- 存储：WMI `Win32_LogicalDisk`
- OS：`GetVersionExW` / WMI `Win32_OperatingSystem`

**macOS 平台**（`src-tauri/src/modules/detection/platform/macos.rs`）：
- CPU：`sysctl` / `mach` API
- GPU：`system_profiler` SPDisplaysDataType
- 内存：`sysctl hw.memsize`
- 存储：`diskutil` / `IOKit`
- OS：`sw_vers` / `NSProcessInfo`

**Linux 平台**（`src-tauri/src/modules/detection/platform/linux.rs`）：
- CPU：读取 `/proc/cpuinfo`
- GPU：`lshw` / `/sys/class/drm/`
- 内存：读取 `/proc/meminfo`
- 存储：`lsblk` / `/sys/block/`
- OS：读取 `/etc/os-release`

#### 5.1.3 平台选择逻辑（`src-tauri/src/modules/detection/mod.rs`）

使用 `#[cfg(target_os = "...")]` 条件编译选择平台实现。

#### 5.1.4 异步检测与缓存

- 使用 `tokio::spawn` 并行检测各硬件
- 实现检测结果缓存（有效期 1 小时）
- 通过 `detection_progress` 事件反馈进度

#### 5.1.5 错误处理

- 单项目检测失败不中断整体流程（验收标准 F-07）
- 失败项标记为"未知"，其余正常展示

### 5.2 验收标准（对应 ACCEPTANCE_CRITERIA）
| 验收项 | 说明 |
|--------|------|
| F-01 | CPU 检测正确获取型号、核心数、线程数、频率 |
| F-02 | GPU 检测正确获取型号、厂商、显存、驱动版本 |
| F-03 | 内存检测正确获取总容量、可用容量 |
| F-04 | 磁盘检测正确获取类型、容量、剩余空间 |
| F-05 | OS 检测正确获取平台、版本、架构 |
| F-06 | 检测进度实时反馈，界面无响应不超过 500ms |
| F-07 | 单项检测失败不中断整体流程 |
| P-03 | 全量检测耗时 ≤ 5 秒 |

---

## 六、Phase 5：硬件跑分模块（关键依赖）

### 6.1 步骤

#### 6.1.1 跑分数据库设计（`src-tauri/src/modules/benchmark/database.rs`）

- 内嵌 JSON/SQLite 存储主流 CPU/GPU 基准分数
- CPU 数据源：PassMark、Geekbench 5/6
- GPU 数据源：3DMark Time Spy、Fire Strike

**数据库结构**：
```
benchmark_data/
├── cpu/
│   ├── intel.json      # Intel CPU 跑分
│   └── amd.json        # AMD CPU 跑分
└── gpu/
    ├── nvidia.json     # NVIDIA GPU 跑分
    ├── amd.json        # AMD GPU 跑分
    └── intel.json      # Intel iGPU 跑分
```

#### 6.1.2 名称归一化器（`src-tauri/src/modules/benchmark/normalizer.rs`）

实现名称提取逻辑：
1. 正则表达式提取型号关键字（如 "i7-12700K"）
2. 品牌识别（Intel/AMD/NVIDIA）
3. 别名匹配（如 "RTX 4070" → "GeForce RTX 4070"）

#### 6.1.3 百分比计算引擎（`src-tauri/src/modules/benchmark/calculator.rs`）

实现计算逻辑：
```
CPU/GPU 超出百分比 = (用户跑分 - 要求跑分) / 要求跑分 * 100%
内存/存储超出百分比 = (用户容量 - 要求容量) / 要求容量 * 100%
```

- 正数表示高于要求，负数表示低于要求
- 返回 `BenchmarkData` 结构

#### 6.1.4 模块入口（`src-tauri/src/modules/benchmark/mod.rs`）

导出统一接口：
- `get_cpu_benchmark(cpu_name: &str) -> Option<BenchmarkData>`
- `get_gpu_benchmark(gpu_name: &str) -> Option<BenchmarkData>`
- `search_benchmark(query: &str, r#type: BenchmarkType) -> Vec<BenchmarkData>`

### 6.2 验收标准
- CPU/GPU 名称匹配准确率 ≥ 90%
- 百分比计算逻辑正确
- 支持离线查询（无网络依赖）

---

## 七、Phase 6：配置对比模块

### 7.1 步骤

#### 7.1.1 对比策略定义（`src-tauri/src/modules/comparison/strategies/`）

**CPU 对比策略**（`cpu.rs`）：
- 核心数对比
- 频率对比
- 基准跑分对比（使用 Phase 5 的数据）

**GPU 对比策略**（`gpu.rs`）：
- 显存大小对比
- 基准跑分对比（使用 Phase 5 的数据）

**内存对比策略**（`memory.rs`）：
- 容量对比（MB）

**存储对比策略**（`storage.rs`）：
- 容量对比（GB）
- 磁盘类型对比（SSD/HDD）

#### 7.1.2 对比引擎（`src-tauri/src/modules/comparison/engine.rs`）

实现核心对比逻辑：
1. 获取用户硬件配置（来自检测缓存）
2. 获取游戏配置要求（来自数据库）
3. 调用各策略逐一对比
4. 计算匹配度分数（0-100）

**匹配度等级**（对应 ACCEPTANCE_CRITERIA F-13）：
- 完美运行（80-100分）：所有配置项超过推荐要求
- 流畅运行（60-79分）：满足推荐配置，部分可优化
- 基本运行（40-59分）：满足最低配置，可能需降低画质
- 无法运行（0-39分）：不满足最低配置要求

#### 7.1.3 报告生成（`src-tauri/src/modules/comparison/report.rs`）

生成结构化报告，包含：
- 各组件的详细对比结果
- 高于/低于配置要求的百分比
- 匹配度等级和分数

### 7.2 验收标准（对应 ACCEPTANCE_CRITERIA）
| 验收项 | 说明 |
|--------|------|
| F-11 | 最低配置比对，输出达标/不达标/未知 |
| F-12 | 推荐配置比对，输出达标/不达标/未知 |
| F-13 | 综合判定为四档：excellent/good/low/failed |
| F-14 | 判定可视化（颜色区分） |
| P-04 | 单款游戏评估耗时 < 100ms |

---

## 八、Phase 7：运行建议模块

### 8.1 步骤

#### 8.1.1 建议生成器（`src-tauri/src/modules/suggestion/generator.rs`）

根据对比结果生成建议：
- 针对不达标的硬件项生成升级建议
- 针对可优化项生成游戏设置建议
- 针对系统层面生成优化建议

#### 8.1.2 游戏设置优化器（`src-tauri/src/modules/suggestion/optimizer.rs`）

根据硬件配置推荐：
- 画质设置（高/中/低）
- 分辨率建议
- 帧率限制建议

#### 8.1.3 硬件升级建议器（`src-tauri/src/modules/suggestion/upgrader.rs`）

针对不满足要求的配置项：
- 推荐具体的硬件型号
- 给出价格范围参考
- 按优先级排序建议

### 8.2 验收标准（对应 ACCEPTANCE_CRITERIA）
| 验收项 | 说明 |
|--------|------|
| F-16 | 每项不达标硬件给出至少 1 条具体建议 |
| F-17 | 建议按优先级排序（高/中/低） |

---

## 九、Phase 8：IPC 命令绑定

### 9.1 步骤

#### 9.1.1 注册硬件检测命令

| 命令名 | 实现位置 |
|--------|----------|
| `detect_hardware` | detection 模块 |
| `detect_cpu` | detection 模块 |
| `detect_gpu` | detection 模块 |
| `detect_memory` | detection 模块 |
| `detect_storage` | detection 模块 |
| `detect_os` | detection 模块 |

#### 9.1.2 注册游戏数据命令

| 命令名 | 实现位置 |
|--------|----------|
| `get_games` | database 模块 |
| `get_game` | database 模块 |
| `get_game_requirements` | database 模块 |

#### 9.1.3 注册对比命令

| 命令名 | 实现位置 |
|--------|----------|
| `compare_config` | comparison 模块（含跑分百分比） |

#### 9.1.4 注册跑分命令

| 命令名 | 实现位置 |
|--------|----------|
| `get_cpu_benchmark` | benchmark 模块 |
| `get_gpu_benchmark` | benchmark 模块 |
| `search_benchmark` | benchmark 模块 |

#### 9.1.5 注册历史记录命令

| 命令名 | 实现位置 |
|--------|----------|
| `get_history` | database 模块 |
| `save_history` | database 模块 |
| `delete_history` | database 模块 |

#### 9.1.6 注册设置命令

| 命令名 | 实现位置 |
|--------|----------|
| `get_settings` | settings 模块 |
| `save_settings` | settings 模块 |

#### 9.1.7 注册事件

| 事件名 | 触发时机 |
|--------|----------|
| `detection_progress` | 硬件检测进度更新 |
| `data_updated` | 游戏配置数据更新完成 |
| `app_update_available` | 检测到应用更新 |

#### 9.1.8 前端 IPC 封装（`src/services/tauri.ts`）

封装所有命令调用：
- 统一错误处理
- 请求/响应类型定义
- 进度事件监听

### 9.2 验收标准
- 所有命令能正常调用
- 数据序列化/反序列化正确
- 错误处理机制完善

---

## 十、Phase 9：前端页面开发

### 10.1 步骤

#### 10.1.1 布局组件（`src/components/Layout/`）
- `Header.tsx` - 顶部导航栏
- `Sidebar.tsx` - 侧边栏菜单

#### 10.1.2 卡片组件（`src/components/Card/`）
- `HardwareCard.tsx` - 硬件信息展示卡片
- `GameCard.tsx` - 游戏卡片
- `ResultCard.tsx` - 对比结果卡片

#### 10.1.3 页面组件（`src/pages/`）

**首页**（`Home.tsx`）：
- 欢迎信息
- 快速检测入口
- 热门游戏推荐

**硬件检测页面**（`Detection.tsx`）：
- 硬件信息展示（CPU/GPU/内存/存储/OS）
- 重新检测按钮
- 检测进度展示

**游戏搜索页面**（`GameSearch.tsx`）：
- 搜索框（支持模糊搜索）
- 分类筛选（AAA/Indie/FPS/RPG等）
- 游戏列表展示（虚拟滚动）

**游戏详情页面**（`GameDetail.tsx`）：
- 游戏信息展示
- 最低/推荐配置要求展示
- 开始对比按钮

**配置对比页面**（`Comparison.tsx`）：
- 对比结果展示
- 匹配度等级和分数
- 各组件详细对比（含百分比）
- 优化建议列表

**历史记录页面**（`History.tsx`）：
- 检测历史列表（最近 10 次）
- 查看详情/删除操作

**设置页面**（`Settings.tsx`）：
- 主题切换（深色/浅色）
- 语言切换（中文/英文）
- 数据更新设置

#### 10.1.4 自定义 Hooks（`src/hooks/`）
- `useHardware.ts` - 硬件检测 Hook
- `useGame.ts` - 游戏数据 Hook
- `useComparison.ts` - 配置对比 Hook

#### 10.1.5 状态管理
- React Context：全局状态（用户偏好、系统配置缓存、主题）
- React Query：服务端状态（游戏列表、配置要求、检测历史、对比结果）

#### 10.1.6 路由配置

```
/                 → Home 首页
/detection        → 硬件检测页面
/games            → 游戏搜索页面
/games/:id        → 游戏详情页面
/comparison/:id   → 配置对比页面
/history          → 检测历史页面
/settings         → 设置页面
```

### 10.2 验收标准（对应 ACCEPTANCE_CRITERIA）
| 验收项 | 说明 |
|--------|------|
| F-09 | 游戏搜索：输入 2 字符开始匹配，响应 < 200ms |
| F-10 | 游戏分类筛选功能 |
| F-15 | 批量检测支持（可选） |
| U-01 | 现代化简约风格，布局清晰 |
| U-02 | 深色/浅色主题切换 |
| U-03 | 国际化支持（中文/英文） |
| U-05 | 错误提示友好 |
| U-07 | 超过 500ms 异步操作展示 loading |
| P-12 | 100 款游戏滚动无卡顿（≥ 60fps） |

---

## 十一、Phase 10：集成测试优化

### 11.1 步骤

#### 11.1.1 单元测试

**Rust 核心逻辑测试**（`src-tauri/tests/`）：
- 硬件检测模块测试
- 跑分模块测试
- 对比引擎测试
- 数据库操作测试

**前端组件测试**（`src/__tests__/`）：
- 核心组件逻辑测试
- Hook 测试

#### 11.1.2 集成测试

- 硬件检测 → 数据库写入 → 兼容性评估 → 报告导出完整链路测试

#### 11.1.3 E2E 测试

关键路径测试：
- 启动应用 → 检测硬件 → 选择游戏 → 查看结果 → 导出报告

#### 11.1.4 性能优化

**后端优化**：
- 延迟初始化非关键模块
- 检测结果缓存（有效期 1 小时）
- SQLite 连接池复用

**前端优化**：
- 路由级别代码分割（React lazy）
- 游戏列表虚拟滚动
- 图片懒加载（WebP 格式）
- 搜索输入防抖（300ms）

#### 11.1.5 跨平台测试

**Windows**（C-01 ~ C-06）：
- Windows 10 1909+ / Windows 11
- 高 DPI 适配（125%/150%/175%/200%）
- GPU 检测（NVIDIA/AMD/Intel）

**macOS**（C-07 ~ C-12）：
- macOS 12+（Intel + Apple Silicon）
- 代码签名验证
- GPU 检测（AMD/NVIDIA/Apple Silicon）

**Linux**（C-13 ~ C-18）：
- Ubuntu 22.04+ / Fedora 38+ / Arch
- .deb 和 .AppImage 格式
- X11 和 Wayland 支持

#### 11.1.6 安全合规验证

| 验证项 | 说明 |
|--------|------|
| S-01 | 离线运行所有功能正常 |
| S-02 | 网络监控确认无外发请求 |
| S-03 | 无埋点、追踪、匿名统计代码 |
| S-04 | 数据库文件存储在标准目录 |
| S-06 | 权限请求最小化 |
| S-08 | 依赖漏洞扫描（`cargo audit` + `npm audit`） |

#### 11.1.7 打包发布

- 构建 Windows MSI 安装包
- 构建 macOS DMG 安装包
- 构建 Linux DEB 和 AppImage 安装包
- 生成 SHA-256 校验文件

### 11.2 验收标准（对应 ACCEPTANCE_CRITERIA）
| 验收项 | 说明 |
|--------|------|
| T-01 | Rust 单元测试覆盖率 ≥ 85% |
| T-02 | 前端测试覆盖率 ≥ 75% |
| T-03 | 集成测试完整链路通过 |
| T-04 | E2E 关键路径无报错 |
| T-05 | 跨平台 CI 构建通过 |
| T-06 | 异常场景程序不崩溃 |
| P-01 | 冷启动时间 ≤ 3 秒 |
| P-02 | 热启动时间 ≤ 1 秒 |
| P-07 | 空闲内存占用 ≤ 100MB |
| P-11 | 安装包体积达标 |

---

## 十二、发布前验收清单

### 12.1 功能完整性
- [ ] 所有 P0 功能验收项通过
- [ ] 至少 80% 的 P1 功能验收项通过
- [ ] 游戏数据库包含 ≥ 100 款游戏

### 12.2 质量门槛
- [ ] 所有 P0 性能验收项达标
- [ ] 三个目标平台均通过安装 + 核心功能测试
- [ ] 单元测试覆盖率达标（Rust ≥ 85%, TS ≥ 75%）
- [ ] `cargo clippy` 零警告、`npm run lint` 零错误
- [ ] 无 P0/P1 级别已知 Bug

### 12.3 安全合规
- [ ] 离线运行验证通过
- [ ] 网络监控确认无数据外泄
- [ ] 安装包代码签名有效
- [ ] 依赖漏洞扫描无高危项

### 12.4 交付物
- [ ] 三个平台安装包（Windows .msi / macOS .dmg / Linux .deb + .AppImage）
- [ ] 每个安装包附带 SHA-256 校验文件
- [ ] 用户使用手册（PDF/HTML）
- [ ] CHANGELOG.md 更新至 v1.0
- [ ] GitHub Release 页面完整

---

## 十三、风险与应对

| 风险 | 应对措施 |
|------|----------|
| 不同操作系统硬件信息获取方式不同 | 按平台实现，先完成 Windows，再扩展 macOS/Linux |
| GPU 信息获取需要特殊权限 | 提供清晰的权限说明，首次运行时弹窗提示 |
| 游戏配置数据可能过时 | 提供数据更新机制（检测更新 + 增量更新） |
| 硬件检测耗时较长 | 采用异步检测，进度实时反馈 |
| 硬件名称匹配不准确 | 完善名称归一化逻辑，支持别名和模糊匹配 |
| 跑分数据量较大 | 采用内嵌数据库，按需加载 |

---

## 十四、迭代规划建议

### v1.0（MVP）
- 完成所有 P0 功能
- 优先支持 Windows 平台
- 预置 ≥ 100 款游戏数据
- 基础对比功能（含跑分百分比）

### v1.1
- 完成 macOS 和 Linux 平台支持
- 批量检测功能（F-15）
- 游戏数据导入/导出（F-21）
- 报告导出 PDF（F-22）

### v1.2
- 自动更新功能（U-10）
- 自定义游戏添加（F-20）
- 更完善的国际化支持
- 性能进一步优化