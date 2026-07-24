#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to as2_msgs__srv__AddStaticTransform_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddStaticTransform_Request {
    /// Parent frame frame id
    pub frame_id: std::string::String,

    /// Child frame id
    pub child_frame_id: std::string::String,

    /// Transform
    pub transform: geometry_msgs::msg::Transform,

}



impl Default for AddStaticTransform_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AddStaticTransform_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AddStaticTransform_Request {
  type RmwMsg = super::srv::rmw::AddStaticTransform_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frame_id: msg.frame_id.as_str().into(),
        child_frame_id: msg.child_frame_id.as_str().into(),
        transform: geometry_msgs::msg::Transform::into_rmw_message(std::borrow::Cow::Owned(msg.transform)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frame_id: msg.frame_id.as_str().into(),
        child_frame_id: msg.child_frame_id.as_str().into(),
        transform: geometry_msgs::msg::Transform::into_rmw_message(std::borrow::Cow::Borrowed(&msg.transform)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      frame_id: msg.frame_id.to_string(),
      child_frame_id: msg.child_frame_id.to_string(),
      transform: geometry_msgs::msg::Transform::from_rmw_message(msg.transform),
    }
  }
}


// Corresponds to as2_msgs__srv__AddStaticTransform_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddStaticTransform_Response {
    /// whether the transform has been set or not
    pub success: bool,

}



impl Default for AddStaticTransform_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AddStaticTransform_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AddStaticTransform_Response {
  type RmwMsg = super::srv::rmw::AddStaticTransform_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to as2_msgs__srv__AddStaticTransformGps_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddStaticTransformGps_Request {
    /// Parent frame frame id
    pub frame_id: std::string::String,

    /// Child frame id
    pub child_frame_id: std::string::String,

    /// Transform
    pub gps_position: sensor_msgs::msg::NavSatFix,

    /// yaw
    pub azimuth: f32,

    /// pitch
    pub elevation: f32,

    /// roll
    pub bank: f32,

}



impl Default for AddStaticTransformGps_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AddStaticTransformGps_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AddStaticTransformGps_Request {
  type RmwMsg = super::srv::rmw::AddStaticTransformGps_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frame_id: msg.frame_id.as_str().into(),
        child_frame_id: msg.child_frame_id.as_str().into(),
        gps_position: sensor_msgs::msg::NavSatFix::into_rmw_message(std::borrow::Cow::Owned(msg.gps_position)).into_owned(),
        azimuth: msg.azimuth,
        elevation: msg.elevation,
        bank: msg.bank,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frame_id: msg.frame_id.as_str().into(),
        child_frame_id: msg.child_frame_id.as_str().into(),
        gps_position: sensor_msgs::msg::NavSatFix::into_rmw_message(std::borrow::Cow::Borrowed(&msg.gps_position)).into_owned(),
      azimuth: msg.azimuth,
      elevation: msg.elevation,
      bank: msg.bank,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      frame_id: msg.frame_id.to_string(),
      child_frame_id: msg.child_frame_id.to_string(),
      gps_position: sensor_msgs::msg::NavSatFix::from_rmw_message(msg.gps_position),
      azimuth: msg.azimuth,
      elevation: msg.elevation,
      bank: msg.bank,
    }
  }
}


// Corresponds to as2_msgs__srv__AddStaticTransformGps_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddStaticTransformGps_Response {
    /// whether the transform has been set or not
    pub success: bool,

}



impl Default for AddStaticTransformGps_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AddStaticTransformGps_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AddStaticTransformGps_Response {
  type RmwMsg = super::srv::rmw::AddStaticTransformGps_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to as2_msgs__srv__DynamicFollower_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DynamicFollower_Request {
    /// Flag to enable follower
    pub enable: bool,

    /// Speed limit (m/s)
    pub speed_limit: geometry_msgs::msg::Twist,

}



impl Default for DynamicFollower_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DynamicFollower_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DynamicFollower_Request {
  type RmwMsg = super::srv::rmw::DynamicFollower_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        enable: msg.enable,
        speed_limit: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(msg.speed_limit)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      enable: msg.enable,
        speed_limit: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(&msg.speed_limit)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      enable: msg.enable,
      speed_limit: geometry_msgs::msg::Twist::from_rmw_message(msg.speed_limit),
    }
  }
}


