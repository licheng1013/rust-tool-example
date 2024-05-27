# RustTool

## describe

- This project is a basic background template that uses the IDEA `RustTool` plug-in for code generation.
- You can think of it as learning the development of `rust web` project. Or learn how to use the `RustTool` plugin.
- Plug-in download address: [RustTool](https://plugins.jetbrains.com/plugin/22428-rusttool)
- Note: If the plug-in cannot be found in the plug-in store, please update your development tools (such as IDEA) to the latest version.

## RustTool plugin tutorial

- Plug-in introduction
- The `rust-tool` directory in the project contains all the configuration of the plug-in
- If you don’t want to share it on other computers, you can exclude it through the `.gitignore` file
- Roughly speaking: the `RustTool` plugin is a code generation tool. At the same time, it also plays a certain role in project management
- function list
- 🆗represent support

| 功能     | 框架               | 状态 | 版本        | 文档                                                                |
|--------|------------------|----|-----------|-------------------------------------------------------------------|
| code generation   | salvo,actix      | 🆗 | -         | [code.md](./docs/code.md)                                         |
| Route navigation   | axum,salvo,actix | 🆗 | 2024.1.1+ | [router.md](./docs/router.md)                                     |
| Terminal interface jump |                  | 🆗 | 2024.1.2+ | [console.md](./docs/console.md)                                   |
| ORM support  | rbatis           | /  | -         | [rbatis.md](./docs/rbatis.md)                                     |
| Class to Dto  |                  | 🆗 | -         | [dto.md](./docs/dto.md)                                           |
| feedback     |                  | 🆗 | -         | [issues](https://github.com/licheng1013/rust-tool-example/issues) |

## getting Started

- The plug-in has many configurations. It is recommended to start learning from the sample project. 
- If the template code does not fit your way, you can modify it yourself. 
- If you encounter problems, you can report bugs or suggestions through issue

### rear end

- 1.Clone project
- 2.Import `t_gorm.sql` into your database
- 3.Modify the database link in the `config.yml` file
- 4.Start the `salvo-websrcmain.rs` file to run the project
- Note: The `actix-web directory` can be deleted directly if it is not needed. Currently, the main thing is to adapt the `salvo-web` directory.

### front end

- 1.Run `pnpm i` in the view directory
- 2.Run `pnpm run dev` to run the front-end project
- 3.Package `pnpm run build` to package the front-end project
- Note: The plug-in can generate front-end templates, but for `numeric types` or other non-string types, you need to convert them from the front-end yourself.

