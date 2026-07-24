#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__Acro() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__Acro__init(msg: *mut Acro) -> bool;
    fn as2_msgs__msg__Acro__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Acro>, size: usize) -> bool;
    fn as2_msgs__msg__Acro__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Acro>);
    fn as2_msgs__msg__Acro__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Acro>, out_seq: *mut rosidl_runtime_rs::Sequence<Acro>) -> bool;
}

// Corresponds to as2_msgs__msg__Acro
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message for RPY rates and thrust (ACRO)

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Acro {
    /// Message header
    pub header: std_msgs::msg::rmw::Header,

    /// Roll-, pitch-, yaw-rate around body axes
    pub angular_rates: geometry_msgs::msg::rmw::Vector3,

    /// Thrust expressed in the body frame.
    /// For a fixed-wing, usually the x-component is used.
    /// For a multi-rotor, usually the z-component is used.
    /// Set all un-used components to 0.
    pub thrust: geometry_msgs::msg::rmw::Vector3,

}



impl Default for Acro {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__Acro__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__Acro__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Acro {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__Acro__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__Acro__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__Acro__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Acro {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Acro where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/Acro";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__Acro() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__AlertEvent() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__AlertEvent__init(msg: *mut AlertEvent) -> bool;
    fn as2_msgs__msg__AlertEvent__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AlertEvent>, size: usize) -> bool;
    fn as2_msgs__msg__AlertEvent__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AlertEvent>);
    fn as2_msgs__msg__AlertEvent__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AlertEvent>, out_seq: *mut rosidl_runtime_rs::Sequence<AlertEvent>) -> bool;
}

// Corresponds to as2_msgs__msg__AlertEvent
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message that encodes different alert Events that can be handled by AS2 framework

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AlertEvent {

    // This member is not documented.
    #[allow(missing_docs)]
    pub alert: i8,

    /// Further description of the alert, for debugging purposes mainly
    pub description: rosidl_runtime_rs::String,

}

impl AlertEvent {
    /// if value is < 0 then this alert will be handled by the platform directly
    pub const KILL_SWITCH: i8 = -1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const EMERGENCY_HOVER: i8 = -2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const EMERGENCY_LAND: i8 = -3;

    /// 0 value is used for ping or info alerts
    pub const INFO_ALERT: i8 = 0;

    /// if value is > 0 then this alert will be handled by the AS2 framework
    pub const FORCE_HOVER: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const FORCE_LAND: i8 = 2;

}


impl Default for AlertEvent {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__AlertEvent__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__AlertEvent__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AlertEvent {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__AlertEvent__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__AlertEvent__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__AlertEvent__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AlertEvent {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AlertEvent where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/AlertEvent";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__AlertEvent() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__BehaviorStatus() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__BehaviorStatus__init(msg: *mut BehaviorStatus) -> bool;
    fn as2_msgs__msg__BehaviorStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BehaviorStatus>, size: usize) -> bool;
    fn as2_msgs__msg__BehaviorStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BehaviorStatus>);
    fn as2_msgs__msg__BehaviorStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BehaviorStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<BehaviorStatus>) -> bool;
}

// Corresponds to as2_msgs__msg__BehaviorStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BehaviorStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: u8,

}

impl BehaviorStatus {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const IDLE: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RUNNING: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PAUSED: u8 = 2;

}


impl Default for BehaviorStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__BehaviorStatus__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__BehaviorStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BehaviorStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__BehaviorStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__BehaviorStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__BehaviorStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BehaviorStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BehaviorStatus where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/BehaviorStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__BehaviorStatus() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__ControlMode() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__ControlMode__init(msg: *mut ControlMode) -> bool;
    fn as2_msgs__msg__ControlMode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ControlMode>, size: usize) -> bool;
    fn as2_msgs__msg__ControlMode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ControlMode>);
    fn as2_msgs__msg__ControlMode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ControlMode>, out_seq: *mut rosidl_runtime_rs::Sequence<ControlMode>) -> bool;
}

