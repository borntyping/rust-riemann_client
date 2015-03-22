// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct State {
    // message fields
    time: ::std::option::Option<i64>,
    state: ::protobuf::SingularField<::std::string::String>,
    service: ::protobuf::SingularField<::std::string::String>,
    host: ::protobuf::SingularField<::std::string::String>,
    description: ::protobuf::SingularField<::std::string::String>,
    once: ::std::option::Option<bool>,
    tags: ::protobuf::RepeatedField<::std::string::String>,
    ttl: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl State {
    pub fn new() -> State {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static State {
        static mut instance: ::protobuf::lazy::Lazy<State> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const State,
        };
        unsafe {
            instance.get(|| {
                State {
                    time: ::std::option::Option::None,
                    state: ::protobuf::SingularField::none(),
                    service: ::protobuf::SingularField::none(),
                    host: ::protobuf::SingularField::none(),
                    description: ::protobuf::SingularField::none(),
                    once: ::std::option::Option::None,
                    tags: ::protobuf::RepeatedField::new(),
                    ttl: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 time = 1;

    pub fn clear_time(&mut self) {
        self.time = ::std::option::Option::None;
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: i64) {
        self.time = ::std::option::Option::Some(v);
    }

    pub fn get_time<'a>(&self) -> i64 {
        self.time.unwrap_or(0)
    }

    // optional string state = 2;

    pub fn clear_state(&mut self) {
        self.state.clear();
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: ::std::string::String) {
        self.state = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.state.is_none() {
            self.state.set_default();
        };
        self.state.as_mut().unwrap()
    }

    // Take field
    pub fn take_state(&mut self) -> ::std::string::String {
        self.state.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_state<'a>(&'a self) -> &'a str {
        match self.state.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string service = 3;

    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    pub fn has_service(&self) -> bool {
        self.service.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: ::std::string::String) {
        self.service = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.service.is_none() {
            self.service.set_default();
        };
        self.service.as_mut().unwrap()
    }

    // Take field
    pub fn take_service(&mut self) -> ::std::string::String {
        self.service.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_service<'a>(&'a self) -> &'a str {
        match self.service.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string host = 4;

    pub fn clear_host(&mut self) {
        self.host.clear();
    }

    pub fn has_host(&self) -> bool {
        self.host.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host(&mut self, v: ::std::string::String) {
        self.host = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_host<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.host.is_none() {
            self.host.set_default();
        };
        self.host.as_mut().unwrap()
    }

    // Take field
    pub fn take_host(&mut self) -> ::std::string::String {
        self.host.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_host<'a>(&'a self) -> &'a str {
        match self.host.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string description = 5;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description<'a>(&'a self) -> &'a str {
        match self.description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool once = 6;

    pub fn clear_once(&mut self) {
        self.once = ::std::option::Option::None;
    }

    pub fn has_once(&self) -> bool {
        self.once.is_some()
    }

    // Param is passed by value, moved
    pub fn set_once(&mut self, v: bool) {
        self.once = ::std::option::Option::Some(v);
    }

    pub fn get_once<'a>(&self) -> bool {
        self.once.unwrap_or(false)
    }

    // repeated string tags = 7;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_tags<'a>(&'a self) -> &'a [::std::string::String] {
        &self.tags
    }

    // optional float ttl = 8;

    pub fn clear_ttl(&mut self) {
        self.ttl = ::std::option::Option::None;
    }

    pub fn has_ttl(&self) -> bool {
        self.ttl.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ttl(&mut self, v: f32) {
        self.ttl = ::std::option::Option::Some(v);
    }

    pub fn get_ttl<'a>(&self) -> f32 {
        self.ttl.unwrap_or(0.)
    }
}

impl ::protobuf::Message for State {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.time = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.state.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.service.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.host.set_default();
                    try!(is.read_string_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.description.set_default();
                    try!(is.read_string_into(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.once = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.tags));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.ttl = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.time.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.state.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.service.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.host.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in self.description.iter() {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        if self.once.is_some() {
            my_size += 2;
        };
        for value in self.tags.iter() {
            my_size += ::protobuf::rt::string_size(7, &value);
        };
        if self.ttl.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.time {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.state.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.service.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.host.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.description.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.once {
            try!(os.write_bool(6, v));
        };
        for v in self.tags.iter() {
            try!(os.write_string(7, &v));
        };
        if let Some(v) = self.ttl {
            try!(os.write_float(8, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<State>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for State {
    fn new() -> State {
        State::new()
    }

    fn descriptor_static(_: ::std::option::Option<State>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "time",
                    State::has_time,
                    State::get_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "state",
                    State::has_state,
                    State::get_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "service",
                    State::has_service,
                    State::get_service,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "host",
                    State::has_host,
                    State::get_host,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "description",
                    State::has_description,
                    State::get_description,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "once",
                    State::has_once,
                    State::get_once,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "tags",
                    State::get_tags,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "ttl",
                    State::has_ttl,
                    State::get_ttl,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<State>(
                    "State",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for State {
    fn clear(&mut self) {
        self.clear_time();
        self.clear_state();
        self.clear_service();
        self.clear_host();
        self.clear_description();
        self.clear_once();
        self.clear_tags();
        self.clear_ttl();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.time == other.time &&
        self.state == other.state &&
        self.service == other.service &&
        self.host == other.host &&
        self.description == other.description &&
        self.once == other.once &&
        self.tags == other.tags &&
        self.ttl == other.ttl &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for State {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Event {
    // message fields
    time: ::std::option::Option<i64>,
    state: ::protobuf::SingularField<::std::string::String>,
    service: ::protobuf::SingularField<::std::string::String>,
    host: ::protobuf::SingularField<::std::string::String>,
    description: ::protobuf::SingularField<::std::string::String>,
    tags: ::protobuf::RepeatedField<::std::string::String>,
    ttl: ::std::option::Option<f32>,
    attributes: ::protobuf::RepeatedField<Attribute>,
    metric_sint64: ::std::option::Option<i64>,
    metric_d: ::std::option::Option<f64>,
    metric_f: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Event {
    pub fn new() -> Event {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Event {
        static mut instance: ::protobuf::lazy::Lazy<Event> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event,
        };
        unsafe {
            instance.get(|| {
                Event {
                    time: ::std::option::Option::None,
                    state: ::protobuf::SingularField::none(),
                    service: ::protobuf::SingularField::none(),
                    host: ::protobuf::SingularField::none(),
                    description: ::protobuf::SingularField::none(),
                    tags: ::protobuf::RepeatedField::new(),
                    ttl: ::std::option::Option::None,
                    attributes: ::protobuf::RepeatedField::new(),
                    metric_sint64: ::std::option::Option::None,
                    metric_d: ::std::option::Option::None,
                    metric_f: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 time = 1;

    pub fn clear_time(&mut self) {
        self.time = ::std::option::Option::None;
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: i64) {
        self.time = ::std::option::Option::Some(v);
    }

    pub fn get_time<'a>(&self) -> i64 {
        self.time.unwrap_or(0)
    }

    // optional string state = 2;

    pub fn clear_state(&mut self) {
        self.state.clear();
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: ::std::string::String) {
        self.state = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.state.is_none() {
            self.state.set_default();
        };
        self.state.as_mut().unwrap()
    }

    // Take field
    pub fn take_state(&mut self) -> ::std::string::String {
        self.state.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_state<'a>(&'a self) -> &'a str {
        match self.state.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string service = 3;

    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    pub fn has_service(&self) -> bool {
        self.service.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: ::std::string::String) {
        self.service = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.service.is_none() {
            self.service.set_default();
        };
        self.service.as_mut().unwrap()
    }

    // Take field
    pub fn take_service(&mut self) -> ::std::string::String {
        self.service.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_service<'a>(&'a self) -> &'a str {
        match self.service.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string host = 4;

    pub fn clear_host(&mut self) {
        self.host.clear();
    }

    pub fn has_host(&self) -> bool {
        self.host.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host(&mut self, v: ::std::string::String) {
        self.host = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_host<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.host.is_none() {
            self.host.set_default();
        };
        self.host.as_mut().unwrap()
    }

    // Take field
    pub fn take_host(&mut self) -> ::std::string::String {
        self.host.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_host<'a>(&'a self) -> &'a str {
        match self.host.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string description = 5;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description<'a>(&'a self) -> &'a str {
        match self.description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated string tags = 7;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_tags<'a>(&'a self) -> &'a [::std::string::String] {
        &self.tags
    }

    // optional float ttl = 8;

    pub fn clear_ttl(&mut self) {
        self.ttl = ::std::option::Option::None;
    }

    pub fn has_ttl(&self) -> bool {
        self.ttl.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ttl(&mut self, v: f32) {
        self.ttl = ::std::option::Option::Some(v);
    }

    pub fn get_ttl<'a>(&self) -> f32 {
        self.ttl.unwrap_or(0.)
    }

    // repeated .Attribute attributes = 9;

    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }

    // Param is passed by value, moved
    pub fn set_attributes(&mut self, v: ::protobuf::RepeatedField<Attribute>) {
        self.attributes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attributes<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Attribute> {
        &mut self.attributes
    }

    // Take field
    pub fn take_attributes(&mut self) -> ::protobuf::RepeatedField<Attribute> {
        ::std::mem::replace(&mut self.attributes, ::protobuf::RepeatedField::new())
    }

    pub fn get_attributes<'a>(&'a self) -> &'a [Attribute] {
        &self.attributes
    }

    // optional sint64 metric_sint64 = 13;

    pub fn clear_metric_sint64(&mut self) {
        self.metric_sint64 = ::std::option::Option::None;
    }

    pub fn has_metric_sint64(&self) -> bool {
        self.metric_sint64.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metric_sint64(&mut self, v: i64) {
        self.metric_sint64 = ::std::option::Option::Some(v);
    }

    pub fn get_metric_sint64<'a>(&self) -> i64 {
        self.metric_sint64.unwrap_or(0)
    }

    // optional double metric_d = 14;

    pub fn clear_metric_d(&mut self) {
        self.metric_d = ::std::option::Option::None;
    }

    pub fn has_metric_d(&self) -> bool {
        self.metric_d.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metric_d(&mut self, v: f64) {
        self.metric_d = ::std::option::Option::Some(v);
    }

    pub fn get_metric_d<'a>(&self) -> f64 {
        self.metric_d.unwrap_or(0.)
    }

    // optional float metric_f = 15;

    pub fn clear_metric_f(&mut self) {
        self.metric_f = ::std::option::Option::None;
    }

    pub fn has_metric_f(&self) -> bool {
        self.metric_f.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metric_f(&mut self, v: f32) {
        self.metric_f = ::std::option::Option::Some(v);
    }

    pub fn get_metric_f<'a>(&self) -> f32 {
        self.metric_f.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Event {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.time = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.state.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.service.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.host.set_default();
                    try!(is.read_string_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.description.set_default();
                    try!(is.read_string_into(tmp))
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.tags));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.ttl = ::std::option::Option::Some(tmp);
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.attributes));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sint64());
                    self.metric_sint64 = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.metric_d = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.metric_f = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.time.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.state.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.service.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.host.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in self.description.iter() {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in self.tags.iter() {
            my_size += ::protobuf::rt::string_size(7, &value);
        };
        if self.ttl.is_some() {
            my_size += 5;
        };
        for value in self.attributes.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.metric_sint64.iter() {
            my_size += ::protobuf::rt::value_size(13, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.metric_d.is_some() {
            my_size += 9;
        };
        if self.metric_f.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.time {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.state.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.service.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.host.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.description.as_ref() {
            try!(os.write_string(5, &v));
        };
        for v in self.tags.iter() {
            try!(os.write_string(7, &v));
        };
        if let Some(v) = self.ttl {
            try!(os.write_float(8, v));
        };
        for v in self.attributes.iter() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.metric_sint64 {
            try!(os.write_sint64(13, v));
        };
        if let Some(v) = self.metric_d {
            try!(os.write_double(14, v));
        };
        if let Some(v) = self.metric_f {
            try!(os.write_float(15, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Event>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Event {
    fn new() -> Event {
        Event::new()
    }

    fn descriptor_static(_: ::std::option::Option<Event>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "time",
                    Event::has_time,
                    Event::get_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "state",
                    Event::has_state,
                    Event::get_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "service",
                    Event::has_service,
                    Event::get_service,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "host",
                    Event::has_host,
                    Event::get_host,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "description",
                    Event::has_description,
                    Event::get_description,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "tags",
                    Event::get_tags,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "ttl",
                    Event::has_ttl,
                    Event::get_ttl,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "attributes",
                    Event::get_attributes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "metric_sint64",
                    Event::has_metric_sint64,
                    Event::get_metric_sint64,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "metric_d",
                    Event::has_metric_d,
                    Event::get_metric_d,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "metric_f",
                    Event::has_metric_f,
                    Event::get_metric_f,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event>(
                    "Event",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Event {
    fn clear(&mut self) {
        self.clear_time();
        self.clear_state();
        self.clear_service();
        self.clear_host();
        self.clear_description();
        self.clear_tags();
        self.clear_ttl();
        self.clear_attributes();
        self.clear_metric_sint64();
        self.clear_metric_d();
        self.clear_metric_f();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.time == other.time &&
        self.state == other.state &&
        self.service == other.service &&
        self.host == other.host &&
        self.description == other.description &&
        self.tags == other.tags &&
        self.ttl == other.ttl &&
        self.attributes == other.attributes &&
        self.metric_sint64 == other.metric_sint64 &&
        self.metric_d == other.metric_d &&
        self.metric_f == other.metric_f &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Event {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Query {
    // message fields
    string: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Query {
    pub fn new() -> Query {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Query {
        static mut instance: ::protobuf::lazy::Lazy<Query> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Query,
        };
        unsafe {
            instance.get(|| {
                Query {
                    string: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string string = 1;

    pub fn clear_string(&mut self) {
        self.string.clear();
    }

    pub fn has_string(&self) -> bool {
        self.string.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string(&mut self, v: ::std::string::String) {
        self.string = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.string.is_none() {
            self.string.set_default();
        };
        self.string.as_mut().unwrap()
    }

    // Take field
    pub fn take_string(&mut self) -> ::std::string::String {
        self.string.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_string<'a>(&'a self) -> &'a str {
        match self.string.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Query {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.string.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.string.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.string.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Query>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Query {
    fn new() -> Query {
        Query::new()
    }

    fn descriptor_static(_: ::std::option::Option<Query>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "string",
                    Query::has_string,
                    Query::get_string,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Query>(
                    "Query",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Query {
    fn clear(&mut self) {
        self.clear_string();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Query {
    fn eq(&self, other: &Query) -> bool {
        self.string == other.string &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Query {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Msg {
    // message fields
    ok: ::std::option::Option<bool>,
    error: ::protobuf::SingularField<::std::string::String>,
    states: ::protobuf::RepeatedField<State>,
    query: ::protobuf::SingularPtrField<Query>,
    events: ::protobuf::RepeatedField<Event>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Msg {
    pub fn new() -> Msg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Msg {
        static mut instance: ::protobuf::lazy::Lazy<Msg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Msg,
        };
        unsafe {
            instance.get(|| {
                Msg {
                    ok: ::std::option::Option::None,
                    error: ::protobuf::SingularField::none(),
                    states: ::protobuf::RepeatedField::new(),
                    query: ::protobuf::SingularPtrField::none(),
                    events: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool ok = 2;

    pub fn clear_ok(&mut self) {
        self.ok = ::std::option::Option::None;
    }

    pub fn has_ok(&self) -> bool {
        self.ok.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ok(&mut self, v: bool) {
        self.ok = ::std::option::Option::Some(v);
    }

    pub fn get_ok<'a>(&self) -> bool {
        self.ok.unwrap_or(false)
    }

    // optional string error = 3;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error<'a>(&'a self) -> &'a str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .State states = 4;

    pub fn clear_states(&mut self) {
        self.states.clear();
    }

    // Param is passed by value, moved
    pub fn set_states(&mut self, v: ::protobuf::RepeatedField<State>) {
        self.states = v;
    }

    // Mutable pointer to the field.
    pub fn mut_states<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<State> {
        &mut self.states
    }

    // Take field
    pub fn take_states(&mut self) -> ::protobuf::RepeatedField<State> {
        ::std::mem::replace(&mut self.states, ::protobuf::RepeatedField::new())
    }

    pub fn get_states<'a>(&'a self) -> &'a [State] {
        &self.states
    }

    // optional .Query query = 5;

    pub fn clear_query(&mut self) {
        self.query.clear();
    }

    pub fn has_query(&self) -> bool {
        self.query.is_some()
    }

    // Param is passed by value, moved
    pub fn set_query(&mut self, v: Query) {
        self.query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query<'a>(&'a mut self) -> &'a mut Query {
        if self.query.is_none() {
            self.query.set_default();
        };
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> Query {
        self.query.take().unwrap_or_else(|| Query::new())
    }

    pub fn get_query<'a>(&'a self) -> &'a Query {
        self.query.as_ref().unwrap_or_else(|| Query::default_instance())
    }

    // repeated .Event events = 6;

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // Param is passed by value, moved
    pub fn set_events(&mut self, v: ::protobuf::RepeatedField<Event>) {
        self.events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_events<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Event> {
        &mut self.events
    }

    // Take field
    pub fn take_events(&mut self) -> ::protobuf::RepeatedField<Event> {
        ::std::mem::replace(&mut self.events, ::protobuf::RepeatedField::new())
    }

    pub fn get_events<'a>(&'a self) -> &'a [Event] {
        &self.events
    }
}

impl ::protobuf::Message for Msg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.ok = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.error.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.states));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.query.set_default();
                    try!(is.merge_message(tmp))
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.events));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.ok.is_some() {
            my_size += 2;
        };
        for value in self.error.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.states.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.query.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.events.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ok {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.error.as_ref() {
            try!(os.write_string(3, &v));
        };
        for v in self.states.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.query.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.events.iter() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Msg>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Msg {
    fn new() -> Msg {
        Msg::new()
    }

    fn descriptor_static(_: ::std::option::Option<Msg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "ok",
                    Msg::has_ok,
                    Msg::get_ok,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "error",
                    Msg::has_error,
                    Msg::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "states",
                    Msg::get_states,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "query",
                    Msg::has_query,
                    Msg::get_query,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "events",
                    Msg::get_events,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Msg>(
                    "Msg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Msg {
    fn clear(&mut self) {
        self.clear_ok();
        self.clear_error();
        self.clear_states();
        self.clear_query();
        self.clear_events();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Msg {
    fn eq(&self, other: &Msg) -> bool {
        self.ok == other.ok &&
        self.error == other.error &&
        self.states == other.states &&
        self.query == other.query &&
        self.events == other.events &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Msg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Attribute {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Attribute {
    pub fn new() -> Attribute {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Attribute {
        static mut instance: ::protobuf::lazy::Lazy<Attribute> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Attribute,
        };
        unsafe {
            instance.get(|| {
                Attribute {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Attribute {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.key.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.key.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Attribute>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Attribute {
    fn new() -> Attribute {
        Attribute::new()
    }

    fn descriptor_static(_: ::std::option::Option<Attribute>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    Attribute::has_key,
                    Attribute::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "value",
                    Attribute::has_value,
                    Attribute::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Attribute>(
                    "Attribute",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Attribute {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Attribute {
    fn eq(&self, other: &Attribute) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Attribute {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x73, 0x72, 0x63, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x6d, 0x6f, 0x64, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x81, 0x01, 0x0a, 0x05, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12,
    0x0c, 0x0a, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x12, 0x0d, 0x0a,
    0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0f, 0x0a, 0x07,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0c, 0x0a,
    0x04, 0x68, 0x6f, 0x73, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x12, 0x13, 0x0a, 0x0b, 0x64,
    0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09,
    0x12, 0x0c, 0x0a, 0x04, 0x6f, 0x6e, 0x63, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0c,
    0x0a, 0x04, 0x74, 0x61, 0x67, 0x73, 0x18, 0x07, 0x20, 0x03, 0x28, 0x09, 0x12, 0x0b, 0x0a, 0x03,
    0x74, 0x74, 0x6c, 0x18, 0x08, 0x20, 0x01, 0x28, 0x02, 0x22, 0xce, 0x01, 0x0a, 0x05, 0x45, 0x76,
    0x65, 0x6e, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x03, 0x12, 0x0d, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
    0x12, 0x0f, 0x0a, 0x07, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x09, 0x12, 0x0c, 0x0a, 0x04, 0x68, 0x6f, 0x73, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x12,
    0x13, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x61, 0x67, 0x73, 0x18, 0x07, 0x20, 0x03,
    0x28, 0x09, 0x12, 0x0b, 0x0a, 0x03, 0x74, 0x74, 0x6c, 0x18, 0x08, 0x20, 0x01, 0x28, 0x02, 0x12,
    0x1e, 0x0a, 0x0a, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x18, 0x09, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x12,
    0x15, 0x0a, 0x0d, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x5f, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34,
    0x18, 0x0d, 0x20, 0x01, 0x28, 0x12, 0x12, 0x10, 0x0a, 0x08, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63,
    0x5f, 0x64, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x01, 0x12, 0x10, 0x0a, 0x08, 0x6d, 0x65, 0x74, 0x72,
    0x69, 0x63, 0x5f, 0x66, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x02, 0x22, 0x17, 0x0a, 0x05, 0x51, 0x75,
    0x65, 0x72, 0x79, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x22, 0x67, 0x0a, 0x03, 0x4d, 0x73, 0x67, 0x12, 0x0a, 0x0a, 0x02, 0x6f, 0x6b,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0d, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x09, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x65, 0x73, 0x18,
    0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x15, 0x0a,
    0x05, 0x71, 0x75, 0x65, 0x72, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x51,
    0x75, 0x65, 0x72, 0x79, 0x12, 0x16, 0x0a, 0x06, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x06,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x22, 0x27, 0x0a, 0x09,
    0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x09, 0x42, 0x1a, 0x0a, 0x11, 0x63, 0x6f, 0x6d, 0x2e, 0x61, 0x70, 0x68,
    0x79, 0x72, 0x2e, 0x72, 0x69, 0x65, 0x6d, 0x61, 0x6e, 0x6e, 0x42, 0x05, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x4a, 0xbf, 0x11, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x2e, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x00, 0x00, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03,
    0x00, 0x00, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x00, 0x07,
    0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x00, 0x07, 0x13,
    0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x00, 0x07, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x00, 0x16, 0x29, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x01, 0x00, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01,
    0x12, 0x03, 0x01, 0x00, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03,
    0x01, 0x07, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x01,
    0x07, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x01,
    0x07, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07, 0x12, 0x03, 0x01, 0x1e, 0x25,
    0x0a, 0x5e, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x05, 0x00, 0x0e, 0x01, 0x1a, 0x52, 0x20, 0x44,
    0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x3b, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x20, 0x77, 0x61, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x65, 0x61, 0x72,
    0x6c, 0x79, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2c, 0x20, 0x62, 0x75, 0x74,
    0x20, 0x6e, 0x6f, 0x74, 0x20, 0x61, 0x6e, 0x79, 0x0a, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x2e, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x05, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x06, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x06, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x06, 0x18,
    0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x07, 0x02, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x07, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x07, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x07, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x07, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x08, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x08, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x08, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x08, 0x12, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x08, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x09, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x09, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x09,
    0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x09, 0x19, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x0a, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x0a, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0b,
    0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0b, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0b, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x06, 0x12, 0x03, 0x0c, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04,
    0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03,
    0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0c, 0x12,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x0c, 0x19, 0x1a, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x0d, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x01, 0x12, 0x03, 0x0d, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12,
    0x03, 0x0d, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x10, 0x00, 0x1d, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x10, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x11, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x11, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x11, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x11, 0x18,
    0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x12, 0x02, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x12, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x12, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03,
    0x13, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x13, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x13, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x13, 0x12, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x13, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x03, 0x12, 0x03, 0x14, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x14, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x14,
    0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x14, 0x19, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x15, 0x02, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x15, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x15, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x15, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x16,
    0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x16, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x16, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x16, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x06, 0x12, 0x03, 0x17, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x04,
    0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03,
    0x17, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x17, 0x11,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x17, 0x17, 0x18, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x07, 0x12, 0x03, 0x18, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x07, 0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x07, 0x06, 0x12, 0x03, 0x18, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07,
    0x01, 0x12, 0x03, 0x18, 0x15, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x03, 0x12,
    0x03, 0x18, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x08, 0x12, 0x03, 0x1a, 0x02,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x05, 0x12, 0x03, 0x1a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x1a, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x08, 0x03, 0x12, 0x03, 0x1a, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x09, 0x12, 0x03, 0x1b, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x04, 0x12,
    0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x05, 0x12, 0x03, 0x1b,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x01, 0x12, 0x03, 0x1b, 0x12, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x03, 0x12, 0x03, 0x1b, 0x1d, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x0a, 0x12, 0x03, 0x1c, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x0a, 0x05, 0x12, 0x03, 0x1c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x01,
    0x12, 0x03, 0x1c, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x03, 0x12, 0x03,
    0x1c, 0x1c, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x1f, 0x00, 0x21, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x00, 0x12, 0x03, 0x20, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x20, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x20, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20,
    0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x20, 0x1b, 0x1c,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x23, 0x00, 0x29, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x03, 0x01, 0x12, 0x03, 0x23, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00,
    0x12, 0x03, 0x24, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x24, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x24, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x10, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x15, 0x16, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x25, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x25, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x25, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x25,
    0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x26, 0x02, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x26, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x02, 0x06, 0x12, 0x03, 0x26, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x26, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12,
    0x03, 0x27, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x27,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x06, 0x12, 0x03, 0x27, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x27, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x27, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x28, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x28, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x06,
    0x12, 0x03, 0x28, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x28, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x28, 0x1a,
    0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2b, 0x00, 0x2e, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x00, 0x12, 0x03, 0x2c, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2c,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x12, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x18, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x2d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x2d, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x2d, 0x1a, 0x1b,
];

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
