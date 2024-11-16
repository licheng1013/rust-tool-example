# RustTool

- [English](README_en.md)

## 描述

- 此项目是一个后台基本模板,使用 IDEA `RustTool` 插件来进行代码生成,
- 你可以当作学习 `rust web` 项目的开发。 或者学习 `RustTool` 插件的使用。
- 插件下载地址: [RustTool](https://plugins.jetbrains.com/plugin/22428-rusttool)
- 注意: 如果在插件商店内搜索不到插件，`请更新你的开发工具(如IDEA)至最新版`。
- 交流QQ群: `289132257` 有任何问题可以在`群内反馈`。

## RustTool插件教程

- 插件介绍
- 项目中的 `rust-tool` 目录则是插件的所有配置
- 如果不想共享在其他电脑上，可以通过 `.gitignore` 文件排除
- 大致上来讲：`RustTool` 插件是一个代码生成工具。同时对于管理项目也起到了一定的帮助
- 功能列表
- 🆗代表支持

| 功能     | 框架               | 状态 | 版本        | 文档                                                                |
|--------|------------------|----|-----------|-------------------------------------------------------------------|
| 代码生成   | salvo,actix      | 🆗 | -         | [code.md](./docs/code.md)                                         |
| 路由导航   | axum,salvo,actix | 🆗 | 2024.1.1+ | [router.md](./docs/router.md)                                     |
| 终端接口跳转 |                  | 🆗 | 2024.1.2+ | [console.md](./docs/console.md)                                   |
| ORM支持  | rbatis           | /  | -         | [rbatis.md](./docs/rbatis.md)                                     |
| 类转Dto  |                  | 🆗 | -         | [dto.md](./docs/dto.md)                                           |
| 反馈     |                  | 🆗 | -         | [issues](https://github.com/licheng1013/rust-tool-example/issues) |

## 入门

- 插件有较多配置,建议从示例项目开始学习。
- 模板代码如果不符合你的方式，可以自行修改。
- 遇到问题，可以通过issue反馈bug或者建议

### 后端

- 1.克隆项目
- 2.导入`t_gorm.sql` 到你的数据库
- 3.修改 `config.yml` 文件中的数据库链接
- 4.启动 `salvo-web/src/main.rs` 文件就可以运行项目了
- 注意: `actix-web目录` 不需要可以直接删除，目前主要的是适配 `salvo-web` 目录

