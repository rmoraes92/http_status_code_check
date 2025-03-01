FROM golang:1.23
WORKDIR /src
RUN curl -L https://github.com/rmoraes92/http_status_code_check/releases/download/0.1.1/http_status_code_check > /bin/http_status_code_check && \
    chmod 755 /bin/http_status_code_check
RUN go mod init example/http_server
COPY <<EOF ./main.go
package main

import (
	"github.com/gin-gonic/gin"
	"net/http"
)

func main() {
	r := gin.Default()

	r.GET("/", func(c *gin.Context) {
		c.String(http.StatusOK, "hello world")
	})

	r.Run()
}
EOF
RUN go get .
RUN go build -o /bin/http_server ./main.go

FROM debian
COPY --from=0 /bin/http_server /bin/http_server
COPY --from=0 /bin/http_status_code_check /bin/http_status_code_check
RUN apt-get update && apt-get install -y libssl-dev
CMD ["http_server"]