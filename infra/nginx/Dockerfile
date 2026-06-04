FROM nginx:1.31-alpine
RUN rm /etc/nginx/conf.d/*
RUN mkdir -p /srv/www/assets
COPY infra/nginx/nginx.conf /etc/nginx/nginx.conf
COPY infra/nginx/conf.d/* /etc/nginx/conf.d/
COPY web/assets/* /srv/www/assets/
