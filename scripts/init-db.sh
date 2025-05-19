#! /bin/bash

docker run \
	--name lifehon_db \
	-t \
	-e POSTGRES_USER=lifehon \
	-e POSTGRES_PASSWORD=password \
	-e POSTGRES_DB=lifehon \
	-p 4321:5432 \
	-d postgres:alpine