// Corresponds to as2_msgs__msg__ControlMode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message that encodes the possible control modes supported in Aerostack2.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ControlMode {
    /// Message header
    pub header: std_msgs::msg::rmw::Header,

    /// Yaw mode
    pub yaw_mode: i8,

    /// Control mode
    pub control_mode: i8,

    /// Reference frame
    pub reference_frame: i8,

}

impl ControlMode {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const NONE: i8 = 0;

    /// Yaw angle control mode (rad)
    pub const YAW_ANGLE: i8 = 1;

    /// Yaw speed control mode (rad/s)
    pub const YAW_SPEED: i8 = 2;

    /// mode when the controller is not set
    pub const UNSET: i8 = 0;

    /// mode when the controller is in hover mode
    pub const HOVER: i8 = 1;

    /// x,   y , z  refs
    pub const POSITION: i8 = 2;

    /// vx, vy , vz refs
    pub const SPEED: i8 = 3;

    /// vx, vy , z refs
    pub const SPEED_IN_A_PLANE: i8 = 4;

    /// roll, pitch, yaw  refs
    pub const ATTITUDE: i8 = 5;

    /// roll, pitch, yawrate  refs
    pub const ACRO: i8 = 6;

    /// x, y z, vx, vy, vz, ax, ay, az refs
    pub const TRAJECTORY: i8 = 7;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const UNDEFINED_FRAME: i8 = 0;

    /// local coordinates (use this by default)
    pub const LOCAL_ENU_FRAME: i8 = 1;

    /// body coordinates
    pub const BODY_FLU_FRAME: i8 = 2;

    /// gnss wcoordinates
    pub const GLOBAL_LAT_LONG_ASML: i8 = 3;

}


impl Default for ControlMode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__ControlMode__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__ControlMode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ControlMode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__ControlMode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__ControlMode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__ControlMode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ControlMode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ControlMode where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/ControlMode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__ControlMode() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__ControllerInfo() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__ControllerInfo__init(msg: *mut ControllerInfo) -> bool;
    fn as2_msgs__msg__ControllerInfo__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ControllerInfo>, size: usize) -> bool;
    fn as2_msgs__msg__ControllerInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ControllerInfo>);
    fn as2_msgs__msg__ControllerInfo__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ControllerInfo>, out_seq: *mut rosidl_runtime_rs::Sequence<ControllerInfo>) -> bool;
}

// Corresponds to as2_msgs__msg__ControllerInfo
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message that shows the controller state and the current input_control_mode

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ControllerInfo {
    /// Message header
    pub header: std_msgs::msg::rmw::Header,

    /// Input control mode
    pub input_control_mode: super::super::msg::rmw::ControlMode,

    /// Output control mode
    pub output_control_mode: super::super::msg::rmw::ControlMode,

}



impl Default for ControllerInfo {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__ControllerInfo__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__ControllerInfo__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ControllerInfo {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__ControllerInfo__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__ControllerInfo__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__ControllerInfo__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ControllerInfo {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ControllerInfo where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/ControllerInfo";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__ControllerInfo() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__FollowTargetInfo() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__FollowTargetInfo__init(msg: *mut FollowTargetInfo) -> bool;
    fn as2_msgs__msg__FollowTargetInfo__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowTargetInfo>, size: usize) -> bool;
    fn as2_msgs__msg__FollowTargetInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowTargetInfo>);
    fn as2_msgs__msg__FollowTargetInfo__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowTargetInfo>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowTargetInfo>) -> bool;
}

// Corresponds to as2_msgs__msg__FollowTargetInfo
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message that encodes the possible follow target info supported in Aerostack2.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowTargetInfo {
    /// Message header
    pub header: std_msgs::msg::rmw::Header,

    /// Follow status
    pub follow_status: i8,

    /// Follow mode
    pub follow_mode: i8,

}

impl FollowTargetInfo {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const WAITING: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RUNNING: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const END: i8 = 2;

    /// Mode when the follow mode is not set
    pub const UNSET: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PICKUP: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const UNPICK: i8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DYNAMIC_LAND: i8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DYNAMIC_FOLLOWER: i8 = 4;

}