// Corresponds to as2_msgs__srv__DynamicFollower_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DynamicFollower_Response {
    /// whether it could be started or not
    pub success: bool,

}



impl Default for DynamicFollower_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DynamicFollower_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DynamicFollower_Response {
  type RmwMsg = super::srv::rmw::DynamicFollower_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to as2_msgs__srv__DynamicLand_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DynamicLand_Request {
    /// Flag to enable land
    pub enable: bool,

    /// speed limit (m/s)
    pub speed_limit: geometry_msgs::msg::Twist,

}



impl Default for DynamicLand_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DynamicLand_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DynamicLand_Request {
  type RmwMsg = super::srv::rmw::DynamicLand_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        enable: msg.enable,
        speed_limit: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(msg.speed_limit)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      enable: msg.enable,
        speed_limit: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(&msg.speed_limit)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      enable: msg.enable,
      speed_limit: geometry_msgs::msg::Twist::from_rmw_message(msg.speed_limit),
    }
  }
}


// Corresponds to as2_msgs__srv__DynamicLand_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DynamicLand_Response {
    /// whether it could be started or not
    pub success: bool,

}



impl Default for DynamicLand_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DynamicLand_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DynamicLand_Response {
  type RmwMsg = super::srv::rmw::DynamicLand_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to as2_msgs__srv__GeopathToPath_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GeopathToPath_Request {
    /// Path in lat/lon and altitude
    pub geo_path: geographic_msgs::msg::GeoPath,

}



impl Default for GeopathToPath_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GeopathToPath_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GeopathToPath_Request {
  type RmwMsg = super::srv::rmw::GeopathToPath_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        geo_path: geographic_msgs::msg::GeoPath::into_rmw_message(std::borrow::Cow::Owned(msg.geo_path)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        geo_path: geographic_msgs::msg::GeoPath::into_rmw_message(std::borrow::Cow::Borrowed(&msg.geo_path)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      geo_path: geographic_msgs::msg::GeoPath::from_rmw_message(msg.geo_path),
    }
  }
}


// Corresponds to as2_msgs__srv__GeopathToPath_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GeopathToPath_Response {
    /// whether the origin has been set or not
    pub success: bool,

    /// Path in meters
    pub path: nav_msgs::msg::Path,

}



impl Default for GeopathToPath_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GeopathToPath_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GeopathToPath_Response {
  type RmwMsg = super::srv::rmw::GeopathToPath_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        path: nav_msgs::msg::Path::into_rmw_message(std::borrow::Cow::Owned(msg.path)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        path: nav_msgs::msg::Path::into_rmw_message(std::borrow::Cow::Borrowed(&msg.path)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      path: nav_msgs::msg::Path::from_rmw_message(msg.path),
    }
  }
}


// Corresponds to as2_msgs__srv__GetGeozone_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetGeozone_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetGeozone_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetGeozone_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetGeozone_Request {
  type RmwMsg = super::srv::rmw::GetGeozone_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to as2_msgs__srv__GetGeozone_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetGeozone_Response {
    /// whether the geofence has been set or not
    pub success: bool,

    /// geofences stored in memory
    pub geozone_list: Vec<super::msg::Geozone>,

}



