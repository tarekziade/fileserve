import grpc
import os

from proto.file_service_pb2_grpc import ShardFileServiceStub
from proto.file_service_pb2 import GetShardFilesRequest, DownloadShardFileRequest


TARGET = os.path.join(os.path.dirname(__file__), "here")
channel = grpc.insecure_channel("localhost:50051")
stub = ShardFileServiceStub(channel)

resp = stub.GetShardFiles(GetShardFilesRequest())

for file in resp.files:
    print(f"Writing {file.path}")
    target = os.path.join(TARGET, file.path)
    os.makedirs(os.path.dirname(target), exist_ok=True)

    with open(target, "wb") as f:
        for chunk in stub.DownloadShardFile(
            DownloadShardFileRequest(relative_path=file.path)
        ):
            f.write(chunk.chunk_data)