impl Default for FollowTargetInfo {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__FollowTargetInfo__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__FollowTargetInfo__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowTargetInfo {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__FollowTargetInfo__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__FollowTargetInfo__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__FollowTargetInfo__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowTargetInfo {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowTargetInfo where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/FollowTargetInfo";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__FollowTargetInfo() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__Geozone() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__Geozone__init(msg: *mut Geozone) -> bool;
    fn as2_msgs__msg__Geozone__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Geozone>, size: usize) -> bool;
    fn as2_msgs__msg__Geozone__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Geozone>);
    fn as2_msgs__msg__Geozone__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Geozone>, out_seq: *mut rosidl_runtime_rs::Sequence<Geozone>) -> bool;
}

// Corresponds to as2_msgs__msg__Geozone
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// GeoStructure defined by an id, alert that generates in case of event and a polygon 
/// that defines the area.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Geozone {
    /// geofence id
    pub id: i8,

    /// alert generated
    pub alert: i8,

    /// geofence or geocage
    pub type_: rosidl_runtime_rs::String,

    /// cartesian or gps
    pub data_type: rosidl_runtime_rs::String,

    /// fence polygon
    pub polygon: geometry_msgs::msg::rmw::Polygon,

    /// height limit up
    pub z_up: f32,

    /// height limit bottom
    pub z_down: f32,

}



impl Default for Geozone {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__Geozone__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__Geozone__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Geozone {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__Geozone__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__Geozone__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__Geozone__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Geozone {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Geozone where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/Geozone";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__Geozone() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__GimbalControl() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__GimbalControl__init(msg: *mut GimbalControl) -> bool;
    fn as2_msgs__msg__GimbalControl__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GimbalControl>, size: usize) -> bool;
    fn as2_msgs__msg__GimbalControl__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GimbalControl>);
    fn as2_msgs__msg__GimbalControl__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GimbalControl>, out_seq: *mut rosidl_runtime_rs::Sequence<GimbalControl>) -> bool;
}

// Corresponds to as2_msgs__msg__GimbalControl
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Gimbal Control message definition

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GimbalControl {

    // This member is not documented.
    #[allow(missing_docs)]
    pub control_mode: u8,

    /// x: roll y: pitch z: yaw
    pub target: geometry_msgs::msg::rmw::Vector3Stamped,

}

impl GimbalControl {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const POSITION_MODE: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SPEED_MODE: u8 = 1;

}


impl Default for GimbalControl {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__GimbalControl__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__GimbalControl__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GimbalControl {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__GimbalControl__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__GimbalControl__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__GimbalControl__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GimbalControl {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GimbalControl where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/GimbalControl";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__GimbalControl() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__MissionEvent() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__MissionEvent__init(msg: *mut MissionEvent) -> bool;
    fn as2_msgs__msg__MissionEvent__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MissionEvent>, size: usize) -> bool;
    fn as2_msgs__msg__MissionEvent__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MissionEvent>);
    fn as2_msgs__msg__MissionEvent__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MissionEvent>, out_seq: *mut rosidl_runtime_rs::Sequence<MissionEvent>) -> bool;
}

// Corresponds to as2_msgs__msg__MissionEvent
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message for trigger mission events

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MissionEvent {
    /// Message header
    pub header: std_msgs::msg::rmw::Header,

    /// (Optional) data to send with the trigger
    pub data: rosidl_runtime_rs::String,

}



impl Default for MissionEvent {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__MissionEvent__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__MissionEvent__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MissionEvent {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__MissionEvent__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__MissionEvent__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__MissionEvent__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MissionEvent {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MissionEvent where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/MissionEvent";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__MissionEvent() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__MissionUpdate() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__MissionUpdate__init(msg: *mut MissionUpdate) -> bool;
    fn as2_msgs__msg__MissionUpdate__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MissionUpdate>, size: usize) -> bool;
    fn as2_msgs__msg__MissionUpdate__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MissionUpdate>);
    fn as2_msgs__msg__MissionUpdate__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MissionUpdate>, out_seq: *mut rosidl_runtime_rs::Sequence<MissionUpdate>) -> bool;
}

