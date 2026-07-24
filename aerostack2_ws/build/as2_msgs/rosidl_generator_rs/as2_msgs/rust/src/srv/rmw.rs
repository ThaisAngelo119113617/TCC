#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__AddStaticTransform_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__AddStaticTransform_Request__init(msg: *mut AddStaticTransform_Request) -> bool;
    fn as2_msgs__srv__AddStaticTransform_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AddStaticTransform_Request>, size: usize) -> bool;
    fn as2_msgs__srv__AddStaticTransform_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AddStaticTransform_Request>);
    fn as2_msgs__srv__AddStaticTransform_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AddStaticTransform_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AddStaticTransform_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__AddStaticTransform_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddStaticTransform_Request {
    /// Parent frame frame id
    pub frame_id: rosidl_runtime_rs::String,

    /// Child frame id
    pub child_frame_id: rosidl_runtime_rs::String,

    /// Transform
    pub transform: geometry_msgs::msg::rmw::Transform,

}



impl Default for AddStaticTransform_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__AddStaticTransform_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__AddStaticTransform_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AddStaticTransform_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__AddStaticTransform_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__AddStaticTransform_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__AddStaticTransform_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AddStaticTransform_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AddStaticTransform_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/AddStaticTransform_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__AddStaticTransform_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__AddStaticTransform_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__AddStaticTransform_Response__init(msg: *mut AddStaticTransform_Response) -> bool;
    fn as2_msgs__srv__AddStaticTransform_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AddStaticTransform_Response>, size: usize) -> bool;
    fn as2_msgs__srv__AddStaticTransform_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AddStaticTransform_Response>);
    fn as2_msgs__srv__AddStaticTransform_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AddStaticTransform_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AddStaticTransform_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__AddStaticTransform_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddStaticTransform_Response {
    /// whether the transform has been set or not
    pub success: bool,

}



impl Default for AddStaticTransform_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__AddStaticTransform_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__AddStaticTransform_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AddStaticTransform_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__AddStaticTransform_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__AddStaticTransform_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__AddStaticTransform_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AddStaticTransform_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AddStaticTransform_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/AddStaticTransform_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__AddStaticTransform_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__AddStaticTransformGps_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__AddStaticTransformGps_Request__init(msg: *mut AddStaticTransformGps_Request) -> bool;
    fn as2_msgs__srv__AddStaticTransformGps_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AddStaticTransformGps_Request>, size: usize) -> bool;
    fn as2_msgs__srv__AddStaticTransformGps_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AddStaticTransformGps_Request>);
    fn as2_msgs__srv__AddStaticTransformGps_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AddStaticTransformGps_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AddStaticTransformGps_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__AddStaticTransformGps_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddStaticTransformGps_Request {
    /// Parent frame frame id
    pub frame_id: rosidl_runtime_rs::String,

    /// Child frame id
    pub child_frame_id: rosidl_runtime_rs::String,

    /// Transform
    pub gps_position: sensor_msgs::msg::rmw::NavSatFix,

    /// yaw
    pub azimuth: f32,

    /// pitch
    pub elevation: f32,

    /// roll
    pub bank: f32,

}



impl Default for AddStaticTransformGps_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__AddStaticTransformGps_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__AddStaticTransformGps_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AddStaticTransformGps_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__AddStaticTransformGps_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__AddStaticTransformGps_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__AddStaticTransformGps_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AddStaticTransformGps_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AddStaticTransformGps_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/AddStaticTransformGps_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__AddStaticTransformGps_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__AddStaticTransformGps_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__AddStaticTransformGps_Response__init(msg: *mut AddStaticTransformGps_Response) -> bool;
    fn as2_msgs__srv__AddStaticTransformGps_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AddStaticTransformGps_Response>, size: usize) -> bool;
    fn as2_msgs__srv__AddStaticTransformGps_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AddStaticTransformGps_Response>);
    fn as2_msgs__srv__AddStaticTransformGps_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AddStaticTransformGps_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AddStaticTransformGps_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__AddStaticTransformGps_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddStaticTransformGps_Response {
    /// whether the transform has been set or not
    pub success: bool,

}



