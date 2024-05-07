![ci](https://github.com/hiterm/bookshelf-api/actions/workflows/ci.yml/badge.svg)

# Bookshelf API

Backend server for [Bookshelf](https://github.com/hiterm/bookshelf/).

## How to run server

### Set up Auth0

Set up auth0 by following:

https://auth0.com/developers/hub/code-samples/api/actix-web-rust/basic-authorization

### Setup .env

```sh
$ mv .env.template .env
$ vim .env  # Fill your value
```

### Run migration

```
$ cargo install sqlx-cli
$ sqlx migrate run
```

### Start server

```
$ cargo run
```

### Run via Docker Compose

```sh
$ mv .env.template .env.docker
$ vim .env.docker  # Fill your value
```

```
$ docker-compose up --build
```

## Test

```
$ cargo test
```

With DB

```
$ docker-compose -f docker-compose-test.yml up -d
$ cargo test -- --include-ignored
```

## GraphQL Playground

Run server and access `/graphql/playground`.

## Generate GraphQL schema

```
$ cargo run --bin gen_schema
```
***

### How it works as a Clean Architecture
![Figure: Clean Architecture](image.png)
```bash
src
├── domain // 🟢エンティティ。最重要ビジネスルールをカプセル化したもの
│   ├── todo.rs
│   └── user.rs
├── domain.rs // 🟢./domain配下のモジュールexport
├── interface_adapter  //🟢./domain配下のモジュールexport
│   ├── todo_viewmodel.rs
│   ├── user_viewmodel.rs
│   ├── web_todo_controller.rs
│   └── web_user_controller.rs
├── interface_adapter.rs // 🟢./interface_adapter配下のモジュールexport
├── lib.rs // 🟢最上位ディレクトリのexport
├── main.rs
├── presenter // 🟢
│   ├── todo.rs
│   └── user.rs
├── presenter.rs // 🟢./presenter配下のモジュールexport
├── usecase // 🟢アプリケーション固有のビジネスルール
│   ├── data_access // 🟢DBの型をusecaseで使用する値と合わせる
│   │   ├── todo.rs
│   │   └── user.rs
│   ├── data_access.rs // 🟢./data_access配下のモジュールexports
│   ├── input_boundary // 🟢interface_adapter→usecase interface_adapterへの直接依存を防ぐためのIF(IF分離の法則)
│   │   ├── todo.rs
│   │   └── user.rs
│   ├── input_boundary.rs // 🟢./input_boundary配下のモジュールexport
│   ├── interactor // 🟢ビジネスロジック実装の定義
│   │   ├── todo.rs
│   │   └── user.rs
│   └── interactor.rs // 🟢./interactor配下のモジュールexport
└── usecase.rs // 🟢./usecase配下のモジュールexport
```

Ref: 
https://zenn.dev/htlsne/articles/rust-clean-architecture
https://github.com/hiterm/bookshelf-api
https://lucid.app/lucidchart/62854681-180e-4860-9d75-9c878674ba6d/edit?viewport_loc=-275%2C368%2C2917%2C1395%2C0_0&invitationId=inv_78d72729-6626-4966-a05d-d5e67e6ca173