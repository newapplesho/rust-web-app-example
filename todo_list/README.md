# TODO API Server

# 主な内容
- JSON API Serverのサンプルコードです。
- Web Frameworkは[actix-web](https://github.com/actix/actix-web)を使います。
- DB関連は[sqlx](https://github.com/launchbadge/sqlx)を使います。
  Rustのウェブ開発だと[Diesel](https://github.com/diesel-rs/diesel)も出てきますが、今回はSQLを用いた薄いwrapperを使います。
  パフォーマンスチューニングがしやすいのでsqlxを選びます（あくまで個人的な意見です）。
- Read, WriteのDB Connection poolを保有して、使い分けれるようにしています。
- 独自Error定義はthiserrorを用います。
- Rustで軽量Dockerfile imageの作り方も用意しました。
- その他
  - DB Migration toolは言語依存がない[Flyway](https://flywaydb.org/)(Docker Image)を使います（Flywayの使い方を紹介するためにあえてテーブル変更のコードを入れます）。
  - API DocはPostmanのCollectionに記述します。

# Using

```
$ cp .env.sample .env # copy enviroment file
```

## Starting the Local Database Server

```
$ docker-compose -f docker-compose-local-db.yml build
$ docker-compose -f docker-compose-local-db.yml up
```

##DB Migration
```
$ docker-compose -f docker-compose-flyway.yml run --rm flyway migrate
```

## Starting Server 
事前にDBは起動しておくこと。

Local起動

```
% cargo build --package api
% cargo run
```

LocalでDocker Composeを使った起動

```
$ docker-compose build
$ docker-compose up
```