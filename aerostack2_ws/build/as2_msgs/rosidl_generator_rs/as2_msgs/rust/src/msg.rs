#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to as2_msgs__msg__Acro
/// Message for RPY rates and thrust (ACRO)

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Acro {
    /// Message header
    pub header: std_msgs::msg::Header,

    /// Roll-, pitch-, yaw-rate around body axes
    pub angular_rates: geometry_msgs::msg::Vector3,

    /// Thrust expressed in the body frame.
    /// For a fixed-wing, usually the x-component is used.
    /// For a multi-rotor, usually the z-component is used.
    /// Set all un-used components to 0.
    pub thrust: geometry_msgs::msg::Vector3,

}



impl Default for Acro {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Acro::default())
  }
}

impl rosidl_runtime_rs::Message for Acro {
  type RmwMsg = super::msg::rmw::Acro;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        angular_rates: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.angular_rates)).into_owned(),
        thrust: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.thrust)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        angular_rates: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.angular_rates)).into_owned(),
        thrust: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.thrust)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      angular_rates: geometry_msgs::msg::Vector3::from_rmw_message(msg.angular_rates),
      thrust: geometry_msgs::msg::Vector3::from_rmw_message(msg.thrust),
    }
  }
}


// Corresponds to as2_msgs__msg__AlertEvent
/// Message that encodes different alert Events that can be handled by AS2 framework

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AlertEvent {

    // This member is not documented.
    #[allow(missing_docs)]
    pub alert: i8,

    /// Further description of the alert, for debugging purposes mainly
    pub description: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AlertEvent::default())
  }
}

impl rosidl_runtime_rs::Message for AlertEvent {
  type RmwMsg = super::msg::rmw::AlertEvent;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        alert: msg.alert,
        description: msg.description.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      alert: msg.alert,
        description: msg.description.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      alert: msg.alert,
      description: msg.description.to_string(),
    }
  }
}


// Corresponds to as2_msgs__msg__BehaviorStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BehaviorStatus::default())
  }
}

impl rosidl_runtime_rs::Message for BehaviorStatus {
  type RmwMsg = super::msg::rmw::BehaviorStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
    }
  }
}


// Corresponds to as2_msgs__msg__ControlMode
/// Message that encodes the possible control modes supported in Aerostack2.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ControlMode {
    /// Message header
    pub header: std_msgs::msg::Header,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ControlMode::default())
  }
}

impl rosidl_runtime_rs::Message for ControlMode {
  type RmwMsg = super::msg::rmw::ControlMode;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        yaw_mode: msg.yaw_mode,
        control_mode: msg.control_mode,
        reference_frame: msg.reference_frame,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      yaw_mode: msg.yaw_mode,
      control_mode: msg.control_mode,
      reference_frame: msg.reference_frame,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      yaw_mode: msg.yaw_mode,
      control_mode: msg.control_mode,
      reference_frame: msg.reference_frame,
    }
  }
}


// Corresponds to as2_msgs__msg__ControllerInfo
/// Message that shows the controller state and the current input_control_mode

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ControllerInfo {
    /// Message header
    pub header: std_msgs::msg::Header,

    /// Input control mode
    pub input_control_mode: super::msg::ControlMode,

    /// Output control mode
    pub output_control_mode: super::msg::ControlMode,

}



impl Default for ControllerInfo {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ControllerInfo::default())
  }
}

impl rosidl_runtime_rs::Message for ControllerInfo {
  type RmwMsg = super::msg::rmw::ControllerInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        input_control_mode: super::msg::ControlMode::into_rmw_message(std::borrow::Cow::Owned(msg.input_control_mode)).into_owned(),
        output_control_mode: super::msg::ControlMode::into_rmw_message(std::borrow::Cow::Owned(msg.output_control_mode)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        input_control_mode: super::msg::ControlMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.input_control_mode)).into_owned(),
        output_control_mode: super::msg::ControlMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.output_control_mode)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      input_control_mode: super::msg::ControlMode::from_rmw_message(msg.input_control_mode),
      output_control_mode: super::msg::ControlMode::from_rmw_message(msg.output_control_mode),
    }
  }
}


// Corresponds to as2_msgs__msg__FollowTargetInfo
/// Message that encodes the possible follow target info supported in Aerostack2.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowTargetInfo {
    /// Message header
    pub header: std_msgs::msg::Header,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::FollowTargetInfo::default())
  }
}

