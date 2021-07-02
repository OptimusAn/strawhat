// MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMM;..HMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM$...NMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMM;NMMN.MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM.MMMM.MMMMMMMMMMMMMMM
// MMMMMMMMMMMMMN.MMMM;MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM.MMMM.?NMMMMMMMMMMMMM
// MMMMMMMMMMM.NMMMMMM.MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM.MMMMMMM?CMMMMMMMMMMM
// MMMMMMMMMMNOMMMMMMMQ.NMMMMMMMMMNQ;......;$NMMMMMMMMMM.?NMMMMMMN-MMMMMMMMMMM
// MMMMMMMMMMM.NMMH!MMMM.HMMMMM-.................MMMMMN.NMMMC:MMM.MMMMMMMMMMMM
// MMMMMMMMMMMNM$NMM.NMMMN.MO......................7M.HMMMM.MMMHMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMM-!MMN7..........................:MMMC;MMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMMMN.N..............................N.MMMMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMMMMH.............?HQHHQ?............CMMMMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMMMC..........!HQQQQQQQQQQQQ7.........:MMMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMMH.........CQQQQQQQQQQQQQQQQQO........OMMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMM.........HQQQQHQQQQQQQQQQHHQQH........MMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMM........................................MMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMM........;.......................-.......MMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMN>....?QQQQQQQQQQQ?:;...;>OQQQQQQQHQHQ:...;NMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMM.QQQQQQQQQQQ:..>NMNMMMMMMMMMMNN:..-HQQQQQQQHH>.MMMMMMMMMMMMMM
// MMMMMMMMMMMMMMH..7QQQQH.:MMMMMMMMMMMMMMMMMMMMMMMMN?.HQQQQQ;.NMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMM.......;MMMMN>...NMMMMMMMM7...NMMMM>......MMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMM........MMM-.......MMMMM!.......NMM;......MMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMM;.......MMQ.........MMMN.........MM.......MMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMM.......MM..........MMM:.........MN......HMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMM........NH.........MMMN.........M.......MMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMM........N!.......MMMMMC.......M.......QMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMM-........MNC..-MMM...NMM$..-NM........MMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMMM.........HMMMMMMH...$MMMMMMN........MMMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMMMM..........ONMMMNN:MMMMMMQ.........NMMMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMMMM$............;HMMMMMN;...........7NMMMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMMM.NMM.........NNNN;M!MMQM........NMN.MMMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMO.MMMM->......?O.-Q!MOO..NN.....?.MMMM;7MMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMC.7...NMMMM.MMMM!...MMNOMM.MMM;MM7.-NMMN.NMMMN...>.>MMMMMMMMMMMM
// MMMMMMMMMMM.MMMMMMMM.?MMMMMMMM.Q..NMM.MMM-.N.MMMMMMMQ.MMMNMMMM>NMMMMMMMMMMM
// MMMMMMMMMMN$MMMMMMM.NMMMMMMMMMN:MMMMMMMMMMM.MMMMMMMMMM;MMMMMMMM:MMMMMMMMMMM
// MMMMMMMMMMM7...MMMM.MMMMMMMMMMMM.MMMMMMMMO?MMMMMMMMMMM.MMMM...-MMMMMMMMMMMM
// MMMMMMMMMMMMMM;MMMM.MMMMMMMMMMMMMMH....;NMMMMMMMMMMMMM.MMMM.MMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMH.H>.QMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMN.>N-?MMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.26.0-devel
// 	protoc        v3.15.8
// source: strawhat.proto

package strawhat

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type Protocol int32

const (
	Protocol_Tcp Protocol = 0
	Protocol_Udp Protocol = 1
)

// Enum value maps for Protocol.
var (
	Protocol_name = map[int32]string{
		0: "Tcp",
		1: "Udp",
	}
	Protocol_value = map[string]int32{
		"Tcp": 0,
		"Udp": 1,
	}
)

func (x Protocol) Enum() *Protocol {
	p := new(Protocol)
	*p = x
	return p
}

func (x Protocol) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (Protocol) Descriptor() protoreflect.EnumDescriptor {
	return file_strawhat_proto_enumTypes[0].Descriptor()
}

func (Protocol) Type() protoreflect.EnumType {
	return &file_strawhat_proto_enumTypes[0]
}

func (x Protocol) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use Protocol.Descriptor instead.
func (Protocol) EnumDescriptor() ([]byte, []int) {
	return file_strawhat_proto_rawDescGZIP(), []int{0}
}

