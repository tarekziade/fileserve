import grpc
from proto.file_service_pb2_grpc import ShardFileServiceStub
from proto.file_service_pb2 import GetShardFilesRequest


channel = grpc.insecure_channel("localhost:50051")
stub = ShardFileServiceStub(channel)

resp = stub.GetShardFiles(GetShardFilesRequest())
print(resp)
