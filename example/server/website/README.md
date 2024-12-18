## Running the server

```
make run
```

## Calling the endpoints
```
grpcurl -proto order/api.proto -plaintext -d '{"id":"123"}' localhost:50051 proto.OrderService/GetOrder
```
