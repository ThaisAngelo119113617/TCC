
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__DetectArucoMarkers_Goal() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__DetectArucoMarkers_Goal__init(msg: *mut DetectArucoMarkers_Goal) -> bool;
    fn as2_msgs__action__DetectArucoMarkers_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_Goal>, size: usize) -> bool;
    fn as2_msgs__action__DetectArucoMarkers_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_Goal>);
    fn as2_msgs__action__DetectArucoMarkers_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectArucoMarkers_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_Goal>) -> bool;
}

// Corresponds to as2_msgs__action__DetectArucoMarkers_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectArucoMarkers_Goal {
    /// Request
    pub target_ids: rosidl_runtime_rs::Sequence<u16>,

}



impl Default for DetectArucoMarkers_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__DetectArucoMarkers_Goal__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__DetectArucoMarkers_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectArucoMarkers_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectArucoMarkers_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectArucoMarkers_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/DetectArucoMarkers_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__DetectArucoMarkers_Goal() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__DetectArucoMarkers_Result() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__DetectArucoMarkers_Result__init(msg: *mut DetectArucoMarkers_Result) -> bool;
    fn as2_msgs__action__DetectArucoMarkers_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_Result>, size: usize) -> bool;
    fn as2_msgs__action__DetectArucoMarkers_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_Result>);
    fn as2_msgs__action__DetectArucoMarkers_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectArucoMarkers_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_Result>) -> bool;
}

// Corresponds to as2_msgs__action__DetectArucoMarkers_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectArucoMarkers_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for DetectArucoMarkers_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__DetectArucoMarkers_Result__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__DetectArucoMarkers_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectArucoMarkers_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectArucoMarkers_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectArucoMarkers_Result where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/DetectArucoMarkers_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__DetectArucoMarkers_Result() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__DetectArucoMarkers_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__DetectArucoMarkers_Feedback__init(msg: *mut DetectArucoMarkers_Feedback) -> bool;
    fn as2_msgs__action__DetectArucoMarkers_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_Feedback>, size: usize) -> bool;
    fn as2_msgs__action__DetectArucoMarkers_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_Feedback>);
    fn as2_msgs__action__DetectArucoMarkers_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectArucoMarkers_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_Feedback>) -> bool;
}

// Corresponds to as2_msgs__action__DetectArucoMarkers_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectArucoMarkers_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub sucess: bool,

}



impl Default for DetectArucoMarkers_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__DetectArucoMarkers_Feedback__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__DetectArucoMarkers_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectArucoMarkers_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectArucoMarkers_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectArucoMarkers_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/DetectArucoMarkers_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__DetectArucoMarkers_Feedback() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__DetectArucoMarkers_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__DetectArucoMarkers_FeedbackMessage__init(msg: *mut DetectArucoMarkers_FeedbackMessage) -> bool;
    fn as2_msgs__action__DetectArucoMarkers_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_FeedbackMessage>, size: usize) -> bool;
    fn as2_msgs__action__DetectArucoMarkers_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_FeedbackMessage>);
    fn as2_msgs__action__DetectArucoMarkers_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectArucoMarkers_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_FeedbackMessage>) -> bool;
}

// Corresponds to as2_msgs__action__DetectArucoMarkers_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectArucoMarkers_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::DetectArucoMarkers_Feedback,

}



impl Default for DetectArucoMarkers_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__DetectArucoMarkers_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__DetectArucoMarkers_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectArucoMarkers_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectArucoMarkers_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectArucoMarkers_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/DetectArucoMarkers_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__DetectArucoMarkers_FeedbackMessage() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowPath_Goal() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__FollowPath_Goal__init(msg: *mut FollowPath_Goal) -> bool;
    fn as2_msgs__action__FollowPath_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowPath_Goal>, size: usize) -> bool;
    fn as2_msgs__action__FollowPath_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowPath_Goal>);
    fn as2_msgs__action__FollowPath_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowPath_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowPath_Goal>) -> bool;
}

// Corresponds to as2_msgs__action__FollowPath_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowPath_Goal {
    /// Request
    /// Message header, with the frame of the pose list
    pub header: std_msgs::msg::rmw::Header,

    /// Yaw goal mode
    pub yaw: super::super::msg::rmw::YawMode,

    /// List of poses with ID in path
    pub path: rosidl_runtime_rs::Sequence<super::super::msg::rmw::PoseWithID>,

    /// Maximum speed desired in path (m/s)
    pub max_speed: f32,

}



impl Default for FollowPath_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__FollowPath_Goal__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__FollowPath_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowPath_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowPath_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowPath_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/FollowPath_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowPath_Goal() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowPath_Result() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__FollowPath_Result__init(msg: *mut FollowPath_Result) -> bool;
    fn as2_msgs__action__FollowPath_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowPath_Result>, size: usize) -> bool;
    fn as2_msgs__action__FollowPath_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowPath_Result>);
    fn as2_msgs__action__FollowPath_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowPath_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowPath_Result>) -> bool;
}

// Corresponds to as2_msgs__action__FollowPath_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowPath_Result {
    /// False if failed to follow_path
    pub follow_path_success: bool,

}



impl Default for FollowPath_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__FollowPath_Result__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__FollowPath_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowPath_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowPath_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowPath_Result where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/FollowPath_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowPath_Result() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowPath_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__FollowPath_Feedback__init(msg: *mut FollowPath_Feedback) -> bool;
    fn as2_msgs__action__FollowPath_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowPath_Feedback>, size: usize) -> bool;
    fn as2_msgs__action__FollowPath_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowPath_Feedback>);
    fn as2_msgs__action__FollowPath_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowPath_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowPath_Feedback>) -> bool;
}

// Corresponds to as2_msgs__action__FollowPath_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowPath_Feedback {
    /// Actual speed (m/s)
    pub actual_speed: f32,

    /// Distance to next waypoint (m)
    pub actual_distance_to_next_waypoint: f32,

    /// Remaining_waypoints
    pub remaining_waypoints: u16,

    /// Next waypoint id in path to follow
    pub next_waypoint_id: rosidl_runtime_rs::String,

}



impl Default for FollowPath_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__FollowPath_Feedback__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__FollowPath_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowPath_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowPath_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowPath_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/FollowPath_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowPath_Feedback() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowPath_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__FollowPath_FeedbackMessage__init(msg: *mut FollowPath_FeedbackMessage) -> bool;
    fn as2_msgs__action__FollowPath_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowPath_FeedbackMessage>, size: usize) -> bool;
    fn as2_msgs__action__FollowPath_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowPath_FeedbackMessage>);
    fn as2_msgs__action__FollowPath_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowPath_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowPath_FeedbackMessage>) -> bool;
}

// Corresponds to as2_msgs__action__FollowPath_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowPath_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::FollowPath_Feedback,

}



impl Default for FollowPath_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__FollowPath_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__FollowPath_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowPath_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowPath_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowPath_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/FollowPath_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowPath_FeedbackMessage() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowReference_Goal() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__FollowReference_Goal__init(msg: *mut FollowReference_Goal) -> bool;
    fn as2_msgs__action__FollowReference_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowReference_Goal>, size: usize) -> bool;
    fn as2_msgs__action__FollowReference_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowReference_Goal>);
    fn as2_msgs__action__FollowReference_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowReference_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowReference_Goal>) -> bool;
}

// Corresponds to as2_msgs__action__FollowReference_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowReference_Goal {
    /// Request
    /// Yaw mode
    pub yaw: super::super::msg::rmw::YawMode,

    /// Goal pose 3D (m)
    pub target_pose: geometry_msgs::msg::rmw::PointStamped,

    /// Maximum speed in x (m/s)
    pub max_speed_x: f32,

    /// Maximum speed in x (m/s)
    pub max_speed_y: f32,

    /// Maximum speed in x (m/s)
    pub max_speed_z: f32,

}



impl Default for FollowReference_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__FollowReference_Goal__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__FollowReference_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowReference_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowReference_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowReference_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/FollowReference_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowReference_Goal() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowReference_Result() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__FollowReference_Result__init(msg: *mut FollowReference_Result) -> bool;
    fn as2_msgs__action__FollowReference_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowReference_Result>, size: usize) -> bool;
    fn as2_msgs__action__FollowReference_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowReference_Result>);
    fn as2_msgs__action__FollowReference_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowReference_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowReference_Result>) -> bool;
}

// Corresponds to as2_msgs__action__FollowReference_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowReference_Result {
    /// False if failed to takeoff
    pub follow_reference_success: bool,

}



impl Default for FollowReference_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__FollowReference_Result__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__FollowReference_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowReference_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowReference_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowReference_Result where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/FollowReference_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowReference_Result() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowReference_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__FollowReference_Feedback__init(msg: *mut FollowReference_Feedback) -> bool;
    fn as2_msgs__action__FollowReference_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowReference_Feedback>, size: usize) -> bool;
    fn as2_msgs__action__FollowReference_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowReference_Feedback>);
    fn as2_msgs__action__FollowReference_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowReference_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowReference_Feedback>) -> bool;
}

// Corresponds to as2_msgs__action__FollowReference_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowReference_Feedback {
    /// Actual speed (m/s)
    pub actual_speed: f32,

    /// Distance to goal (m)
    pub actual_distance_to_goal: f32,

}



impl Default for FollowReference_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__FollowReference_Feedback__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__FollowReference_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowReference_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowReference_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowReference_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/FollowReference_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowReference_Feedback() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowReference_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__FollowReference_FeedbackMessage__init(msg: *mut FollowReference_FeedbackMessage) -> bool;
    fn as2_msgs__action__FollowReference_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowReference_FeedbackMessage>, size: usize) -> bool;
    fn as2_msgs__action__FollowReference_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowReference_FeedbackMessage>);
    fn as2_msgs__action__FollowReference_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowReference_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowReference_FeedbackMessage>) -> bool;
}

// Corresponds to as2_msgs__action__FollowReference_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowReference_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::FollowReference_Feedback,

}



impl Default for FollowReference_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__FollowReference_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__FollowReference_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowReference_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowReference_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowReference_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/FollowReference_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowReference_FeedbackMessage() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__ForceEstimation_Goal() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__ForceEstimation_Goal__init(msg: *mut ForceEstimation_Goal) -> bool;
    fn as2_msgs__action__ForceEstimation_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_Goal>, size: usize) -> bool;
    fn as2_msgs__action__ForceEstimation_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_Goal>);
    fn as2_msgs__action__ForceEstimation_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ForceEstimation_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_Goal>) -> bool;
}

// Corresponds to as2_msgs__action__ForceEstimation_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ForceEstimation_Goal {
    /// Request
    pub request: bool,

}



impl Default for ForceEstimation_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__ForceEstimation_Goal__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__ForceEstimation_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ForceEstimation_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ForceEstimation_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ForceEstimation_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/ForceEstimation_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__ForceEstimation_Goal() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__ForceEstimation_Result() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__ForceEstimation_Result__init(msg: *mut ForceEstimation_Result) -> bool;
    fn as2_msgs__action__ForceEstimation_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_Result>, size: usize) -> bool;
    fn as2_msgs__action__ForceEstimation_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_Result>);
    fn as2_msgs__action__ForceEstimation_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ForceEstimation_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_Result>) -> bool;
}

// Corresponds to as2_msgs__action__ForceEstimation_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ForceEstimation_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for ForceEstimation_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__ForceEstimation_Result__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__ForceEstimation_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ForceEstimation_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ForceEstimation_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ForceEstimation_Result where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/ForceEstimation_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__ForceEstimation_Result() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__ForceEstimation_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__ForceEstimation_Feedback__init(msg: *mut ForceEstimation_Feedback) -> bool;
    fn as2_msgs__action__ForceEstimation_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_Feedback>, size: usize) -> bool;
    fn as2_msgs__action__ForceEstimation_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_Feedback>);
    fn as2_msgs__action__ForceEstimation_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ForceEstimation_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_Feedback>) -> bool;
}

// Corresponds to as2_msgs__action__ForceEstimation_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ForceEstimation_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ForceEstimation_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__ForceEstimation_Feedback__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__ForceEstimation_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ForceEstimation_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ForceEstimation_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ForceEstimation_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/ForceEstimation_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__ForceEstimation_Feedback() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__ForceEstimation_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__ForceEstimation_FeedbackMessage__init(msg: *mut ForceEstimation_FeedbackMessage) -> bool;
    fn as2_msgs__action__ForceEstimation_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_FeedbackMessage>, size: usize) -> bool;
    fn as2_msgs__action__ForceEstimation_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_FeedbackMessage>);
    fn as2_msgs__action__ForceEstimation_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ForceEstimation_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_FeedbackMessage>) -> bool;
}

// Corresponds to as2_msgs__action__ForceEstimation_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ForceEstimation_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::ForceEstimation_Feedback,

}



impl Default for ForceEstimation_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__ForceEstimation_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__ForceEstimation_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ForceEstimation_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ForceEstimation_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ForceEstimation_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/ForceEstimation_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__ForceEstimation_FeedbackMessage() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_Goal() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GeneratePolynomialTrajectory_Goal__init(msg: *mut GeneratePolynomialTrajectory_Goal) -> bool;
    fn as2_msgs__action__GeneratePolynomialTrajectory_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_Goal>, size: usize) -> bool;
    fn as2_msgs__action__GeneratePolynomialTrajectory_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_Goal>);
    fn as2_msgs__action__GeneratePolynomialTrajectory_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_Goal>) -> bool;
}

