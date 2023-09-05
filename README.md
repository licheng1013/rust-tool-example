# RustTool

- [English](README_en.md)

## 开始

- 1: 安装 Jetbrains 插件 https://plugins.jetbrains.com/plugin/22428-rusttool
- 2: 导入t_gorm.sql到你的数据库并在 main.rs 文件修改为你的数据库链接
- 3: 运行 main.rs 文件

## 介绍

- RustTool 是一个针对于 Rust web开发的一个idea插件，目前支持 actix-web 和 salvo 两个框架和 rbatis 增删改查代码生成.
- y代表支持,n暂不支持.

| 框架/插件                       | 功能              | 支持) |
|-----------------------------|-----------------|-----|
| actix-web / salvo +  rbatis | 路由导航            | y   |
| actix-web / salvo +  rbatis | 增删改查代码生成        | y   |
| actix-web / salvo           | Api文档导出         | y   |
| rust-tool                   | 创建目录时创建mod文件    | y   |
| rust-tool                   | mod自动关联目录下新rs文件 | y   |
| rbatis                      | html和方法导航       | n   |
| rust-tool                   | SDK扩展           | n   |

## 功能演示

![](images/doc.png)

- 自动导入mod.rs你需要选中一个项目目录，在你mod.rs目录创建文件并修改时，会自动导入到mod.rs中。
- 由于idea的需要延迟刷新文件内容。你可能需要切换下 软件窗口 或 关闭文件再次打开。即可看到效果。

- ![](images/doc1.png)

## 反馈

-
请通过issue反馈bug或者建议 [https://github.com/licheng1013/rust-tool-example/issues](https://github.com/licheng1013/rust-tool-example/issues)