// Corresponds to as2_msgs__msg__MissionUpdate
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message that sends a mission to the interpreter

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MissionUpdate {
    /// ID of the drone that will execute the mission
    pub drone_id: rosidl_runtime_rs::String,

    /// ID of the mission to be executed
    pub mission_id: i32,

    /// ID of the item to be executed
    pub item_id: i32,

    /// Action to be performed in the interpreter
    pub action: u8,

    /// JSON formatted mission to be executed
    pub mission: rosidl_runtime_rs::String,

}

impl MissionUpdate {
    /// Execute a mission in the interpreter
    pub const EXECUTE: u8 = 0;

    /// Load a mission to the interpreter
    pub const LOAD: u8 = 1;

    /// Start the execution of a mission
    pub const START: u8 = 2;

    /// Pause the execution of a mission
    pub const PAUSE: u8 = 3;

    /// Resume the execution of a mission
    pub const RESUME: u8 = 4;

    /// Stop the execution of a mission
    pub const STOP: u8 = 5;

    /// Execute the next item in the mission
    pub const NEXT_ITEM: u8 = 6;

    /// Repeat the execution of a mission
    pub const REPEAT: u8 = 7;

    /// Insert an item in the mission
    pub const INSERT: u8 = 8;

    /// Modify an item in the mission
    pub const MODIFY: u8 = 9;

    /// Remove an item in the mission
    pub const REMOVE: u8 = 10;

    /// Reset the interpreter
    pub const RESET: u8 = 11;

}


impl Default for MissionUpdate {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__MissionUpdate__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__MissionUpdate__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MissionUpdate {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__MissionUpdate__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__MissionUpdate__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__MissionUpdate__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MissionUpdate {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MissionUpdate where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/MissionUpdate";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__MissionUpdate() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__NodeStatus() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__NodeStatus__init(msg: *mut NodeStatus) -> bool;
    fn as2_msgs__msg__NodeStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NodeStatus>, size: usize) -> bool;
    fn as2_msgs__msg__NodeStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NodeStatus>);
    fn as2_msgs__msg__NodeStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NodeStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<NodeStatus>) -> bool;
}

// Corresponds to as2_msgs__msg__NodeStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message that shows the node status

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NodeStatus {
    /// node status
    pub status: i8,

}

impl NodeStatus {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const UNCONFIGURED: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INACTIVE: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ACTIVE: i8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const FINALIZED: i8 = 3;

}


impl Default for NodeStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__NodeStatus__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__NodeStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NodeStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__NodeStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__NodeStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__NodeStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NodeStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NodeStatus where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/NodeStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__NodeStatus() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__PlatformInfo() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__PlatformInfo__init(msg: *mut PlatformInfo) -> bool;
    fn as2_msgs__msg__PlatformInfo__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlatformInfo>, size: usize) -> bool;
    fn as2_msgs__msg__PlatformInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlatformInfo>);
    fn as2_msgs__msg__PlatformInfo__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlatformInfo>, out_seq: *mut rosidl_runtime_rs::Sequence<PlatformInfo>) -> bool;
}

// Corresponds to as2_msgs__msg__PlatformInfo
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message that shows the platform status and the current control mode

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlatformInfo {
    /// Message header
    pub header: std_msgs::msg::rmw::Header,

    /// Whether the platform is connected or not
    pub connected: bool,

    /// Whether the platform is armed or not
    pub armed: bool,

    /// Whether the offboard mode is set or not
    pub offboard: bool,

    /// Platform status
    pub status: super::super::msg::rmw::PlatformStatus,

    /// Platform control mode
    pub current_control_mode: super::super::msg::rmw::ControlMode,

}



impl Default for PlatformInfo {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__PlatformInfo__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__PlatformInfo__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlatformInfo {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PlatformInfo__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PlatformInfo__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PlatformInfo__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlatformInfo {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlatformInfo where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/PlatformInfo";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__PlatformInfo() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__PlatformStateMachineEvent() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__PlatformStateMachineEvent__init(msg: *mut PlatformStateMachineEvent) -> bool;
    fn as2_msgs__msg__PlatformStateMachineEvent__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlatformStateMachineEvent>, size: usize) -> bool;
    fn as2_msgs__msg__PlatformStateMachineEvent__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlatformStateMachineEvent>);
    fn as2_msgs__msg__PlatformStateMachineEvent__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlatformStateMachineEvent>, out_seq: *mut rosidl_runtime_rs::Sequence<PlatformStateMachineEvent>) -> bool;
}