// Corresponds to as2_msgs__action__GeneratePolynomialTrajectory_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GeneratePolynomialTrajectory_Goal {
    /// Request
    /// Request timestamp
    pub stamp: builtin_interfaces::msg::rmw::Time,

    /// Yaw goal mode
    pub yaw: super::super::msg::rmw::YawMode,

    /// List of poses with ID in path, with each frame id and time stamp
    pub path: rosidl_runtime_rs::Sequence<super::super::msg::rmw::PoseStampedWithID>,

    /// Maximum speed desired in path (m/s)
    pub max_speed: f32,

}



impl Default for GeneratePolynomialTrajectory_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GeneratePolynomialTrajectory_Goal__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GeneratePolynomialTrajectory_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GeneratePolynomialTrajectory_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GeneratePolynomialTrajectory_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GeneratePolynomialTrajectory_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GeneratePolynomialTrajectory_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_Goal() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_Result() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GeneratePolynomialTrajectory_Result__init(msg: *mut GeneratePolynomialTrajectory_Result) -> bool;
    fn as2_msgs__action__GeneratePolynomialTrajectory_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_Result>, size: usize) -> bool;
    fn as2_msgs__action__GeneratePolynomialTrajectory_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_Result>);
    fn as2_msgs__action__GeneratePolynomialTrajectory_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_Result>) -> bool;
}

// Corresponds to as2_msgs__action__GeneratePolynomialTrajectory_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GeneratePolynomialTrajectory_Result {
    /// False if failed to follow the generated trajectory
    pub trajectory_generator_success: bool,

}



impl Default for GeneratePolynomialTrajectory_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GeneratePolynomialTrajectory_Result__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GeneratePolynomialTrajectory_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GeneratePolynomialTrajectory_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GeneratePolynomialTrajectory_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GeneratePolynomialTrajectory_Result where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GeneratePolynomialTrajectory_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_Result() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GeneratePolynomialTrajectory_Feedback__init(msg: *mut GeneratePolynomialTrajectory_Feedback) -> bool;
    fn as2_msgs__action__GeneratePolynomialTrajectory_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_Feedback>, size: usize) -> bool;
    fn as2_msgs__action__GeneratePolynomialTrajectory_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_Feedback>);
    fn as2_msgs__action__GeneratePolynomialTrajectory_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_Feedback>) -> bool;
}

// Corresponds to as2_msgs__action__GeneratePolynomialTrajectory_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GeneratePolynomialTrajectory_Feedback {
    /// Next waypoint id in path to follow
    pub next_waypoint_id: rosidl_runtime_rs::String,

    /// Number of remaining waypoints to follow
    pub remaining_waypoints: u16,

}



impl Default for GeneratePolynomialTrajectory_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GeneratePolynomialTrajectory_Feedback__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GeneratePolynomialTrajectory_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GeneratePolynomialTrajectory_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GeneratePolynomialTrajectory_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GeneratePolynomialTrajectory_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GeneratePolynomialTrajectory_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_Feedback() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GeneratePolynomialTrajectory_FeedbackMessage__init(msg: *mut GeneratePolynomialTrajectory_FeedbackMessage) -> bool;
    fn as2_msgs__action__GeneratePolynomialTrajectory_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_FeedbackMessage>, size: usize) -> bool;
    fn as2_msgs__action__GeneratePolynomialTrajectory_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_FeedbackMessage>);
    fn as2_msgs__action__GeneratePolynomialTrajectory_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_FeedbackMessage>) -> bool;
}

// Corresponds to as2_msgs__action__GeneratePolynomialTrajectory_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GeneratePolynomialTrajectory_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::GeneratePolynomialTrajectory_Feedback,

}



impl Default for GeneratePolynomialTrajectory_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GeneratePolynomialTrajectory_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GeneratePolynomialTrajectory_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GeneratePolynomialTrajectory_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GeneratePolynomialTrajectory_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GeneratePolynomialTrajectory_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GeneratePolynomialTrajectory_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_FeedbackMessage() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GoToWaypoint_Goal() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GoToWaypoint_Goal__init(msg: *mut GoToWaypoint_Goal) -> bool;
    fn as2_msgs__action__GoToWaypoint_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_Goal>, size: usize) -> bool;
    fn as2_msgs__action__GoToWaypoint_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_Goal>);
    fn as2_msgs__action__GoToWaypoint_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GoToWaypoint_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_Goal>) -> bool;
}

// Corresponds to as2_msgs__action__GoToWaypoint_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoToWaypoint_Goal {
    /// Request
    /// Yaw mode
    pub yaw: super::super::msg::rmw::YawMode,

    /// Goal pose 3D (m)
    pub target_pose: geometry_msgs::msg::rmw::PointStamped,

    /// Maximum speed (m/s)
    pub max_speed: f32,

}



impl Default for GoToWaypoint_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GoToWaypoint_Goal__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GoToWaypoint_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GoToWaypoint_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GoToWaypoint_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GoToWaypoint_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GoToWaypoint_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GoToWaypoint_Goal() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GoToWaypoint_Result() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GoToWaypoint_Result__init(msg: *mut GoToWaypoint_Result) -> bool;
    fn as2_msgs__action__GoToWaypoint_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_Result>, size: usize) -> bool;
    fn as2_msgs__action__GoToWaypoint_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_Result>);
    fn as2_msgs__action__GoToWaypoint_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GoToWaypoint_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_Result>) -> bool;
}

// Corresponds to as2_msgs__action__GoToWaypoint_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoToWaypoint_Result {
    /// False if failed to takeoff
    pub go_to_success: bool,

}



impl Default for GoToWaypoint_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GoToWaypoint_Result__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GoToWaypoint_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GoToWaypoint_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GoToWaypoint_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GoToWaypoint_Result where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GoToWaypoint_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GoToWaypoint_Result() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GoToWaypoint_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GoToWaypoint_Feedback__init(msg: *mut GoToWaypoint_Feedback) -> bool;
    fn as2_msgs__action__GoToWaypoint_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_Feedback>, size: usize) -> bool;
    fn as2_msgs__action__GoToWaypoint_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_Feedback>);
    fn as2_msgs__action__GoToWaypoint_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GoToWaypoint_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_Feedback>) -> bool;
}

// Corresponds to as2_msgs__action__GoToWaypoint_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoToWaypoint_Feedback {
    /// Actual speed (m/s)
    pub actual_speed: f32,

    /// Distance to goal (m)
    pub actual_distance_to_goal: f32,

}



impl Default for GoToWaypoint_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GoToWaypoint_Feedback__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GoToWaypoint_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GoToWaypoint_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GoToWaypoint_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GoToWaypoint_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GoToWaypoint_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GoToWaypoint_Feedback() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GoToWaypoint_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GoToWaypoint_FeedbackMessage__init(msg: *mut GoToWaypoint_FeedbackMessage) -> bool;
    fn as2_msgs__action__GoToWaypoint_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_FeedbackMessage>, size: usize) -> bool;
    fn as2_msgs__action__GoToWaypoint_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_FeedbackMessage>);
    fn as2_msgs__action__GoToWaypoint_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GoToWaypoint_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_FeedbackMessage>) -> bool;
}

// Corresponds to as2_msgs__action__GoToWaypoint_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoToWaypoint_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::GoToWaypoint_Feedback,

}



impl Default for GoToWaypoint_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GoToWaypoint_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GoToWaypoint_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GoToWaypoint_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GoToWaypoint_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GoToWaypoint_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GoToWaypoint_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GoToWaypoint_FeedbackMessage() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GripperHandler_Goal() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GripperHandler_Goal__init(msg: *mut GripperHandler_Goal) -> bool;
    fn as2_msgs__action__GripperHandler_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_Goal>, size: usize) -> bool;
    fn as2_msgs__action__GripperHandler_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_Goal>);
    fn as2_msgs__action__GripperHandler_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperHandler_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_Goal>) -> bool;
}

// Corresponds to as2_msgs__action__GripperHandler_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperHandler_Goal {
    /// Request
    /// Request to active the gripper. True: Close, False: Open
    pub request_gripper: bool,

}



impl Default for GripperHandler_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GripperHandler_Goal__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GripperHandler_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperHandler_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperHandler_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperHandler_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GripperHandler_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GripperHandler_Goal() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GripperHandler_Result() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GripperHandler_Result__init(msg: *mut GripperHandler_Result) -> bool;
    fn as2_msgs__action__GripperHandler_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_Result>, size: usize) -> bool;
    fn as2_msgs__action__GripperHandler_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_Result>);
    fn as2_msgs__action__GripperHandler_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperHandler_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_Result>) -> bool;
}

// Corresponds to as2_msgs__action__GripperHandler_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperHandler_Result {
    /// false if failed to handler the gripper
    pub gripper_success: bool,

}



impl Default for GripperHandler_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GripperHandler_Result__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GripperHandler_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperHandler_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperHandler_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperHandler_Result where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GripperHandler_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GripperHandler_Result() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GripperHandler_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GripperHandler_Feedback__init(msg: *mut GripperHandler_Feedback) -> bool;
    fn as2_msgs__action__GripperHandler_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_Feedback>, size: usize) -> bool;
    fn as2_msgs__action__GripperHandler_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_Feedback>);
    fn as2_msgs__action__GripperHandler_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperHandler_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_Feedback>) -> bool;
}

// Corresponds to as2_msgs__action__GripperHandler_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperHandler_Feedback {
    /// True: Close, False: Open
    pub state_gripper: bool,

}



impl Default for GripperHandler_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GripperHandler_Feedback__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GripperHandler_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperHandler_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperHandler_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperHandler_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GripperHandler_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GripperHandler_Feedback() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GripperHandler_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GripperHandler_FeedbackMessage__init(msg: *mut GripperHandler_FeedbackMessage) -> bool;
    fn as2_msgs__action__GripperHandler_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_FeedbackMessage>, size: usize) -> bool;
    fn as2_msgs__action__GripperHandler_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_FeedbackMessage>);
    fn as2_msgs__action__GripperHandler_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperHandler_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_FeedbackMessage>) -> bool;
}

// Corresponds to as2_msgs__action__GripperHandler_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperHandler_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::GripperHandler_Feedback,

}



impl Default for GripperHandler_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GripperHandler_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GripperHandler_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperHandler_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperHandler_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperHandler_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GripperHandler_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GripperHandler_FeedbackMessage() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Land_Goal() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__Land_Goal__init(msg: *mut Land_Goal) -> bool;
    fn as2_msgs__action__Land_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Land_Goal>, size: usize) -> bool;
    fn as2_msgs__action__Land_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Land_Goal>);
    fn as2_msgs__action__Land_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Land_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<Land_Goal>) -> bool;
}

// Corresponds to as2_msgs__action__Land_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Land_Goal {
    /// Request
    /// land speed (m/s)
    pub land_speed: f32,

}



impl Default for Land_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__Land_Goal__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__Land_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Land_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Land_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Land_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/Land_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Land_Goal() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Land_Result() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__Land_Result__init(msg: *mut Land_Result) -> bool;
    fn as2_msgs__action__Land_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Land_Result>, size: usize) -> bool;
    fn as2_msgs__action__Land_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Land_Result>);
    fn as2_msgs__action__Land_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Land_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<Land_Result>) -> bool;
}

// Corresponds to as2_msgs__action__Land_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Land_Result {
    /// false if failed to land
    pub land_success: bool,

}



impl Default for Land_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__Land_Result__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__Land_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Land_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Land_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Land_Result where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/Land_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Land_Result() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Land_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__Land_Feedback__init(msg: *mut Land_Feedback) -> bool;
    fn as2_msgs__action__Land_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Land_Feedback>, size: usize) -> bool;
    fn as2_msgs__action__Land_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Land_Feedback>);
    fn as2_msgs__action__Land_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Land_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<Land_Feedback>) -> bool;
}

// Corresponds to as2_msgs__action__Land_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Land_Feedback {
    /// actual speed (m/s)
    pub actual_land_speed: f32,

    /// actual height (m)
    pub actual_land_height: f32,

}



impl Default for Land_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__Land_Feedback__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__Land_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Land_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Land_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Land_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/Land_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Land_Feedback() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Land_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__Land_FeedbackMessage__init(msg: *mut Land_FeedbackMessage) -> bool;
    fn as2_msgs__action__Land_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Land_FeedbackMessage>, size: usize) -> bool;
    fn as2_msgs__action__Land_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Land_FeedbackMessage>);
    fn as2_msgs__action__Land_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Land_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<Land_FeedbackMessage>) -> bool;
}

// Corresponds to as2_msgs__action__Land_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Land_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::Land_Feedback,

}



impl Default for Land_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__Land_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__Land_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Land_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Land_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Land_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/Land_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Land_FeedbackMessage() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__MassEstimation_Goal() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__MassEstimation_Goal__init(msg: *mut MassEstimation_Goal) -> bool;
    fn as2_msgs__action__MassEstimation_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_Goal>, size: usize) -> bool;
    fn as2_msgs__action__MassEstimation_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_Goal>);
    fn as2_msgs__action__MassEstimation_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MassEstimation_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_Goal>) -> bool;
}

// Corresponds to as2_msgs__action__MassEstimation_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MassEstimation_Goal {
    /// Request
    pub request: bool,

}



