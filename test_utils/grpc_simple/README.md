## Running the server

```
make run
```

## Calling the endpoints
```
grpcurl -proto greeter.proto -plaintext -d '{"name":"Alice"}' localhost:50051 proto.Greeter/SayHello

grpcurl -proto timekeeper.proto -plaintext localhost:50051 proto.TimeKeeper/GetDate 

grpcurl -proto timekeeper.proto -plaintext -d '{"number":1}' localhost:50051 proto.TimeKeeper/GetNameOfMonth

 grpcurl -proto debugger.proto -plaintext -H 'your-metadata-key: your-metadata-value' localhost:50051 proto.Debugger/GetMetadata
```
