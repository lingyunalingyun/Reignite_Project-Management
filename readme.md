# Project Graveyard 文件参考结构

```text
project-graveyard/
├─ Cargo.toml                    （Rust 项目配置与依赖管理）
├─ Cargo.lock                    （锁定依赖版本，保证两人环境一致）
├─ build.rs                      （编译 Slint 界面文件）
├─ README.md                     （项目说明）
├─ .gitignore                    （Git 忽略规则）
│
├─ ui/                           （Slint GUI 界面文件）
│  ├─ main.slint                 （主窗口与界面入口）
│  ├─ pages/                     （各个独立页面）
│  │  ├─ dashboard.slint         （首页 / 总览页）
│  │  ├─ scan.slint              （项目扫描页面）
│  │  ├─ projects.slint          （项目列表页面）
│  │  ├─ project-detail.slint    （项目详情页面）
│  │  └─ settings.slint          （设置页面）
│  │
│  └─ components/                （可复用 GUI 组件）
│     ├─ sidebar.slint           （侧边导航栏）
│     ├─ project-card.slint      （项目卡片）
│     ├─ status-badge.slint      （项目状态标签）
│     └─ section-title.slint     （通用区域标题）
│
├─ src/                          （Rust 源代码）
│  ├─ main.rs                    （程序入口，启动 Slint 与组装程序）
│  │
│  ├─ app/                       （GUI 与核心逻辑之间的连接层）
│  │  ├─ mod.rs                  （app 模块入口）
│  │  ├─ state.rs                （应用状态管理）
│  │  └─ events.rs               （GUI 与核心之间的事件定义）
│  │
│  ├─ core/                      （项目核心功能）
│  │  ├─ mod.rs                  （core 模块入口）
│  │  ├─ scanner.rs              （递归扫描目录）
│  │  ├─ detector.rs             （识别代码项目及项目类型）
│  │  ├─ analyzer.rs             （分析项目元数据）
│  │  ├─ classifier.rs           （判断 Alive / Sleeping / Dead 等状态）
│  │  └─ ignore.rs               （处理 target、node_modules 等忽略目录）
│  │
│  ├─ model/                     （核心数据结构）
│  │  ├─ mod.rs                  （model 模块入口）
│  │  ├─ project.rs              （Project 项目数据结构）
│  │  ├─ project_type.rs         （项目类型枚举）
│  │  ├─ project_status.rs       （项目状态枚举）
│  │  └─ scan_report.rs          （扫描结果数据结构）
│  │
│  ├─ storage/                   （本地数据存储）
│  │  ├─ mod.rs                  （storage 模块入口）
│  │  └─ json.rs                 （JSON 保存与读取）
│  │
│  └─ error.rs                   （统一错误类型）
│
├─ tests/                        （测试代码）
│  └─ fixtures/                  （用于测试的模拟项目目录）
│
└─ docs/                         （项目开发文档）
   ├─ architecture.md            （整体架构说明）
   ├─ core-api.md                （核心模块对外接口说明）
   └─ development.md             （开发规范与协作说明）
```