impl Default for MassEstimation_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__MassEstimation_Goal__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__MassEstimation_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MassEstimation_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MassEstimation_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MassEstimation_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/MassEstimation_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__MassEstimation_Goal() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__MassEstimation_Result() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__MassEstimation_Result__init(msg: *mut MassEstimation_Result) -> bool;
    fn as2_msgs__action__MassEstimation_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_Result>, size: usize) -> bool;
    fn as2_msgs__action__MassEstimation_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_Result>);
    fn as2_msgs__action__MassEstimation_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MassEstimation_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_Result>) -> bool;
}

// Corresponds to as2_msgs__action__MassEstimation_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MassEstimation_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for MassEstimation_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__MassEstimation_Result__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__MassEstimation_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MassEstimation_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MassEstimation_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MassEstimation_Result where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/MassEstimation_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__MassEstimation_Result() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__MassEstimation_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__MassEstimation_Feedback__init(msg: *mut MassEstimation_Feedback) -> bool;
    fn as2_msgs__action__MassEstimation_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_Feedback>, size: usize) -> bool;
    fn as2_msgs__action__MassEstimation_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_Feedback>);
    fn as2_msgs__action__MassEstimation_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MassEstimation_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_Feedback>) -> bool;
}

// Corresponds to as2_msgs__action__MassEstimation_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MassEstimation_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for MassEstimation_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__MassEstimation_Feedback__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__MassEstimation_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MassEstimation_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MassEstimation_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MassEstimation_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/MassEstimation_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__MassEstimation_Feedback() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__MassEstimation_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__MassEstimation_FeedbackMessage__init(msg: *mut MassEstimation_FeedbackMessage) -> bool;
    fn as2_msgs__action__MassEstimation_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_FeedbackMessage>, size: usize) -> bool;
    fn as2_msgs__action__MassEstimation_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_FeedbackMessage>);
    fn as2_msgs__action__MassEstimation_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MassEstimation_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_FeedbackMessage>) -> bool;
}

// Corresponds to as2_msgs__action__MassEstimation_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MassEstimation_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::MassEstimation_Feedback,

}



impl Default for MassEstimation_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__MassEstimation_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__MassEstimation_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MassEstimation_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MassEstimation_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MassEstimation_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/MassEstimation_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__MassEstimation_FeedbackMessage() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__NavigateToPoint_Goal() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__NavigateToPoint_Goal__init(msg: *mut NavigateToPoint_Goal) -> bool;
    fn as2_msgs__action__NavigateToPoint_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_Goal>, size: usize) -> bool;
    fn as2_msgs__action__NavigateToPoint_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_Goal>);
    fn as2_msgs__action__NavigateToPoint_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateToPoint_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_Goal>) -> bool;
}

// Corresponds to as2_msgs__action__NavigateToPoint_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateToPoint_Goal {
    /// Request
    /// Goal pose 3D (m)
    pub point: geometry_msgs::msg::rmw::PointStamped,

    /// Yaw goal mode
    pub yaw: super::super::msg::rmw::YawMode,

    /// Maximum speed desired in path (m/s)
    pub navigation_speed: f32,

}



impl Default for NavigateToPoint_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__NavigateToPoint_Goal__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__NavigateToPoint_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateToPoint_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateToPoint_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateToPoint_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/NavigateToPoint_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__NavigateToPoint_Goal() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__NavigateToPoint_Result() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__NavigateToPoint_Result__init(msg: *mut NavigateToPoint_Result) -> bool;
    fn as2_msgs__action__NavigateToPoint_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_Result>, size: usize) -> bool;
    fn as2_msgs__action__NavigateToPoint_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_Result>);
    fn as2_msgs__action__NavigateToPoint_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateToPoint_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_Result>) -> bool;
}

// Corresponds to as2_msgs__action__NavigateToPoint_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateToPoint_Result {
    /// Point reached?
    pub success: bool,

}



impl Default for NavigateToPoint_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__NavigateToPoint_Result__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__NavigateToPoint_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateToPoint_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateToPoint_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateToPoint_Result where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/NavigateToPoint_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__NavigateToPoint_Result() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__NavigateToPoint_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__NavigateToPoint_Feedback__init(msg: *mut NavigateToPoint_Feedback) -> bool;
    fn as2_msgs__action__NavigateToPoint_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_Feedback>, size: usize) -> bool;
    fn as2_msgs__action__NavigateToPoint_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_Feedback>);
    fn as2_msgs__action__NavigateToPoint_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateToPoint_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_Feedback>) -> bool;
}

// Corresponds to as2_msgs__action__NavigateToPoint_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateToPoint_Feedback {
    /// Current pose (m)
    pub current_pose: geometry_msgs::msg::rmw::PoseStamped,

    /// Current speed (m/s)
    pub current_speed: geometry_msgs::msg::rmw::TwistStamped,

    /// Time from departure (s)
    pub navigation_time: builtin_interfaces::msg::rmw::Duration,

    /// Time to goal (s)
    pub estimated_time_remaining: builtin_interfaces::msg::rmw::Duration,

    /// Distance to goal (m)
    pub distance_remaining: f32,

}



impl Default for NavigateToPoint_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__NavigateToPoint_Feedback__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__NavigateToPoint_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateToPoint_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateToPoint_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateToPoint_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/NavigateToPoint_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__NavigateToPoint_Feedback() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__NavigateToPoint_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__NavigateToPoint_FeedbackMessage__init(msg: *mut NavigateToPoint_FeedbackMessage) -> bool;
    fn as2_msgs__action__NavigateToPoint_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_FeedbackMessage>, size: usize) -> bool;
    fn as2_msgs__action__NavigateToPoint_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_FeedbackMessage>);
    fn as2_msgs__action__NavigateToPoint_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateToPoint_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_FeedbackMessage>) -> bool;
}

// Corresponds to as2_msgs__action__NavigateToPoint_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateToPoint_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::NavigateToPoint_Feedback,

}



impl Default for NavigateToPoint_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__NavigateToPoint_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__NavigateToPoint_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateToPoint_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateToPoint_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateToPoint_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/NavigateToPoint_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__NavigateToPoint_FeedbackMessage() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PointGimbal_Goal() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__PointGimbal_Goal__init(msg: *mut PointGimbal_Goal) -> bool;
    fn as2_msgs__action__PointGimbal_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_Goal>, size: usize) -> bool;
    fn as2_msgs__action__PointGimbal_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_Goal>);
    fn as2_msgs__action__PointGimbal_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointGimbal_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_Goal>) -> bool;
}

// Corresponds to as2_msgs__action__PointGimbal_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointGimbal_Goal {
    /// Request
    /// Goal target
    pub control: super::super::msg::rmw::GimbalControl,

    /// Keep following after reach target
    pub follow_mode: bool,

    /// point gimbal to reference or move gimbal
    pub mode: u8,

}

impl PointGimbal_Goal {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const POINT_MODE: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOVE_MODE: u8 = 1;

}


impl Default for PointGimbal_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__PointGimbal_Goal__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__PointGimbal_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointGimbal_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointGimbal_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointGimbal_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/PointGimbal_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PointGimbal_Goal() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PointGimbal_Result() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__PointGimbal_Result__init(msg: *mut PointGimbal_Result) -> bool;
    fn as2_msgs__action__PointGimbal_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_Result>, size: usize) -> bool;
    fn as2_msgs__action__PointGimbal_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_Result>);
    fn as2_msgs__action__PointGimbal_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointGimbal_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_Result>) -> bool;
}

// Corresponds to as2_msgs__action__PointGimbal_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointGimbal_Result {
    /// False if failed to point to target
    pub success: bool,

}



impl Default for PointGimbal_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__PointGimbal_Result__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__PointGimbal_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointGimbal_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointGimbal_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointGimbal_Result where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/PointGimbal_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PointGimbal_Result() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PointGimbal_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__PointGimbal_Feedback__init(msg: *mut PointGimbal_Feedback) -> bool;
    fn as2_msgs__action__PointGimbal_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_Feedback>, size: usize) -> bool;
    fn as2_msgs__action__PointGimbal_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_Feedback>);
    fn as2_msgs__action__PointGimbal_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointGimbal_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_Feedback>) -> bool;
}

// Corresponds to as2_msgs__action__PointGimbal_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointGimbal_Feedback {
    /// Current attitude (rad)
    pub gimbal_attitude: geometry_msgs::msg::rmw::Vector3Stamped,

    /// Current speed (rad/s)
    pub gimbal_speed: geometry_msgs::msg::rmw::Vector3Stamped,

}



impl Default for PointGimbal_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__PointGimbal_Feedback__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__PointGimbal_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointGimbal_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointGimbal_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointGimbal_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/PointGimbal_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PointGimbal_Feedback() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PointGimbal_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__PointGimbal_FeedbackMessage__init(msg: *mut PointGimbal_FeedbackMessage) -> bool;
    fn as2_msgs__action__PointGimbal_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_FeedbackMessage>, size: usize) -> bool;
    fn as2_msgs__action__PointGimbal_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_FeedbackMessage>);
    fn as2_msgs__action__PointGimbal_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointGimbal_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_FeedbackMessage>) -> bool;
}

// Corresponds to as2_msgs__action__PointGimbal_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointGimbal_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::PointGimbal_Feedback,

}



impl Default for PointGimbal_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__PointGimbal_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__PointGimbal_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointGimbal_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointGimbal_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointGimbal_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/PointGimbal_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PointGimbal_FeedbackMessage() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PrecisionLanding_Goal() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__PrecisionLanding_Goal__init(msg: *mut PrecisionLanding_Goal) -> bool;
    fn as2_msgs__action__PrecisionLanding_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_Goal>, size: usize) -> bool;
    fn as2_msgs__action__PrecisionLanding_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_Goal>);
    fn as2_msgs__action__PrecisionLanding_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PrecisionLanding_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_Goal>) -> bool;
}

// Corresponds to as2_msgs__action__PrecisionLanding_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PrecisionLanding_Goal {
    /// Request
    /// marker frame ID
    pub marker_frame_id: rosidl_runtime_rs::String,

}



impl Default for PrecisionLanding_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__PrecisionLanding_Goal__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__PrecisionLanding_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PrecisionLanding_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PrecisionLanding_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PrecisionLanding_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/PrecisionLanding_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PrecisionLanding_Goal() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PrecisionLanding_Result() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__PrecisionLanding_Result__init(msg: *mut PrecisionLanding_Result) -> bool;
    fn as2_msgs__action__PrecisionLanding_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_Result>, size: usize) -> bool;
    fn as2_msgs__action__PrecisionLanding_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_Result>);
    fn as2_msgs__action__PrecisionLanding_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PrecisionLanding_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_Result>) -> bool;
}

// Corresponds to as2_msgs__action__PrecisionLanding_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PrecisionLanding_Result {
    /// false if failed to land
    pub precision_landing_success: bool,

}



impl Default for PrecisionLanding_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__PrecisionLanding_Result__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__PrecisionLanding_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PrecisionLanding_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PrecisionLanding_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PrecisionLanding_Result where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/PrecisionLanding_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PrecisionLanding_Result() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PrecisionLanding_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__PrecisionLanding_Feedback__init(msg: *mut PrecisionLanding_Feedback) -> bool;
    fn as2_msgs__action__PrecisionLanding_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_Feedback>, size: usize) -> bool;
    fn as2_msgs__action__PrecisionLanding_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_Feedback>);
    fn as2_msgs__action__PrecisionLanding_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PrecisionLanding_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_Feedback>) -> bool;
}

// Corresponds to as2_msgs__action__PrecisionLanding_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PrecisionLanding_Feedback {
    /// actual speed (m/s)
    pub precision_landing_speed: f32,

    /// actual height (m)
    pub precision_landing_height: f32,

    /// distance to target in xy (m)
    pub distance_xy: f32,

    /// distance to target in z (m)
    pub distance_z: f32,

}



impl Default for PrecisionLanding_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__PrecisionLanding_Feedback__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__PrecisionLanding_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PrecisionLanding_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PrecisionLanding_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PrecisionLanding_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/PrecisionLanding_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PrecisionLanding_Feedback() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PrecisionLanding_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__PrecisionLanding_FeedbackMessage__init(msg: *mut PrecisionLanding_FeedbackMessage) -> bool;
    fn as2_msgs__action__PrecisionLanding_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_FeedbackMessage>, size: usize) -> bool;
    fn as2_msgs__action__PrecisionLanding_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_FeedbackMessage>);
    fn as2_msgs__action__PrecisionLanding_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PrecisionLanding_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_FeedbackMessage>) -> bool;
}

// Corresponds to as2_msgs__action__PrecisionLanding_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PrecisionLanding_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::PrecisionLanding_Feedback,

}



impl Default for PrecisionLanding_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__PrecisionLanding_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__PrecisionLanding_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PrecisionLanding_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PrecisionLanding_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PrecisionLanding_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/PrecisionLanding_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PrecisionLanding_FeedbackMessage() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetArmingState_Goal() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SetArmingState_Goal__init(msg: *mut SetArmingState_Goal) -> bool;
    fn as2_msgs__action__SetArmingState_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_Goal>, size: usize) -> bool;
    fn as2_msgs__action__SetArmingState_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_Goal>);
    fn as2_msgs__action__SetArmingState_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetArmingState_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_Goal>) -> bool;
}

// Corresponds to as2_msgs__action__SetArmingState_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetArmingState_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: bool,

}



