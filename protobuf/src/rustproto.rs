// This file is generated by rust-protobuf 3.0.0-alpha.13. Do not edit
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

//! Generated file from `rustproto.proto`

/// Extension fields
pub mod exts {

    pub const expose_fields_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, bool> = crate::ext::ExtFieldOptional::new(17003, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const generate_accessors_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, bool> = crate::ext::ExtFieldOptional::new(17004, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const generate_getter_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, bool> = crate::ext::ExtFieldOptional::new(17005, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const tokio_bytes_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, bool> = crate::ext::ExtFieldOptional::new(17011, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const tokio_bytes_for_string_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, bool> = crate::ext::ExtFieldOptional::new(17012, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const lite_runtime_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, bool> = crate::ext::ExtFieldOptional::new(17035, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const expose_fields: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, bool> = crate::ext::ExtFieldOptional::new(17003, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const generate_accessors: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, bool> = crate::ext::ExtFieldOptional::new(17004, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const generate_getter: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, bool> = crate::ext::ExtFieldOptional::new(17005, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const tokio_bytes: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, bool> = crate::ext::ExtFieldOptional::new(17011, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const tokio_bytes_for_string: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, bool> = crate::ext::ExtFieldOptional::new(17012, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const expose_fields_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, bool> = crate::ext::ExtFieldOptional::new(17003, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const generate_accessors_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, bool> = crate::ext::ExtFieldOptional::new(17004, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const generate_getter_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, bool> = crate::ext::ExtFieldOptional::new(17005, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const tokio_bytes_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, bool> = crate::ext::ExtFieldOptional::new(17011, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const tokio_bytes_for_string_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, bool> = crate::ext::ExtFieldOptional::new(17012, crate::descriptor::field_descriptor_proto::Type::TYPE_BOOL);
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0frustproto.proto\x12\trustproto\x1a\x20google/protobuf/descriptor.p\
    roto:J\n\x11expose_fields_all\x18\xeb\x84\x01\x20\x01(\x08\x12\x1c.googl\
    e.protobuf.FileOptionsR\x0fexposeFieldsAll:T\n\x16generate_accessors_all\
    \x18\xec\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x14ge\
    nerateAccessorsAll:N\n\x13generate_getter_all\x18\xed\x84\x01\x20\x01(\
    \x08\x12\x1c.google.protobuf.FileOptionsR\x11generateGetterAll:F\n\x0fto\
    kio_bytes_all\x18\xf3\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileO\
    ptionsR\rtokioBytesAll:Z\n\x1atokio_bytes_for_string_all\x18\xf4\x84\x01\
    \x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x16tokioBytesForStrin\
    gAll:H\n\x10lite_runtime_all\x18\x8b\x85\x01\x20\x01(\x08\x12\x1c.google\
    .protobuf.FileOptionsR\x0eliteRuntimeAll:F\n\rexpose_fields\x18\xeb\x84\
    \x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x0cexposeField\
    s:P\n\x12generate_accessors\x18\xec\x84\x01\x20\x01(\x08\x12\x1f.google.\
    protobuf.MessageOptionsR\x11generateAccessors:J\n\x0fgenerate_getter\x18\
    \xed\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x0egen\
    erateGetter:B\n\x0btokio_bytes\x18\xf3\x84\x01\x20\x01(\x08\x12\x1f.goog\
    le.protobuf.MessageOptionsR\ntokioBytes:V\n\x16tokio_bytes_for_string\
    \x18\xf4\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\
    \x13tokioBytesForString:O\n\x13expose_fields_field\x18\xeb\x84\x01\x20\
    \x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x11exposeFieldsField:Y\n\
    \x18generate_accessors_field\x18\xec\x84\x01\x20\x01(\x08\x12\x1d.google\
    .protobuf.FieldOptionsR\x16generateAccessorsField:S\n\x15generate_getter\
    _field\x18\xed\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptions\
    R\x13generateGetterField:K\n\x11tokio_bytes_field\x18\xf3\x84\x01\x20\
    \x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x0ftokioBytesField:_\n\
    \x1ctokio_bytes_for_string_field\x18\xf4\x84\x01\x20\x01(\x08\x12\x1d.go\
    ogle.protobuf.FieldOptionsR\x18tokioBytesForStringFieldJ\xb2\x12\n\x06\
    \x12\x04\0\04\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\
    \x03\x02\0*\n\xe5\x01\n\x01\x02\x12\x03\n\0\x122^\x20see\x20https://gith\
    ub.com/gogo/protobuf/blob/master/gogoproto/gogo.proto\n\x20for\x20the\
    \x20original\x20idea\n2{\x20Generated\x20files\x20can\x20be\x20customize\
    d\x20using\x20this\x20proto\n\x20or\x20using\x20`Customize`\x20struct\
    \x20when\x20codegen\x20is\x20invoked\x20programmatically.\n\n\t\n\x01\
    \x07\x12\x04\x0c\0\x1a\x01\nI\n\x02\x07\0\x12\x03\x0e\x04,\x1a>\x20When\
    \x20true\x20all\x20fields\x20are\x20public,\x20and\x20not\x20accessors\
    \x20generated\n\n\n\n\x03\x07\0\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\0\
    \x04\x12\x03\x0e\x04\x0c\n\n\n\x03\x07\0\x05\x12\x03\x0e\r\x11\n\n\n\x03\
    \x07\0\x01\x12\x03\x0e\x12#\n\n\n\x03\x07\0\x03\x12\x03\x0e&+\nP\n\x02\
    \x07\x01\x12\x03\x10\x041\x1aE\x20When\x20false,\x20`get_`,\x20`set_`,\
    \x20`mut_`\x20etc.\x20accessors\x20are\x20not\x20generated\n\n\n\n\x03\
    \x07\x01\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x01\x04\x12\x03\x10\x04\x0c\
    \n\n\n\x03\x07\x01\x05\x12\x03\x10\r\x11\n\n\n\x03\x07\x01\x01\x12\x03\
    \x10\x12(\n\n\n\x03\x07\x01\x03\x12\x03\x10+0\nL\n\x02\x07\x02\x12\x03\
    \x12\x04.\x1aA\x20When\x20false,\x20`get_`\x20is\x20not\x20generated\x20\
    even\x20if\x20`syntax\x20=\x20\"proto2\"`\n\n\n\n\x03\x07\x02\x02\x12\
    \x03\x0c\x07\"\n\n\n\x03\x07\x02\x04\x12\x03\x12\x04\x0c\n\n\n\x03\x07\
    \x02\x05\x12\x03\x12\r\x11\n\n\n\x03\x07\x02\x01\x12\x03\x12\x12%\n\n\n\
    \x03\x07\x02\x03\x12\x03\x12(-\n2\n\x02\x07\x03\x12\x03\x14\x04*\x1a'\
    \x20Use\x20`bytes::Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\x03\x07\
    \x03\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x03\x04\x12\x03\x14\x04\x0c\n\n\
    \n\x03\x07\x03\x05\x12\x03\x14\r\x11\n\n\n\x03\x07\x03\x01\x12\x03\x14\
    \x12!\n\n\n\x03\x07\x03\x03\x12\x03\x14$)\n3\n\x02\x07\x04\x12\x03\x16\
    \x045\x1a(\x20Use\x20`bytes::Bytes`\x20for\x20`string`\x20fields\n\n\n\n\
    \x03\x07\x04\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x04\x04\x12\x03\x16\x04\
    \x0c\n\n\n\x03\x07\x04\x05\x12\x03\x16\r\x11\n\n\n\x03\x07\x04\x01\x12\
    \x03\x16\x12,\n\n\n\x03\x07\x04\x03\x12\x03\x16/4\nN\n\x02\x07\x05\x12\
    \x03\x19\x04+\x1aC\x20When\x20true,\x20will\x20only\x20generate\x20codes\
    \x20that\x20works\x20with\x20lite\x20runtime.\n\n\n\n\x03\x07\x05\x02\
    \x12\x03\x0c\x07\"\n\n\n\x03\x07\x05\x04\x12\x03\x19\x04\x0c\n\n\n\x03\
    \x07\x05\x05\x12\x03\x19\r\x11\n\n\n\x03\x07\x05\x01\x12\x03\x19\x12\"\n\
    \n\n\x03\x07\x05\x03\x12\x03\x19%*\n\t\n\x01\x07\x12\x04\x1c\0'\x01\nI\n\
    \x02\x07\x06\x12\x03\x1e\x04(\x1a>\x20When\x20true\x20all\x20fields\x20a\
    re\x20public,\x20and\x20not\x20accessors\x20generated\n\n\n\n\x03\x07\
    \x06\x02\x12\x03\x1c\x07%\n\n\n\x03\x07\x06\x04\x12\x03\x1e\x04\x0c\n\n\
    \n\x03\x07\x06\x05\x12\x03\x1e\r\x11\n\n\n\x03\x07\x06\x01\x12\x03\x1e\
    \x12\x1f\n\n\n\x03\x07\x06\x03\x12\x03\x1e\"'\nP\n\x02\x07\x07\x12\x03\
    \x20\x04-\x1aE\x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\
    \x20accessors\x20are\x20not\x20generated\n\n\n\n\x03\x07\x07\x02\x12\x03\
    \x1c\x07%\n\n\n\x03\x07\x07\x04\x12\x03\x20\x04\x0c\n\n\n\x03\x07\x07\
    \x05\x12\x03\x20\r\x11\n\n\n\x03\x07\x07\x01\x12\x03\x20\x12$\n\n\n\x03\
    \x07\x07\x03\x12\x03\x20',\nL\n\x02\x07\x08\x12\x03\"\x04*\x1aA\x20When\
    \x20false,\x20`get_`\x20is\x20not\x20generated\x20even\x20if\x20`syntax\
    \x20=\x20\"proto2\"`\n\n\n\n\x03\x07\x08\x02\x12\x03\x1c\x07%\n\n\n\x03\
    \x07\x08\x04\x12\x03\"\x04\x0c\n\n\n\x03\x07\x08\x05\x12\x03\"\r\x11\n\n\
    \n\x03\x07\x08\x01\x12\x03\"\x12!\n\n\n\x03\x07\x08\x03\x12\x03\"$)\n2\n\
    \x02\x07\t\x12\x03$\x04&\x1a'\x20Use\x20`bytes::Bytes`\x20for\x20`bytes`\
    \x20fields\n\n\n\n\x03\x07\t\x02\x12\x03\x1c\x07%\n\n\n\x03\x07\t\x04\
    \x12\x03$\x04\x0c\n\n\n\x03\x07\t\x05\x12\x03$\r\x11\n\n\n\x03\x07\t\x01\
    \x12\x03$\x12\x1d\n\n\n\x03\x07\t\x03\x12\x03$\x20%\n3\n\x02\x07\n\x12\
    \x03&\x041\x1a(\x20Use\x20`bytes::Bytes`\x20for\x20`string`\x20fields\n\
    \n\n\n\x03\x07\n\x02\x12\x03\x1c\x07%\n\n\n\x03\x07\n\x04\x12\x03&\x04\
    \x0c\n\n\n\x03\x07\n\x05\x12\x03&\r\x11\n\n\n\x03\x07\n\x01\x12\x03&\x12\
    (\n\n\n\x03\x07\n\x03\x12\x03&+0\n\t\n\x01\x07\x12\x04)\04\x01\nI\n\x02\
    \x07\x0b\x12\x03+\x04.\x1a>\x20When\x20true\x20all\x20fields\x20are\x20p\
    ublic,\x20and\x20not\x20accessors\x20generated\n\n\n\n\x03\x07\x0b\x02\
    \x12\x03)\x07#\n\n\n\x03\x07\x0b\x04\x12\x03+\x04\x0c\n\n\n\x03\x07\x0b\
    \x05\x12\x03+\r\x11\n\n\n\x03\x07\x0b\x01\x12\x03+\x12%\n\n\n\x03\x07\
    \x0b\x03\x12\x03+(-\nP\n\x02\x07\x0c\x12\x03-\x043\x1aE\x20When\x20false\
    ,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20accessors\x20are\x20not\x20\
    generated\n\n\n\n\x03\x07\x0c\x02\x12\x03)\x07#\n\n\n\x03\x07\x0c\x04\
    \x12\x03-\x04\x0c\n\n\n\x03\x07\x0c\x05\x12\x03-\r\x11\n\n\n\x03\x07\x0c\
    \x01\x12\x03-\x12*\n\n\n\x03\x07\x0c\x03\x12\x03--2\nL\n\x02\x07\r\x12\
    \x03/\x040\x1aA\x20When\x20false,\x20`get_`\x20is\x20not\x20generated\
    \x20even\x20if\x20`syntax\x20=\x20\"proto2\"`\n\n\n\n\x03\x07\r\x02\x12\
    \x03)\x07#\n\n\n\x03\x07\r\x04\x12\x03/\x04\x0c\n\n\n\x03\x07\r\x05\x12\
    \x03/\r\x11\n\n\n\x03\x07\r\x01\x12\x03/\x12'\n\n\n\x03\x07\r\x03\x12\
    \x03/*/\n2\n\x02\x07\x0e\x12\x031\x04,\x1a'\x20Use\x20`bytes::Bytes`\x20\
    for\x20`bytes`\x20fields\n\n\n\n\x03\x07\x0e\x02\x12\x03)\x07#\n\n\n\x03\
    \x07\x0e\x04\x12\x031\x04\x0c\n\n\n\x03\x07\x0e\x05\x12\x031\r\x11\n\n\n\
    \x03\x07\x0e\x01\x12\x031\x12#\n\n\n\x03\x07\x0e\x03\x12\x031&+\n3\n\x02\
    \x07\x0f\x12\x033\x047\x1a(\x20Use\x20`bytes::Bytes`\x20for\x20`string`\
    \x20fields\n\n\n\n\x03\x07\x0f\x02\x12\x03)\x07#\n\n\n\x03\x07\x0f\x04\
    \x12\x033\x04\x0c\n\n\n\x03\x07\x0f\x05\x12\x033\r\x11\n\n\n\x03\x07\x0f\
    \x01\x12\x033\x12.\n\n\n\x03\x07\x0f\x03\x12\x03316\
";

/// `FileDescriptorProto` object which was a source for this generated file
pub fn file_descriptor_proto() -> &'static crate::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: crate::rt::Lazy<crate::descriptor::FileDescriptorProto> = crate::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        crate::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> crate::reflect::FileDescriptor {
    static file_descriptor_lazy: crate::rt::Lazy<crate::reflect::GeneratedFileDescriptor> = crate::rt::Lazy::new();
    let file_descriptor = file_descriptor_lazy.get(|| {
        let mut deps = ::std::vec::Vec::with_capacity(1);
        deps.push(crate::descriptor::file_descriptor());
        let mut messages = ::std::vec::Vec::with_capacity(0);
        let mut enums = ::std::vec::Vec::with_capacity(0);
        crate::reflect::GeneratedFileDescriptor::new_generated(
            file_descriptor_proto(),
            deps,
            messages,
            enums,
        )
    });
    crate::reflect::FileDescriptor::new_generated_2(file_descriptor)
}
