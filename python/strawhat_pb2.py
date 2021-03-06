# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: strawhat.proto
"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='strawhat.proto',
  package='strawhat',
  syntax='proto3',
  serialized_options=b'Z\n./strawhat',
  create_key=_descriptor._internal_create_key,
  serialized_pb=b'\n\x0estrawhat.proto\x12\x08strawhat\"E\n\x08Strawhat\x12\x11\n\tserviceID\x18\x01 \x01(\t\x12&\n\ttransport\x18\x02 \x01(\x0e\x32\x13.strawhat.Transport*(\n\tTransport\x12\t\n\x05\x45mpty\x10\x00\x12\x07\n\x03Tcp\x10\x01\x12\x07\n\x03Udp\x10\x03\x42\x0cZ\n./strawhatb\x06proto3'
)

_TRANSPORT = _descriptor.EnumDescriptor(
  name='Transport',
  full_name='strawhat.Transport',
  filename=None,
  file=DESCRIPTOR,
  create_key=_descriptor._internal_create_key,
  values=[
    _descriptor.EnumValueDescriptor(
      name='Empty', index=0, number=0,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='Tcp', index=1, number=1,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='Udp', index=2, number=3,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=99,
  serialized_end=139,
)
_sym_db.RegisterEnumDescriptor(_TRANSPORT)

Transport = enum_type_wrapper.EnumTypeWrapper(_TRANSPORT)
Empty = 0
Tcp = 1
Udp = 3



_STRAWHAT = _descriptor.Descriptor(
  name='Strawhat',
  full_name='strawhat.Strawhat',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='serviceID', full_name='strawhat.Strawhat.serviceID', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='transport', full_name='strawhat.Strawhat.transport', index=1,
      number=2, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=28,
  serialized_end=97,
)

_STRAWHAT.fields_by_name['transport'].enum_type = _TRANSPORT
DESCRIPTOR.message_types_by_name['Strawhat'] = _STRAWHAT
DESCRIPTOR.enum_types_by_name['Transport'] = _TRANSPORT
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

Strawhat = _reflection.GeneratedProtocolMessageType('Strawhat', (_message.Message,), {
  'DESCRIPTOR' : _STRAWHAT,
  '__module__' : 'strawhat_pb2'
  # @@protoc_insertion_point(class_scope:strawhat.Strawhat)
  })
_sym_db.RegisterMessage(Strawhat)


DESCRIPTOR._options = None
# @@protoc_insertion_point(module_scope)