// Corresponds to as2_msgs__msg__PlatformStateMachineEvent
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Platform events that actives aerial platform state machine 

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlatformStateMachineEvent {
    /// Platform state machine event
    pub event: i8,

}

impl PlatformStateMachineEvent {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const EMERGENCY: i8 = -1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ARM: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DISARM: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TAKE_OFF: i8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TOOK_OFF: i8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LAND: i8 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LANDED: i8 = 5;

}


impl Default for PlatformStateMachineEvent {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__PlatformStateMachineEvent__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__PlatformStateMachineEvent__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlatformStateMachineEvent {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PlatformStateMachineEvent__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PlatformStateMachineEvent__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PlatformStateMachineEvent__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlatformStateMachineEvent {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlatformStateMachineEvent where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/PlatformStateMachineEvent";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__PlatformStateMachineEvent() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__PlatformStatus() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__PlatformStatus__init(msg: *mut PlatformStatus) -> bool;
    fn as2_msgs__msg__PlatformStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlatformStatus>, size: usize) -> bool;
    fn as2_msgs__msg__PlatformStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlatformStatus>);
    fn as2_msgs__msg__PlatformStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlatformStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<PlatformStatus>) -> bool;
}

// Corresponds to as2_msgs__msg__PlatformStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Platform states that an aerial platform can have

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlatformStatus {
    /// platform status
    pub state: i8,

}

impl PlatformStatus {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const EMERGENCY: i8 = -1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DISARMED: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LANDED: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TAKING_OFF: i8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const FLYING: i8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LANDING: i8 = 4;

}


impl Default for PlatformStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__PlatformStatus__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__PlatformStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlatformStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PlatformStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PlatformStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PlatformStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlatformStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlatformStatus where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/PlatformStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__PlatformStatus() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__PolygonList() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__PolygonList__init(msg: *mut PolygonList) -> bool;
    fn as2_msgs__msg__PolygonList__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PolygonList>, size: usize) -> bool;
    fn as2_msgs__msg__PolygonList__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PolygonList>);
    fn as2_msgs__msg__PolygonList__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PolygonList>, out_seq: *mut rosidl_runtime_rs::Sequence<PolygonList>) -> bool;
}

// Corresponds to as2_msgs__msg__PolygonList
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// List of polygons to visualize multiple geozones in RVIZ2

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolygonList {

    // This member is not documented.
    #[allow(missing_docs)]
    pub polygons: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::PolygonStamped>,

}



impl Default for PolygonList {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__PolygonList__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__PolygonList__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PolygonList {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PolygonList__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PolygonList__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PolygonList__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PolygonList {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PolygonList where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/PolygonList";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__PolygonList() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__PoseStampedWithID() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__PoseStampedWithID__init(msg: *mut PoseStampedWithID) -> bool;
    fn as2_msgs__msg__PoseStampedWithID__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PoseStampedWithID>, size: usize) -> bool;
    fn as2_msgs__msg__PoseStampedWithID__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PoseStampedWithID>);
    fn as2_msgs__msg__PoseStampedWithID__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PoseStampedWithID>, out_seq: *mut rosidl_runtime_rs::Sequence<PoseStampedWithID>) -> bool;
}

// Corresponds to as2_msgs__msg__PoseStampedWithID
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A Pose stamped with an string id

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PoseStampedWithID {
    /// Identification string
    pub id: rosidl_runtime_rs::String,

    /// Pose
    pub pose: geometry_msgs::msg::rmw::PoseStamped,

}



impl Default for PoseStampedWithID {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__PoseStampedWithID__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__PoseStampedWithID__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PoseStampedWithID {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PoseStampedWithID__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PoseStampedWithID__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PoseStampedWithID__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PoseStampedWithID {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PoseStampedWithID where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/PoseStampedWithID";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__PoseStampedWithID() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__PoseStampedWithIDArray() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__PoseStampedWithIDArray__init(msg: *mut PoseStampedWithIDArray) -> bool;
    fn as2_msgs__msg__PoseStampedWithIDArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PoseStampedWithIDArray>, size: usize) -> bool;
    fn as2_msgs__msg__PoseStampedWithIDArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PoseStampedWithIDArray>);
    fn as2_msgs__msg__PoseStampedWithIDArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PoseStampedWithIDArray>, out_seq: *mut rosidl_runtime_rs::Sequence<PoseStampedWithIDArray>) -> bool;
}