impl Default for SetArmingState_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SetArmingState_Goal__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SetArmingState_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetArmingState_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetArmingState_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetArmingState_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SetArmingState_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetArmingState_Goal() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetArmingState_Result() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SetArmingState_Result__init(msg: *mut SetArmingState_Result) -> bool;
    fn as2_msgs__action__SetArmingState_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_Result>, size: usize) -> bool;
    fn as2_msgs__action__SetArmingState_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_Result>);
    fn as2_msgs__action__SetArmingState_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetArmingState_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_Result>) -> bool;
}

// Corresponds to as2_msgs__action__SetArmingState_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetArmingState_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for SetArmingState_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SetArmingState_Result__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SetArmingState_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetArmingState_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetArmingState_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetArmingState_Result where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SetArmingState_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetArmingState_Result() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetArmingState_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SetArmingState_Feedback__init(msg: *mut SetArmingState_Feedback) -> bool;
    fn as2_msgs__action__SetArmingState_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_Feedback>, size: usize) -> bool;
    fn as2_msgs__action__SetArmingState_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_Feedback>);
    fn as2_msgs__action__SetArmingState_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetArmingState_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_Feedback>) -> bool;
}

// Corresponds to as2_msgs__action__SetArmingState_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetArmingState_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for SetArmingState_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SetArmingState_Feedback__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SetArmingState_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetArmingState_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetArmingState_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetArmingState_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SetArmingState_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetArmingState_Feedback() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetArmingState_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SetArmingState_FeedbackMessage__init(msg: *mut SetArmingState_FeedbackMessage) -> bool;
    fn as2_msgs__action__SetArmingState_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_FeedbackMessage>, size: usize) -> bool;
    fn as2_msgs__action__SetArmingState_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_FeedbackMessage>);
    fn as2_msgs__action__SetArmingState_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetArmingState_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_FeedbackMessage>) -> bool;
}

// Corresponds to as2_msgs__action__SetArmingState_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetArmingState_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::SetArmingState_Feedback,

}



impl Default for SetArmingState_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SetArmingState_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SetArmingState_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetArmingState_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetArmingState_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetArmingState_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SetArmingState_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetArmingState_FeedbackMessage() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetOffboardMode_Goal() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SetOffboardMode_Goal__init(msg: *mut SetOffboardMode_Goal) -> bool;
    fn as2_msgs__action__SetOffboardMode_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_Goal>, size: usize) -> bool;
    fn as2_msgs__action__SetOffboardMode_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_Goal>);
    fn as2_msgs__action__SetOffboardMode_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOffboardMode_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_Goal>) -> bool;
}

// Corresponds to as2_msgs__action__SetOffboardMode_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOffboardMode_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: bool,

}



impl Default for SetOffboardMode_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SetOffboardMode_Goal__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SetOffboardMode_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOffboardMode_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOffboardMode_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOffboardMode_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SetOffboardMode_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetOffboardMode_Goal() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetOffboardMode_Result() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SetOffboardMode_Result__init(msg: *mut SetOffboardMode_Result) -> bool;
    fn as2_msgs__action__SetOffboardMode_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_Result>, size: usize) -> bool;
    fn as2_msgs__action__SetOffboardMode_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_Result>);
    fn as2_msgs__action__SetOffboardMode_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOffboardMode_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_Result>) -> bool;
}

// Corresponds to as2_msgs__action__SetOffboardMode_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOffboardMode_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for SetOffboardMode_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SetOffboardMode_Result__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SetOffboardMode_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOffboardMode_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOffboardMode_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOffboardMode_Result where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SetOffboardMode_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetOffboardMode_Result() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetOffboardMode_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SetOffboardMode_Feedback__init(msg: *mut SetOffboardMode_Feedback) -> bool;
    fn as2_msgs__action__SetOffboardMode_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_Feedback>, size: usize) -> bool;
    fn as2_msgs__action__SetOffboardMode_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_Feedback>);
    fn as2_msgs__action__SetOffboardMode_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOffboardMode_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_Feedback>) -> bool;
}

// Corresponds to as2_msgs__action__SetOffboardMode_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOffboardMode_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for SetOffboardMode_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SetOffboardMode_Feedback__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SetOffboardMode_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOffboardMode_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOffboardMode_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOffboardMode_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SetOffboardMode_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetOffboardMode_Feedback() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetOffboardMode_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SetOffboardMode_FeedbackMessage__init(msg: *mut SetOffboardMode_FeedbackMessage) -> bool;
    fn as2_msgs__action__SetOffboardMode_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_FeedbackMessage>, size: usize) -> bool;
    fn as2_msgs__action__SetOffboardMode_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_FeedbackMessage>);
    fn as2_msgs__action__SetOffboardMode_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOffboardMode_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_FeedbackMessage>) -> bool;
}

// Corresponds to as2_msgs__action__SetOffboardMode_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOffboardMode_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::SetOffboardMode_Feedback,

}



impl Default for SetOffboardMode_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SetOffboardMode_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SetOffboardMode_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOffboardMode_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOffboardMode_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOffboardMode_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SetOffboardMode_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetOffboardMode_FeedbackMessage() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SwarmFlocking_Goal() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SwarmFlocking_Goal__init(msg: *mut SwarmFlocking_Goal) -> bool;
    fn as2_msgs__action__SwarmFlocking_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_Goal>, size: usize) -> bool;
    fn as2_msgs__action__SwarmFlocking_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_Goal>);
    fn as2_msgs__action__SwarmFlocking_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwarmFlocking_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_Goal>) -> bool;
}

// Corresponds to as2_msgs__action__SwarmFlocking_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwarmFlocking_Goal {
    /// Request
    /// Offset of the virtual centroid to the following frame
    pub virtual_centroid: geometry_msgs::msg::rmw::PoseStamped,

    /// Pose of the drones with respect to the virtual centroid
    pub swarm_formation: rosidl_runtime_rs::Sequence<super::super::msg::rmw::PoseWithID>,

    /// Namespaces of the drones in the swarm
    pub drones_namespace: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for SwarmFlocking_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SwarmFlocking_Goal__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SwarmFlocking_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwarmFlocking_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwarmFlocking_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwarmFlocking_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SwarmFlocking_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SwarmFlocking_Goal() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SwarmFlocking_Result() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SwarmFlocking_Result__init(msg: *mut SwarmFlocking_Result) -> bool;
    fn as2_msgs__action__SwarmFlocking_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_Result>, size: usize) -> bool;
    fn as2_msgs__action__SwarmFlocking_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_Result>);
    fn as2_msgs__action__SwarmFlocking_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwarmFlocking_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_Result>) -> bool;
}

// Corresponds to as2_msgs__action__SwarmFlocking_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwarmFlocking_Result {
    /// False if failed to swarm_success
    pub swarm_success: bool,

}



impl Default for SwarmFlocking_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SwarmFlocking_Result__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SwarmFlocking_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwarmFlocking_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwarmFlocking_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwarmFlocking_Result where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SwarmFlocking_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SwarmFlocking_Result() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SwarmFlocking_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SwarmFlocking_Feedback__init(msg: *mut SwarmFlocking_Feedback) -> bool;
    fn as2_msgs__action__SwarmFlocking_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_Feedback>, size: usize) -> bool;
    fn as2_msgs__action__SwarmFlocking_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_Feedback>);
    fn as2_msgs__action__SwarmFlocking_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwarmFlocking_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_Feedback>) -> bool;
}

// Corresponds to as2_msgs__action__SwarmFlocking_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwarmFlocking_Feedback {
    /// Current swarm pose
    pub swarm_pose: geometry_msgs::msg::rmw::Pose,

}



impl Default for SwarmFlocking_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SwarmFlocking_Feedback__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SwarmFlocking_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwarmFlocking_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwarmFlocking_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwarmFlocking_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SwarmFlocking_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SwarmFlocking_Feedback() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SwarmFlocking_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SwarmFlocking_FeedbackMessage__init(msg: *mut SwarmFlocking_FeedbackMessage) -> bool;
    fn as2_msgs__action__SwarmFlocking_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_FeedbackMessage>, size: usize) -> bool;
    fn as2_msgs__action__SwarmFlocking_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_FeedbackMessage>);
    fn as2_msgs__action__SwarmFlocking_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwarmFlocking_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_FeedbackMessage>) -> bool;
}

// Corresponds to as2_msgs__action__SwarmFlocking_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwarmFlocking_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::SwarmFlocking_Feedback,

}



impl Default for SwarmFlocking_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SwarmFlocking_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SwarmFlocking_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwarmFlocking_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwarmFlocking_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwarmFlocking_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SwarmFlocking_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SwarmFlocking_FeedbackMessage() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Takeoff_Goal() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__Takeoff_Goal__init(msg: *mut Takeoff_Goal) -> bool;
    fn as2_msgs__action__Takeoff_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_Goal>, size: usize) -> bool;
    fn as2_msgs__action__Takeoff_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_Goal>);
    fn as2_msgs__action__Takeoff_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Takeoff_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<Takeoff_Goal>) -> bool;
}

// Corresponds to as2_msgs__action__Takeoff_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Takeoff_Goal {
    /// Request
    /// Takeoff height (m)
    pub takeoff_height: f32,

    /// Takeoff speed (m/s)
    pub takeoff_speed: f32,

}



impl Default for Takeoff_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__Takeoff_Goal__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__Takeoff_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Takeoff_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Takeoff_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Takeoff_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/Takeoff_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Takeoff_Goal() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Takeoff_Result() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__Takeoff_Result__init(msg: *mut Takeoff_Result) -> bool;
    fn as2_msgs__action__Takeoff_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_Result>, size: usize) -> bool;
    fn as2_msgs__action__Takeoff_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_Result>);
    fn as2_msgs__action__Takeoff_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Takeoff_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<Takeoff_Result>) -> bool;
}

// Corresponds to as2_msgs__action__Takeoff_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Takeoff_Result {
    /// false if failed to takeoff
    pub takeoff_success: bool,

}



impl Default for Takeoff_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__Takeoff_Result__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__Takeoff_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Takeoff_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Takeoff_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Takeoff_Result where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/Takeoff_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Takeoff_Result() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Takeoff_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__Takeoff_Feedback__init(msg: *mut Takeoff_Feedback) -> bool;
    fn as2_msgs__action__Takeoff_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_Feedback>, size: usize) -> bool;
    fn as2_msgs__action__Takeoff_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_Feedback>);
    fn as2_msgs__action__Takeoff_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Takeoff_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<Takeoff_Feedback>) -> bool;
}

// Corresponds to as2_msgs__action__Takeoff_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Takeoff_Feedback {
    /// actual speed (m/s)
    pub actual_takeoff_speed: f32,

    /// actual height (m)
    pub actual_takeoff_height: f32,

}



impl Default for Takeoff_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__Takeoff_Feedback__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__Takeoff_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Takeoff_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Takeoff_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Takeoff_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/Takeoff_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Takeoff_Feedback() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Takeoff_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__Takeoff_FeedbackMessage__init(msg: *mut Takeoff_FeedbackMessage) -> bool;
    fn as2_msgs__action__Takeoff_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_FeedbackMessage>, size: usize) -> bool;
    fn as2_msgs__action__Takeoff_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_FeedbackMessage>);
    fn as2_msgs__action__Takeoff_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Takeoff_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<Takeoff_FeedbackMessage>) -> bool;
}

// Corresponds to as2_msgs__action__Takeoff_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Takeoff_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::Takeoff_Feedback,

}



impl Default for Takeoff_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__Takeoff_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__Takeoff_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Takeoff_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Takeoff_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Takeoff_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/Takeoff_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Takeoff_FeedbackMessage() }
  }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__DetectArucoMarkers_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__DetectArucoMarkers_SendGoal_Request__init(msg: *mut DetectArucoMarkers_SendGoal_Request) -> bool;
    fn as2_msgs__action__DetectArucoMarkers_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_SendGoal_Request>, size: usize) -> bool;
    fn as2_msgs__action__DetectArucoMarkers_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_SendGoal_Request>);
    fn as2_msgs__action__DetectArucoMarkers_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectArucoMarkers_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_SendGoal_Request>) -> bool;
}

// Corresponds to as2_msgs__action__DetectArucoMarkers_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectArucoMarkers_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::DetectArucoMarkers_Goal,

}



impl Default for DetectArucoMarkers_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__DetectArucoMarkers_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__DetectArucoMarkers_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectArucoMarkers_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectArucoMarkers_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectArucoMarkers_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/DetectArucoMarkers_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__DetectArucoMarkers_SendGoal_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__DetectArucoMarkers_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__DetectArucoMarkers_SendGoal_Response__init(msg: *mut DetectArucoMarkers_SendGoal_Response) -> bool;
    fn as2_msgs__action__DetectArucoMarkers_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_SendGoal_Response>, size: usize) -> bool;
    fn as2_msgs__action__DetectArucoMarkers_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_SendGoal_Response>);
    fn as2_msgs__action__DetectArucoMarkers_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectArucoMarkers_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_SendGoal_Response>) -> bool;
}

// Corresponds to as2_msgs__action__DetectArucoMarkers_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectArucoMarkers_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for DetectArucoMarkers_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__DetectArucoMarkers_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__DetectArucoMarkers_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectArucoMarkers_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectArucoMarkers_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectArucoMarkers_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/DetectArucoMarkers_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__DetectArucoMarkers_SendGoal_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__DetectArucoMarkers_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__DetectArucoMarkers_GetResult_Request__init(msg: *mut DetectArucoMarkers_GetResult_Request) -> bool;
    fn as2_msgs__action__DetectArucoMarkers_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_GetResult_Request>, size: usize) -> bool;
    fn as2_msgs__action__DetectArucoMarkers_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_GetResult_Request>);
    fn as2_msgs__action__DetectArucoMarkers_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectArucoMarkers_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_GetResult_Request>) -> bool;
}

