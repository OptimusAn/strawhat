// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Strawhat {
    // message fields
    pub serviceID: ::std::string::String,
    pub transport: Transport,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Strawhat {}

impl Strawhat {
    pub fn new() -> Strawhat {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Strawhat {
        static mut instance: ::protobuf::lazy::Lazy<Strawhat> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Strawhat,
        };
        unsafe {
            instance.get(Strawhat::new)
        }
    }

    // string serviceID = 1;

    pub fn clear_serviceID(&mut self) {
        self.serviceID.clear();
    }

    // Param is passed by value, moved
    pub fn set_serviceID(&mut self, v: ::std::string::String) {
        self.serviceID = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serviceID(&mut self) -> &mut ::std::string::String {
        &mut self.serviceID
    }

    // Take field
    pub fn take_serviceID(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.serviceID, ::std::string::String::new())
    }

    pub fn get_serviceID(&self) -> &str {
        &self.serviceID
    }

    fn get_serviceID_for_reflect(&self) -> &::std::string::String {
        &self.serviceID
    }

    fn mut_serviceID_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.serviceID
    }

    // .strawhat.Transport transport = 2;

    pub fn clear_transport(&mut self) {
        self.transport = Transport::Empty;
    }

    // Param is passed by value, moved
    pub fn set_transport(&mut self, v: Transport) {
        self.transport = v;
    }

    pub fn get_transport(&self) -> Transport {
        self.transport
    }

    fn get_transport_for_reflect(&self) -> &Transport {
        &self.transport
    }

    fn mut_transport_for_reflect(&mut self) -> &mut Transport {
        &mut self.transport
    }
}

