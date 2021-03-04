# ip_noticer

Google Cloud DNS に IP を自動通知するための Rust アプリ

docker build -t ip_noticer .
docker run -d ip_noticer

gcloud dns record-sets transaction start --zone="nserver"

gcloud dns record-sets transaction add 10.2.3.4 \
 --name="home.n0pj.com" \
 --ttl="30" \
 --type="A" \
 --zone="nserver"

gcloud dns record-sets transaction remove 10.2.3.4 \
 --name="home.n0pj.com" \
 --ttl="30" \
 --type="A" \
 --zone="nserver"

gcloud dns record-sets transaction execute --zone="nserver"
