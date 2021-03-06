# ip_noticer

Google Cloud DNS に IP を自動通知するための Rust アプリ

```shell
docker exec -it ip_noticer bash
```

docker container (ip_noticer) 内

```shell
gcloud auth login
```

画面に沿って GCP のアカウントを選択
verification code を入力する
以下でプロジェクトを選択する

```shell
gcloud config set project PROJECT_ID
```
