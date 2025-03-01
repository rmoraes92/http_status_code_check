# Demo

We have two small http echo servers using Python and Go:

- Dockerfile.python
- Dockerfile.go

The focus here is on the compose file. Specifically in the healthcheck section:

```
    healthcheck:
      test: ["CMD", "http_status_code_check", "-u", "http://localhost:8080/", "-s", "200"]
      interval: 2s
      timeout: 5s
      retries: 3
```

This more or less replicates the scenario where AWS ECS Task definition will
ask you to have an internal call/script/routine that can confirm the server is
up.

Using `http_status_code_check` will handle the task of checking for the HTTP
status code without providing any extra function reducing the amount of vectors
we could had if the container get compromised.