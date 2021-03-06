FROM datasetteproject/datasette:0.53
VOLUME /data

RUN mkdir /app

ADD metadata.json /app
COPY templates /app/templates
COPY plugins /app/plugins

RUN pip install datasette-template-sql

ENTRYPOINT datasette --metadata /app/metadata.json -p 8001 -h 0.0.0.0 --template-dir=/app/templates/ --plugins-dir=/app/plugins/ /data/highlights.db