impl rosidl_runtime_rs::Message for FollowTargetInfo {
  type RmwMsg = super::msg::rmw::FollowTargetInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        follow_status: msg.follow_status,
        follow_mode: msg.follow_mode,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      follow_status: msg.follow_status,
      follow_mode: msg.follow_mode,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      follow_status: msg.follow_status,
      follow_mode: msg.follow_mode,
    }
  }
}


// Corresponds to as2_msgs__msg__Geozone
/// GeoStructure defined by an id, alert that generates in case of event and a polygon 
/// that defines the area.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Geozone {
    /// geofence id
    pub id: i8,

    /// alert generated
    pub alert: i8,

    /// geofence or geocage
    pub type_: std::string::String,

    /// cartesian or gps
    pub data_type: std::string::String,

    /// fence polygon
    pub polygon: geometry_msgs::msg::Polygon,

    /// height limit up
    pub z_up: f32,

    /// height limit bottom
    pub z_down: f32,

}



impl Default for Geozone {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Geozone::default())
  }
}

impl rosidl_runtime_rs::Message for Geozone {
  type RmwMsg = super::msg::rmw::Geozone;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        alert: msg.alert,
        type_: msg.type_.as_str().into(),
        data_type: msg.data_type.as_str().into(),
        polygon: geometry_msgs::msg::Polygon::into_rmw_message(std::borrow::Cow::Owned(msg.polygon)).into_owned(),
        z_up: msg.z_up,
        z_down: msg.z_down,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      alert: msg.alert,
        type_: msg.type_.as_str().into(),
        data_type: msg.data_type.as_str().into(),
        polygon: geometry_msgs::msg::Polygon::into_rmw_message(std::borrow::Cow::Borrowed(&msg.polygon)).into_owned(),
      z_up: msg.z_up,
      z_down: msg.z_down,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      alert: msg.alert,
      type_: msg.type_.to_string(),
      data_type: msg.data_type.to_string(),
      polygon: geometry_msgs::msg::Polygon::from_rmw_message(msg.polygon),
      z_up: msg.z_up,
      z_down: msg.z_down,
    }
  }
}


// Corresponds to as2_msgs__msg__GimbalControl
/// Gimbal Control message definition

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GimbalControl {

    // This member is not documented.
    #[allow(missing_docs)]
    pub control_mode: u8,

    /// x: roll y: pitch z: yaw
    pub target: geometry_msgs::msg::Vector3Stamped,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::GimbalControl::default())
  }
}

impl rosidl_runtime_rs::Message for GimbalControl {
  type RmwMsg = super::msg::rmw::GimbalControl;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        control_mode: msg.control_mode,
        target: geometry_msgs::msg::Vector3Stamped::into_rmw_message(std::borrow::Cow::Owned(msg.target)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      control_mode: msg.control_mode,
        target: geometry_msgs::msg::Vector3Stamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.target)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      control_mode: msg.control_mode,
      target: geometry_msgs::msg::Vector3Stamped::from_rmw_message(msg.target),
    }
  }
}


// Corresponds to as2_msgs__msg__MissionEvent
/// Message for trigger mission events

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MissionEvent {
    /// Message header
    pub header: std_msgs::msg::Header,

    /// (Optional) data to send with the trigger
    pub data: std::string::String,

}



impl Default for MissionEvent {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MissionEvent::default())
  }
}

impl rosidl_runtime_rs::Message for MissionEvent {
  type RmwMsg = super::msg::rmw::MissionEvent;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        data: msg.data.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        data: msg.data.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      data: msg.data.to_string(),
    }
  }
}


// Corresponds to as2_msgs__msg__MissionUpdate
/// Message that sends a mission to the interpreter

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MissionUpdate {
    /// ID of the drone that will execute the mission
    pub drone_id: std::string::String,

    /// ID of the mission to be executed
    pub mission_id: i32,

    /// ID of the item to be executed
    pub item_id: i32,

    /// Action to be performed in the interpreter
    pub action: u8,

    /// JSON formatted mission to be executed
    pub mission: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MissionUpdate::default())
  }
}