impl Default for GetGeozone_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetGeozone_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetGeozone_Response {
  type RmwMsg = super::srv::rmw::GetGeozone_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        geozone_list: msg.geozone_list
          .into_iter()
          .map(|elem| super::msg::Geozone::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        geozone_list: msg.geozone_list
          .iter()
          .map(|elem| super::msg::Geozone::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      geozone_list: msg.geozone_list
          .into_iter()
          .map(super::msg::Geozone::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to as2_msgs__srv__GetOrigin_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOrigin_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetOrigin_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetOrigin_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetOrigin_Request {
  type RmwMsg = super::srv::rmw::GetOrigin_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to as2_msgs__srv__GetOrigin_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOrigin_Response {
    /// whether the origin has been set or not
    pub success: bool,

    /// origin
    pub origin: geographic_msgs::msg::GeoPoint,

}



impl Default for GetOrigin_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetOrigin_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetOrigin_Response {
  type RmwMsg = super::srv::rmw::GetOrigin_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        origin: geographic_msgs::msg::GeoPoint::into_rmw_message(std::borrow::Cow::Owned(msg.origin)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        origin: geographic_msgs::msg::GeoPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.origin)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      origin: geographic_msgs::msg::GeoPoint::from_rmw_message(msg.origin),
    }
  }
}


// Corresponds to as2_msgs__srv__ListControlModes_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListControlModes_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListControlModes_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListControlModes_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ListControlModes_Request {
  type RmwMsg = super::srv::rmw::ListControlModes_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to as2_msgs__srv__ListControlModes_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListControlModes_Response {
    /// Control modes source
    pub source: std::string::String,

    /// Control modes list
    pub control_modes: Vec<u8>,

}



impl Default for ListControlModes_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListControlModes_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ListControlModes_Response {
  type RmwMsg = super::srv::rmw::ListControlModes_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        source: msg.source.as_str().into(),
        control_modes: msg.control_modes.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        source: msg.source.as_str().into(),
        control_modes: msg.control_modes.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      source: msg.source.to_string(),
      control_modes: msg.control_modes
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to as2_msgs__srv__ModifySwarm_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModifySwarm_Request {
    /// Detach the drone reference from the swarm
    pub detach_drone: bool,

    /// Add new drone reference to the swarm
    pub new_drone: bool,

    /// New reference to follow
    pub new_virtual_centroid_ref: bool,

    /// Offset of the virtual centroid to the following frame
    pub virtual_centroid: geometry_msgs::msg::PoseStamped,

    /// Topics to modify the flocking
    pub swarm_formation: Vec<super::msg::PoseWithID>,

}



impl Default for ModifySwarm_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ModifySwarm_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ModifySwarm_Request {
  type RmwMsg = super::srv::rmw::ModifySwarm_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        detach_drone: msg.detach_drone,
        new_drone: msg.new_drone,
        new_virtual_centroid_ref: msg.new_virtual_centroid_ref,
        virtual_centroid: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.virtual_centroid)).into_owned(),
        swarm_formation: msg.swarm_formation
          .into_iter()
          .map(|elem| super::msg::PoseWithID::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      detach_drone: msg.detach_drone,
      new_drone: msg.new_drone,
      new_virtual_centroid_ref: msg.new_virtual_centroid_ref,
        virtual_centroid: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.virtual_centroid)).into_owned(),
        swarm_formation: msg.swarm_formation
          .iter()
          .map(|elem| super::msg::PoseWithID::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      detach_drone: msg.detach_drone,
      new_drone: msg.new_drone,
      new_virtual_centroid_ref: msg.new_virtual_centroid_ref,
      virtual_centroid: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.virtual_centroid),
      swarm_formation: msg.swarm_formation
          .into_iter()
          .map(super::msg::PoseWithID::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to as2_msgs__srv__ModifySwarm_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModifySwarm_Response {
    /// whether the SwarmBehavior has been set or not
    pub success: bool,

}



impl Default for ModifySwarm_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ModifySwarm_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ModifySwarm_Response {
  type RmwMsg = super::srv::rmw::ModifySwarm_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to as2_msgs__srv__PackagePickUp_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PackagePickUp_Request {
    /// Flag to enable pickup
    pub enable: bool,

    /// speed limit (m/s)
    pub speed_limit: geometry_msgs::msg::Twist,

}



impl Default for PackagePickUp_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PackagePickUp_Request::default())
  }
}

impl rosidl_runtime_rs::Message for PackagePickUp_Request {
  type RmwMsg = super::srv::rmw::PackagePickUp_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        enable: msg.enable,
        speed_limit: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(msg.speed_limit)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      enable: msg.enable,
        speed_limit: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(&msg.speed_limit)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      enable: msg.enable,
      speed_limit: geometry_msgs::msg::Twist::from_rmw_message(msg.speed_limit),
    }
  }
}


