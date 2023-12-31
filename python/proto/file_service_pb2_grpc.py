# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

from proto import file_service_pb2 as proto_dot_file__service__pb2


class ShardFileServiceStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.GetShardFiles = channel.unary_unary(
                '/file_service.ShardFileService/GetShardFiles',
                request_serializer=proto_dot_file__service__pb2.GetShardFilesRequest.SerializeToString,
                response_deserializer=proto_dot_file__service__pb2.GetShardFilesResponse.FromString,
                )
        self.DownloadShardFile = channel.unary_stream(
                '/file_service.ShardFileService/DownloadShardFile',
                request_serializer=proto_dot_file__service__pb2.DownloadShardFileRequest.SerializeToString,
                response_deserializer=proto_dot_file__service__pb2.DownloadShardFileResponse.FromString,
                )


class ShardFileServiceServicer(object):
    """Missing associated documentation comment in .proto file."""

    def GetShardFiles(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def DownloadShardFile(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_ShardFileServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'GetShardFiles': grpc.unary_unary_rpc_method_handler(
                    servicer.GetShardFiles,
                    request_deserializer=proto_dot_file__service__pb2.GetShardFilesRequest.FromString,
                    response_serializer=proto_dot_file__service__pb2.GetShardFilesResponse.SerializeToString,
            ),
            'DownloadShardFile': grpc.unary_stream_rpc_method_handler(
                    servicer.DownloadShardFile,
                    request_deserializer=proto_dot_file__service__pb2.DownloadShardFileRequest.FromString,
                    response_serializer=proto_dot_file__service__pb2.DownloadShardFileResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'file_service.ShardFileService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class ShardFileService(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def GetShardFiles(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/file_service.ShardFileService/GetShardFiles',
            proto_dot_file__service__pb2.GetShardFilesRequest.SerializeToString,
            proto_dot_file__service__pb2.GetShardFilesResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def DownloadShardFile(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_stream(request, target, '/file_service.ShardFileService/DownloadShardFile',
            proto_dot_file__service__pb2.DownloadShardFileRequest.SerializeToString,
            proto_dot_file__service__pb2.DownloadShardFileResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)