impl Default for AddStaticTransformGps_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__AddStaticTransformGps_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__AddStaticTransformGps_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AddStaticTransformGps_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__AddStaticTransformGps_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__AddStaticTransformGps_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__AddStaticTransformGps_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AddStaticTransformGps_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AddStaticTransformGps_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/AddStaticTransformGps_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__AddStaticTransformGps_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__DynamicFollower_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__DynamicFollower_Request__init(msg: *mut DynamicFollower_Request) -> bool;
    fn as2_msgs__srv__DynamicFollower_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DynamicFollower_Request>, size: usize) -> bool;
    fn as2_msgs__srv__DynamicFollower_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DynamicFollower_Request>);
    fn as2_msgs__srv__DynamicFollower_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DynamicFollower_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DynamicFollower_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__DynamicFollower_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DynamicFollower_Request {
    /// Flag to enable follower
    pub enable: bool,

    /// Speed limit (m/s)
    pub speed_limit: geometry_msgs::msg::rmw::Twist,

}



impl Default for DynamicFollower_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__DynamicFollower_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__DynamicFollower_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DynamicFollower_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__DynamicFollower_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__DynamicFollower_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__DynamicFollower_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DynamicFollower_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DynamicFollower_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/DynamicFollower_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__DynamicFollower_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__DynamicFollower_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__DynamicFollower_Response__init(msg: *mut DynamicFollower_Response) -> bool;
    fn as2_msgs__srv__DynamicFollower_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DynamicFollower_Response>, size: usize) -> bool;
    fn as2_msgs__srv__DynamicFollower_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DynamicFollower_Response>);
    fn as2_msgs__srv__DynamicFollower_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DynamicFollower_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DynamicFollower_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__DynamicFollower_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DynamicFollower_Response {
    /// whether it could be started or not
    pub success: bool,

}



impl Default for DynamicFollower_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__DynamicFollower_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__DynamicFollower_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DynamicFollower_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__DynamicFollower_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__DynamicFollower_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__DynamicFollower_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DynamicFollower_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DynamicFollower_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/DynamicFollower_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__DynamicFollower_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__DynamicLand_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__DynamicLand_Request__init(msg: *mut DynamicLand_Request) -> bool;
    fn as2_msgs__srv__DynamicLand_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DynamicLand_Request>, size: usize) -> bool;
    fn as2_msgs__srv__DynamicLand_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DynamicLand_Request>);
    fn as2_msgs__srv__DynamicLand_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DynamicLand_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DynamicLand_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__DynamicLand_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DynamicLand_Request {
    /// Flag to enable land
    pub enable: bool,

    /// speed limit (m/s)
    pub speed_limit: geometry_msgs::msg::rmw::Twist,

}



impl Default for DynamicLand_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__DynamicLand_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__DynamicLand_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DynamicLand_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__DynamicLand_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__DynamicLand_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__DynamicLand_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DynamicLand_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DynamicLand_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/DynamicLand_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__DynamicLand_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__DynamicLand_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__DynamicLand_Response__init(msg: *mut DynamicLand_Response) -> bool;
    fn as2_msgs__srv__DynamicLand_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DynamicLand_Response>, size: usize) -> bool;
    fn as2_msgs__srv__DynamicLand_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DynamicLand_Response>);
    fn as2_msgs__srv__DynamicLand_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DynamicLand_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DynamicLand_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__DynamicLand_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DynamicLand_Response {
    /// whether it could be started or not
    pub success: bool,

}



impl Default for DynamicLand_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__DynamicLand_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__DynamicLand_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DynamicLand_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__DynamicLand_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__DynamicLand_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__DynamicLand_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DynamicLand_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DynamicLand_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/DynamicLand_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__DynamicLand_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__GeopathToPath_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__GeopathToPath_Request__init(msg: *mut GeopathToPath_Request) -> bool;
    fn as2_msgs__srv__GeopathToPath_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GeopathToPath_Request>, size: usize) -> bool;
    fn as2_msgs__srv__GeopathToPath_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GeopathToPath_Request>);
    fn as2_msgs__srv__GeopathToPath_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GeopathToPath_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GeopathToPath_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__GeopathToPath_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GeopathToPath_Request {
    /// Path in lat/lon and altitude
    pub geo_path: geographic_msgs::msg::rmw::GeoPath,

}