// Corresponds to as2_msgs__srv__PackagePickUp_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PackagePickUp_Response {
    /// whether it could be started or not
    pub success: bool,

}



impl Default for PackagePickUp_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PackagePickUp_Response::default())
  }
}

impl rosidl_runtime_rs::Message for PackagePickUp_Response {
  type RmwMsg = super::srv::rmw::PackagePickUp_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to as2_msgs__srv__PackageUnPick_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PackageUnPick_Request {
    /// Flag to enable unpick
    pub enable: bool,

    /// speed limit (m/s)
    pub speed_limit: geometry_msgs::msg::Twist,

}



impl Default for PackageUnPick_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PackageUnPick_Request::default())
  }
}

impl rosidl_runtime_rs::Message for PackageUnPick_Request {
  type RmwMsg = super::srv::rmw::PackageUnPick_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        enable: msg.enable,
        speed_limit: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(msg.speed_limit)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      enable: msg.enable,
        speed_limit: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(&msg.speed_limit)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      enable: msg.enable,
      speed_limit: geometry_msgs::msg::Twist::from_rmw_message(msg.speed_limit),
    }
  }
}


// Corresponds to as2_msgs__srv__PackageUnPick_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PackageUnPick_Response {
    /// whether it could be started or not
    pub success: bool,

}



impl Default for PackageUnPick_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PackageUnPick_Response::default())
  }
}

impl rosidl_runtime_rs::Message for PackageUnPick_Response {
  type RmwMsg = super::srv::rmw::PackageUnPick_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to as2_msgs__srv__PathToGeopath_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PathToGeopath_Request {
    /// Path (m)
    pub path: nav_msgs::msg::Path,

}



impl Default for PathToGeopath_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PathToGeopath_Request::default())
  }
}

impl rosidl_runtime_rs::Message for PathToGeopath_Request {
  type RmwMsg = super::srv::rmw::PathToGeopath_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        path: nav_msgs::msg::Path::into_rmw_message(std::borrow::Cow::Owned(msg.path)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        path: nav_msgs::msg::Path::into_rmw_message(std::borrow::Cow::Borrowed(&msg.path)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      path: nav_msgs::msg::Path::from_rmw_message(msg.path),
    }
  }
}


// Corresponds to as2_msgs__srv__PathToGeopath_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PathToGeopath_Response {
    /// whether the origin has been set or not
    pub success: bool,

    /// Path in lat/lon and altitude
    pub geo_path: geographic_msgs::msg::GeoPath,

}



impl Default for PathToGeopath_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PathToGeopath_Response::default())
  }
}

impl rosidl_runtime_rs::Message for PathToGeopath_Response {
  type RmwMsg = super::srv::rmw::PathToGeopath_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        geo_path: geographic_msgs::msg::GeoPath::into_rmw_message(std::borrow::Cow::Owned(msg.geo_path)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        geo_path: geographic_msgs::msg::GeoPath::into_rmw_message(std::borrow::Cow::Borrowed(&msg.geo_path)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      geo_path: geographic_msgs::msg::GeoPath::from_rmw_message(msg.geo_path),
    }
  }
}


// Corresponds to as2_msgs__srv__SetControlMode_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetControlMode_Request {
    /// Control mode to set
    pub control_mode: super::msg::ControlMode,

}



impl Default for SetControlMode_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetControlMode_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetControlMode_Request {
  type RmwMsg = super::srv::rmw::SetControlMode_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        control_mode: super::msg::ControlMode::into_rmw_message(std::borrow::Cow::Owned(msg.control_mode)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        control_mode: super::msg::ControlMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.control_mode)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      control_mode: super::msg::ControlMode::from_rmw_message(msg.control_mode),
    }
  }
}


// Corresponds to as2_msgs__srv__SetControlMode_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetControlMode_Response {
    /// whether the control mode has been set or not
    pub success: bool,

}



