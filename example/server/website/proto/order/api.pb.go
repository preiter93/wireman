// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.35.2
// 	protoc        v5.26.1
// source: order/api.proto

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

type ListOrdersReq struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *ListOrdersReq) Reset() {
	*x = ListOrdersReq{}
	mi := &file_order_api_proto_msgTypes[0]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *ListOrdersReq) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ListOrdersReq) ProtoMessage() {}

func (x *ListOrdersReq) ProtoReflect() protoreflect.Message {
	mi := &file_order_api_proto_msgTypes[0]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ListOrdersReq.ProtoReflect.Descriptor instead.
func (*ListOrdersReq) Descriptor() ([]byte, []int) {
	return file_order_api_proto_rawDescGZIP(), []int{0}
}

type ListOrdersResp struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Order []*Order `protobuf:"bytes,1,rep,name=order,proto3" json:"order,omitempty"`
}

func (x *ListOrdersResp) Reset() {
	*x = ListOrdersResp{}
	mi := &file_order_api_proto_msgTypes[1]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *ListOrdersResp) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ListOrdersResp) ProtoMessage() {}

func (x *ListOrdersResp) ProtoReflect() protoreflect.Message {
	mi := &file_order_api_proto_msgTypes[1]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ListOrdersResp.ProtoReflect.Descriptor instead.
func (*ListOrdersResp) Descriptor() ([]byte, []int) {
	return file_order_api_proto_rawDescGZIP(), []int{1}
}

func (x *ListOrdersResp) GetOrder() []*Order {
	if x != nil {
		return x.Order
	}
	return nil
}

type GetOrderReq struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Id string `protobuf:"bytes,1,opt,name=id,proto3" json:"id,omitempty"`
}

func (x *GetOrderReq) Reset() {
	*x = GetOrderReq{}
	mi := &file_order_api_proto_msgTypes[2]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *GetOrderReq) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetOrderReq) ProtoMessage() {}

func (x *GetOrderReq) ProtoReflect() protoreflect.Message {
	mi := &file_order_api_proto_msgTypes[2]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetOrderReq.ProtoReflect.Descriptor instead.
func (*GetOrderReq) Descriptor() ([]byte, []int) {
	return file_order_api_proto_rawDescGZIP(), []int{2}
}

func (x *GetOrderReq) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

type GetOrderResp struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Order *Order `protobuf:"bytes,1,opt,name=order,proto3" json:"order,omitempty"`
}

func (x *GetOrderResp) Reset() {
	*x = GetOrderResp{}
	mi := &file_order_api_proto_msgTypes[3]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *GetOrderResp) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetOrderResp) ProtoMessage() {}

func (x *GetOrderResp) ProtoReflect() protoreflect.Message {
	mi := &file_order_api_proto_msgTypes[3]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetOrderResp.ProtoReflect.Descriptor instead.
func (*GetOrderResp) Descriptor() ([]byte, []int) {
	return file_order_api_proto_rawDescGZIP(), []int{3}
}

func (x *GetOrderResp) GetOrder() *Order {
	if x != nil {
		return x.Order
	}
	return nil
}

type UpdateOrderReq struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Id   string `protobuf:"bytes,1,opt,name=id,proto3" json:"id,omitempty"`
	Name string `protobuf:"bytes,2,opt,name=name,proto3" json:"name,omitempty"`
}

func (x *UpdateOrderReq) Reset() {
	*x = UpdateOrderReq{}
	mi := &file_order_api_proto_msgTypes[4]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *UpdateOrderReq) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UpdateOrderReq) ProtoMessage() {}

func (x *UpdateOrderReq) ProtoReflect() protoreflect.Message {
	mi := &file_order_api_proto_msgTypes[4]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UpdateOrderReq.ProtoReflect.Descriptor instead.
func (*UpdateOrderReq) Descriptor() ([]byte, []int) {
	return file_order_api_proto_rawDescGZIP(), []int{4}
}

func (x *UpdateOrderReq) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *UpdateOrderReq) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

type UpdateOrderResp struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Order *Order `protobuf:"bytes,1,opt,name=order,proto3" json:"order,omitempty"`
}

func (x *UpdateOrderResp) Reset() {
	*x = UpdateOrderResp{}
	mi := &file_order_api_proto_msgTypes[5]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *UpdateOrderResp) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UpdateOrderResp) ProtoMessage() {}

func (x *UpdateOrderResp) ProtoReflect() protoreflect.Message {
	mi := &file_order_api_proto_msgTypes[5]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UpdateOrderResp.ProtoReflect.Descriptor instead.
func (*UpdateOrderResp) Descriptor() ([]byte, []int) {
	return file_order_api_proto_rawDescGZIP(), []int{5}
}

func (x *UpdateOrderResp) GetOrder() *Order {
	if x != nil {
		return x.Order
	}
	return nil
}

type DeleteOrderReq struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Id string `protobuf:"bytes,1,opt,name=id,proto3" json:"id,omitempty"`
}

func (x *DeleteOrderReq) Reset() {
	*x = DeleteOrderReq{}
	mi := &file_order_api_proto_msgTypes[6]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *DeleteOrderReq) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DeleteOrderReq) ProtoMessage() {}

