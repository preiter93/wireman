// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.30.0
// 	protoc        v3.21.5
// source: timekeeper.proto

package proto

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	timestamppb "google.golang.org/protobuf/types/known/timestamppb"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type GetDateReq struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *GetDateReq) Reset() {
	*x = GetDateReq{}
	if protoimpl.UnsafeEnabled {
		mi := &file_timekeeper_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetDateReq) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetDateReq) ProtoMessage() {}

func (x *GetDateReq) ProtoReflect() protoreflect.Message {
	mi := &file_timekeeper_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetDateReq.ProtoReflect.Descriptor instead.
func (*GetDateReq) Descriptor() ([]byte, []int) {
	return file_timekeeper_proto_rawDescGZIP(), []int{0}
}

type GetDateResp struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Date *timestamppb.Timestamp `protobuf:"bytes,1,opt,name=date,proto3" json:"date,omitempty"`
}

func (x *GetDateResp) Reset() {
	*x = GetDateResp{}
	if protoimpl.UnsafeEnabled {
		mi := &file_timekeeper_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetDateResp) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetDateResp) ProtoMessage() {}

func (x *GetDateResp) ProtoReflect() protoreflect.Message {
	mi := &file_timekeeper_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetDateResp.ProtoReflect.Descriptor instead.
func (*GetDateResp) Descriptor() ([]byte, []int) {
	return file_timekeeper_proto_rawDescGZIP(), []int{1}
}

func (x *GetDateResp) GetDate() *timestamppb.Timestamp {
	if x != nil {
		return x.Date
	}
	return nil
}

type GetNameOfMonthReq struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Number int32 `protobuf:"varint,1,opt,name=number,proto3" json:"number,omitempty"`
}

func (x *GetNameOfMonthReq) Reset() {
	*x = GetNameOfMonthReq{}
	if protoimpl.UnsafeEnabled {
		mi := &file_timekeeper_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetNameOfMonthReq) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetNameOfMonthReq) ProtoMessage() {}

func (x *GetNameOfMonthReq) ProtoReflect() protoreflect.Message {
	mi := &file_timekeeper_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetNameOfMonthReq.ProtoReflect.Descriptor instead.
func (*GetNameOfMonthReq) Descriptor() ([]byte, []int) {
	return file_timekeeper_proto_rawDescGZIP(), []int{2}
}

func (x *GetNameOfMonthReq) GetNumber() int32 {
	if x != nil {
		return x.Number
	}
	return 0
}

type GetNameOfMonthResp struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Name string `protobuf:"bytes,1,opt,name=name,proto3" json:"name,omitempty"`
}

func (x *GetNameOfMonthResp) Reset() {
	*x = GetNameOfMonthResp{}
	if protoimpl.UnsafeEnabled {
		mi := &file_timekeeper_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetNameOfMonthResp) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetNameOfMonthResp) ProtoMessage() {}