type Strawhat struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	ServiceID string   `protobuf:"bytes,1,opt,name=serviceID,proto3" json:"serviceID,omitempty"`
	Protocol  Protocol `protobuf:"varint,2,opt,name=protocol,proto3,enum=strawhat.Protocol" json:"protocol,omitempty"`
}

func (x *Strawhat) Reset() {
	*x = Strawhat{}
	if protoimpl.UnsafeEnabled {
		mi := &file_strawhat_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Strawhat) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Strawhat) ProtoMessage() {}

func (x *Strawhat) ProtoReflect() protoreflect.Message {
	mi := &file_strawhat_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Strawhat.ProtoReflect.Descriptor instead.
func (*Strawhat) Descriptor() ([]byte, []int) {
	return file_strawhat_proto_rawDescGZIP(), []int{0}
}

func (x *Strawhat) GetServiceID() string {
	if x != nil {
		return x.ServiceID
	}
	return ""
}

func (x *Strawhat) GetProtocol() Protocol {
	if x != nil {
		return x.Protocol
	}
	return Protocol_Tcp
}

var File_strawhat_proto protoreflect.FileDescriptor

var file_strawhat_proto_rawDesc = []byte{
	0x0a, 0x0e, 0x73, 0x74, 0x72, 0x61, 0x77, 0x68, 0x61, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x12, 0x08, 0x73, 0x74, 0x72, 0x61, 0x77, 0x68, 0x61, 0x74, 0x22, 0x58, 0x0a, 0x08, 0x53, 0x74,
	0x72, 0x61, 0x77, 0x68, 0x61, 0x74, 0x12, 0x1c, 0x0a, 0x09, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
	0x65, 0x49, 0x44, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x73, 0x65, 0x72, 0x76, 0x69,
	0x63, 0x65, 0x49, 0x44, 0x12, 0x2e, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c,
	0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x73, 0x74, 0x72, 0x61, 0x77, 0x68, 0x61,
	0x74, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x52, 0x08, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x63, 0x6f, 0x6c, 0x2a, 0x1c, 0x0a, 0x08, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c,
	0x12, 0x07, 0x0a, 0x03, 0x54, 0x63, 0x70, 0x10, 0x00, 0x12, 0x07, 0x0a, 0x03, 0x55, 0x64, 0x70,
	0x10, 0x01, 0x42, 0x0c, 0x5a, 0x0a, 0x2e, 0x2f, 0x73, 0x74, 0x72, 0x61, 0x77, 0x68, 0x61, 0x74,
	0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_strawhat_proto_rawDescOnce sync.Once
	file_strawhat_proto_rawDescData = file_strawhat_proto_rawDesc
)

func file_strawhat_proto_rawDescGZIP() []byte {
	file_strawhat_proto_rawDescOnce.Do(func() {
		file_strawhat_proto_rawDescData = protoimpl.X.CompressGZIP(file_strawhat_proto_rawDescData)
	})
	return file_strawhat_proto_rawDescData
}

var file_strawhat_proto_enumTypes = make([]protoimpl.EnumInfo, 1)
var file_strawhat_proto_msgTypes = make([]protoimpl.MessageInfo, 1)
var file_strawhat_proto_goTypes = []interface{}{
	(Protocol)(0),    // 0: strawhat.Protocol
	(*Strawhat)(nil), // 1: strawhat.Strawhat
}
var file_strawhat_proto_depIdxs = []int32{
	0, // 0: strawhat.Strawhat.protocol:type_name -> strawhat.Protocol
	1, // [1:1] is the sub-list for method output_type
	1, // [1:1] is the sub-list for method input_type
	1, // [1:1] is the sub-list for extension type_name
	1, // [1:1] is the sub-list for extension extendee
	0, // [0:1] is the sub-list for field type_name
}

func init() { file_strawhat_proto_init() }
func file_strawhat_proto_init() {
	if File_strawhat_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_strawhat_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Strawhat); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_strawhat_proto_rawDesc,
			NumEnums:      1,
			NumMessages:   1,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_strawhat_proto_goTypes,
		DependencyIndexes: file_strawhat_proto_depIdxs,
		EnumInfos:         file_strawhat_proto_enumTypes,
		MessageInfos:      file_strawhat_proto_msgTypes,
	}.Build()
	File_strawhat_proto = out.File
	file_strawhat_proto_rawDesc = nil
	file_strawhat_proto_goTypes = nil
	file_strawhat_proto_depIdxs = nil
}
