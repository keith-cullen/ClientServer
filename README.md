# Client Server

A gRPC/TLS/TCP client/server in Go and Rust, TLS/TCP client/Server in Go and Rust and TCP client/server in Go and Rust.

## Instructions

### gRPC

#### Go

1. Install protoc

        $ cd /tmp

        $ wget https://github.com/protocolbuffers/protobuf/releases/download/v28.0/protoc-28.0-linux-x86_64.zip

        $ unzip protoc-28.0-linux-x86_64.zip

        $ sudo cp bin/protoc /usr/local/bin

        $ sudo cp -r include/google /usr/local/include

2. Install gRPC and protobuf tools for Go

        $ go install google.golang.org/protobuf/cmd/protoc-gen-go@latest

        $ go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest

3. Generate client protobuf code

        $ cd grpc/go/client

        $ protoc --proto_path=../proto app.proto --go-grpc_out=app --go_out=app

4. Generate server protobuf code

        $ cd grpc/go/server

        $ protoc --proto_path=../proto app.proto --go-grpc_out=app --go_out=app

5. Run the server

        $ cd grpc/go/server

        $ go run main.go

6. Run the client

        $ cd grpc/go/client

        $ go run main.go

##### Use grpcurl on the Go Server

1. Install grpcurl

        $ go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest

2. Run grpcurl on the service definition

        $ cd grpc/go/proto

        $ grpcurl -plaintext -import-path . -proto app.proto describe app.App.get

        $ grpcurl -plaintext -import-path . -proto app.proto describe .app.Req

        $ grpcurl -plaintext -import-path . -proto app.proto describe .app.Resp

3. Run grpcurl on a protoset

        $ cd grpc/go/proto

        $ protoc --proto_path=. --include_imports --descriptor_set_out=app.protoset app.proto

        $ grpcurl -protoset app.protoset describe app.App.get

        $ grpcurl -protoset app.protoset describe .app.Req

        $ grpcurl -protoset app.protoset describe .app.Resp

4. Interact with the server

        $ cd grpc/go/server

        $ go run main.go

        $ cd ../../..

        $ grpcurl -insecure -cert certs/client_cert.pem -key certs/client_privkey.pem -import-path ./grpc/go/proto -proto app.proto -d '{"name": "key1"}' localhost:50051 app.App.get

##### Use k6 on the Go Server

1. Install k6

        $ sudo gpg -k

        $ sudo gpg --no-default-keyring --keyring /usr/share/keyrings/k6-archive-keyring.gpg --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys C5AD17C747E3415A3642D57D77C6C491D6AC1D69

        $ echo "deb [signed-by=/usr/share/keyrings/k6-archive-keyring.gpg] https://dl.k6.io/deb stable main" | sudo tee /etc/apt/sources.list.d/k6.list

        $ sudo apt-get update

        $ sudo apt-get install k6

4. Interact with the server

        $ cd grpc/go/server

        $ go run main.go

        $ cd ..

        $ k6 run k6.js

#### Rust

1. Run the server

        $ cd grpc/rust

        $ cargo run --bin server

2. Run the client

        $ cd grpc/rust

        $ cargo run --bin client

##### Use grpcurl on the Rust server

1. Install grpcurl

        $ go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest

2. Run grpcurl on the service definition

        $ cd grpc/rust/proto

        $ grpcurl -plaintext -import-path . -proto app.proto describe app.App.get

        $ grpcurl -plaintext -import-path . -proto app.proto describe .app.Req

        $ grpcurl -plaintext -import-path . -proto app.proto describe .app.Resp

3. Run grpcurl on a protoset

        $ cd grpc/rust/proto

        $ protoc --proto_path=. --include_imports --descriptor_set_out=app.protoset app.proto

        $ grpcurl -protoset app.protoset describe app.App.get

        $ grpcurl -protoset app.protoset describe .app.Req

        $ grpcurl -protoset app.protoset describe .app.Resp

4. Interact with the server

        $ cd grpc/rust

        $ cargo run --bin server

        $ cd ../..

        $ grpcurl -insecure -cert certs/client_cert.pem -key certs/client_privkey.pem -import-path ./grpc/rust/proto -proto app.proto -d '{"name": "key1"}' localhost:50052 app.App.get

##### Use k6 on the Rust Server

1. Install k6

        $ sudo gpg -k

        $ sudo gpg --no-default-keyring --keyring /usr/share/keyrings/k6-archive-keyring.gpg --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys C5AD17C747E3415A3642D57D77C6C491D6AC1D69

        $ echo "deb [signed-by=/usr/share/keyrings/k6-archive-keyring.gpg] https://dl.k6.io/deb stable main" | sudo tee /etc/apt/sources.list.d/k6.list

        $ sudo apt-get update

        $ sudo apt-get install k6

4. Interact with the server

        $ cd grpc/rust

        $ cargo run --bin server

        $ k6 run k6.js

### TLS

#### Go

1. Run the server

        $ cd tls/go/server

        $ go run main.go

2. Run the client

        $ cd tls/go/client

        $ go run main.go

#### Rust

1. Run the server

        $ cd tls/rust

        $ cargo run --bin server

2. Run the client

        $ cd tls/rust

        $ cargo run --bin client

### TCP

#### Go

1. Run the server

        $ cd tcp/go/server

        $ go run main.go

2. Run the client

        $ cd tcp/go/client

        $ go run main.go

#### Rust

1. Run the server

        $ cd tcp/rust

        $ cargo run --bin server

2. Run the client

        $ cd tcp/rust

        $ cargo run --bin client