// Corresponds to as2_msgs__action__DetectArucoMarkers_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectArucoMarkers_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for DetectArucoMarkers_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__DetectArucoMarkers_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__DetectArucoMarkers_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectArucoMarkers_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectArucoMarkers_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectArucoMarkers_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/DetectArucoMarkers_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__DetectArucoMarkers_GetResult_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__DetectArucoMarkers_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__DetectArucoMarkers_GetResult_Response__init(msg: *mut DetectArucoMarkers_GetResult_Response) -> bool;
    fn as2_msgs__action__DetectArucoMarkers_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_GetResult_Response>, size: usize) -> bool;
    fn as2_msgs__action__DetectArucoMarkers_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_GetResult_Response>);
    fn as2_msgs__action__DetectArucoMarkers_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectArucoMarkers_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectArucoMarkers_GetResult_Response>) -> bool;
}

// Corresponds to as2_msgs__action__DetectArucoMarkers_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectArucoMarkers_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::DetectArucoMarkers_Result,

}



impl Default for DetectArucoMarkers_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__DetectArucoMarkers_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__DetectArucoMarkers_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectArucoMarkers_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__DetectArucoMarkers_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectArucoMarkers_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectArucoMarkers_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/DetectArucoMarkers_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__DetectArucoMarkers_GetResult_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowPath_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__FollowPath_SendGoal_Request__init(msg: *mut FollowPath_SendGoal_Request) -> bool;
    fn as2_msgs__action__FollowPath_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowPath_SendGoal_Request>, size: usize) -> bool;
    fn as2_msgs__action__FollowPath_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowPath_SendGoal_Request>);
    fn as2_msgs__action__FollowPath_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowPath_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowPath_SendGoal_Request>) -> bool;
}

// Corresponds to as2_msgs__action__FollowPath_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowPath_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::FollowPath_Goal,

}



impl Default for FollowPath_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__FollowPath_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__FollowPath_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowPath_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowPath_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowPath_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/FollowPath_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowPath_SendGoal_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowPath_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__FollowPath_SendGoal_Response__init(msg: *mut FollowPath_SendGoal_Response) -> bool;
    fn as2_msgs__action__FollowPath_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowPath_SendGoal_Response>, size: usize) -> bool;
    fn as2_msgs__action__FollowPath_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowPath_SendGoal_Response>);
    fn as2_msgs__action__FollowPath_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowPath_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowPath_SendGoal_Response>) -> bool;
}

// Corresponds to as2_msgs__action__FollowPath_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowPath_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for FollowPath_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__FollowPath_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__FollowPath_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowPath_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowPath_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowPath_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/FollowPath_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowPath_SendGoal_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowPath_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__FollowPath_GetResult_Request__init(msg: *mut FollowPath_GetResult_Request) -> bool;
    fn as2_msgs__action__FollowPath_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowPath_GetResult_Request>, size: usize) -> bool;
    fn as2_msgs__action__FollowPath_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowPath_GetResult_Request>);
    fn as2_msgs__action__FollowPath_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowPath_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowPath_GetResult_Request>) -> bool;
}

// Corresponds to as2_msgs__action__FollowPath_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowPath_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for FollowPath_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__FollowPath_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__FollowPath_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowPath_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowPath_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowPath_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/FollowPath_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowPath_GetResult_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowPath_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__FollowPath_GetResult_Response__init(msg: *mut FollowPath_GetResult_Response) -> bool;
    fn as2_msgs__action__FollowPath_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowPath_GetResult_Response>, size: usize) -> bool;
    fn as2_msgs__action__FollowPath_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowPath_GetResult_Response>);
    fn as2_msgs__action__FollowPath_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowPath_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowPath_GetResult_Response>) -> bool;
}

// Corresponds to as2_msgs__action__FollowPath_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowPath_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::FollowPath_Result,

}



impl Default for FollowPath_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__FollowPath_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__FollowPath_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowPath_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowPath_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowPath_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowPath_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/FollowPath_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowPath_GetResult_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowReference_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__FollowReference_SendGoal_Request__init(msg: *mut FollowReference_SendGoal_Request) -> bool;
    fn as2_msgs__action__FollowReference_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowReference_SendGoal_Request>, size: usize) -> bool;
    fn as2_msgs__action__FollowReference_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowReference_SendGoal_Request>);
    fn as2_msgs__action__FollowReference_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowReference_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowReference_SendGoal_Request>) -> bool;
}

// Corresponds to as2_msgs__action__FollowReference_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowReference_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::FollowReference_Goal,

}



impl Default for FollowReference_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__FollowReference_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__FollowReference_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowReference_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowReference_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowReference_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/FollowReference_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowReference_SendGoal_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowReference_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__FollowReference_SendGoal_Response__init(msg: *mut FollowReference_SendGoal_Response) -> bool;
    fn as2_msgs__action__FollowReference_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowReference_SendGoal_Response>, size: usize) -> bool;
    fn as2_msgs__action__FollowReference_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowReference_SendGoal_Response>);
    fn as2_msgs__action__FollowReference_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowReference_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowReference_SendGoal_Response>) -> bool;
}

// Corresponds to as2_msgs__action__FollowReference_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowReference_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for FollowReference_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__FollowReference_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__FollowReference_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowReference_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowReference_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowReference_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/FollowReference_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowReference_SendGoal_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowReference_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__FollowReference_GetResult_Request__init(msg: *mut FollowReference_GetResult_Request) -> bool;
    fn as2_msgs__action__FollowReference_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowReference_GetResult_Request>, size: usize) -> bool;
    fn as2_msgs__action__FollowReference_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowReference_GetResult_Request>);
    fn as2_msgs__action__FollowReference_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowReference_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowReference_GetResult_Request>) -> bool;
}

// Corresponds to as2_msgs__action__FollowReference_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowReference_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for FollowReference_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__FollowReference_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__FollowReference_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowReference_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowReference_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowReference_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/FollowReference_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowReference_GetResult_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowReference_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__FollowReference_GetResult_Response__init(msg: *mut FollowReference_GetResult_Response) -> bool;
    fn as2_msgs__action__FollowReference_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowReference_GetResult_Response>, size: usize) -> bool;
    fn as2_msgs__action__FollowReference_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowReference_GetResult_Response>);
    fn as2_msgs__action__FollowReference_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowReference_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowReference_GetResult_Response>) -> bool;
}

// Corresponds to as2_msgs__action__FollowReference_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowReference_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::FollowReference_Result,

}



impl Default for FollowReference_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__FollowReference_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__FollowReference_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowReference_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__FollowReference_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowReference_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowReference_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/FollowReference_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__FollowReference_GetResult_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__ForceEstimation_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__ForceEstimation_SendGoal_Request__init(msg: *mut ForceEstimation_SendGoal_Request) -> bool;
    fn as2_msgs__action__ForceEstimation_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_SendGoal_Request>, size: usize) -> bool;
    fn as2_msgs__action__ForceEstimation_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_SendGoal_Request>);
    fn as2_msgs__action__ForceEstimation_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ForceEstimation_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_SendGoal_Request>) -> bool;
}

// Corresponds to as2_msgs__action__ForceEstimation_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ForceEstimation_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::ForceEstimation_Goal,

}



impl Default for ForceEstimation_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__ForceEstimation_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__ForceEstimation_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ForceEstimation_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ForceEstimation_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ForceEstimation_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/ForceEstimation_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__ForceEstimation_SendGoal_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__ForceEstimation_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__ForceEstimation_SendGoal_Response__init(msg: *mut ForceEstimation_SendGoal_Response) -> bool;
    fn as2_msgs__action__ForceEstimation_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_SendGoal_Response>, size: usize) -> bool;
    fn as2_msgs__action__ForceEstimation_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_SendGoal_Response>);
    fn as2_msgs__action__ForceEstimation_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ForceEstimation_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_SendGoal_Response>) -> bool;
}

// Corresponds to as2_msgs__action__ForceEstimation_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ForceEstimation_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for ForceEstimation_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__ForceEstimation_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__ForceEstimation_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ForceEstimation_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ForceEstimation_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ForceEstimation_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/ForceEstimation_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__ForceEstimation_SendGoal_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__ForceEstimation_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__ForceEstimation_GetResult_Request__init(msg: *mut ForceEstimation_GetResult_Request) -> bool;
    fn as2_msgs__action__ForceEstimation_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_GetResult_Request>, size: usize) -> bool;
    fn as2_msgs__action__ForceEstimation_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_GetResult_Request>);
    fn as2_msgs__action__ForceEstimation_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ForceEstimation_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_GetResult_Request>) -> bool;
}

// Corresponds to as2_msgs__action__ForceEstimation_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ForceEstimation_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for ForceEstimation_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__ForceEstimation_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__ForceEstimation_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ForceEstimation_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ForceEstimation_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ForceEstimation_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/ForceEstimation_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__ForceEstimation_GetResult_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__ForceEstimation_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__ForceEstimation_GetResult_Response__init(msg: *mut ForceEstimation_GetResult_Response) -> bool;
    fn as2_msgs__action__ForceEstimation_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_GetResult_Response>, size: usize) -> bool;
    fn as2_msgs__action__ForceEstimation_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_GetResult_Response>);
    fn as2_msgs__action__ForceEstimation_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ForceEstimation_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ForceEstimation_GetResult_Response>) -> bool;
}

// Corresponds to as2_msgs__action__ForceEstimation_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ForceEstimation_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::ForceEstimation_Result,

}



impl Default for ForceEstimation_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__ForceEstimation_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__ForceEstimation_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ForceEstimation_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__ForceEstimation_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ForceEstimation_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ForceEstimation_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/ForceEstimation_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__ForceEstimation_GetResult_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Request__init(msg: *mut GeneratePolynomialTrajectory_SendGoal_Request) -> bool;
    fn as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_SendGoal_Request>, size: usize) -> bool;
    fn as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_SendGoal_Request>);
    fn as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_SendGoal_Request>) -> bool;
}

// Corresponds to as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GeneratePolynomialTrajectory_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::GeneratePolynomialTrajectory_Goal,

}



impl Default for GeneratePolynomialTrajectory_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GeneratePolynomialTrajectory_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GeneratePolynomialTrajectory_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GeneratePolynomialTrajectory_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GeneratePolynomialTrajectory_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Response__init(msg: *mut GeneratePolynomialTrajectory_SendGoal_Response) -> bool;
    fn as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_SendGoal_Response>, size: usize) -> bool;
    fn as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_SendGoal_Response>);
    fn as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_SendGoal_Response>) -> bool;
}

// Corresponds to as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GeneratePolynomialTrajectory_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for GeneratePolynomialTrajectory_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GeneratePolynomialTrajectory_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GeneratePolynomialTrajectory_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GeneratePolynomialTrajectory_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GeneratePolynomialTrajectory_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_SendGoal_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Request__init(msg: *mut GeneratePolynomialTrajectory_GetResult_Request) -> bool;
    fn as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_GetResult_Request>, size: usize) -> bool;
    fn as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_GetResult_Request>);
    fn as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_GetResult_Request>) -> bool;
}

// Corresponds to as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GeneratePolynomialTrajectory_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for GeneratePolynomialTrajectory_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GeneratePolynomialTrajectory_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GeneratePolynomialTrajectory_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GeneratePolynomialTrajectory_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GeneratePolynomialTrajectory_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Response__init(msg: *mut GeneratePolynomialTrajectory_GetResult_Response) -> bool;
    fn as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_GetResult_Response>, size: usize) -> bool;
    fn as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_GetResult_Response>);
    fn as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GeneratePolynomialTrajectory_GetResult_Response>) -> bool;
}

// Corresponds to as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GeneratePolynomialTrajectory_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::GeneratePolynomialTrajectory_Result,

}



impl Default for GeneratePolynomialTrajectory_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GeneratePolynomialTrajectory_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GeneratePolynomialTrajectory_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GeneratePolynomialTrajectory_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GeneratePolynomialTrajectory_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_GetResult_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GoToWaypoint_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GoToWaypoint_SendGoal_Request__init(msg: *mut GoToWaypoint_SendGoal_Request) -> bool;
    fn as2_msgs__action__GoToWaypoint_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_SendGoal_Request>, size: usize) -> bool;
    fn as2_msgs__action__GoToWaypoint_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_SendGoal_Request>);
    fn as2_msgs__action__GoToWaypoint_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GoToWaypoint_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_SendGoal_Request>) -> bool;
}

// Corresponds to as2_msgs__action__GoToWaypoint_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoToWaypoint_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::GoToWaypoint_Goal,

}



impl Default for GoToWaypoint_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GoToWaypoint_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GoToWaypoint_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GoToWaypoint_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GoToWaypoint_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GoToWaypoint_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GoToWaypoint_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GoToWaypoint_SendGoal_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GoToWaypoint_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GoToWaypoint_SendGoal_Response__init(msg: *mut GoToWaypoint_SendGoal_Response) -> bool;
    fn as2_msgs__action__GoToWaypoint_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_SendGoal_Response>, size: usize) -> bool;
    fn as2_msgs__action__GoToWaypoint_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_SendGoal_Response>);
    fn as2_msgs__action__GoToWaypoint_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GoToWaypoint_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_SendGoal_Response>) -> bool;
}