impl rosidl_runtime_rs::Message for MissionUpdate {
  type RmwMsg = super::msg::rmw::MissionUpdate;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        drone_id: msg.drone_id.as_str().into(),
        mission_id: msg.mission_id,
        item_id: msg.item_id,
        action: msg.action,
        mission: msg.mission.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        drone_id: msg.drone_id.as_str().into(),
      mission_id: msg.mission_id,
      item_id: msg.item_id,
      action: msg.action,
        mission: msg.mission.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      drone_id: msg.drone_id.to_string(),
      mission_id: msg.mission_id,
      item_id: msg.item_id,
      action: msg.action,
      mission: msg.mission.to_string(),
    }
  }
}


// Corresponds to as2_msgs__msg__NodeStatus
/// Message that shows the node status

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::NodeStatus::default())
  }
}

impl rosidl_runtime_rs::Message for NodeStatus {
  type RmwMsg = super::msg::rmw::NodeStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
    }
  }
}


// Corresponds to as2_msgs__msg__PlatformInfo
/// Message that shows the platform status and the current control mode

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlatformInfo {
    /// Message header
    pub header: std_msgs::msg::Header,

    /// Whether the platform is connected or not
    pub connected: bool,

    /// Whether the platform is armed or not
    pub armed: bool,

    /// Whether the offboard mode is set or not
    pub offboard: bool,

    /// Platform status
    pub status: super::msg::PlatformStatus,

    /// Platform control mode
    pub current_control_mode: super::msg::ControlMode,

}



impl Default for PlatformInfo {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PlatformInfo::default())
  }
}

impl rosidl_runtime_rs::Message for PlatformInfo {
  type RmwMsg = super::msg::rmw::PlatformInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        connected: msg.connected,
        armed: msg.armed,
        offboard: msg.offboard,
        status: super::msg::PlatformStatus::into_rmw_message(std::borrow::Cow::Owned(msg.status)).into_owned(),
        current_control_mode: super::msg::ControlMode::into_rmw_message(std::borrow::Cow::Owned(msg.current_control_mode)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      connected: msg.connected,
      armed: msg.armed,
      offboard: msg.offboard,
        status: super::msg::PlatformStatus::into_rmw_message(std::borrow::Cow::Borrowed(&msg.status)).into_owned(),
        current_control_mode: super::msg::ControlMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.current_control_mode)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      connected: msg.connected,
      armed: msg.armed,
      offboard: msg.offboard,
      status: super::msg::PlatformStatus::from_rmw_message(msg.status),
      current_control_mode: super::msg::ControlMode::from_rmw_message(msg.current_control_mode),
    }
  }
}


// Corresponds to as2_msgs__msg__PlatformStateMachineEvent
/// Platform events that actives aerial platform state machine 

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PlatformStateMachineEvent::default())
  }
}

impl rosidl_runtime_rs::Message for PlatformStateMachineEvent {
  type RmwMsg = super::msg::rmw::PlatformStateMachineEvent;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        event: msg.event,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      event: msg.event,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      event: msg.event,
    }
  }
}


// Corresponds to as2_msgs__msg__PlatformStatus
/// Platform states that an aerial platform can have

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PlatformStatus::default())
  }
}

impl rosidl_runtime_rs::Message for PlatformStatus {
  type RmwMsg = super::msg::rmw::PlatformStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      state: msg.state,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: msg.state,
    }
  }
}


// Corresponds to as2_msgs__msg__PolygonList
/// List of polygons to visualize multiple geozones in RVIZ2

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolygonList {

    // This member is not documented.
    #[allow(missing_docs)]
    pub polygons: Vec<geometry_msgs::msg::PolygonStamped>,

}



impl Default for PolygonList {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PolygonList::default())
  }
}

impl rosidl_runtime_rs::Message for PolygonList {
  type RmwMsg = super::msg::rmw::PolygonList;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        polygons: msg.polygons
          .into_iter()
          .map(|elem| geometry_msgs::msg::PolygonStamped::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        polygons: msg.polygons
          .iter()
          .map(|elem| geometry_msgs::msg::PolygonStamped::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      polygons: msg.polygons
          .into_iter()
          .map(geometry_msgs::msg::PolygonStamped::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to as2_msgs__msg__PoseStampedWithID
/// A Pose stamped with an string id

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PoseStampedWithID {
    /// Identification string
    pub id: std::string::String,

    /// Pose
    pub pose: geometry_msgs::msg::PoseStamped,

}



impl Default for PoseStampedWithID {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PoseStampedWithID::default())
  }
}

impl rosidl_runtime_rs::Message for PoseStampedWithID {
  type RmwMsg = super::msg::rmw::PoseStampedWithID;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id.as_str().into(),
        pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id.as_str().into(),
        pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id.to_string(),
      pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.pose),
    }
  }
}


