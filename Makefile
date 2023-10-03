

.PHONY: proto
proto:
	bin/python -m grpc_tools.protoc proto/file_service.proto -I ./ --python_out=./python/ --grpc_python_out=./python


