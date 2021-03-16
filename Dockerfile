FROM ubuntu:20.04
RUN DEBIAN_FRONTEND=noninteractive

RUN apt-get update
RUN echo "deb [signed-by=/usr/share/keyrings/cloud.google.gpg] https://packages.cloud.google.com/apt cloud-sdk main" | tee -a /etc/apt/sources.list.d/google-cloud-sdk.list
RUN apt-get install -y apt-transport-https ca-certificates gnupg curl
RUN curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | apt-key --keyring /usr/share/keyrings/cloud.google.gpg add -
RUN apt-get update && apt-get install -y google-cloud-sdk

COPY ./.env /
COPY ./ip_noticer /

RUN apt-get install -y cron
COPY crontab /var/spool/crontab/root
RUN crontab /var/spool/crontab/root
# RUN chmod 0644 /etc/cron.d/crontab

# RUN service cron start

# change timezone
RUN apt-get install tzdata
RUN cp /usr/share/zoneinfo/Asia/Tokyo /etc/localtime
RUN echo "Asia/Tokyo" > /etc/timezone

CMD ["cron", "-f"]
