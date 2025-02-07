FROM httpd:latest

# Copy the Vue build files
COPY dist/ /usr/local/apache2/htdocs/

# Copy SSL certificates
COPY apache2/server.crt /usr/local/apache2/conf/server.crt
COPY apache2/server.key /usr/local/apache2/conf/server.key

# Copy SSL configuration
COPY apache2/httpd-ssl.conf /usr/local/apache2/conf/extra/httpd-ssl.conf

# Enable SSL in Apache
RUN sed -i '/LoadModule rewrite_module/s/^#//g' /usr/local/apache2/conf/httpd.conf
RUN echo "Include conf/extra/httpd-ssl.conf" >> /usr/local/apache2/conf/httpd.conf

EXPOSE 443

CMD ["httpd-foreground"]