// Corresponds to as2_msgs__msg__PoseStampedWithIDArray
/// Pose Stamped with an string id array

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PoseStampedWithIDArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub poses: Vec<super::msg::PoseStampedWithID>,

}



impl Default for PoseStampedWithIDArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PoseStampedWithIDArray::default())
  }
}

impl rosidl_runtime_rs::Message for PoseStampedWithIDArray {
  type RmwMsg = super::msg::rmw::PoseStampedWithIDArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        poses: msg.poses
          .into_iter()
          .map(|elem| super::msg::PoseStampedWithID::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        poses: msg.poses
          .iter()
          .map(|elem| super::msg::PoseStampedWithID::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      poses: msg.poses
          .into_iter()
          .map(super::msg::PoseStampedWithID::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to as2_msgs__msg__PoseWithID
/// A Pose with an string id

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PoseWithID {
    /// Identification string
    pub id: std::string::String,

    /// Pose
    pub pose: geometry_msgs::msg::Pose,

}



impl Default for PoseWithID {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PoseWithID::default())
  }
}

impl rosidl_runtime_rs::Message for PoseWithID {
  type RmwMsg = super::msg::rmw::PoseWithID;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id.as_str().into(),
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id.as_str().into(),
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id.to_string(),
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
    }
  }
}


// Corresponds to as2_msgs__msg__PoseWithIDArray
/// Pose with an string id array

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PoseWithIDArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub poses: Vec<super::msg::PoseWithID>,

}



impl Default for PoseWithIDArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PoseWithIDArray::default())
  }
}

impl rosidl_runtime_rs::Message for PoseWithIDArray {
  type RmwMsg = super::msg::rmw::PoseWithIDArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        poses: msg.poses
          .into_iter()
          .map(|elem| super::msg::PoseWithID::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        poses: msg.poses
          .iter()
          .map(|elem| super::msg::PoseWithID::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      poses: msg.poses
          .into_iter()
          .map(super::msg::PoseWithID::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to as2_msgs__msg__Speed
/// Speed message

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Speed {
    /// Message header
    pub header: std_msgs::msg::Header,

    /// speed (m/s)
    pub speed: f32,

}



impl Default for Speed {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Speed::default())
  }
}

impl rosidl_runtime_rs::Message for Speed {
  type RmwMsg = super::msg::rmw::Speed;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        speed: msg.speed,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      speed: msg.speed,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      speed: msg.speed,
    }
  }
}


// Corresponds to as2_msgs__msg__Thrust
/// Message for encoding the desired thrust value

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Thrust {
    /// Message header
    pub header: std_msgs::msg::Header,

    /// Thrust (N)
    pub thrust: f32,

    /// Thrust normalized [0,1]
    pub thrust_normalized: f32,

}



impl Default for Thrust {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Thrust::default())
  }
}

impl rosidl_runtime_rs::Message for Thrust {
  type RmwMsg = super::msg::rmw::Thrust;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        thrust: msg.thrust,
        thrust_normalized: msg.thrust_normalized,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      thrust: msg.thrust,
      thrust_normalized: msg.thrust_normalized,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      thrust: msg.thrust,
      thrust_normalized: msg.thrust_normalized,
    }
  }
}


// Corresponds to as2_msgs__msg__TrajGenInfo
/// Message that shows the trajectory generator state

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajGenInfo {
    /// Message header
    pub header: std_msgs::msg::Header,

    /// Node status
    pub node_status: super::msg::NodeStatus,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TrajGenInfo::default())
  }
}

impl rosidl_runtime_rs::Message for TrajGenInfo {
  type RmwMsg = super::msg::rmw::TrajGenInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        node_status: super::msg::NodeStatus::into_rmw_message(std::borrow::Cow::Owned(msg.node_status)).into_owned(),
        active_status: msg.active_status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        node_status: super::msg::NodeStatus::into_rmw_message(std::borrow::Cow::Borrowed(&msg.node_status)).into_owned(),
      active_status: msg.active_status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      node_status: super::msg::NodeStatus::from_rmw_message(msg.node_status),
      active_status: msg.active_status,
    }
  }
}