// Corresponds to as2_msgs__action__GoToWaypoint_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoToWaypoint_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for GoToWaypoint_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GoToWaypoint_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GoToWaypoint_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GoToWaypoint_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GoToWaypoint_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GoToWaypoint_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GoToWaypoint_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GoToWaypoint_SendGoal_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GoToWaypoint_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GoToWaypoint_GetResult_Request__init(msg: *mut GoToWaypoint_GetResult_Request) -> bool;
    fn as2_msgs__action__GoToWaypoint_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_GetResult_Request>, size: usize) -> bool;
    fn as2_msgs__action__GoToWaypoint_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_GetResult_Request>);
    fn as2_msgs__action__GoToWaypoint_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GoToWaypoint_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_GetResult_Request>) -> bool;
}

// Corresponds to as2_msgs__action__GoToWaypoint_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoToWaypoint_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for GoToWaypoint_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GoToWaypoint_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GoToWaypoint_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GoToWaypoint_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GoToWaypoint_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GoToWaypoint_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GoToWaypoint_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GoToWaypoint_GetResult_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GoToWaypoint_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GoToWaypoint_GetResult_Response__init(msg: *mut GoToWaypoint_GetResult_Response) -> bool;
    fn as2_msgs__action__GoToWaypoint_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_GetResult_Response>, size: usize) -> bool;
    fn as2_msgs__action__GoToWaypoint_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_GetResult_Response>);
    fn as2_msgs__action__GoToWaypoint_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GoToWaypoint_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GoToWaypoint_GetResult_Response>) -> bool;
}

// Corresponds to as2_msgs__action__GoToWaypoint_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoToWaypoint_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::GoToWaypoint_Result,

}



impl Default for GoToWaypoint_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GoToWaypoint_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GoToWaypoint_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GoToWaypoint_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GoToWaypoint_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GoToWaypoint_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GoToWaypoint_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GoToWaypoint_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GoToWaypoint_GetResult_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GripperHandler_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GripperHandler_SendGoal_Request__init(msg: *mut GripperHandler_SendGoal_Request) -> bool;
    fn as2_msgs__action__GripperHandler_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_SendGoal_Request>, size: usize) -> bool;
    fn as2_msgs__action__GripperHandler_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_SendGoal_Request>);
    fn as2_msgs__action__GripperHandler_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperHandler_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_SendGoal_Request>) -> bool;
}

// Corresponds to as2_msgs__action__GripperHandler_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperHandler_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::GripperHandler_Goal,

}



impl Default for GripperHandler_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GripperHandler_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GripperHandler_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperHandler_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperHandler_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperHandler_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GripperHandler_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GripperHandler_SendGoal_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GripperHandler_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GripperHandler_SendGoal_Response__init(msg: *mut GripperHandler_SendGoal_Response) -> bool;
    fn as2_msgs__action__GripperHandler_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_SendGoal_Response>, size: usize) -> bool;
    fn as2_msgs__action__GripperHandler_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_SendGoal_Response>);
    fn as2_msgs__action__GripperHandler_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperHandler_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_SendGoal_Response>) -> bool;
}

// Corresponds to as2_msgs__action__GripperHandler_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperHandler_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for GripperHandler_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GripperHandler_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GripperHandler_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperHandler_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperHandler_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperHandler_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GripperHandler_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GripperHandler_SendGoal_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GripperHandler_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GripperHandler_GetResult_Request__init(msg: *mut GripperHandler_GetResult_Request) -> bool;
    fn as2_msgs__action__GripperHandler_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_GetResult_Request>, size: usize) -> bool;
    fn as2_msgs__action__GripperHandler_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_GetResult_Request>);
    fn as2_msgs__action__GripperHandler_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperHandler_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_GetResult_Request>) -> bool;
}

// Corresponds to as2_msgs__action__GripperHandler_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperHandler_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for GripperHandler_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GripperHandler_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GripperHandler_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperHandler_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperHandler_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperHandler_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GripperHandler_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GripperHandler_GetResult_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GripperHandler_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__GripperHandler_GetResult_Response__init(msg: *mut GripperHandler_GetResult_Response) -> bool;
    fn as2_msgs__action__GripperHandler_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_GetResult_Response>, size: usize) -> bool;
    fn as2_msgs__action__GripperHandler_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_GetResult_Response>);
    fn as2_msgs__action__GripperHandler_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperHandler_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperHandler_GetResult_Response>) -> bool;
}

// Corresponds to as2_msgs__action__GripperHandler_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperHandler_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::GripperHandler_Result,

}



impl Default for GripperHandler_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__GripperHandler_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__GripperHandler_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperHandler_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__GripperHandler_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperHandler_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperHandler_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/GripperHandler_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__GripperHandler_GetResult_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Land_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__Land_SendGoal_Request__init(msg: *mut Land_SendGoal_Request) -> bool;
    fn as2_msgs__action__Land_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Land_SendGoal_Request>, size: usize) -> bool;
    fn as2_msgs__action__Land_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Land_SendGoal_Request>);
    fn as2_msgs__action__Land_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Land_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Land_SendGoal_Request>) -> bool;
}

// Corresponds to as2_msgs__action__Land_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Land_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::Land_Goal,

}



impl Default for Land_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__Land_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__Land_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Land_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Land_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Land_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/Land_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Land_SendGoal_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Land_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__Land_SendGoal_Response__init(msg: *mut Land_SendGoal_Response) -> bool;
    fn as2_msgs__action__Land_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Land_SendGoal_Response>, size: usize) -> bool;
    fn as2_msgs__action__Land_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Land_SendGoal_Response>);
    fn as2_msgs__action__Land_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Land_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Land_SendGoal_Response>) -> bool;
}

// Corresponds to as2_msgs__action__Land_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Land_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for Land_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__Land_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__Land_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Land_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Land_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Land_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/Land_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Land_SendGoal_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Land_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__Land_GetResult_Request__init(msg: *mut Land_GetResult_Request) -> bool;
    fn as2_msgs__action__Land_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Land_GetResult_Request>, size: usize) -> bool;
    fn as2_msgs__action__Land_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Land_GetResult_Request>);
    fn as2_msgs__action__Land_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Land_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Land_GetResult_Request>) -> bool;
}

// Corresponds to as2_msgs__action__Land_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Land_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for Land_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__Land_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__Land_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Land_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Land_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Land_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/Land_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Land_GetResult_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Land_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__Land_GetResult_Response__init(msg: *mut Land_GetResult_Response) -> bool;
    fn as2_msgs__action__Land_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Land_GetResult_Response>, size: usize) -> bool;
    fn as2_msgs__action__Land_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Land_GetResult_Response>);
    fn as2_msgs__action__Land_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Land_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Land_GetResult_Response>) -> bool;
}

// Corresponds to as2_msgs__action__Land_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Land_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::Land_Result,

}



impl Default for Land_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__Land_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__Land_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Land_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Land_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Land_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Land_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/Land_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Land_GetResult_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__MassEstimation_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__MassEstimation_SendGoal_Request__init(msg: *mut MassEstimation_SendGoal_Request) -> bool;
    fn as2_msgs__action__MassEstimation_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_SendGoal_Request>, size: usize) -> bool;
    fn as2_msgs__action__MassEstimation_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_SendGoal_Request>);
    fn as2_msgs__action__MassEstimation_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MassEstimation_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_SendGoal_Request>) -> bool;
}

// Corresponds to as2_msgs__action__MassEstimation_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MassEstimation_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::MassEstimation_Goal,

}



impl Default for MassEstimation_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__MassEstimation_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__MassEstimation_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MassEstimation_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MassEstimation_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MassEstimation_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/MassEstimation_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__MassEstimation_SendGoal_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__MassEstimation_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__MassEstimation_SendGoal_Response__init(msg: *mut MassEstimation_SendGoal_Response) -> bool;
    fn as2_msgs__action__MassEstimation_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_SendGoal_Response>, size: usize) -> bool;
    fn as2_msgs__action__MassEstimation_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_SendGoal_Response>);
    fn as2_msgs__action__MassEstimation_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MassEstimation_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_SendGoal_Response>) -> bool;
}

// Corresponds to as2_msgs__action__MassEstimation_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MassEstimation_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for MassEstimation_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__MassEstimation_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__MassEstimation_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MassEstimation_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MassEstimation_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MassEstimation_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/MassEstimation_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__MassEstimation_SendGoal_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__MassEstimation_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__MassEstimation_GetResult_Request__init(msg: *mut MassEstimation_GetResult_Request) -> bool;
    fn as2_msgs__action__MassEstimation_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_GetResult_Request>, size: usize) -> bool;
    fn as2_msgs__action__MassEstimation_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_GetResult_Request>);
    fn as2_msgs__action__MassEstimation_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MassEstimation_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_GetResult_Request>) -> bool;
}

// Corresponds to as2_msgs__action__MassEstimation_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MassEstimation_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for MassEstimation_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__MassEstimation_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__MassEstimation_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MassEstimation_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MassEstimation_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MassEstimation_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/MassEstimation_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__MassEstimation_GetResult_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__MassEstimation_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__MassEstimation_GetResult_Response__init(msg: *mut MassEstimation_GetResult_Response) -> bool;
    fn as2_msgs__action__MassEstimation_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_GetResult_Response>, size: usize) -> bool;
    fn as2_msgs__action__MassEstimation_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_GetResult_Response>);
    fn as2_msgs__action__MassEstimation_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MassEstimation_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MassEstimation_GetResult_Response>) -> bool;
}

// Corresponds to as2_msgs__action__MassEstimation_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MassEstimation_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::MassEstimation_Result,

}



impl Default for MassEstimation_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__MassEstimation_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__MassEstimation_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MassEstimation_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__MassEstimation_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MassEstimation_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MassEstimation_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/MassEstimation_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__MassEstimation_GetResult_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__NavigateToPoint_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__NavigateToPoint_SendGoal_Request__init(msg: *mut NavigateToPoint_SendGoal_Request) -> bool;
    fn as2_msgs__action__NavigateToPoint_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_SendGoal_Request>, size: usize) -> bool;
    fn as2_msgs__action__NavigateToPoint_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_SendGoal_Request>);
    fn as2_msgs__action__NavigateToPoint_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateToPoint_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_SendGoal_Request>) -> bool;
}

// Corresponds to as2_msgs__action__NavigateToPoint_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateToPoint_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::NavigateToPoint_Goal,

}



impl Default for NavigateToPoint_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__NavigateToPoint_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__NavigateToPoint_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateToPoint_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateToPoint_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateToPoint_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/NavigateToPoint_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__NavigateToPoint_SendGoal_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__NavigateToPoint_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__NavigateToPoint_SendGoal_Response__init(msg: *mut NavigateToPoint_SendGoal_Response) -> bool;
    fn as2_msgs__action__NavigateToPoint_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_SendGoal_Response>, size: usize) -> bool;
    fn as2_msgs__action__NavigateToPoint_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_SendGoal_Response>);
    fn as2_msgs__action__NavigateToPoint_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateToPoint_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_SendGoal_Response>) -> bool;
}

// Corresponds to as2_msgs__action__NavigateToPoint_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateToPoint_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for NavigateToPoint_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__NavigateToPoint_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__NavigateToPoint_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateToPoint_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateToPoint_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateToPoint_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/NavigateToPoint_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__NavigateToPoint_SendGoal_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__NavigateToPoint_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__NavigateToPoint_GetResult_Request__init(msg: *mut NavigateToPoint_GetResult_Request) -> bool;
    fn as2_msgs__action__NavigateToPoint_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_GetResult_Request>, size: usize) -> bool;
    fn as2_msgs__action__NavigateToPoint_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_GetResult_Request>);
    fn as2_msgs__action__NavigateToPoint_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateToPoint_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_GetResult_Request>) -> bool;
}

// Corresponds to as2_msgs__action__NavigateToPoint_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateToPoint_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for NavigateToPoint_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__NavigateToPoint_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__NavigateToPoint_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateToPoint_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateToPoint_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateToPoint_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/NavigateToPoint_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__NavigateToPoint_GetResult_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__NavigateToPoint_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__NavigateToPoint_GetResult_Response__init(msg: *mut NavigateToPoint_GetResult_Response) -> bool;
    fn as2_msgs__action__NavigateToPoint_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_GetResult_Response>, size: usize) -> bool;
    fn as2_msgs__action__NavigateToPoint_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_GetResult_Response>);
    fn as2_msgs__action__NavigateToPoint_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateToPoint_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateToPoint_GetResult_Response>) -> bool;
}

// Corresponds to as2_msgs__action__NavigateToPoint_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateToPoint_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::NavigateToPoint_Result,

}



impl Default for NavigateToPoint_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__NavigateToPoint_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__NavigateToPoint_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateToPoint_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__NavigateToPoint_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateToPoint_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateToPoint_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/NavigateToPoint_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__NavigateToPoint_GetResult_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PointGimbal_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__PointGimbal_SendGoal_Request__init(msg: *mut PointGimbal_SendGoal_Request) -> bool;
    fn as2_msgs__action__PointGimbal_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_SendGoal_Request>, size: usize) -> bool;
    fn as2_msgs__action__PointGimbal_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_SendGoal_Request>);
    fn as2_msgs__action__PointGimbal_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointGimbal_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_SendGoal_Request>) -> bool;
}