func (x *DeleteOrderReq) ProtoReflect() protoreflect.Message {
	mi := &file_order_api_proto_msgTypes[6]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use DeleteOrderReq.ProtoReflect.Descriptor instead.
func (*DeleteOrderReq) Descriptor() ([]byte, []int) {
	return file_order_api_proto_rawDescGZIP(), []int{6}
}

func (x *DeleteOrderReq) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

type DeleteOrderResp struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *DeleteOrderResp) Reset() {
	*x = DeleteOrderResp{}
	mi := &file_order_api_proto_msgTypes[7]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *DeleteOrderResp) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DeleteOrderResp) ProtoMessage() {}

func (x *DeleteOrderResp) ProtoReflect() protoreflect.Message {
	mi := &file_order_api_proto_msgTypes[7]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use DeleteOrderResp.ProtoReflect.Descriptor instead.
func (*DeleteOrderResp) Descriptor() ([]byte, []int) {
	return file_order_api_proto_rawDescGZIP(), []int{7}
}

type Order struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Id        string                 `protobuf:"bytes,1,opt,name=id,proto3" json:"id,omitempty"`
	Name      string                 `protobuf:"bytes,2,opt,name=name,proto3" json:"name,omitempty"`
	Status    string                 `protobuf:"bytes,3,opt,name=status,proto3" json:"status,omitempty"`
	CreatedAt *timestamppb.Timestamp `protobuf:"bytes,4,opt,name=created_at,json=createdAt,proto3" json:"created_at,omitempty"`
	UpdatedAt *timestamppb.Timestamp `protobuf:"bytes,5,opt,name=updated_at,json=updatedAt,proto3" json:"updated_at,omitempty"`
}

func (x *Order) Reset() {
	*x = Order{}
	mi := &file_order_api_proto_msgTypes[8]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *Order) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Order) ProtoMessage() {}

func (x *Order) ProtoReflect() protoreflect.Message {
	mi := &file_order_api_proto_msgTypes[8]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Order.ProtoReflect.Descriptor instead.
func (*Order) Descriptor() ([]byte, []int) {
	return file_order_api_proto_rawDescGZIP(), []int{8}
}

func (x *Order) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *Order) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Order) GetStatus() string {
	if x != nil {
		return x.Status
	}
	return ""
}

func (x *Order) GetCreatedAt() *timestamppb.Timestamp {
	if x != nil {
		return x.CreatedAt
	}
	return nil
}

func (x *Order) GetUpdatedAt() *timestamppb.Timestamp {
	if x != nil {
		return x.UpdatedAt
	}
	return nil
}

var File_order_api_proto protoreflect.FileDescriptor

var file_order_api_proto_rawDesc = []byte{
	0x0a, 0x0f, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x2f, 0x61, 0x70, 0x69, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x12, 0x05, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
	0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
	0x61, 0x6d, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x0f, 0x0a, 0x0d, 0x4c, 0x69, 0x73,
	0x74, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x73, 0x52, 0x65, 0x71, 0x22, 0x34, 0x0a, 0x0e, 0x4c, 0x69,
	0x73, 0x74, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x73, 0x52, 0x65, 0x73, 0x70, 0x12, 0x22, 0x0a, 0x05,
	0x6f, 0x72, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x2e, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x52, 0x05, 0x6f, 0x72, 0x64, 0x65, 0x72,
	0x22, 0x1d, 0x0a, 0x0b, 0x47, 0x65, 0x74, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x52, 0x65, 0x71, 0x12,
	0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x22,
	0x32, 0x0a, 0x0c, 0x47, 0x65, 0x74, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x12,
	0x22, 0x0a, 0x05, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x52, 0x05, 0x6f, 0x72,
	0x64, 0x65, 0x72, 0x22, 0x34, 0x0a, 0x0e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x4f, 0x72, 0x64,
	0x65, 0x72, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x35, 0x0a, 0x0f, 0x55, 0x70, 0x64,
	0x61, 0x74, 0x65, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x12, 0x22, 0x0a, 0x05,
	0x6f, 0x72, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x2e, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x52, 0x05, 0x6f, 0x72, 0x64, 0x65, 0x72,
	0x22, 0x20, 0x0a, 0x0e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x52,
	0x65, 0x71, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02,
	0x69, 0x64, 0x22, 0x11, 0x0a, 0x0f, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x4f, 0x72, 0x64, 0x65,
	0x72, 0x52, 0x65, 0x73, 0x70, 0x22, 0xb9, 0x01, 0x0a, 0x05, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x12,
	0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12,
	0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
	0x61, 0x6d, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x03, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x39, 0x0a, 0x0a, 0x63,
	0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
	0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x52, 0x09, 0x63, 0x72, 0x65,
	0x61, 0x74, 0x65, 0x64, 0x41, 0x74, 0x12, 0x39, 0x0a, 0x0a, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65,
	0x64, 0x5f, 0x61, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f,
	0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d,
	0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x52, 0x09, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x41,
	0x74, 0x32, 0x82, 0x02, 0x0a, 0x0c, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x53, 0x65, 0x72, 0x76, 0x69,
	0x63, 0x65, 0x12, 0x35, 0x0a, 0x08, 0x47, 0x65, 0x74, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x12, 0x12,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x47, 0x65, 0x74, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x52,
	0x65, 0x71, 0x1a, 0x13, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x47, 0x65, 0x74, 0x4f, 0x72,
	0x64, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x22, 0x00, 0x12, 0x3b, 0x0a, 0x0a, 0x4c, 0x69, 0x73,
	0x74, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x73, 0x12, 0x14, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
	0x4c, 0x69, 0x73, 0x74, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x73, 0x52, 0x65, 0x71, 0x1a, 0x15, 0x2e,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x73,
	0x52, 0x65, 0x73, 0x70, 0x22, 0x00, 0x12, 0x3e, 0x0a, 0x0b, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
	0x4f, 0x72, 0x64, 0x65, 0x72, 0x12, 0x15, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x55, 0x70,
	0x64, 0x61, 0x74, 0x65, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x52, 0x65, 0x71, 0x1a, 0x16, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x4f, 0x72, 0x64, 0x65, 0x72,
	0x52, 0x65, 0x73, 0x70, 0x22, 0x00, 0x12, 0x3e, 0x0a, 0x0b, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65,
	0x4f, 0x72, 0x64, 0x65, 0x72, 0x12, 0x15, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x44, 0x65,
	0x6c, 0x65, 0x74, 0x65, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x52, 0x65, 0x71, 0x1a, 0x16, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x4f, 0x72, 0x64, 0x65, 0x72,
	0x52, 0x65, 0x73, 0x70, 0x22, 0x00, 0x42, 0x09, 0x5a, 0x07, 0x2e, 0x2f, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_order_api_proto_rawDescOnce sync.Once
	file_order_api_proto_rawDescData = file_order_api_proto_rawDesc
)