// Corresponds to as2_msgs__msg__PoseStampedWithIDArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Pose Stamped with an string id array

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PoseStampedWithIDArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub poses: rosidl_runtime_rs::Sequence<super::super::msg::rmw::PoseStampedWithID>,

}



impl Default for PoseStampedWithIDArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__PoseStampedWithIDArray__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__PoseStampedWithIDArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PoseStampedWithIDArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PoseStampedWithIDArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PoseStampedWithIDArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PoseStampedWithIDArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PoseStampedWithIDArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PoseStampedWithIDArray where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/PoseStampedWithIDArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__PoseStampedWithIDArray() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__PoseWithID() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__PoseWithID__init(msg: *mut PoseWithID) -> bool;
    fn as2_msgs__msg__PoseWithID__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PoseWithID>, size: usize) -> bool;
    fn as2_msgs__msg__PoseWithID__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PoseWithID>);
    fn as2_msgs__msg__PoseWithID__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PoseWithID>, out_seq: *mut rosidl_runtime_rs::Sequence<PoseWithID>) -> bool;
}

// Corresponds to as2_msgs__msg__PoseWithID
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A Pose with an string id

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PoseWithID {
    /// Identification string
    pub id: rosidl_runtime_rs::String,

    /// Pose
    pub pose: geometry_msgs::msg::rmw::Pose,

}



impl Default for PoseWithID {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__PoseWithID__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__PoseWithID__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PoseWithID {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PoseWithID__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PoseWithID__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PoseWithID__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PoseWithID {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PoseWithID where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/PoseWithID";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__PoseWithID() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__PoseWithIDArray() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__PoseWithIDArray__init(msg: *mut PoseWithIDArray) -> bool;
    fn as2_msgs__msg__PoseWithIDArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PoseWithIDArray>, size: usize) -> bool;
    fn as2_msgs__msg__PoseWithIDArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PoseWithIDArray>);
    fn as2_msgs__msg__PoseWithIDArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PoseWithIDArray>, out_seq: *mut rosidl_runtime_rs::Sequence<PoseWithIDArray>) -> bool;
}

// Corresponds to as2_msgs__msg__PoseWithIDArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Pose with an string id array

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PoseWithIDArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub poses: rosidl_runtime_rs::Sequence<super::super::msg::rmw::PoseWithID>,

}



impl Default for PoseWithIDArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__PoseWithIDArray__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__PoseWithIDArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PoseWithIDArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PoseWithIDArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PoseWithIDArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__PoseWithIDArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PoseWithIDArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PoseWithIDArray where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/PoseWithIDArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__PoseWithIDArray() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__Speed() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__Speed__init(msg: *mut Speed) -> bool;
    fn as2_msgs__msg__Speed__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Speed>, size: usize) -> bool;
    fn as2_msgs__msg__Speed__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Speed>);
    fn as2_msgs__msg__Speed__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Speed>, out_seq: *mut rosidl_runtime_rs::Sequence<Speed>) -> bool;
}

// Corresponds to as2_msgs__msg__Speed
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Speed message

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Speed {
    /// Message header
    pub header: std_msgs::msg::rmw::Header,

    /// speed (m/s)
    pub speed: f32,

}



impl Default for Speed {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__Speed__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__Speed__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Speed {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__Speed__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__Speed__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__Speed__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Speed {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Speed where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/Speed";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__Speed() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__Thrust() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__Thrust__init(msg: *mut Thrust) -> bool;
    fn as2_msgs__msg__Thrust__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Thrust>, size: usize) -> bool;
    fn as2_msgs__msg__Thrust__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Thrust>);
    fn as2_msgs__msg__Thrust__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Thrust>, out_seq: *mut rosidl_runtime_rs::Sequence<Thrust>) -> bool;
}