// Corresponds to as2_msgs__action__PointGimbal_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointGimbal_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::PointGimbal_Goal,

}



impl Default for PointGimbal_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__PointGimbal_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__PointGimbal_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointGimbal_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointGimbal_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointGimbal_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/PointGimbal_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PointGimbal_SendGoal_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PointGimbal_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__PointGimbal_SendGoal_Response__init(msg: *mut PointGimbal_SendGoal_Response) -> bool;
    fn as2_msgs__action__PointGimbal_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_SendGoal_Response>, size: usize) -> bool;
    fn as2_msgs__action__PointGimbal_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_SendGoal_Response>);
    fn as2_msgs__action__PointGimbal_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointGimbal_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_SendGoal_Response>) -> bool;
}

// Corresponds to as2_msgs__action__PointGimbal_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointGimbal_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for PointGimbal_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__PointGimbal_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__PointGimbal_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointGimbal_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointGimbal_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointGimbal_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/PointGimbal_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PointGimbal_SendGoal_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PointGimbal_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__PointGimbal_GetResult_Request__init(msg: *mut PointGimbal_GetResult_Request) -> bool;
    fn as2_msgs__action__PointGimbal_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_GetResult_Request>, size: usize) -> bool;
    fn as2_msgs__action__PointGimbal_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_GetResult_Request>);
    fn as2_msgs__action__PointGimbal_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointGimbal_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_GetResult_Request>) -> bool;
}

// Corresponds to as2_msgs__action__PointGimbal_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointGimbal_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for PointGimbal_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__PointGimbal_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__PointGimbal_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointGimbal_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointGimbal_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointGimbal_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/PointGimbal_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PointGimbal_GetResult_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PointGimbal_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__PointGimbal_GetResult_Response__init(msg: *mut PointGimbal_GetResult_Response) -> bool;
    fn as2_msgs__action__PointGimbal_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_GetResult_Response>, size: usize) -> bool;
    fn as2_msgs__action__PointGimbal_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_GetResult_Response>);
    fn as2_msgs__action__PointGimbal_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointGimbal_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PointGimbal_GetResult_Response>) -> bool;
}

// Corresponds to as2_msgs__action__PointGimbal_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointGimbal_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::PointGimbal_Result,

}



impl Default for PointGimbal_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__PointGimbal_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__PointGimbal_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointGimbal_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PointGimbal_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointGimbal_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointGimbal_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/PointGimbal_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PointGimbal_GetResult_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PrecisionLanding_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__PrecisionLanding_SendGoal_Request__init(msg: *mut PrecisionLanding_SendGoal_Request) -> bool;
    fn as2_msgs__action__PrecisionLanding_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_SendGoal_Request>, size: usize) -> bool;
    fn as2_msgs__action__PrecisionLanding_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_SendGoal_Request>);
    fn as2_msgs__action__PrecisionLanding_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PrecisionLanding_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_SendGoal_Request>) -> bool;
}

// Corresponds to as2_msgs__action__PrecisionLanding_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PrecisionLanding_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::PrecisionLanding_Goal,

}



impl Default for PrecisionLanding_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__PrecisionLanding_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__PrecisionLanding_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PrecisionLanding_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PrecisionLanding_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PrecisionLanding_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/PrecisionLanding_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PrecisionLanding_SendGoal_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PrecisionLanding_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__PrecisionLanding_SendGoal_Response__init(msg: *mut PrecisionLanding_SendGoal_Response) -> bool;
    fn as2_msgs__action__PrecisionLanding_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_SendGoal_Response>, size: usize) -> bool;
    fn as2_msgs__action__PrecisionLanding_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_SendGoal_Response>);
    fn as2_msgs__action__PrecisionLanding_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PrecisionLanding_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_SendGoal_Response>) -> bool;
}

// Corresponds to as2_msgs__action__PrecisionLanding_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PrecisionLanding_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for PrecisionLanding_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__PrecisionLanding_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__PrecisionLanding_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PrecisionLanding_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PrecisionLanding_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PrecisionLanding_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/PrecisionLanding_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PrecisionLanding_SendGoal_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PrecisionLanding_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__PrecisionLanding_GetResult_Request__init(msg: *mut PrecisionLanding_GetResult_Request) -> bool;
    fn as2_msgs__action__PrecisionLanding_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_GetResult_Request>, size: usize) -> bool;
    fn as2_msgs__action__PrecisionLanding_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_GetResult_Request>);
    fn as2_msgs__action__PrecisionLanding_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PrecisionLanding_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_GetResult_Request>) -> bool;
}

// Corresponds to as2_msgs__action__PrecisionLanding_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PrecisionLanding_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for PrecisionLanding_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__PrecisionLanding_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__PrecisionLanding_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PrecisionLanding_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PrecisionLanding_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PrecisionLanding_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/PrecisionLanding_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PrecisionLanding_GetResult_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PrecisionLanding_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__PrecisionLanding_GetResult_Response__init(msg: *mut PrecisionLanding_GetResult_Response) -> bool;
    fn as2_msgs__action__PrecisionLanding_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_GetResult_Response>, size: usize) -> bool;
    fn as2_msgs__action__PrecisionLanding_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_GetResult_Response>);
    fn as2_msgs__action__PrecisionLanding_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PrecisionLanding_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PrecisionLanding_GetResult_Response>) -> bool;
}

// Corresponds to as2_msgs__action__PrecisionLanding_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PrecisionLanding_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::PrecisionLanding_Result,

}



impl Default for PrecisionLanding_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__PrecisionLanding_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__PrecisionLanding_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PrecisionLanding_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__PrecisionLanding_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PrecisionLanding_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PrecisionLanding_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/PrecisionLanding_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__PrecisionLanding_GetResult_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetArmingState_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SetArmingState_SendGoal_Request__init(msg: *mut SetArmingState_SendGoal_Request) -> bool;
    fn as2_msgs__action__SetArmingState_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_SendGoal_Request>, size: usize) -> bool;
    fn as2_msgs__action__SetArmingState_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_SendGoal_Request>);
    fn as2_msgs__action__SetArmingState_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetArmingState_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_SendGoal_Request>) -> bool;
}

// Corresponds to as2_msgs__action__SetArmingState_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetArmingState_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::SetArmingState_Goal,

}



impl Default for SetArmingState_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SetArmingState_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SetArmingState_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetArmingState_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetArmingState_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetArmingState_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SetArmingState_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetArmingState_SendGoal_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetArmingState_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SetArmingState_SendGoal_Response__init(msg: *mut SetArmingState_SendGoal_Response) -> bool;
    fn as2_msgs__action__SetArmingState_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_SendGoal_Response>, size: usize) -> bool;
    fn as2_msgs__action__SetArmingState_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_SendGoal_Response>);
    fn as2_msgs__action__SetArmingState_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetArmingState_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_SendGoal_Response>) -> bool;
}

// Corresponds to as2_msgs__action__SetArmingState_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetArmingState_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for SetArmingState_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SetArmingState_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SetArmingState_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetArmingState_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetArmingState_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetArmingState_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SetArmingState_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetArmingState_SendGoal_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetArmingState_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SetArmingState_GetResult_Request__init(msg: *mut SetArmingState_GetResult_Request) -> bool;
    fn as2_msgs__action__SetArmingState_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_GetResult_Request>, size: usize) -> bool;
    fn as2_msgs__action__SetArmingState_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_GetResult_Request>);
    fn as2_msgs__action__SetArmingState_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetArmingState_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_GetResult_Request>) -> bool;
}

// Corresponds to as2_msgs__action__SetArmingState_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetArmingState_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for SetArmingState_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SetArmingState_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SetArmingState_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetArmingState_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetArmingState_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetArmingState_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SetArmingState_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetArmingState_GetResult_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetArmingState_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SetArmingState_GetResult_Response__init(msg: *mut SetArmingState_GetResult_Response) -> bool;
    fn as2_msgs__action__SetArmingState_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_GetResult_Response>, size: usize) -> bool;
    fn as2_msgs__action__SetArmingState_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_GetResult_Response>);
    fn as2_msgs__action__SetArmingState_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetArmingState_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetArmingState_GetResult_Response>) -> bool;
}

// Corresponds to as2_msgs__action__SetArmingState_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetArmingState_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::SetArmingState_Result,

}



impl Default for SetArmingState_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SetArmingState_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SetArmingState_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetArmingState_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetArmingState_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetArmingState_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetArmingState_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SetArmingState_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetArmingState_GetResult_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetOffboardMode_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SetOffboardMode_SendGoal_Request__init(msg: *mut SetOffboardMode_SendGoal_Request) -> bool;
    fn as2_msgs__action__SetOffboardMode_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_SendGoal_Request>, size: usize) -> bool;
    fn as2_msgs__action__SetOffboardMode_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_SendGoal_Request>);
    fn as2_msgs__action__SetOffboardMode_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOffboardMode_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_SendGoal_Request>) -> bool;
}

// Corresponds to as2_msgs__action__SetOffboardMode_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOffboardMode_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::SetOffboardMode_Goal,

}



impl Default for SetOffboardMode_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SetOffboardMode_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SetOffboardMode_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOffboardMode_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOffboardMode_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOffboardMode_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SetOffboardMode_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetOffboardMode_SendGoal_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetOffboardMode_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SetOffboardMode_SendGoal_Response__init(msg: *mut SetOffboardMode_SendGoal_Response) -> bool;
    fn as2_msgs__action__SetOffboardMode_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_SendGoal_Response>, size: usize) -> bool;
    fn as2_msgs__action__SetOffboardMode_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_SendGoal_Response>);
    fn as2_msgs__action__SetOffboardMode_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOffboardMode_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_SendGoal_Response>) -> bool;
}

// Corresponds to as2_msgs__action__SetOffboardMode_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOffboardMode_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for SetOffboardMode_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SetOffboardMode_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SetOffboardMode_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOffboardMode_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOffboardMode_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOffboardMode_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SetOffboardMode_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetOffboardMode_SendGoal_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetOffboardMode_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SetOffboardMode_GetResult_Request__init(msg: *mut SetOffboardMode_GetResult_Request) -> bool;
    fn as2_msgs__action__SetOffboardMode_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_GetResult_Request>, size: usize) -> bool;
    fn as2_msgs__action__SetOffboardMode_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_GetResult_Request>);
    fn as2_msgs__action__SetOffboardMode_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOffboardMode_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_GetResult_Request>) -> bool;
}

// Corresponds to as2_msgs__action__SetOffboardMode_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOffboardMode_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for SetOffboardMode_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SetOffboardMode_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SetOffboardMode_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOffboardMode_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOffboardMode_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOffboardMode_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SetOffboardMode_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetOffboardMode_GetResult_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetOffboardMode_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SetOffboardMode_GetResult_Response__init(msg: *mut SetOffboardMode_GetResult_Response) -> bool;
    fn as2_msgs__action__SetOffboardMode_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_GetResult_Response>, size: usize) -> bool;
    fn as2_msgs__action__SetOffboardMode_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_GetResult_Response>);
    fn as2_msgs__action__SetOffboardMode_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOffboardMode_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOffboardMode_GetResult_Response>) -> bool;
}

// Corresponds to as2_msgs__action__SetOffboardMode_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOffboardMode_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::SetOffboardMode_Result,

}



impl Default for SetOffboardMode_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SetOffboardMode_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SetOffboardMode_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOffboardMode_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SetOffboardMode_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOffboardMode_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOffboardMode_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SetOffboardMode_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SetOffboardMode_GetResult_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SwarmFlocking_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SwarmFlocking_SendGoal_Request__init(msg: *mut SwarmFlocking_SendGoal_Request) -> bool;
    fn as2_msgs__action__SwarmFlocking_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_SendGoal_Request>, size: usize) -> bool;
    fn as2_msgs__action__SwarmFlocking_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_SendGoal_Request>);
    fn as2_msgs__action__SwarmFlocking_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwarmFlocking_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_SendGoal_Request>) -> bool;
}

// Corresponds to as2_msgs__action__SwarmFlocking_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwarmFlocking_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::SwarmFlocking_Goal,

}



impl Default for SwarmFlocking_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SwarmFlocking_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SwarmFlocking_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwarmFlocking_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwarmFlocking_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwarmFlocking_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SwarmFlocking_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SwarmFlocking_SendGoal_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SwarmFlocking_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SwarmFlocking_SendGoal_Response__init(msg: *mut SwarmFlocking_SendGoal_Response) -> bool;
    fn as2_msgs__action__SwarmFlocking_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_SendGoal_Response>, size: usize) -> bool;
    fn as2_msgs__action__SwarmFlocking_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_SendGoal_Response>);
    fn as2_msgs__action__SwarmFlocking_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwarmFlocking_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_SendGoal_Response>) -> bool;
}

// Corresponds to as2_msgs__action__SwarmFlocking_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwarmFlocking_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for SwarmFlocking_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SwarmFlocking_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SwarmFlocking_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwarmFlocking_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwarmFlocking_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwarmFlocking_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SwarmFlocking_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SwarmFlocking_SendGoal_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SwarmFlocking_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SwarmFlocking_GetResult_Request__init(msg: *mut SwarmFlocking_GetResult_Request) -> bool;
    fn as2_msgs__action__SwarmFlocking_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_GetResult_Request>, size: usize) -> bool;
    fn as2_msgs__action__SwarmFlocking_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_GetResult_Request>);
    fn as2_msgs__action__SwarmFlocking_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwarmFlocking_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_GetResult_Request>) -> bool;
}