impl Default for GeopathToPath_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__GeopathToPath_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__GeopathToPath_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GeopathToPath_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GeopathToPath_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GeopathToPath_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GeopathToPath_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GeopathToPath_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GeopathToPath_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/GeopathToPath_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__GeopathToPath_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__GeopathToPath_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__GeopathToPath_Response__init(msg: *mut GeopathToPath_Response) -> bool;
    fn as2_msgs__srv__GeopathToPath_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GeopathToPath_Response>, size: usize) -> bool;
    fn as2_msgs__srv__GeopathToPath_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GeopathToPath_Response>);
    fn as2_msgs__srv__GeopathToPath_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GeopathToPath_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GeopathToPath_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__GeopathToPath_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GeopathToPath_Response {
    /// whether the origin has been set or not
    pub success: bool,

    /// Path in meters
    pub path: nav_msgs::msg::rmw::Path,

}



impl Default for GeopathToPath_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__GeopathToPath_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__GeopathToPath_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GeopathToPath_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GeopathToPath_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GeopathToPath_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GeopathToPath_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GeopathToPath_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GeopathToPath_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/GeopathToPath_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__GeopathToPath_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__GetGeozone_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__GetGeozone_Request__init(msg: *mut GetGeozone_Request) -> bool;
    fn as2_msgs__srv__GetGeozone_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetGeozone_Request>, size: usize) -> bool;
    fn as2_msgs__srv__GetGeozone_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetGeozone_Request>);
    fn as2_msgs__srv__GetGeozone_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetGeozone_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetGeozone_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__GetGeozone_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetGeozone_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetGeozone_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__GetGeozone_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__GetGeozone_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetGeozone_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GetGeozone_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GetGeozone_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GetGeozone_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetGeozone_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetGeozone_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/GetGeozone_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__GetGeozone_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__GetGeozone_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__GetGeozone_Response__init(msg: *mut GetGeozone_Response) -> bool;
    fn as2_msgs__srv__GetGeozone_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetGeozone_Response>, size: usize) -> bool;
    fn as2_msgs__srv__GetGeozone_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetGeozone_Response>);
    fn as2_msgs__srv__GetGeozone_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetGeozone_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetGeozone_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__GetGeozone_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetGeozone_Response {
    /// whether the geofence has been set or not
    pub success: bool,

    /// geofences stored in memory
    pub geozone_list: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Geozone>,

}



impl Default for GetGeozone_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__GetGeozone_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__GetGeozone_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetGeozone_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GetGeozone_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GetGeozone_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GetGeozone_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetGeozone_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetGeozone_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/GetGeozone_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__GetGeozone_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__GetOrigin_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__GetOrigin_Request__init(msg: *mut GetOrigin_Request) -> bool;
    fn as2_msgs__srv__GetOrigin_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetOrigin_Request>, size: usize) -> bool;
    fn as2_msgs__srv__GetOrigin_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetOrigin_Request>);
    fn as2_msgs__srv__GetOrigin_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetOrigin_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetOrigin_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__GetOrigin_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOrigin_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetOrigin_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__GetOrigin_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__GetOrigin_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetOrigin_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GetOrigin_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GetOrigin_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GetOrigin_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetOrigin_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetOrigin_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/GetOrigin_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__GetOrigin_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__GetOrigin_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__GetOrigin_Response__init(msg: *mut GetOrigin_Response) -> bool;
    fn as2_msgs__srv__GetOrigin_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetOrigin_Response>, size: usize) -> bool;
    fn as2_msgs__srv__GetOrigin_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetOrigin_Response>);
    fn as2_msgs__srv__GetOrigin_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetOrigin_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetOrigin_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__GetOrigin_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOrigin_Response {
    /// whether the origin has been set or not
    pub success: bool,

    /// origin
    pub origin: geographic_msgs::msg::rmw::GeoPoint,

}



