// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `task.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:scyna.StartTaskRequest)
pub struct StartTaskRequest {
    // message fields
    // @@protoc_insertion_point(field:scyna.StartTaskRequest.Module)
    pub Module: ::std::string::String,
    // @@protoc_insertion_point(field:scyna.StartTaskRequest.Topic)
    pub Topic: ::std::string::String,
    // @@protoc_insertion_point(field:scyna.StartTaskRequest.Data)
    pub Data: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:scyna.StartTaskRequest.Time)
    pub Time: i64,
    // @@protoc_insertion_point(field:scyna.StartTaskRequest.Interval)
    pub Interval: i64,
    // @@protoc_insertion_point(field:scyna.StartTaskRequest.Loop)
    pub Loop: u64,
    // special fields
    // @@protoc_insertion_point(special_field:scyna.StartTaskRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartTaskRequest {
    fn default() -> &'a StartTaskRequest {
        <StartTaskRequest as ::protobuf::Message>::default_instance()
    }
}

impl StartTaskRequest {
    pub fn new() -> StartTaskRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "Module",
            |m: &StartTaskRequest| { &m.Module },
            |m: &mut StartTaskRequest| { &mut m.Module },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "Topic",
            |m: &StartTaskRequest| { &m.Topic },
            |m: &mut StartTaskRequest| { &mut m.Topic },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "Data",
            |m: &StartTaskRequest| { &m.Data },
            |m: &mut StartTaskRequest| { &mut m.Data },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "Time",
            |m: &StartTaskRequest| { &m.Time },
            |m: &mut StartTaskRequest| { &mut m.Time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "Interval",
            |m: &StartTaskRequest| { &m.Interval },
            |m: &mut StartTaskRequest| { &mut m.Interval },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "Loop",
            |m: &StartTaskRequest| { &m.Loop },
            |m: &mut StartTaskRequest| { &mut m.Loop },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartTaskRequest>(
            "StartTaskRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartTaskRequest {
    const NAME: &'static str = "StartTaskRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.Module = is.read_string()?;
                },
                18 => {
                    self.Topic = is.read_string()?;
                },
                26 => {
                    self.Data = is.read_bytes()?;
                },
                32 => {
                    self.Time = is.read_int64()?;
                },
                40 => {
                    self.Interval = is.read_int64()?;
                },
                48 => {
                    self.Loop = is.read_uint64()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.Module.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.Module);
        }
        if !self.Topic.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.Topic);
        }
        if !self.Data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.Data);
        }
        if self.Time != 0 {
            my_size += ::protobuf::rt::int64_size(4, self.Time);
        }
        if self.Interval != 0 {
            my_size += ::protobuf::rt::int64_size(5, self.Interval);
        }
        if self.Loop != 0 {
            my_size += ::protobuf::rt::uint64_size(6, self.Loop);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.Module.is_empty() {
            os.write_string(1, &self.Module)?;
        }
        if !self.Topic.is_empty() {
            os.write_string(2, &self.Topic)?;
        }
        if !self.Data.is_empty() {
            os.write_bytes(3, &self.Data)?;
        }
        if self.Time != 0 {
            os.write_int64(4, self.Time)?;
        }
        if self.Interval != 0 {
            os.write_int64(5, self.Interval)?;
        }
        if self.Loop != 0 {
            os.write_uint64(6, self.Loop)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> StartTaskRequest {
        StartTaskRequest::new()
    }

    fn clear(&mut self) {
        self.Module.clear();
        self.Topic.clear();
        self.Data.clear();
        self.Time = 0;
        self.Interval = 0;
        self.Loop = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartTaskRequest {
        static instance: StartTaskRequest = StartTaskRequest {
            Module: ::std::string::String::new(),
            Topic: ::std::string::String::new(),
            Data: ::std::vec::Vec::new(),
            Time: 0,
            Interval: 0,
            Loop: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartTaskRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartTaskRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartTaskRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartTaskRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:scyna.StartTaskResponse)
pub struct StartTaskResponse {
    // message fields
    // @@protoc_insertion_point(field:scyna.StartTaskResponse.Id)
    pub Id: u64,
    // special fields
    // @@protoc_insertion_point(special_field:scyna.StartTaskResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartTaskResponse {
    fn default() -> &'a StartTaskResponse {
        <StartTaskResponse as ::protobuf::Message>::default_instance()
    }
}

impl StartTaskResponse {
    pub fn new() -> StartTaskResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "Id",
            |m: &StartTaskResponse| { &m.Id },
            |m: &mut StartTaskResponse| { &mut m.Id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartTaskResponse>(
            "StartTaskResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartTaskResponse {
    const NAME: &'static str = "StartTaskResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.Id = is.read_uint64()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.Id != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.Id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.Id != 0 {
            os.write_uint64(1, self.Id)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> StartTaskResponse {
        StartTaskResponse::new()
    }

    fn clear(&mut self) {
        self.Id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartTaskResponse {
        static instance: StartTaskResponse = StartTaskResponse {
            Id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartTaskResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartTaskResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartTaskResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartTaskResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:scyna.StopTaskRequest)
pub struct StopTaskRequest {
    // message fields
    // @@protoc_insertion_point(field:scyna.StopTaskRequest.Id)
    pub Id: u64,
    // special fields
    // @@protoc_insertion_point(special_field:scyna.StopTaskRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StopTaskRequest {
    fn default() -> &'a StopTaskRequest {
        <StopTaskRequest as ::protobuf::Message>::default_instance()
    }
}

impl StopTaskRequest {
    pub fn new() -> StopTaskRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "Id",
            |m: &StopTaskRequest| { &m.Id },
            |m: &mut StopTaskRequest| { &mut m.Id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StopTaskRequest>(
            "StopTaskRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StopTaskRequest {
    const NAME: &'static str = "StopTaskRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.Id = is.read_uint64()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.Id != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.Id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.Id != 0 {
            os.write_uint64(1, self.Id)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> StopTaskRequest {
        StopTaskRequest::new()
    }

    fn clear(&mut self) {
        self.Id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StopTaskRequest {
        static instance: StopTaskRequest = StopTaskRequest {
            Id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StopTaskRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StopTaskRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StopTaskRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StopTaskRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:scyna.Task)
pub struct Task {
    // message fields
    // @@protoc_insertion_point(field:scyna.Task.TraceID)
    pub TraceID: u64,
    // @@protoc_insertion_point(field:scyna.Task.Data)
    pub Data: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:scyna.Task.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Task {
    fn default() -> &'a Task {
        <Task as ::protobuf::Message>::default_instance()
    }
}

impl Task {
    pub fn new() -> Task {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "TraceID",
            |m: &Task| { &m.TraceID },
            |m: &mut Task| { &mut m.TraceID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "Data",
            |m: &Task| { &m.Data },
            |m: &mut Task| { &mut m.Data },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Task>(
            "Task",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Task {
    const NAME: &'static str = "Task";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.TraceID = is.read_uint64()?;
                },
                18 => {
                    self.Data = is.read_bytes()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.TraceID != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.TraceID);
        }
        if !self.Data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.Data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.TraceID != 0 {
            os.write_uint64(1, self.TraceID)?;
        }
        if !self.Data.is_empty() {
            os.write_bytes(2, &self.Data)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Task {
        Task::new()
    }

    fn clear(&mut self) {
        self.TraceID = 0;
        self.Data.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Task {
        static instance: Task = Task {
            TraceID: 0,
            Data: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Task {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Task").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Task {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Task {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\ntask.proto\x12\x05scyna\"\x98\x01\n\x10StartTaskRequest\x12\x16\n\
    \x06Module\x18\x01\x20\x01(\tR\x06Module\x12\x14\n\x05Topic\x18\x02\x20\
    \x01(\tR\x05Topic\x12\x12\n\x04Data\x18\x03\x20\x01(\x0cR\x04Data\x12\
    \x12\n\x04Time\x18\x04\x20\x01(\x03R\x04Time\x12\x1a\n\x08Interval\x18\
    \x05\x20\x01(\x03R\x08Interval\x12\x12\n\x04Loop\x18\x06\x20\x01(\x04R\
    \x04Loop\"#\n\x11StartTaskResponse\x12\x0e\n\x02Id\x18\x01\x20\x01(\x04R\
    \x02Id\"!\n\x0fStopTaskRequest\x12\x0e\n\x02Id\x18\x01\x20\x01(\x04R\x02\
    Id\"4\n\x04Task\x12\x18\n\x07TraceID\x18\x01\x20\x01(\x04R\x07TraceID\
    \x12\x12\n\x04Data\x18\x02\x20\x01(\x0cR\x04DataB2\n\x0eio.scyna.protoP\
    \x01H\x02Z\x0e./;scyna_proto\xaa\x02\x0bscyna.protoJ\xd7\x07\n\x06\x12\
    \x04\0\0\x1d\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\
    \x02\x08\r\n\x08\n\x01\x08\x12\x03\x03\0(\n\t\n\x02\x08\x01\x12\x03\x03\
    \0(\n\x08\n\x01\x08\x12\x03\x04\0#\n\t\n\x02\x08\n\x12\x03\x04\0#\n\x08\
    \n\x01\x08\x12\x03\x05\0)\n\t\n\x02\x08%\x12\x03\x05\0)\n\x08\n\x01\x08\
    \x12\x03\x06\0!\n\t\n\x02\x08\t\x12\x03\x06\0!\n\x08\n\x01\x08\x12\x03\
    \x07\0&\n\t\n\x02\x08\x0b\x12\x03\x07\0&\n\n\n\x02\x04\0\x12\x04\t\0\x10\
    \x01\n\n\n\x03\x04\0\x01\x12\x03\t\x08\x18\n\x0b\n\x04\x04\0\x02\0\x12\
    \x03\n\x02\x14\n\r\n\x05\x04\0\x02\0\x04\x12\x04\n\x02\t\x1a\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\n\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\
    \t\x0f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\n\x12\x13\n\x0b\n\x04\x04\0\
    \x02\x01\x12\x03\x0b\x02\x13\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x0b\x02\
    \n\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x0b\x02\x08\n\x0c\n\x05\x04\
    \0\x02\x01\x01\x12\x03\x0b\t\x0e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\
    \x0b\x11\x12\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x0c\x02\x12\n\r\n\x05\x04\
    \0\x02\x02\x04\x12\x04\x0c\x02\x0b\x13\n\x0c\n\x05\x04\0\x02\x02\x05\x12\
    \x03\x0c\x02\x07\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0c\t\r\n\x0c\n\
    \x05\x04\0\x02\x02\x03\x12\x03\x0c\x10\x11\n\x1b\n\x04\x04\0\x02\x03\x12\
    \x03\r\x02\x12\"\x0e\x20Unit:\x20second\x20\n\r\n\x05\x04\0\x02\x03\x04\
    \x12\x04\r\x02\x0c\x12\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\r\x02\x07\n\
    \x0c\n\x05\x04\0\x02\x03\x01\x12\x03\r\t\r\n\x0c\n\x05\x04\0\x02\x03\x03\
    \x12\x03\r\x10\x11\n1\n\x04\x04\0\x02\x04\x12\x03\x0e\x02\x16\"$\x20In\
    \x20second,\x20must\x20be\x20greater\x20than\x2060\x20\n\r\n\x05\x04\0\
    \x02\x04\x04\x12\x04\x0e\x02\r\x12\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\
    \x0e\x02\x07\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x0e\t\x11\n\x0c\n\x05\
    \x04\0\x02\x04\x03\x12\x03\x0e\x14\x15\n\x0b\n\x04\x04\0\x02\x05\x12\x03\
    \x0f\x02\x12\n\r\n\x05\x04\0\x02\x05\x04\x12\x04\x0f\x02\x0e\x16\n\x0c\n\
    \x05\x04\0\x02\x05\x05\x12\x03\x0f\x02\x08\n\x0c\n\x05\x04\0\x02\x05\x01\
    \x12\x03\x0f\t\r\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x0f\x10\x11\n\n\n\
    \x02\x04\x01\x12\x04\x12\0\x14\x01\n\n\n\x03\x04\x01\x01\x12\x03\x12\x08\
    \x19\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x13\x02\x10\n\r\n\x05\x04\x01\x02\
    \0\x04\x12\x04\x13\x02\x12\x1b\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x13\
    \x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x13\t\x0b\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\x13\x0e\x0f\n\n\n\x02\x04\x02\x12\x04\x16\0\x18\
    \x01\n\n\n\x03\x04\x02\x01\x12\x03\x16\x08\x17\n\x0b\n\x04\x04\x02\x02\0\
    \x12\x03\x17\x02\x10\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x17\x02\x16\x19\
    \n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x17\x02\x08\n\x0c\n\x05\x04\x02\
    \x02\0\x01\x12\x03\x17\t\x0b\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x17\
    \x0e\x0f\n\n\n\x02\x04\x03\x12\x04\x1a\0\x1d\x01\n\n\n\x03\x04\x03\x01\
    \x12\x03\x1a\x08\x0c\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x1b\x02\x15\n\r\n\
    \x05\x04\x03\x02\0\x04\x12\x04\x1b\x02\x1a\x0e\n\x0c\n\x05\x04\x03\x02\0\
    \x05\x12\x03\x1b\x02\x08\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x1b\t\x10\
    \n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x1b\x13\x14\n\x0b\n\x04\x04\x03\
    \x02\x01\x12\x03\x1c\x02\x12\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04\x1c\
    \x02\x1b\x15\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03\x1c\x02\x07\n\x0c\n\
    \x05\x04\x03\x02\x01\x01\x12\x03\x1c\t\r\n\x0c\n\x05\x04\x03\x02\x01\x03\
    \x12\x03\x1c\x10\x11b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(StartTaskRequest::generated_message_descriptor_data());
            messages.push(StartTaskResponse::generated_message_descriptor_data());
            messages.push(StopTaskRequest::generated_message_descriptor_data());
            messages.push(Task::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
