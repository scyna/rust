protoc -I=. --rust_out=engine engine.proto
protoc -I=. --rust_out=error error.proto
protoc -I=. --rust_out=scyna scyna.proto
protoc -I=. --rust_out=task task.proto