impl Default for GetOrigin_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__GetOrigin_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__GetOrigin_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetOrigin_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GetOrigin_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GetOrigin_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__GetOrigin_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetOrigin_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetOrigin_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/GetOrigin_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__GetOrigin_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__ListControlModes_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__ListControlModes_Request__init(msg: *mut ListControlModes_Request) -> bool;
    fn as2_msgs__srv__ListControlModes_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListControlModes_Request>, size: usize) -> bool;
    fn as2_msgs__srv__ListControlModes_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListControlModes_Request>);
    fn as2_msgs__srv__ListControlModes_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListControlModes_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ListControlModes_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__ListControlModes_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListControlModes_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListControlModes_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__ListControlModes_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__ListControlModes_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListControlModes_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__ListControlModes_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__ListControlModes_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__ListControlModes_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListControlModes_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListControlModes_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/ListControlModes_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__ListControlModes_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__ListControlModes_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__ListControlModes_Response__init(msg: *mut ListControlModes_Response) -> bool;
    fn as2_msgs__srv__ListControlModes_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListControlModes_Response>, size: usize) -> bool;
    fn as2_msgs__srv__ListControlModes_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListControlModes_Response>);
    fn as2_msgs__srv__ListControlModes_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListControlModes_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ListControlModes_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__ListControlModes_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListControlModes_Response {
    /// Control modes source
    pub source: rosidl_runtime_rs::String,

    /// Control modes list
    pub control_modes: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for ListControlModes_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__ListControlModes_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__ListControlModes_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListControlModes_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__ListControlModes_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__ListControlModes_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__ListControlModes_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListControlModes_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListControlModes_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/ListControlModes_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__ListControlModes_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__ModifySwarm_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__ModifySwarm_Request__init(msg: *mut ModifySwarm_Request) -> bool;
    fn as2_msgs__srv__ModifySwarm_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ModifySwarm_Request>, size: usize) -> bool;
    fn as2_msgs__srv__ModifySwarm_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ModifySwarm_Request>);
    fn as2_msgs__srv__ModifySwarm_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ModifySwarm_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ModifySwarm_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__ModifySwarm_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModifySwarm_Request {
    /// Detach the drone reference from the swarm
    pub detach_drone: bool,

    /// Add new drone reference to the swarm
    pub new_drone: bool,

    /// New reference to follow
    pub new_virtual_centroid_ref: bool,

    /// Offset of the virtual centroid to the following frame
    pub virtual_centroid: geometry_msgs::msg::rmw::PoseStamped,

    /// Topics to modify the flocking
    pub swarm_formation: rosidl_runtime_rs::Sequence<super::super::msg::rmw::PoseWithID>,

}



impl Default for ModifySwarm_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__ModifySwarm_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__ModifySwarm_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ModifySwarm_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__ModifySwarm_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__ModifySwarm_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__ModifySwarm_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ModifySwarm_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ModifySwarm_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/ModifySwarm_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__ModifySwarm_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__ModifySwarm_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__ModifySwarm_Response__init(msg: *mut ModifySwarm_Response) -> bool;
    fn as2_msgs__srv__ModifySwarm_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ModifySwarm_Response>, size: usize) -> bool;
    fn as2_msgs__srv__ModifySwarm_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ModifySwarm_Response>);
    fn as2_msgs__srv__ModifySwarm_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ModifySwarm_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ModifySwarm_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__ModifySwarm_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModifySwarm_Response {
    /// whether the SwarmBehavior has been set or not
    pub success: bool,

}



impl Default for ModifySwarm_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__ModifySwarm_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__ModifySwarm_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ModifySwarm_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__ModifySwarm_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__ModifySwarm_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__ModifySwarm_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ModifySwarm_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ModifySwarm_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/ModifySwarm_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__ModifySwarm_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__PackagePickUp_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__PackagePickUp_Request__init(msg: *mut PackagePickUp_Request) -> bool;
    fn as2_msgs__srv__PackagePickUp_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PackagePickUp_Request>, size: usize) -> bool;
    fn as2_msgs__srv__PackagePickUp_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PackagePickUp_Request>);
    fn as2_msgs__srv__PackagePickUp_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PackagePickUp_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PackagePickUp_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__PackagePickUp_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PackagePickUp_Request {
    /// Flag to enable pickup
    pub enable: bool,

    /// speed limit (m/s)
    pub speed_limit: geometry_msgs::msg::rmw::Twist,

}



impl Default for PackagePickUp_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__PackagePickUp_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__PackagePickUp_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PackagePickUp_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PackagePickUp_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PackagePickUp_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PackagePickUp_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PackagePickUp_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PackagePickUp_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/PackagePickUp_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__PackagePickUp_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__PackagePickUp_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__PackagePickUp_Response__init(msg: *mut PackagePickUp_Response) -> bool;
    fn as2_msgs__srv__PackagePickUp_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PackagePickUp_Response>, size: usize) -> bool;
    fn as2_msgs__srv__PackagePickUp_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PackagePickUp_Response>);
    fn as2_msgs__srv__PackagePickUp_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PackagePickUp_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PackagePickUp_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__PackagePickUp_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PackagePickUp_Response {
    /// whether it could be started or not
    pub success: bool,

}