func file_order_api_proto_rawDescGZIP() []byte {
	file_order_api_proto_rawDescOnce.Do(func() {
		file_order_api_proto_rawDescData = protoimpl.X.CompressGZIP(file_order_api_proto_rawDescData)
	})
	return file_order_api_proto_rawDescData
}

var file_order_api_proto_msgTypes = make([]protoimpl.MessageInfo, 9)
var file_order_api_proto_goTypes = []any{
	(*ListOrdersReq)(nil),         // 0: proto.ListOrdersReq
	(*ListOrdersResp)(nil),        // 1: proto.ListOrdersResp
	(*GetOrderReq)(nil),           // 2: proto.GetOrderReq
	(*GetOrderResp)(nil),          // 3: proto.GetOrderResp
	(*UpdateOrderReq)(nil),        // 4: proto.UpdateOrderReq
	(*UpdateOrderResp)(nil),       // 5: proto.UpdateOrderResp
	(*DeleteOrderReq)(nil),        // 6: proto.DeleteOrderReq
	(*DeleteOrderResp)(nil),       // 7: proto.DeleteOrderResp
	(*Order)(nil),                 // 8: proto.Order
	(*timestamppb.Timestamp)(nil), // 9: google.protobuf.Timestamp
}
var file_order_api_proto_depIdxs = []int32{
	8, // 0: proto.ListOrdersResp.order:type_name -> proto.Order
	8, // 1: proto.GetOrderResp.order:type_name -> proto.Order
	8, // 2: proto.UpdateOrderResp.order:type_name -> proto.Order
	9, // 3: proto.Order.created_at:type_name -> google.protobuf.Timestamp
	9, // 4: proto.Order.updated_at:type_name -> google.protobuf.Timestamp
	2, // 5: proto.OrderService.GetOrder:input_type -> proto.GetOrderReq
	0, // 6: proto.OrderService.ListOrders:input_type -> proto.ListOrdersReq
	4, // 7: proto.OrderService.UpdateOrder:input_type -> proto.UpdateOrderReq
	6, // 8: proto.OrderService.DeleteOrder:input_type -> proto.DeleteOrderReq
	3, // 9: proto.OrderService.GetOrder:output_type -> proto.GetOrderResp
	1, // 10: proto.OrderService.ListOrders:output_type -> proto.ListOrdersResp
	5, // 11: proto.OrderService.UpdateOrder:output_type -> proto.UpdateOrderResp
	7, // 12: proto.OrderService.DeleteOrder:output_type -> proto.DeleteOrderResp
	9, // [9:13] is the sub-list for method output_type
	5, // [5:9] is the sub-list for method input_type
	5, // [5:5] is the sub-list for extension type_name
	5, // [5:5] is the sub-list for extension extendee
	0, // [0:5] is the sub-list for field type_name
}

func init() { file_order_api_proto_init() }
func file_order_api_proto_init() {
	if File_order_api_proto != nil {
		return
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_order_api_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   9,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_order_api_proto_goTypes,
		DependencyIndexes: file_order_api_proto_depIdxs,
		MessageInfos:      file_order_api_proto_msgTypes,
	}.Build()
	File_order_api_proto = out.File
	file_order_api_proto_rawDesc = nil
	file_order_api_proto_goTypes = nil
	file_order_api_proto_depIdxs = nil
}
