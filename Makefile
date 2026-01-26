# CIRISRegistry Makefile

.PHONY: all build test clean proto migrate run

# Go parameters
GOCMD=go
GOBUILD=$(GOCMD) build
GOTEST=$(GOCMD) test
GOMOD=$(GOCMD) mod
BINARY_NAME=registry

# Proto parameters
PROTOC=protoc
PROTO_DIR=protocol
PROTO_OUT=proto/v1

# Build
all: proto build

build:
	$(GOBUILD) -o bin/$(BINARY_NAME) ./cmd/registry

# Generate protobuf code
proto:
	mkdir -p $(PROTO_OUT)
	$(PROTOC) \
		--go_out=$(PROTO_OUT) --go_opt=paths=source_relative \
		--go-grpc_out=$(PROTO_OUT) --go-grpc_opt=paths=source_relative \
		$(PROTO_DIR)/*.proto

# Run tests
test:
	$(GOTEST) -v -race ./...

# Run with default settings
run: build
	./bin/$(BINARY_NAME)

# Tidy dependencies
tidy:
	$(GOMOD) tidy

# Clean build artifacts
clean:
	rm -rf bin/
	rm -rf $(PROTO_OUT)/*.pb.go

# Development database (requires docker)
db-start:
	docker run -d \
		--name ciris-postgres \
		-e POSTGRES_USER=ciris \
		-e POSTGRES_PASSWORD=ciris_dev \
		-e POSTGRES_DB=ciris_registry \
		-p 5432:5432 \
		postgres:15

db-stop:
	docker stop ciris-postgres && docker rm ciris-postgres

# Run migrations only (useful for CI/CD)
migrate:
	$(GOBUILD) -o bin/migrate ./cmd/migrate
	./bin/migrate

# Generate migration status report
migrate-status:
	./bin/$(BINARY_NAME) --migrate-status

# Lint
lint:
	golangci-lint run ./...

# Format
fmt:
	gofmt -s -w .