impl Default for PackagePickUp_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__PackagePickUp_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__PackagePickUp_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PackagePickUp_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PackagePickUp_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PackagePickUp_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PackagePickUp_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PackagePickUp_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PackagePickUp_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/PackagePickUp_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__PackagePickUp_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__PackageUnPick_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__PackageUnPick_Request__init(msg: *mut PackageUnPick_Request) -> bool;
    fn as2_msgs__srv__PackageUnPick_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PackageUnPick_Request>, size: usize) -> bool;
    fn as2_msgs__srv__PackageUnPick_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PackageUnPick_Request>);
    fn as2_msgs__srv__PackageUnPick_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PackageUnPick_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PackageUnPick_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__PackageUnPick_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PackageUnPick_Request {
    /// Flag to enable unpick
    pub enable: bool,

    /// speed limit (m/s)
    pub speed_limit: geometry_msgs::msg::rmw::Twist,

}



impl Default for PackageUnPick_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__PackageUnPick_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__PackageUnPick_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PackageUnPick_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PackageUnPick_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PackageUnPick_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PackageUnPick_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PackageUnPick_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PackageUnPick_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/PackageUnPick_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__PackageUnPick_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__PackageUnPick_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__PackageUnPick_Response__init(msg: *mut PackageUnPick_Response) -> bool;
    fn as2_msgs__srv__PackageUnPick_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PackageUnPick_Response>, size: usize) -> bool;
    fn as2_msgs__srv__PackageUnPick_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PackageUnPick_Response>);
    fn as2_msgs__srv__PackageUnPick_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PackageUnPick_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PackageUnPick_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__PackageUnPick_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PackageUnPick_Response {
    /// whether it could be started or not
    pub success: bool,

}



impl Default for PackageUnPick_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__PackageUnPick_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__PackageUnPick_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PackageUnPick_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PackageUnPick_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PackageUnPick_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PackageUnPick_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PackageUnPick_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PackageUnPick_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/PackageUnPick_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__PackageUnPick_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__PathToGeopath_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__PathToGeopath_Request__init(msg: *mut PathToGeopath_Request) -> bool;
    fn as2_msgs__srv__PathToGeopath_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PathToGeopath_Request>, size: usize) -> bool;
    fn as2_msgs__srv__PathToGeopath_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PathToGeopath_Request>);
    fn as2_msgs__srv__PathToGeopath_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PathToGeopath_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PathToGeopath_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__PathToGeopath_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PathToGeopath_Request {
    /// Path (m)
    pub path: nav_msgs::msg::rmw::Path,

}



impl Default for PathToGeopath_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__PathToGeopath_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__PathToGeopath_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PathToGeopath_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PathToGeopath_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PathToGeopath_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PathToGeopath_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PathToGeopath_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PathToGeopath_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/PathToGeopath_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__PathToGeopath_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__PathToGeopath_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__PathToGeopath_Response__init(msg: *mut PathToGeopath_Response) -> bool;
    fn as2_msgs__srv__PathToGeopath_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PathToGeopath_Response>, size: usize) -> bool;
    fn as2_msgs__srv__PathToGeopath_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PathToGeopath_Response>);
    fn as2_msgs__srv__PathToGeopath_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PathToGeopath_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PathToGeopath_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__PathToGeopath_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PathToGeopath_Response {
    /// whether the origin has been set or not
    pub success: bool,

    /// Path in lat/lon and altitude
    pub geo_path: geographic_msgs::msg::rmw::GeoPath,

}



impl Default for PathToGeopath_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__PathToGeopath_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__PathToGeopath_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PathToGeopath_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PathToGeopath_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PathToGeopath_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__PathToGeopath_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PathToGeopath_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PathToGeopath_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/PathToGeopath_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__PathToGeopath_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetControlMode_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__SetControlMode_Request__init(msg: *mut SetControlMode_Request) -> bool;
    fn as2_msgs__srv__SetControlMode_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetControlMode_Request>, size: usize) -> bool;
    fn as2_msgs__srv__SetControlMode_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetControlMode_Request>);
    fn as2_msgs__srv__SetControlMode_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetControlMode_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetControlMode_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__SetControlMode_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetControlMode_Request {
    /// Control mode to set
    pub control_mode: super::super::msg::rmw::ControlMode,

}



