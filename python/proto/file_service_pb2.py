# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: proto/file_service.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder

# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(
    b'\n\x18proto/file_service.proto\x12\x0c\x66ile_service"\x16\n\x14GetShardFilesRequest"N\n\x15GetShardFilesResponse\x12\r\n\x05total\x18\x01 \x01(\x05\x12&\n\x05\x66iles\x18\x02 \x03(\x0b\x32\x17.file_service.ShardFile"\'\n\tShardFile\x12\x0c\n\x04path\x18\x01 \x01(\t\x12\x0c\n\x04size\x18\x02 \x01(\x04\x32n\n\x10ShardFileService\x12Z\n\rGetShardFiles\x12".file_service.GetShardFilesRequest\x1a#.file_service.GetShardFilesResponse"\x00\x62\x06proto3'
)

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, "proto.file_service_pb2", _globals)
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    _globals["_GETSHARDFILESREQUEST"]._serialized_start = 42
    _globals["_GETSHARDFILESREQUEST"]._serialized_end = 64
    _globals["_GETSHARDFILESRESPONSE"]._serialized_start = 66
    _globals["_GETSHARDFILESRESPONSE"]._serialized_end = 144
    _globals["_SHARDFILE"]._serialized_start = 146
    _globals["_SHARDFILE"]._serialized_end = 185
    _globals["_SHARDFILESERVICE"]._serialized_start = 187
    _globals["_SHARDFILESERVICE"]._serialized_end = 297
# @@protoc_insertion_point(module_scope)