impl ::protobuf::Message for Strawhat {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.serviceID)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.transport, 2, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.serviceID.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.serviceID);
        }
        if self.transport != Transport::Empty {
            my_size += ::protobuf::rt::enum_size(2, self.transport);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.serviceID.is_empty() {
            os.write_string(1, &self.serviceID)?;
        }
        if self.transport != Transport::Empty {
            os.write_enum(2, self.transport.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Strawhat {
    fn new() -> Strawhat {
        Strawhat::new()
    }

    fn descriptor_static(_: ::std::option::Option<Strawhat>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "serviceID",
                    Strawhat::get_serviceID_for_reflect,
                    Strawhat::mut_serviceID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Transport>>(
                    "transport",
                    Strawhat::get_transport_for_reflect,
                    Strawhat::mut_transport_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Strawhat>(
                    "Strawhat",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Strawhat {
    fn clear(&mut self) {
        self.clear_serviceID();
        self.clear_transport();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Strawhat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Strawhat {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Transport {
    Empty = 0,
    Tcp = 1,
    Udp = 3,
}

impl ::protobuf::ProtobufEnum for Transport {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Transport> {
        match value {
            0 => ::std::option::Option::Some(Transport::Empty),
            1 => ::std::option::Option::Some(Transport::Tcp),
            3 => ::std::option::Option::Some(Transport::Udp),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Transport] = &[
            Transport::Empty,
            Transport::Tcp,
            Transport::Udp,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Transport>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Transport", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Transport {
}

impl ::std::default::Default for Transport {
    fn default() -> Self {
        Transport::Empty
    }
}

impl ::protobuf::reflect::ProtobufValue for Transport {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0estrawhat.proto\x12\x08strawhat\"[\n\x08Strawhat\x12\x1c\n\tservice\
    ID\x18\x01\x20\x01(\tR\tserviceID\x121\n\ttransport\x18\x02\x20\x01(\x0e\
    2\x13.strawhat.TransportR\ttransport*(\n\tTransport\x12\t\n\x05Empty\x10\
    \0\x12\x07\n\x03Tcp\x10\x01\x12\x07\n\x03Udp\x10\x03B\x0cZ\n./strawhatJ\
    \xd6\x1a\n\x06\x12\x04)\07\x01\n\x93\x18\n\x01\x0c\x12\x03)\0\x122\x88\
    \x18\x20MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM\
    MMMMMMMMMMM\n\x20MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM\
    MMMMMMMMMMMMMMMMMMMM\n\x20MMMMMMMMMMMMMMM;..HMMMMMMMMMMMMMMMMMMMMMMMMMMM\
    MMMMMMMMM$...NMMMMMMMMMMMMMMM\n\x20MMMMMMMMMMMMMM;NMMN.MMMMMMMMMMMMMMMMM\
    MMMMMMMMMMMMMMMMM.MMMM.MMMMMMMMMMMMMMM\n\x20MMMMMMMMMMMMMN.MMMM;MMMMMMMM\
    MMMMMMMMMMMMMMMMMMMMMMMMMM.MMMM.?NMMMMMMMMMMMMM\n\x20MMMMMMMMMMM.NMMMMMM\
    .MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM.MMMMMMM?CMMMMMMMMMMM\n\x20MMMMMMMMMM\
    NOMMMMMMMQ.NMMMMMMMMMNQ;......;$NMMMMMMMMMM.?NMMMMMMN-MMMMMMMMMMM\n\x20M\
    MMMMMMMMMM.NMMH!MMMM.HMMMMM-.................MMMMMN.NMMMC:MMM.MMMMMMMMMM\
    MM\n\x20MMMMMMMMMMMNM$NMM.NMMMN.MO......................7M.HMMMM.MMMHMMM\
    MMMMMMMMMMM\n\x20MMMMMMMMMMMMMMMMMM-!MMN7..........................:MMMC\
    ;MMMMMMMMMMMMMMMMMMM\n\x20MMMMMMMMMMMMMMMMMMMN.N........................\
    ......N.MMMMMMMMMMMMMMMMMMMMM\n\x20MMMMMMMMMMMMMMMMMMMMH.............?HQ\
    HHQ?............CMMMMMMMMMMMMMMMMMMMMM\n\x20MMMMMMMMMMMMMMMMMMMC........\
    ..!HQQQQQQQQQQQQ7.........:MMMMMMMMMMMMMMMMMMMM\n\x20MMMMMMMMMMMMMMMMMMH\
    .........CQQQQQQQQQQQQQQQQQO........OMMMMMMMMMMMMMMMMMMM\n\x20MMMMMMMMMM\
    MMMMMMMM.........HQQQQHQQQQQQQQQQHHQQH........MMMMMMMMMMMMMMMMMMM\n\x20M\
    MMMMMMMMMMMMMMMM........................................MMMMMMMMMMMMMMMM\
    MM\n\x20MMMMMMMMMMMMMMMMM........;.......................-.......MMMMMMM\
    MMMMMMMMMMM\n\x20MMMMMMMMMMMMMMMN>....?QQQQQQQQQQQ?:;...;>OQQQQQQQHQHQ:.\
    ..;NMMMMMMMMMMMMMMMM\n\x20MMMMMMMMMMMMMM.QQQQQQQQQQQ:..>NMNMMMMMMMMMMNN:\
    ..-HQQQQQQQHH>.MMMMMMMMMMMMMM\n\x20MMMMMMMMMMMMMMH..7QQQQH.:MMMMMMMMMMMM\
    MMMMMMMMMMMMN?.HQQQQQ;.NMMMMMMMMMMMMMM\n\x20MMMMMMMMMMMMMMMM.......;MMMM\
    N>...NMMMMMMMM7...NMMMM>......MMMMMMMMMMMMMMMMM\n\x20MMMMMMMMMMMMMMMM...\
    .....MMM-.......MMMMM!.......NMM;......MMMMMMMMMMMMMMMMM\n\x20MMMMMMMMMM\
    MMMMMM;.......MMQ.........MMMN.........MM.......MMMMMMMMMMMMMMMMM\n\x20M\
    MMMMMMMMMMMMMMMM.......MM..........MMM:.........MN......HMMMMMMMMMMMMMMM\
    MM\n\x20MMMMMMMMMMMMMMMMM........NH.........MMMN.........M.......MMMMMMM\
    MMMMMMMMMMM\n\x20MMMMMMMMMMMMMMMMMM........N!.......MMMMMC.......M......\
    .QMMMMMMMMMMMMMMMMMM\n\x20MMMMMMMMMMMMMMMMMM-........MNC..-MMM...NMM$..-\
    NM........MMMMMMMMMMMMMMMMMMM\n\x20MMMMMMMMMMMMMMMMMMM.........HMMMMMMH.\
    ..$MMMMMMN........MMMMMMMMMMMMMMMMMMMM\n\x20MMMMMMMMMMMMMMMMMMMM........\
    ..ONMMMNN:MMMMMMQ.........NMMMMMMMMMMMMMMMMMMMM\n\x20MMMMMMMMMMMMMMMMMMM\
    M$............;HMMMMMN;...........7NMMMMMMMMMMMMMMMMMMMM\n\x20MMMMMMMMMM\
    MMMMMMMMM.NMM.........NNNN;M!MMQM........NMN.MMMMMMMMMMMMMMMMMMMM\n\x20M\
    MMMMMMMMMMMMMMMMO.MMMM->......?O.-Q!MOO..NN.....?.MMMM;7MMMMMMMMMMMMMMMM\
    MM\n\x20MMMMMMMMMMMC.7...NMMMM.MMMM!...MMNOMM.MMM;MM7.-NMMN.NMMMN...>.>M\
    MMMMMMMMMMM\n\x20MMMMMMMMMMM.MMMMMMMM.?MMMMMMMM.Q..NMM.MMM-.N.MMMMMMMQ.M\
    MMNMMMM>NMMMMMMMMMMM\n\x20MMMMMMMMMMN$MMMMMMM.NMMMMMMMMMN:MMMMMMMMMMM.MM\
    MMMMMMMM;MMMMMMMM:MMMMMMMMMMM\n\x20MMMMMMMMMMM7...MMMM.MMMMMMMMMMMM.MMMM\
    MMMMO?MMMMMMMMMMM.MMMM...-MMMMMMMMMMMM\n\x20MMMMMMMMMMMMMM;MMMM.MMMMMMMM\
    MMMMMMH....;NMMMMMMMMMMMMM.MMMM.MMMMMMMMMMMMMMM\n\x20MMMMMMMMMMMMMMH.H>.\
    QMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMN.>N-?MMMMMMMMMMMMMMM\n\x20MMMMMMMMMM\
    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM\n\x20M\
    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM\
    MM\n\n\x08\n\x01\x02\x12\x03+\0\x11\n\x08\n\x01\x08\x12\x03,\0!\n\t\n\
    \x02\x08\x0b\x12\x03,\0!\n\n\n\x02\x05\0\x12\x04.\02\x01\n\n\n\x03\x05\0\
    \x01\x12\x03.\x05\x0e\n\x0b\n\x04\x05\0\x02\0\x12\x03/\x08\x12\n\x0c\n\
    \x05\x05\0\x02\0\x01\x12\x03/\x08\r\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03/\
    \x10\x11\n\x0b\n\x04\x05\0\x02\x01\x12\x030\x08\x12\n\x0c\n\x05\x05\0\
    \x02\x01\x01\x12\x030\x08\x0b\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x030\x10\
    \x11\n\x0b\n\x04\x05\0\x02\x02\x12\x031\x08\x12\n\x0c\n\x05\x05\0\x02\
    \x02\x01\x12\x031\x08\x0b\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x031\x10\x11\
    \n\n\n\x02\x04\0\x12\x044\07\x01\n\n\n\x03\x04\0\x01\x12\x034\x08\x10\n\
    \x0b\n\x04\x04\0\x02\0\x12\x035\x08\x20\n\x0c\n\x05\x04\0\x02\0\x05\x12\
    \x035\x08\x0e\n\x0c\n\x05\x04\0\x02\0\x01\x12\x035\x12\x1b\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x035\x1e\x1f\n\x0b\n\x04\x04\0\x02\x01\x12\x036\x08\
    !\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x036\x08\x11\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x036\x12\x1b\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x036\x1f\x20\
    b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
