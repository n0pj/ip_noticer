gcloud dns record-sets transaction start --zone="nserver"
gcloud dns record-sets transaction remove 14.132.166.234 --zone="nserver" --name="home.n0pj.com" --ttl="30" --type="A"
gcloud dns record-sets transaction add 14.132.166.234 --zone="nserver" --name="home.n0pj.com" --ttl="30" --type="A"
gcloud dns record-sets transaction execute --zone="nserver"
