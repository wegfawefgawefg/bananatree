#!/bin/bash
curl -X POST http://localhost:8080/post_banana \
-H "Content-Type: application/json" \
-d '{"user_id": 1, "content": "Banana 1 is the best!"}'

curl -X POST http://localhost:8080/post_banana \
-H "Content-Type: application/json" \
-d '{"user_id": 2, "content": "I prefer Banana 2."}'

curl -X POST http://localhost:8080/post_banana \
-H "Content-Type: application/json" \
-d '{"user_id": 3, "content": "Banana 3 for the win!"}'
