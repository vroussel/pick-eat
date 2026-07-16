FROM nginx:1.31-alpine
RUN rm /etc/nginx/conf.d/*
RUN mkdir -p /srv/www/assets
COPY nginx.conf /etc/nginx/nginx.conf
COPY conf.d/* /etc/nginx/conf.d/
COPY --from=assets . /srv/www/assets/