// Corresponds to as2_msgs__msg__Thrust
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message for encoding the desired thrust value

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Thrust {
    /// Message header
    pub header: std_msgs::msg::rmw::Header,

    /// Thrust (N)
    pub thrust: f32,

    /// Thrust normalized [0,1]
    pub thrust_normalized: f32,

}



impl Default for Thrust {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__Thrust__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__Thrust__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Thrust {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__Thrust__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__Thrust__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__Thrust__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Thrust {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Thrust where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/Thrust";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__Thrust() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__TrajGenInfo() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__TrajGenInfo__init(msg: *mut TrajGenInfo) -> bool;
    fn as2_msgs__msg__TrajGenInfo__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TrajGenInfo>, size: usize) -> bool;
    fn as2_msgs__msg__TrajGenInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TrajGenInfo>);
    fn as2_msgs__msg__TrajGenInfo__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TrajGenInfo>, out_seq: *mut rosidl_runtime_rs::Sequence<TrajGenInfo>) -> bool;
}

// Corresponds to as2_msgs__msg__TrajGenInfo
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message that shows the trajectory generator state

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajGenInfo {
    /// Message header
    pub header: std_msgs::msg::rmw::Header,

    /// Node status
    pub node_status: super::super::msg::rmw::NodeStatus,

    /// Active status
    pub active_status: u8,

}

impl TrajGenInfo {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const WAITING: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const EVALUATING: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STOPPED: u8 = 2;

}


impl Default for TrajGenInfo {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__TrajGenInfo__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__TrajGenInfo__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TrajGenInfo {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__TrajGenInfo__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__TrajGenInfo__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__TrajGenInfo__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TrajGenInfo {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TrajGenInfo where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/TrajGenInfo";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__TrajGenInfo() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__TrajectoryPoint() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__TrajectoryPoint__init(msg: *mut TrajectoryPoint) -> bool;
    fn as2_msgs__msg__TrajectoryPoint__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryPoint>, size: usize) -> bool;
    fn as2_msgs__msg__TrajectoryPoint__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryPoint>);
    fn as2_msgs__msg__TrajectoryPoint__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TrajectoryPoint>, out_seq: *mut rosidl_runtime_rs::Sequence<TrajectoryPoint>) -> bool;
}

// Corresponds to as2_msgs__msg__TrajectoryPoint
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Definition of a point of a trajectory

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryPoint {
    /// Position of the vehicle in the frame_id frame
    pub position: geometry_msgs::msg::rmw::Vector3,

    /// Twist of the vehicle in the frame_id frame
    pub twist: geometry_msgs::msg::rmw::Vector3,

    /// Acceleration of the vehicle in the frame_id frame
    pub acceleration: geometry_msgs::msg::rmw::Vector3,

    /// Yaw angle of the vehicle (rad) in the frame_id frame
    pub yaw_angle: f32,

}



impl Default for TrajectoryPoint {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__TrajectoryPoint__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__TrajectoryPoint__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TrajectoryPoint {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__TrajectoryPoint__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__TrajectoryPoint__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__TrajectoryPoint__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TrajectoryPoint {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TrajectoryPoint where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/TrajectoryPoint";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__TrajectoryPoint() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__TrajectorySetpoints() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__TrajectorySetpoints__init(msg: *mut TrajectorySetpoints) -> bool;
    fn as2_msgs__msg__TrajectorySetpoints__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TrajectorySetpoints>, size: usize) -> bool;
    fn as2_msgs__msg__TrajectorySetpoints__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TrajectorySetpoints>);
    fn as2_msgs__msg__TrajectorySetpoints__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TrajectorySetpoints>, out_seq: *mut rosidl_runtime_rs::Sequence<TrajectorySetpoints>) -> bool;
}

// Corresponds to as2_msgs__msg__TrajectorySetpoints
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Definition of a point of a trajectory

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectorySetpoints {
    /// Message header with the frame_id of the point
    pub header: std_msgs::msg::rmw::Header,

    /// Array of setpoints of the vehicle in the frame_id frame
    pub setpoints: rosidl_runtime_rs::Sequence<super::super::msg::rmw::TrajectoryPoint>,

}



