# ip_noticer

Google Cloud DNS に IP を自動通知するための Rust アプリ

最初は、以下を実行

```
./ip_noticer
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

.env の設定を反映させたい時

```
docker stop ip_noticer  && docker rm ip_noticer
```

cron の実行結果を知りたい時

```
docker exec -it ip_noticer bash
cat /var/tmp/cron_log
```