impl Default for SetControlMode_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__SetControlMode_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__SetControlMode_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetControlMode_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetControlMode_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetControlMode_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetControlMode_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetControlMode_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetControlMode_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/SetControlMode_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetControlMode_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetControlMode_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__SetControlMode_Response__init(msg: *mut SetControlMode_Response) -> bool;
    fn as2_msgs__srv__SetControlMode_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetControlMode_Response>, size: usize) -> bool;
    fn as2_msgs__srv__SetControlMode_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetControlMode_Response>);
    fn as2_msgs__srv__SetControlMode_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetControlMode_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetControlMode_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__SetControlMode_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetControlMode_Response {
    /// whether the control mode has been set or not
    pub success: bool,

}



impl Default for SetControlMode_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__SetControlMode_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__SetControlMode_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetControlMode_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetControlMode_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetControlMode_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetControlMode_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetControlMode_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetControlMode_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/SetControlMode_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetControlMode_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetGeozone_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__SetGeozone_Request__init(msg: *mut SetGeozone_Request) -> bool;
    fn as2_msgs__srv__SetGeozone_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetGeozone_Request>, size: usize) -> bool;
    fn as2_msgs__srv__SetGeozone_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetGeozone_Request>);
    fn as2_msgs__srv__SetGeozone_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetGeozone_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetGeozone_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__SetGeozone_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGeozone_Request {
    /// geostructure to set
    pub geozone: super::super::msg::rmw::Geozone,

}



impl Default for SetGeozone_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__SetGeozone_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__SetGeozone_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetGeozone_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetGeozone_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetGeozone_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetGeozone_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetGeozone_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetGeozone_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/SetGeozone_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetGeozone_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetGeozone_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__SetGeozone_Response__init(msg: *mut SetGeozone_Response) -> bool;
    fn as2_msgs__srv__SetGeozone_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetGeozone_Response>, size: usize) -> bool;
    fn as2_msgs__srv__SetGeozone_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetGeozone_Response>);
    fn as2_msgs__srv__SetGeozone_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetGeozone_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetGeozone_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__SetGeozone_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGeozone_Response {
    /// whether the geoStructure has been set or not
    pub success: bool,

}



impl Default for SetGeozone_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__SetGeozone_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__SetGeozone_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetGeozone_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetGeozone_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetGeozone_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetGeozone_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetGeozone_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetGeozone_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/SetGeozone_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetGeozone_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetOrigin_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__SetOrigin_Request__init(msg: *mut SetOrigin_Request) -> bool;
    fn as2_msgs__srv__SetOrigin_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOrigin_Request>, size: usize) -> bool;
    fn as2_msgs__srv__SetOrigin_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOrigin_Request>);
    fn as2_msgs__srv__SetOrigin_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOrigin_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOrigin_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__SetOrigin_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOrigin_Request {
    /// origin to set
    pub origin: geographic_msgs::msg::rmw::GeoPoint,

}



impl Default for SetOrigin_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__SetOrigin_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__SetOrigin_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOrigin_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetOrigin_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetOrigin_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetOrigin_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOrigin_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOrigin_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/SetOrigin_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetOrigin_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetOrigin_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__SetOrigin_Response__init(msg: *mut SetOrigin_Response) -> bool;
    fn as2_msgs__srv__SetOrigin_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOrigin_Response>, size: usize) -> bool;
    fn as2_msgs__srv__SetOrigin_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOrigin_Response>);
    fn as2_msgs__srv__SetOrigin_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOrigin_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOrigin_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__SetOrigin_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOrigin_Response {
    /// whether the origin has been set or not
    pub success: bool,

}



impl Default for SetOrigin_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__SetOrigin_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__SetOrigin_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOrigin_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetOrigin_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetOrigin_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetOrigin_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOrigin_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOrigin_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/SetOrigin_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetOrigin_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetPlatformStateMachineEvent_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__SetPlatformStateMachineEvent_Request__init(msg: *mut SetPlatformStateMachineEvent_Request) -> bool;
    fn as2_msgs__srv__SetPlatformStateMachineEvent_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetPlatformStateMachineEvent_Request>, size: usize) -> bool;
    fn as2_msgs__srv__SetPlatformStateMachineEvent_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetPlatformStateMachineEvent_Request>);
    fn as2_msgs__srv__SetPlatformStateMachineEvent_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetPlatformStateMachineEvent_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetPlatformStateMachineEvent_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__SetPlatformStateMachineEvent_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPlatformStateMachineEvent_Request {
    /// event to set
    pub event: super::super::msg::rmw::PlatformStateMachineEvent,

}