impl Default for SetControlMode_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetControlMode_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetControlMode_Response {
  type RmwMsg = super::srv::rmw::SetControlMode_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to as2_msgs__srv__SetGeozone_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGeozone_Request {
    /// geostructure to set
    pub geozone: super::msg::Geozone,

}



impl Default for SetGeozone_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetGeozone_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetGeozone_Request {
  type RmwMsg = super::srv::rmw::SetGeozone_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        geozone: super::msg::Geozone::into_rmw_message(std::borrow::Cow::Owned(msg.geozone)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        geozone: super::msg::Geozone::into_rmw_message(std::borrow::Cow::Borrowed(&msg.geozone)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      geozone: super::msg::Geozone::from_rmw_message(msg.geozone),
    }
  }
}


// Corresponds to as2_msgs__srv__SetGeozone_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGeozone_Response {
    /// whether the geoStructure has been set or not
    pub success: bool,

}



impl Default for SetGeozone_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetGeozone_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetGeozone_Response {
  type RmwMsg = super::srv::rmw::SetGeozone_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to as2_msgs__srv__SetOrigin_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOrigin_Request {
    /// origin to set
    pub origin: geographic_msgs::msg::GeoPoint,

}



impl Default for SetOrigin_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetOrigin_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetOrigin_Request {
  type RmwMsg = super::srv::rmw::SetOrigin_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        origin: geographic_msgs::msg::GeoPoint::into_rmw_message(std::borrow::Cow::Owned(msg.origin)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        origin: geographic_msgs::msg::GeoPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.origin)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      origin: geographic_msgs::msg::GeoPoint::from_rmw_message(msg.origin),
    }
  }
}


// Corresponds to as2_msgs__srv__SetOrigin_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOrigin_Response {
    /// whether the origin has been set or not
    pub success: bool,

}



impl Default for SetOrigin_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetOrigin_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetOrigin_Response {
  type RmwMsg = super::srv::rmw::SetOrigin_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to as2_msgs__srv__SetPlatformStateMachineEvent_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPlatformStateMachineEvent_Request {
    /// event to set
    pub event: super::msg::PlatformStateMachineEvent,

}



impl Default for SetPlatformStateMachineEvent_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetPlatformStateMachineEvent_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetPlatformStateMachineEvent_Request {
  type RmwMsg = super::srv::rmw::SetPlatformStateMachineEvent_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        event: super::msg::PlatformStateMachineEvent::into_rmw_message(std::borrow::Cow::Owned(msg.event)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        event: super::msg::PlatformStateMachineEvent::into_rmw_message(std::borrow::Cow::Borrowed(&msg.event)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      event: super::msg::PlatformStateMachineEvent::from_rmw_message(msg.event),
    }
  }
}


// Corresponds to as2_msgs__srv__SetPlatformStateMachineEvent_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPlatformStateMachineEvent_Response {
    /// whether the PSM has been set or not
    pub success: bool,

    /// PSM result of the aircraft
    pub current_state: super::msg::PlatformStatus,

}



impl Default for SetPlatformStateMachineEvent_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetPlatformStateMachineEvent_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetPlatformStateMachineEvent_Response {
  type RmwMsg = super::srv::rmw::SetPlatformStateMachineEvent_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        current_state: super::msg::PlatformStatus::into_rmw_message(std::borrow::Cow::Owned(msg.current_state)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        current_state: super::msg::PlatformStatus::into_rmw_message(std::borrow::Cow::Borrowed(&msg.current_state)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      current_state: super::msg::PlatformStatus::from_rmw_message(msg.current_state),
    }
  }
}


// Corresponds to as2_msgs__srv__SetSpeed_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetSpeed_Request {
    /// speed to send
    pub speed: super::msg::Speed,

}



impl Default for SetSpeed_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetSpeed_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetSpeed_Request {
  type RmwMsg = super::srv::rmw::SetSpeed_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        speed: super::msg::Speed::into_rmw_message(std::borrow::Cow::Owned(msg.speed)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        speed: super::msg::Speed::into_rmw_message(std::borrow::Cow::Borrowed(&msg.speed)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      speed: super::msg::Speed::from_rmw_message(msg.speed),
    }
  }
}


// Corresponds to as2_msgs__srv__SetSpeed_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetSpeed_Response {
    /// whether the speed has been received or not
    pub success: bool,

}



impl Default for SetSpeed_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetSpeed_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetSpeed_Response {
  type RmwMsg = super::srv::rmw::SetSpeed_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
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