func (x *GetNameOfMonthResp) ProtoReflect() protoreflect.Message {
	mi := &file_timekeeper_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetNameOfMonthResp.ProtoReflect.Descriptor instead.
func (*GetNameOfMonthResp) Descriptor() ([]byte, []int) {
	return file_timekeeper_proto_rawDescGZIP(), []int{3}
}

func (x *GetNameOfMonthResp) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

var File_timekeeper_proto protoreflect.FileDescriptor

var file_timekeeper_proto_rawDesc = []byte{
	0x0a, 0x10, 0x74, 0x69, 0x6d, 0x65, 0x6b, 0x65, 0x65, 0x70, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x12, 0x05, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
	0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x74, 0x69, 0x6d, 0x65, 0x73,
	0x74, 0x61, 0x6d, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x0c, 0x0a, 0x0a, 0x47, 0x65,
	0x74, 0x44, 0x61, 0x74, 0x65, 0x52, 0x65, 0x71, 0x22, 0x3d, 0x0a, 0x0b, 0x47, 0x65, 0x74, 0x44,
	0x61, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x12, 0x2e, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x65, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d,
	0x70, 0x52, 0x04, 0x64, 0x61, 0x74, 0x65, 0x22, 0x2b, 0x0a, 0x11, 0x47, 0x65, 0x74, 0x4e, 0x61,
	0x6d, 0x65, 0x4f, 0x66, 0x4d, 0x6f, 0x6e, 0x74, 0x68, 0x52, 0x65, 0x71, 0x12, 0x16, 0x0a, 0x06,
	0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x6e, 0x75,
	0x6d, 0x62, 0x65, 0x72, 0x22, 0x28, 0x0a, 0x12, 0x47, 0x65, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x4f,
	0x66, 0x4d, 0x6f, 0x6e, 0x74, 0x68, 0x52, 0x65, 0x73, 0x70, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61,
	0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x32, 0x89,
	0x01, 0x0a, 0x0a, 0x54, 0x69, 0x6d, 0x65, 0x6b, 0x65, 0x65, 0x70, 0x65, 0x72, 0x12, 0x32, 0x0a,
	0x07, 0x47, 0x65, 0x74, 0x44, 0x61, 0x74, 0x65, 0x12, 0x11, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x2e, 0x47, 0x65, 0x74, 0x44, 0x61, 0x74, 0x65, 0x52, 0x65, 0x71, 0x1a, 0x12, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x2e, 0x47, 0x65, 0x74, 0x44, 0x61, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x22,
	0x00, 0x12, 0x47, 0x0a, 0x0e, 0x47, 0x65, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x4f, 0x66, 0x4d, 0x6f,
	0x6e, 0x74, 0x68, 0x12, 0x18, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x47, 0x65, 0x74, 0x4e,
	0x61, 0x6d, 0x65, 0x4f, 0x66, 0x4d, 0x6f, 0x6e, 0x74, 0x68, 0x52, 0x65, 0x71, 0x1a, 0x19, 0x2e,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x47, 0x65, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x4f, 0x66, 0x4d,
	0x6f, 0x6e, 0x74, 0x68, 0x52, 0x65, 0x73, 0x70, 0x22, 0x00, 0x42, 0x09, 0x5a, 0x07, 0x2e, 0x2f,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_timekeeper_proto_rawDescOnce sync.Once
	file_timekeeper_proto_rawDescData = file_timekeeper_proto_rawDesc
)

func file_timekeeper_proto_rawDescGZIP() []byte {
	file_timekeeper_proto_rawDescOnce.Do(func() {
		file_timekeeper_proto_rawDescData = protoimpl.X.CompressGZIP(file_timekeeper_proto_rawDescData)
	})
	return file_timekeeper_proto_rawDescData
}

var file_timekeeper_proto_msgTypes = make([]protoimpl.MessageInfo, 4)
var file_timekeeper_proto_goTypes = []interface{}{
	(*GetDateReq)(nil),            // 0: proto.GetDateReq
	(*GetDateResp)(nil),           // 1: proto.GetDateResp
	(*GetNameOfMonthReq)(nil),     // 2: proto.GetNameOfMonthReq
	(*GetNameOfMonthResp)(nil),    // 3: proto.GetNameOfMonthResp
	(*timestamppb.Timestamp)(nil), // 4: google.protobuf.Timestamp
}
var file_timekeeper_proto_depIdxs = []int32{
	4, // 0: proto.GetDateResp.date:type_name -> google.protobuf.Timestamp
	0, // 1: proto.Timekeeper.GetDate:input_type -> proto.GetDateReq
	2, // 2: proto.Timekeeper.GetNameOfMonth:input_type -> proto.GetNameOfMonthReq
	1, // 3: proto.Timekeeper.GetDate:output_type -> proto.GetDateResp
	3, // 4: proto.Timekeeper.GetNameOfMonth:output_type -> proto.GetNameOfMonthResp
	3, // [3:5] is the sub-list for method output_type
	1, // [1:3] is the sub-list for method input_type
	1, // [1:1] is the sub-list for extension type_name
	1, // [1:1] is the sub-list for extension extendee
	0, // [0:1] is the sub-list for field type_name
}

func init() { file_timekeeper_proto_init() }
func file_timekeeper_proto_init() {
	if File_timekeeper_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_timekeeper_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetDateReq); i {
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
		file_timekeeper_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetDateResp); i {
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
		file_timekeeper_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetNameOfMonthReq); i {
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
		file_timekeeper_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetNameOfMonthResp); i {
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
			RawDescriptor: file_timekeeper_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   4,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_timekeeper_proto_goTypes,
		DependencyIndexes: file_timekeeper_proto_depIdxs,
		MessageInfos:      file_timekeeper_proto_msgTypes,
	}.Build()
	File_timekeeper_proto = out.File
	file_timekeeper_proto_rawDesc = nil
	file_timekeeper_proto_goTypes = nil
	file_timekeeper_proto_depIdxs = nil
}