impl Default for SetPlatformStateMachineEvent_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__SetPlatformStateMachineEvent_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__SetPlatformStateMachineEvent_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetPlatformStateMachineEvent_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetPlatformStateMachineEvent_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetPlatformStateMachineEvent_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetPlatformStateMachineEvent_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetPlatformStateMachineEvent_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetPlatformStateMachineEvent_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/SetPlatformStateMachineEvent_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetPlatformStateMachineEvent_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetPlatformStateMachineEvent_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__SetPlatformStateMachineEvent_Response__init(msg: *mut SetPlatformStateMachineEvent_Response) -> bool;
    fn as2_msgs__srv__SetPlatformStateMachineEvent_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetPlatformStateMachineEvent_Response>, size: usize) -> bool;
    fn as2_msgs__srv__SetPlatformStateMachineEvent_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetPlatformStateMachineEvent_Response>);
    fn as2_msgs__srv__SetPlatformStateMachineEvent_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetPlatformStateMachineEvent_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetPlatformStateMachineEvent_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__SetPlatformStateMachineEvent_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPlatformStateMachineEvent_Response {
    /// whether the PSM has been set or not
    pub success: bool,

    /// PSM result of the aircraft
    pub current_state: super::super::msg::rmw::PlatformStatus,

}



impl Default for SetPlatformStateMachineEvent_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__SetPlatformStateMachineEvent_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__SetPlatformStateMachineEvent_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetPlatformStateMachineEvent_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetPlatformStateMachineEvent_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetPlatformStateMachineEvent_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetPlatformStateMachineEvent_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetPlatformStateMachineEvent_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetPlatformStateMachineEvent_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/SetPlatformStateMachineEvent_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetPlatformStateMachineEvent_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetSpeed_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__SetSpeed_Request__init(msg: *mut SetSpeed_Request) -> bool;
    fn as2_msgs__srv__SetSpeed_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetSpeed_Request>, size: usize) -> bool;
    fn as2_msgs__srv__SetSpeed_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetSpeed_Request>);
    fn as2_msgs__srv__SetSpeed_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetSpeed_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetSpeed_Request>) -> bool;
}

// Corresponds to as2_msgs__srv__SetSpeed_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetSpeed_Request {
    /// speed to send
    pub speed: super::super::msg::rmw::Speed,

}



impl Default for SetSpeed_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__SetSpeed_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__SetSpeed_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetSpeed_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetSpeed_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetSpeed_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetSpeed_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetSpeed_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetSpeed_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/SetSpeed_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetSpeed_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetSpeed_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__srv__SetSpeed_Response__init(msg: *mut SetSpeed_Response) -> bool;
    fn as2_msgs__srv__SetSpeed_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetSpeed_Response>, size: usize) -> bool;
    fn as2_msgs__srv__SetSpeed_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetSpeed_Response>);
    fn as2_msgs__srv__SetSpeed_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetSpeed_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetSpeed_Response>) -> bool;
}

// Corresponds to as2_msgs__srv__SetSpeed_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetSpeed_Response {
    /// whether the speed has been received or not
    pub success: bool,

}



impl Default for SetSpeed_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__srv__SetSpeed_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__srv__SetSpeed_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetSpeed_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetSpeed_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetSpeed_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__srv__SetSpeed_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetSpeed_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetSpeed_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/srv/SetSpeed_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__srv__SetSpeed_Response() }
  }
}






#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__AddStaticTransform() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__AddStaticTransform
#[allow(missing_docs, non_camel_case_types)]
pub struct AddStaticTransform;

