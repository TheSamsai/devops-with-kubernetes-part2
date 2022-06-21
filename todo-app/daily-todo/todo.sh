#! /bin/bash

article=$(curl -i https://en.wikipedia.org/wiki/Special:Random| tr -d '\r' | sed -En 's/^location: (.*)/\1/p')

curl -X POST http://todo-app-svc:2345/todos -H 'Content-Type: application/json' -d '{"todo": "Read '"$article"'" }'
