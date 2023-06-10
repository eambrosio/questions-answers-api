# Questions & Answers API

Rust-based API for a StackOverflow-like app.

This API has two primary features:

- Question creation, retrieval & deletion
- Answer creation, retrieval & deletion

![api-gif](./api.gif)

## Persistence layer

For this purpose, we will PostgreSQL to store the info about Questions and Answers. The tables are:

### Question

| Name          | Type         | Description                                  |
|---------------|--------------|----------------------------------------------|
| question_uuid | UUID         | Generated identifier unique to each question |
| title         | VARCHAR(255) | Title of the question                        |
| description   | VARCHAR(255) | Description of the question                  |
| created_at    | TIMESTAMP    | Creation timestamp of the question           |

### Answer

| Name          | Type         | Description                                  |
|---------------|--------------|----------------------------------------------|
| answer_uuid   | UUID         | Generated identifier unique to each answer   |
| question_uuid | UUID         | Generated identifier unique to each question |
| content       | VARCHAR(255) | Content of the answer                        |
| created_at    | TIMESTAMP    | Creation timestamp of the answer             |

As you can see, this is a very simple 1:N relationship between Questions and Answers. One question might have N answers potentially. If a question is deleted, all the related answeres will be deleted as well.

## Testing

In order to test the, you need to run a docker container with a Postgres instances and execute the migration process to create the expected tables:
```bash
docker run --name stack-overflow-db -e POSTGRES_PASSWORD=postgrespw -p 55008:5432 -d postgres

sqlx migrate run
```

NOTE: If you don't have sql installed already, you can install execute this cargo command:
```bash
cargo install sqlx-cli   
```
Use the following cURL operations to add and retrieve some data

### Create question

```text
curl --request POST \
  --url http://localhost:8000/question \
  --header 'Accept: application/json' \
  --data '{
	"title": "Title",
	"description": "Description"
}'
```

### Get questions

```text
curl --request GET \
  --url http://localhost:8000/questions \
  --header 'Accept: application/json'
```

### Delete question

```text
curl --request DELETE \
  --url http://localhost:8000/question \
  --header 'Accept: application/json' \
  --data '{
	"question_uuid": "[UUID of a created question]"
}'
```

### Create answer

```text
curl --request POST \
  --url http://localhost:8000/answer \
  --header 'Accept: application/json' \
  --data '{
	"question_uuid": "[UUID of a created question]",
	"content": "Content"
}'
```

### Get answers

```text
curl --request GET \
  --url http://localhost:8000/answers \
  --header 'Accept: application/json' \
  --data '{
	"question_uuid": "[UUID of a created question]"
}'
```


### Delete answer

```text
curl --request DELETE \
  --url http://localhost:8000/answer \
  --header 'Accept: application/json' \
  --data '{
	"answer_uuid": "[UUID of a created answer]"
}'
```
