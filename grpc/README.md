# Compiling protobuf in `go`

1. `go install google.golang.org/protobuf/cmd/protoc-gen-go@v1.28`
2. `go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@v1.2`
3. be sure to have `PATH` correctly configured
   eg. `$env.PATH = ($env.PATH | split row (char esep) | prepend $"(go env GOPATH)/bin")`
4. add `option go_package = "pb/simple";` (NEEDED)
5. this horrendous command: `protoc -I=../protobuf --go_out=. --go-grpc_out=. ../protobuf/simple.proto`