impl Default for TrajectorySetpoints {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__TrajectorySetpoints__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__TrajectorySetpoints__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TrajectorySetpoints {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__TrajectorySetpoints__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__TrajectorySetpoints__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__TrajectorySetpoints__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TrajectorySetpoints {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TrajectorySetpoints where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/TrajectorySetpoints";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__TrajectorySetpoints() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__UInt16MultiArrayStamped() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__UInt16MultiArrayStamped__init(msg: *mut UInt16MultiArrayStamped) -> bool;
    fn as2_msgs__msg__UInt16MultiArrayStamped__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UInt16MultiArrayStamped>, size: usize) -> bool;
    fn as2_msgs__msg__UInt16MultiArrayStamped__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UInt16MultiArrayStamped>);
    fn as2_msgs__msg__UInt16MultiArrayStamped__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UInt16MultiArrayStamped>, out_seq: *mut rosidl_runtime_rs::Sequence<UInt16MultiArrayStamped>) -> bool;
}

// Corresponds to as2_msgs__msg__UInt16MultiArrayStamped
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Please look at the std_msgs/MultiArrayLayout message definition for
/// documentation on all multiarrays.
/// This message is a multiarray of uint16 values with a timestamp, based on
/// the std_msgs/MultiArrayLayout message.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UInt16MultiArrayStamped {
    /// Message timestamp
    pub stamp: builtin_interfaces::msg::rmw::Time,

    /// Specification of data layout
    pub layout: std_msgs::msg::rmw::MultiArrayLayout,

    /// Array of data
    pub data: rosidl_runtime_rs::Sequence<u16>,

}



impl Default for UInt16MultiArrayStamped {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__UInt16MultiArrayStamped__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__UInt16MultiArrayStamped__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UInt16MultiArrayStamped {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__UInt16MultiArrayStamped__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__UInt16MultiArrayStamped__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__UInt16MultiArrayStamped__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UInt16MultiArrayStamped {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UInt16MultiArrayStamped where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/UInt16MultiArrayStamped";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__UInt16MultiArrayStamped() }
  }
}


#[link(name = "as2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__YawMode() -> *const std::ffi::c_void;
}

#[link(name = "as2_msgs__rosidl_generator_c")]
extern "C" {
    fn as2_msgs__msg__YawMode__init(msg: *mut YawMode) -> bool;
    fn as2_msgs__msg__YawMode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<YawMode>, size: usize) -> bool;
    fn as2_msgs__msg__YawMode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<YawMode>);
    fn as2_msgs__msg__YawMode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<YawMode>, out_seq: *mut rosidl_runtime_rs::Sequence<YawMode>) -> bool;
}

// Corresponds to as2_msgs__msg__YawMode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Yaw goal

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct YawMode {
    /// Yaw mode
    pub mode: u8,

    /// Fixed yaw (rad)
    pub angle: f32,

}

impl YawMode {
    /// Keep the current yaw angle
    pub const KEEP_YAW: u8 = 0;

    /// Yaw angle is aligned with the path
    pub const PATH_FACING: u8 = 1;

    /// Yaw angle is fixed to a given angle
    pub const FIXED_YAW: u8 = 2;

    /// Yaw angle is set by a topic
    pub const YAW_FROM_TOPIC: u8 = 3;

    /// Yaw angle is set by pose orientation
    pub const YAW_FROM_ORIENTATION: u8 = 4;

    /// Yaw angle is set to face the used frame
    pub const YAW_TO_FRAME: u8 = 5;

    /// Yaw angle is set to face the next reference
    pub const FACE_REFERENCE: u8 = 6;

}


impl Default for YawMode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !as2_msgs__msg__YawMode__init(&mut msg as *mut _) {
        panic!("Call to as2_msgs__msg__YawMode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for YawMode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__YawMode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__YawMode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { as2_msgs__msg__YawMode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for YawMode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for YawMode where Self: Sized {
  const TYPE_NAME: &'static str = "as2_msgs/msg/YawMode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__as2_msgs__msg__YawMode() }
  }
}