// Corresponds to as2_msgs__action__SwarmFlocking_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwarmFlocking_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for SwarmFlocking_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SwarmFlocking_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SwarmFlocking_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwarmFlocking_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwarmFlocking_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwarmFlocking_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SwarmFlocking_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SwarmFlocking_GetResult_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SwarmFlocking_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__SwarmFlocking_GetResult_Response__init(msg: *mut SwarmFlocking_GetResult_Response) -> bool;
    fn as2_msgs__action__SwarmFlocking_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_GetResult_Response>, size: usize) -> bool;
    fn as2_msgs__action__SwarmFlocking_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_GetResult_Response>);
    fn as2_msgs__action__SwarmFlocking_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwarmFlocking_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SwarmFlocking_GetResult_Response>) -> bool;
}

// Corresponds to as2_msgs__action__SwarmFlocking_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwarmFlocking_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::SwarmFlocking_Result,

}



impl Default for SwarmFlocking_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__SwarmFlocking_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__SwarmFlocking_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwarmFlocking_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__SwarmFlocking_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwarmFlocking_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwarmFlocking_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/SwarmFlocking_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__SwarmFlocking_GetResult_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Takeoff_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__Takeoff_SendGoal_Request__init(msg: *mut Takeoff_SendGoal_Request) -> bool;
    fn as2_msgs__action__Takeoff_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_SendGoal_Request>, size: usize) -> bool;
    fn as2_msgs__action__Takeoff_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_SendGoal_Request>);
    fn as2_msgs__action__Takeoff_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Takeoff_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Takeoff_SendGoal_Request>) -> bool;
}

// Corresponds to as2_msgs__action__Takeoff_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Takeoff_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::Takeoff_Goal,

}



impl Default for Takeoff_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__Takeoff_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__Takeoff_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Takeoff_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Takeoff_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Takeoff_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/Takeoff_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Takeoff_SendGoal_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Takeoff_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__Takeoff_SendGoal_Response__init(msg: *mut Takeoff_SendGoal_Response) -> bool;
    fn as2_msgs__action__Takeoff_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_SendGoal_Response>, size: usize) -> bool;
    fn as2_msgs__action__Takeoff_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_SendGoal_Response>);
    fn as2_msgs__action__Takeoff_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Takeoff_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Takeoff_SendGoal_Response>) -> bool;
}

// Corresponds to as2_msgs__action__Takeoff_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Takeoff_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for Takeoff_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__Takeoff_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__Takeoff_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Takeoff_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Takeoff_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Takeoff_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/Takeoff_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Takeoff_SendGoal_Response() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Takeoff_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__Takeoff_GetResult_Request__init(msg: *mut Takeoff_GetResult_Request) -> bool;
    fn as2_msgs__action__Takeoff_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_GetResult_Request>, size: usize) -> bool;
    fn as2_msgs__action__Takeoff_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_GetResult_Request>);
    fn as2_msgs__action__Takeoff_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Takeoff_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Takeoff_GetResult_Request>) -> bool;
}

// Corresponds to as2_msgs__action__Takeoff_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Takeoff_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for Takeoff_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__Takeoff_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__Takeoff_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Takeoff_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Takeoff_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Takeoff_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/Takeoff_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Takeoff_GetResult_Request() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Takeoff_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__action__Takeoff_GetResult_Response__init(msg: *mut Takeoff_GetResult_Response) -> bool;
    fn as2_msgs__action__Takeoff_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_GetResult_Response>, size: usize) -> bool;
    fn as2_msgs__action__Takeoff_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_GetResult_Response>);
    fn as2_msgs__action__Takeoff_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Takeoff_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Takeoff_GetResult_Response>) -> bool;
}

// Corresponds to as2_msgs__action__Takeoff_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Takeoff_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::Takeoff_Result,

}



impl Default for Takeoff_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__action__Takeoff_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__action__Takeoff_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Takeoff_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__action__Takeoff_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Takeoff_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Takeoff_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/action/Takeoff_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__action__Takeoff_GetResult_Response() }
  }
}






#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__DetectArucoMarkers_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__DetectArucoMarkers_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct DetectArucoMarkers_SendGoal;

impl rosidl_runtime_rs::Service for DetectArucoMarkers_SendGoal {
    type Request = DetectArucoMarkers_SendGoal_Request;
    type Response = DetectArucoMarkers_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__DetectArucoMarkers_SendGoal() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__DetectArucoMarkers_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__DetectArucoMarkers_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct DetectArucoMarkers_GetResult;

impl rosidl_runtime_rs::Service for DetectArucoMarkers_GetResult {
    type Request = DetectArucoMarkers_GetResult_Request;
    type Response = DetectArucoMarkers_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__DetectArucoMarkers_GetResult() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__FollowPath_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__FollowPath_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct FollowPath_SendGoal;

impl rosidl_runtime_rs::Service for FollowPath_SendGoal {
    type Request = FollowPath_SendGoal_Request;
    type Response = FollowPath_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__FollowPath_SendGoal() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__FollowPath_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__FollowPath_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct FollowPath_GetResult;

impl rosidl_runtime_rs::Service for FollowPath_GetResult {
    type Request = FollowPath_GetResult_Request;
    type Response = FollowPath_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__FollowPath_GetResult() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__FollowReference_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__FollowReference_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct FollowReference_SendGoal;

impl rosidl_runtime_rs::Service for FollowReference_SendGoal {
    type Request = FollowReference_SendGoal_Request;
    type Response = FollowReference_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__FollowReference_SendGoal() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__FollowReference_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__FollowReference_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct FollowReference_GetResult;

impl rosidl_runtime_rs::Service for FollowReference_GetResult {
    type Request = FollowReference_GetResult_Request;
    type Response = FollowReference_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__FollowReference_GetResult() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__ForceEstimation_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__ForceEstimation_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct ForceEstimation_SendGoal;

impl rosidl_runtime_rs::Service for ForceEstimation_SendGoal {
    type Request = ForceEstimation_SendGoal_Request;
    type Response = ForceEstimation_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__ForceEstimation_SendGoal() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__ForceEstimation_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__ForceEstimation_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct ForceEstimation_GetResult;

impl rosidl_runtime_rs::Service for ForceEstimation_GetResult {
    type Request = ForceEstimation_GetResult_Request;
    type Response = ForceEstimation_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__ForceEstimation_GetResult() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__GeneratePolynomialTrajectory_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct GeneratePolynomialTrajectory_SendGoal;

impl rosidl_runtime_rs::Service for GeneratePolynomialTrajectory_SendGoal {
    type Request = GeneratePolynomialTrajectory_SendGoal_Request;
    type Response = GeneratePolynomialTrajectory_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_SendGoal() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__GeneratePolynomialTrajectory_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct GeneratePolynomialTrajectory_GetResult;

impl rosidl_runtime_rs::Service for GeneratePolynomialTrajectory_GetResult {
    type Request = GeneratePolynomialTrajectory_GetResult_Request;
    type Response = GeneratePolynomialTrajectory_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__GeneratePolynomialTrajectory_GetResult() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__GoToWaypoint_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__GoToWaypoint_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct GoToWaypoint_SendGoal;

impl rosidl_runtime_rs::Service for GoToWaypoint_SendGoal {
    type Request = GoToWaypoint_SendGoal_Request;
    type Response = GoToWaypoint_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__GoToWaypoint_SendGoal() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__GoToWaypoint_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__GoToWaypoint_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct GoToWaypoint_GetResult;

impl rosidl_runtime_rs::Service for GoToWaypoint_GetResult {
    type Request = GoToWaypoint_GetResult_Request;
    type Response = GoToWaypoint_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__GoToWaypoint_GetResult() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__GripperHandler_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__GripperHandler_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct GripperHandler_SendGoal;

impl rosidl_runtime_rs::Service for GripperHandler_SendGoal {
    type Request = GripperHandler_SendGoal_Request;
    type Response = GripperHandler_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__GripperHandler_SendGoal() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__GripperHandler_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__GripperHandler_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct GripperHandler_GetResult;

impl rosidl_runtime_rs::Service for GripperHandler_GetResult {
    type Request = GripperHandler_GetResult_Request;
    type Response = GripperHandler_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__GripperHandler_GetResult() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__Land_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__Land_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct Land_SendGoal;

impl rosidl_runtime_rs::Service for Land_SendGoal {
    type Request = Land_SendGoal_Request;
    type Response = Land_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__Land_SendGoal() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__Land_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__Land_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct Land_GetResult;

impl rosidl_runtime_rs::Service for Land_GetResult {
    type Request = Land_GetResult_Request;
    type Response = Land_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__Land_GetResult() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__MassEstimation_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__MassEstimation_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct MassEstimation_SendGoal;

impl rosidl_runtime_rs::Service for MassEstimation_SendGoal {
    type Request = MassEstimation_SendGoal_Request;
    type Response = MassEstimation_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__MassEstimation_SendGoal() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__MassEstimation_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__MassEstimation_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct MassEstimation_GetResult;

impl rosidl_runtime_rs::Service for MassEstimation_GetResult {
    type Request = MassEstimation_GetResult_Request;
    type Response = MassEstimation_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__MassEstimation_GetResult() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__NavigateToPoint_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__NavigateToPoint_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct NavigateToPoint_SendGoal;

impl rosidl_runtime_rs::Service for NavigateToPoint_SendGoal {
    type Request = NavigateToPoint_SendGoal_Request;
    type Response = NavigateToPoint_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__NavigateToPoint_SendGoal() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__NavigateToPoint_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__NavigateToPoint_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct NavigateToPoint_GetResult;

impl rosidl_runtime_rs::Service for NavigateToPoint_GetResult {
    type Request = NavigateToPoint_GetResult_Request;
    type Response = NavigateToPoint_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__NavigateToPoint_GetResult() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__PointGimbal_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__PointGimbal_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct PointGimbal_SendGoal;

impl rosidl_runtime_rs::Service for PointGimbal_SendGoal {
    type Request = PointGimbal_SendGoal_Request;
    type Response = PointGimbal_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__PointGimbal_SendGoal() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__PointGimbal_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__PointGimbal_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct PointGimbal_GetResult;

impl rosidl_runtime_rs::Service for PointGimbal_GetResult {
    type Request = PointGimbal_GetResult_Request;
    type Response = PointGimbal_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__PointGimbal_GetResult() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__PrecisionLanding_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__PrecisionLanding_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct PrecisionLanding_SendGoal;

impl rosidl_runtime_rs::Service for PrecisionLanding_SendGoal {
    type Request = PrecisionLanding_SendGoal_Request;
    type Response = PrecisionLanding_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__PrecisionLanding_SendGoal() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__PrecisionLanding_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__PrecisionLanding_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct PrecisionLanding_GetResult;

impl rosidl_runtime_rs::Service for PrecisionLanding_GetResult {
    type Request = PrecisionLanding_GetResult_Request;
    type Response = PrecisionLanding_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__PrecisionLanding_GetResult() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__SetArmingState_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__SetArmingState_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct SetArmingState_SendGoal;

impl rosidl_runtime_rs::Service for SetArmingState_SendGoal {
    type Request = SetArmingState_SendGoal_Request;
    type Response = SetArmingState_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__SetArmingState_SendGoal() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__SetArmingState_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__SetArmingState_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct SetArmingState_GetResult;

impl rosidl_runtime_rs::Service for SetArmingState_GetResult {
    type Request = SetArmingState_GetResult_Request;
    type Response = SetArmingState_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__SetArmingState_GetResult() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__SetOffboardMode_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__SetOffboardMode_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct SetOffboardMode_SendGoal;

impl rosidl_runtime_rs::Service for SetOffboardMode_SendGoal {
    type Request = SetOffboardMode_SendGoal_Request;
    type Response = SetOffboardMode_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__SetOffboardMode_SendGoal() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__SetOffboardMode_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__SetOffboardMode_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct SetOffboardMode_GetResult;

impl rosidl_runtime_rs::Service for SetOffboardMode_GetResult {
    type Request = SetOffboardMode_GetResult_Request;
    type Response = SetOffboardMode_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__SetOffboardMode_GetResult() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__SwarmFlocking_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__SwarmFlocking_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct SwarmFlocking_SendGoal;

impl rosidl_runtime_rs::Service for SwarmFlocking_SendGoal {
    type Request = SwarmFlocking_SendGoal_Request;
    type Response = SwarmFlocking_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__SwarmFlocking_SendGoal() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__SwarmFlocking_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__SwarmFlocking_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct SwarmFlocking_GetResult;

impl rosidl_runtime_rs::Service for SwarmFlocking_GetResult {
    type Request = SwarmFlocking_GetResult_Request;
    type Response = SwarmFlocking_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__SwarmFlocking_GetResult() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__Takeoff_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__Takeoff_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct Takeoff_SendGoal;

impl rosidl_runtime_rs::Service for Takeoff_SendGoal {
    type Request = Takeoff_SendGoal_Request;
    type Response = Takeoff_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__Takeoff_SendGoal() }
    }
}




#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__Takeoff_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to as2_msgs__action__Takeoff_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct Takeoff_GetResult;

impl rosidl_runtime_rs::Service for Takeoff_GetResult {
    type Request = Takeoff_GetResult_Request;
    type Response = Takeoff_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__as2_msgs__action__Takeoff_GetResult() }
    }
}


