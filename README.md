![ci](https://github.com/hiterm/bookshelf-api/actions/workflows/ci.yml/badge.svg)

# TODO API

## How to run server

### Setup .env

```sh
$ mv .env.template .env
$ vim .env  # Fill your value
```

### Run via Docker Compose

```sh
$ mv .env.template .env.docker
$ vim .env.docker  # Fill your value
```

```
$ docker-compose up --build
```

***

### Architectural Description - Clean Architecture

![Figure: Clean Architecture](image.png)

```bash
src
├── db // 🟢DB モデル
│   ├── init_todo_db.rs
│   ├── init_user_db.rs
│   ├── ram_zatsu_todo_db.rs
│   └── ram_zatsu_user_db.rs
├── db.rs
├── domain // 🟢エンティティ。最重要ビジネスルールをカプセル化したもの
│   ├── todo.rs
│   └── user.rs
├── domain.rs
├── framework　// 🟢APIハンドラ
│   └── api_server.rs
├── framework.rs
├── interface_adapter // 🟢MVCを保持しているレイヤー
│   ├── controller // 🟢 リクエストを受け取り、レスポンスを返すための窓口
│   │   ├── web_todo.rs
│   │   └── web_user.rs
│   ├── controller.rs
│   ├── presenter // 🟢 Output DataからView Modelで定義されている型に変換するための処理
│   │   ├── todo.rs
│   │   └── user.rs
│   ├── presenter.rs
│   ├── viewmodel // 🟢 レスポンスの型
│   │   ├── todo.rs
│   │   └── user.rs
│   └── viewmodel.rs
├── interface_adapter.rs
├── lib.rs
├── main.rs
├── usecase // 🟢アプリケーション固有のビジネスルール
│   ├── data_access // 🟢DBの型をusecaseで使用する型と合わせる
│   │   ├── todo.rs
│   │   └── user.rs
│   ├── data_access.rs
│   ├── input_boundary // 🟢 interface_adapter→usecaseへの直接依存を防ぐためのIF(IF分離の法則)
│   │   ├── todo.rs
│   │   └── user.rs
│   ├── input_boundary.rs
│   ├── interactor // 🟢ビジネスロジック実装の定義
│   │   ├── todo.rs
│   │   └── user.rs
│   ├── interactor.rs
│   ├── output_data // 🟢ユースケース層出力値の定義
│   │   ├── todo.rs
│   │   └── user.rs
│   └── output_data.rs
└── usecase.rs // 🟢アプリケーション固有のビジネスルール
```

### GET: `/todo`
```bash
curl -XGET localhost:8081/todo
```

### POST: `/todo`
```bash
curl -XPOST localhost:8081/todo -H "Content-Type: application/json" -d '{"title" : "綾鷹", "content": "選ばれたのは", "user_id": "1" }'
```

### PUT: `/todo/title`
```bash
curl -XPUT localhost:8081/todo/title -H "Content-Type: application/json" -d '{"id" : "1", "title": "爽健美茶" }'
```

### PUT: `/todo/content`
```bash
curl -XPUT localhost:8081/todo/content -H "Content-Type: application/json" -d '{"id" : "1", "content": "Selected..." }'
```

### DELETE: `/todo`
```bash
curl -XDELETE localhost:8081/todo -H "Content-Type: application/json" -d '{"id" : "1"}'
```

***

### GET: `/users`
```bash
curl -XGET localhost:8081/users
```

### POST: `/users`
```bash
curl -XPOST localhost:8081/users -H "Content-Type: application/json" -d '{"name" : "綾鷹" }'
```

### PUT: `/users`
```bash
curl -XPUT localhost:8081/users -H "Content-Type: application/json" -d '{"name" : "伊右衛門" }'
```

### DELETE: `/users`
```bash
curl -XDELETE localhost:8081/users -H "Content-Type: application/json" -d '{"id" : "1" }'
```


Ref: 
https://zenn.dev/htlsne/articles/rust-clean-architecture
https://github.com/hiterm/bookshelf-api
https://lucid.app/lucidchart/62854681-180e-4860-9d75-9c878674ba6d/edit?viewport_loc=-275%2C368%2C2917%2C1395%2C0_0&invitationId=inv_78d72729-6626-4966-a05d-d5e67e6ca173