impl rosidl_runtime_rs::Service for AddStaticTransform {
    type Request = AddStaticTransform_Request;
    type Response = AddStaticTransform_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__AddStaticTransform() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__AddStaticTransformGps() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__AddStaticTransformGps
#[allow(missing_docs, non_camel_case_types)]
pub struct AddStaticTransformGps;

impl rosidl_runtime_rs::Service for AddStaticTransformGps {
    type Request = AddStaticTransformGps_Request;
    type Response = AddStaticTransformGps_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__AddStaticTransformGps() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__DynamicFollower() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__DynamicFollower
#[allow(missing_docs, non_camel_case_types)]
pub struct DynamicFollower;

impl rosidl_runtime_rs::Service for DynamicFollower {
    type Request = DynamicFollower_Request;
    type Response = DynamicFollower_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__DynamicFollower() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__DynamicLand() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__DynamicLand
#[allow(missing_docs, non_camel_case_types)]
pub struct DynamicLand;

impl rosidl_runtime_rs::Service for DynamicLand {
    type Request = DynamicLand_Request;
    type Response = DynamicLand_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__DynamicLand() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__GeopathToPath() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__GeopathToPath
#[allow(missing_docs, non_camel_case_types)]
pub struct GeopathToPath;

impl rosidl_runtime_rs::Service for GeopathToPath {
    type Request = GeopathToPath_Request;
    type Response = GeopathToPath_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__GeopathToPath() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__GetGeozone() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__GetGeozone
#[allow(missing_docs, non_camel_case_types)]
pub struct GetGeozone;

impl rosidl_runtime_rs::Service for GetGeozone {
    type Request = GetGeozone_Request;
    type Response = GetGeozone_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__GetGeozone() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__GetOrigin() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__GetOrigin
#[allow(missing_docs, non_camel_case_types)]
pub struct GetOrigin;

impl rosidl_runtime_rs::Service for GetOrigin {
    type Request = GetOrigin_Request;
    type Response = GetOrigin_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__GetOrigin() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__ListControlModes() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__ListControlModes
#[allow(missing_docs, non_camel_case_types)]
pub struct ListControlModes;

impl rosidl_runtime_rs::Service for ListControlModes {
    type Request = ListControlModes_Request;
    type Response = ListControlModes_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__ListControlModes() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__ModifySwarm() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__ModifySwarm
#[allow(missing_docs, non_camel_case_types)]
pub struct ModifySwarm;

impl rosidl_runtime_rs::Service for ModifySwarm {
    type Request = ModifySwarm_Request;
    type Response = ModifySwarm_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__ModifySwarm() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__PackagePickUp() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__PackagePickUp
#[allow(missing_docs, non_camel_case_types)]
pub struct PackagePickUp;

impl rosidl_runtime_rs::Service for PackagePickUp {
    type Request = PackagePickUp_Request;
    type Response = PackagePickUp_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__PackagePickUp() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__PackageUnPick() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__PackageUnPick
#[allow(missing_docs, non_camel_case_types)]
pub struct PackageUnPick;

impl rosidl_runtime_rs::Service for PackageUnPick {
    type Request = PackageUnPick_Request;
    type Response = PackageUnPick_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__PackageUnPick() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__PathToGeopath() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__PathToGeopath
#[allow(missing_docs, non_camel_case_types)]
pub struct PathToGeopath;

impl rosidl_runtime_rs::Service for PathToGeopath {
    type Request = PathToGeopath_Request;
    type Response = PathToGeopath_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__PathToGeopath() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__SetControlMode() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__SetControlMode
#[allow(missing_docs, non_camel_case_types)]
pub struct SetControlMode;

impl rosidl_runtime_rs::Service for SetControlMode {
    type Request = SetControlMode_Request;
    type Response = SetControlMode_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__SetControlMode() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__SetGeozone() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__SetGeozone
#[allow(missing_docs, non_camel_case_types)]
pub struct SetGeozone;

impl rosidl_runtime_rs::Service for SetGeozone {
    type Request = SetGeozone_Request;
    type Response = SetGeozone_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__SetGeozone() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__SetOrigin() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__SetOrigin
#[allow(missing_docs, non_camel_case_types)]
pub struct SetOrigin;

impl rosidl_runtime_rs::Service for SetOrigin {
    type Request = SetOrigin_Request;
    type Response = SetOrigin_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__SetOrigin() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__SetPlatformStateMachineEvent() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__SetPlatformStateMachineEvent
#[allow(missing_docs, non_camel_case_types)]
pub struct SetPlatformStateMachineEvent;

impl rosidl_runtime_rs::Service for SetPlatformStateMachineEvent {
    type Request = SetPlatformStateMachineEvent_Request;
    type Response = SetPlatformStateMachineEvent_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__SetPlatformStateMachineEvent() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__SetSpeed() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__srv__SetSpeed
#[allow(missing_docs, non_camel_case_types)]
pub struct SetSpeed;

impl rosidl_runtime_rs::Service for SetSpeed {
    type Request = SetSpeed_Request;
    type Response = SetSpeed_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__srv__SetSpeed() }
    }
}


