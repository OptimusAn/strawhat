// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: strawhat.proto

// This CPP symbol can be defined to use imports that match up to the framework
// imports needed when using CocoaPods.
#if !defined(GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS)
 #define GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS 0
#endif

#if GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS
 #import <Protobuf/GPBProtocolBuffers.h>
#else
 #import "GPBProtocolBuffers.h"
#endif

#if GOOGLE_PROTOBUF_OBJC_VERSION < 30004
#error This file was generated by a newer version of protoc which is incompatible with your Protocol Buffer library sources.
#endif
#if 30004 < GOOGLE_PROTOBUF_OBJC_MIN_SUPPORTED_VERSION
#error This file was generated by an older version of protoc which is incompatible with your Protocol Buffer library sources.
#endif

// @@protoc_insertion_point(imports)

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"

CF_EXTERN_C_BEGIN

NS_ASSUME_NONNULL_BEGIN

#pragma mark - Enum Protocol_Enum

typedef GPB_ENUM(Protocol_Enum) {
  /**
   * Value used if any message's field encounters a value that is not defined
   * by this enum. The message will also have C functions to get/set the rawValue
   * of the field.
   **/
  Protocol_Enum_GPBUnrecognizedEnumeratorValue = kGPBUnrecognizedEnumeratorValue,
  Protocol_Enum_Tcp = 0,
  Protocol_Enum_Udp = 1,
};

GPBEnumDescriptor *Protocol_Enum_EnumDescriptor(void);

/**
 * Checks to see if the given value is defined by the enum or was not known at
 * the time this source was generated.
 **/
BOOL Protocol_Enum_IsValidValue(int32_t value);

#pragma mark - StrawhatRoot

/**
 * Exposes the extension registry for this file.
 *
 * The base class provides:
 * @code
 *   + (GPBExtensionRegistry *)extensionRegistry;
 * @endcode
 * which is a @c GPBExtensionRegistry that includes all the extensions defined by
 * this file and all files that it depends on.
 **/
GPB_FINAL @interface StrawhatRoot : GPBRootObject
@end

#pragma mark - Strawhat

typedef GPB_ENUM(Strawhat_FieldNumber) {
  Strawhat_FieldNumber_ServiceId = 1,
  Strawhat_FieldNumber_Protocol = 2,
};

GPB_FINAL @interface Strawhat : GPBMessage

@property(nonatomic, readwrite, copy, null_resettable) NSString *serviceId;

@property(nonatomic, readwrite) Protocol_Enum protocol;

@end

/**
 * Fetches the raw value of a @c Strawhat's @c protocol property, even
 * if the value was not defined by the enum at the time the code was generated.
 **/
int32_t Strawhat_Protocol_RawValue(Strawhat *message);
/**
 * Sets the raw value of an @c Strawhat's @c protocol property, allowing
 * it to be set to a value that was not defined by the enum at the time the code
 * was generated.
 **/
void SetStrawhat_Protocol_RawValue(Strawhat *message, int32_t value);

NS_ASSUME_NONNULL_END

CF_EXTERN_C_END

#pragma clang diagnostic pop

// @@protoc_insertion_point(global_scope)