// Corresponds to as2_msgs__msg__TrajectoryPoint
/// Definition of a point of a trajectory

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryPoint {
    /// Position of the vehicle in the frame_id frame
    pub position: geometry_msgs::msg::Vector3,

    /// Twist of the vehicle in the frame_id frame
    pub twist: geometry_msgs::msg::Vector3,

    /// Acceleration of the vehicle in the frame_id frame
    pub acceleration: geometry_msgs::msg::Vector3,

    /// Yaw angle of the vehicle (rad) in the frame_id frame
    pub yaw_angle: f32,

}



impl Default for TrajectoryPoint {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TrajectoryPoint::default())
  }
}

impl rosidl_runtime_rs::Message for TrajectoryPoint {
  type RmwMsg = super::msg::rmw::TrajectoryPoint;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        position: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.position)).into_owned(),
        twist: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.twist)).into_owned(),
        acceleration: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.acceleration)).into_owned(),
        yaw_angle: msg.yaw_angle,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        position: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.position)).into_owned(),
        twist: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.twist)).into_owned(),
        acceleration: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.acceleration)).into_owned(),
      yaw_angle: msg.yaw_angle,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      position: geometry_msgs::msg::Vector3::from_rmw_message(msg.position),
      twist: geometry_msgs::msg::Vector3::from_rmw_message(msg.twist),
      acceleration: geometry_msgs::msg::Vector3::from_rmw_message(msg.acceleration),
      yaw_angle: msg.yaw_angle,
    }
  }
}


// Corresponds to as2_msgs__msg__TrajectorySetpoints
/// Definition of a point of a trajectory

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectorySetpoints {
    /// Message header with the frame_id of the point
    pub header: std_msgs::msg::Header,

    /// Array of setpoints of the vehicle in the frame_id frame
    pub setpoints: Vec<super::msg::TrajectoryPoint>,

}



impl Default for TrajectorySetpoints {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TrajectorySetpoints::default())
  }
}

impl rosidl_runtime_rs::Message for TrajectorySetpoints {
  type RmwMsg = super::msg::rmw::TrajectorySetpoints;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        setpoints: msg.setpoints
          .into_iter()
          .map(|elem| super::msg::TrajectoryPoint::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        setpoints: msg.setpoints
          .iter()
          .map(|elem| super::msg::TrajectoryPoint::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      setpoints: msg.setpoints
          .into_iter()
          .map(super::msg::TrajectoryPoint::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to as2_msgs__msg__UInt16MultiArrayStamped
/// Please look at the std_msgs/MultiArrayLayout message definition for
/// documentation on all multiarrays.
/// This message is a multiarray of uint16 values with a timestamp, based on
/// the std_msgs/MultiArrayLayout message.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UInt16MultiArrayStamped {
    /// Message timestamp
    pub stamp: builtin_interfaces::msg::Time,

    /// Specification of data layout
    pub layout: std_msgs::msg::MultiArrayLayout,

    /// Array of data
    pub data: Vec<u16>,

}



impl Default for UInt16MultiArrayStamped {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::UInt16MultiArrayStamped::default())
  }
}

impl rosidl_runtime_rs::Message for UInt16MultiArrayStamped {
  type RmwMsg = super::msg::rmw::UInt16MultiArrayStamped;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
        layout: std_msgs::msg::MultiArrayLayout::into_rmw_message(std::borrow::Cow::Owned(msg.layout)).into_owned(),
        data: msg.data.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
        layout: std_msgs::msg::MultiArrayLayout::into_rmw_message(std::borrow::Cow::Borrowed(&msg.layout)).into_owned(),
        data: msg.data.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
      layout: std_msgs::msg::MultiArrayLayout::from_rmw_message(msg.layout),
      data: msg.data
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to as2_msgs__msg__YawMode
/// Yaw goal

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::YawMode::default())
  }
}

impl rosidl_runtime_rs::Message for YawMode {
  type RmwMsg = super::msg::rmw::YawMode;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        angle: msg.angle,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      angle: msg.angle,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      angle: msg.angle,
    }
  }
}


