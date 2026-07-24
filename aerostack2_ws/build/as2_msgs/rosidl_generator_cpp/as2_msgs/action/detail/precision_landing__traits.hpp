// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from as2_msgs:action/PrecisionLanding.idl
// generated code does not contain a copyright notice

#ifndef AS2_MSGS__ACTION__DETAIL__PRECISION_LANDING__TRAITS_HPP_
#define AS2_MSGS__ACTION__DETAIL__PRECISION_LANDING__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "as2_msgs/action/detail/precision_landing__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace as2_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const PrecisionLanding_Goal & msg,
  std::ostream & out)
{
  out << "{";
  // member: marker_frame_id
  {
    out << "marker_frame_id: ";
    rosidl_generator_traits::value_to_yaml(msg.marker_frame_id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const PrecisionLanding_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: marker_frame_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "marker_frame_id: ";
    rosidl_generator_traits::value_to_yaml(msg.marker_frame_id, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const PrecisionLanding_Goal & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace as2_msgs

namespace rosidl_generator_traits
{

[[deprecated("use as2_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const as2_msgs::action::PrecisionLanding_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  as2_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use as2_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const as2_msgs::action::PrecisionLanding_Goal & msg)
{
  return as2_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<as2_msgs::action::PrecisionLanding_Goal>()
{
  return "as2_msgs::action::PrecisionLanding_Goal";
}

template<>
inline const char * name<as2_msgs::action::PrecisionLanding_Goal>()
{
  return "as2_msgs/action/PrecisionLanding_Goal";
}

template<>
struct has_fixed_size<as2_msgs::action::PrecisionLanding_Goal>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<as2_msgs::action::PrecisionLanding_Goal>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<as2_msgs::action::PrecisionLanding_Goal>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace as2_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const PrecisionLanding_Result & msg,
  std::ostream & out)
{
  out << "{";
  // member: precision_landing_success
  {
    out << "precision_landing_success: ";
    rosidl_generator_traits::value_to_yaml(msg.precision_landing_success, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const PrecisionLanding_Result & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: precision_landing_success
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "precision_landing_success: ";
    rosidl_generator_traits::value_to_yaml(msg.precision_landing_success, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const PrecisionLanding_Result & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace as2_msgs

namespace rosidl_generator_traits
{

[[deprecated("use as2_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const as2_msgs::action::PrecisionLanding_Result & msg,
  std::ostream & out, size_t indentation = 0)
{
  as2_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use as2_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const as2_msgs::action::PrecisionLanding_Result & msg)
{
  return as2_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<as2_msgs::action::PrecisionLanding_Result>()
{
  return "as2_msgs::action::PrecisionLanding_Result";
}

template<>
inline const char * name<as2_msgs::action::PrecisionLanding_Result>()
{
  return "as2_msgs/action/PrecisionLanding_Result";
}

template<>
struct has_fixed_size<as2_msgs::action::PrecisionLanding_Result>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<as2_msgs::action::PrecisionLanding_Result>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<as2_msgs::action::PrecisionLanding_Result>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace as2_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const PrecisionLanding_Feedback & msg,
  std::ostream & out)
{
  out << "{";
  // member: precision_landing_speed
  {
    out << "precision_landing_speed: ";
    rosidl_generator_traits::value_to_yaml(msg.precision_landing_speed, out);
    out << ", ";
  }

  // member: precision_landing_height
  {
    out << "precision_landing_height: ";
    rosidl_generator_traits::value_to_yaml(msg.precision_landing_height, out);
    out << ", ";
  }

  // member: distance_xy
  {
    out << "distance_xy: ";
    rosidl_generator_traits::value_to_yaml(msg.distance_xy, out);
    out << ", ";
  }

  // member: distance_z
  {
    out << "distance_z: ";
    rosidl_generator_traits::value_to_yaml(msg.distance_z, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const PrecisionLanding_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: precision_landing_speed
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "precision_landing_speed: ";
    rosidl_generator_traits::value_to_yaml(msg.precision_landing_speed, out);
    out << "\n";
  }

  // member: precision_landing_height
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "precision_landing_height: ";
    rosidl_generator_traits::value_to_yaml(msg.precision_landing_height, out);
    out << "\n";
  }

  // member: distance_xy
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "distance_xy: ";
    rosidl_generator_traits::value_to_yaml(msg.distance_xy, out);
    out << "\n";
  }

  // member: distance_z
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "distance_z: ";
    rosidl_generator_traits::value_to_yaml(msg.distance_z, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const PrecisionLanding_Feedback & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace as2_msgs

namespace rosidl_generator_traits
{

[[deprecated("use as2_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const as2_msgs::action::PrecisionLanding_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  as2_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use as2_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const as2_msgs::action::PrecisionLanding_Feedback & msg)
{
  return as2_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<as2_msgs::action::PrecisionLanding_Feedback>()
{
  return "as2_msgs::action::PrecisionLanding_Feedback";
}

template<>
inline const char * name<as2_msgs::action::PrecisionLanding_Feedback>()
{
  return "as2_msgs/action/PrecisionLanding_Feedback";
}

template<>
struct has_fixed_size<as2_msgs::action::PrecisionLanding_Feedback>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<as2_msgs::action::PrecisionLanding_Feedback>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<as2_msgs::action::PrecisionLanding_Feedback>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"
// Member 'goal'
#include "as2_msgs/action/detail/precision_landing__traits.hpp"

namespace as2_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const PrecisionLanding_SendGoal_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
    out << ", ";
  }

  // member: goal
  {
    out << "goal: ";
    to_flow_style_yaml(msg.goal, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const PrecisionLanding_SendGoal_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }

  // member: goal
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal:\n";
    to_block_style_yaml(msg.goal, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const PrecisionLanding_SendGoal_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace as2_msgs

namespace rosidl_generator_traits
{

[[deprecated("use as2_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const as2_msgs::action::PrecisionLanding_SendGoal_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  as2_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use as2_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const as2_msgs::action::PrecisionLanding_SendGoal_Request & msg)
{
  return as2_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<as2_msgs::action::PrecisionLanding_SendGoal_Request>()
{
  return "as2_msgs::action::PrecisionLanding_SendGoal_Request";
}

template<>
inline const char * name<as2_msgs::action::PrecisionLanding_SendGoal_Request>()
{
  return "as2_msgs/action/PrecisionLanding_SendGoal_Request";
}

template<>
struct has_fixed_size<as2_msgs::action::PrecisionLanding_SendGoal_Request>
  : std::integral_constant<bool, has_fixed_size<as2_msgs::action::PrecisionLanding_Goal>::value && has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<as2_msgs::action::PrecisionLanding_SendGoal_Request>
  : std::integral_constant<bool, has_bounded_size<as2_msgs::action::PrecisionLanding_Goal>::value && has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<as2_msgs::action::PrecisionLanding_SendGoal_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__traits.hpp"

namespace as2_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const PrecisionLanding_SendGoal_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: accepted
  {
    out << "accepted: ";
    rosidl_generator_traits::value_to_yaml(msg.accepted, out);
    out << ", ";
  }

  // member: stamp
  {
    out << "stamp: ";
    to_flow_style_yaml(msg.stamp, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const PrecisionLanding_SendGoal_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: accepted
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "accepted: ";
    rosidl_generator_traits::value_to_yaml(msg.accepted, out);
    out << "\n";
  }

  // member: stamp
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "stamp:\n";
    to_block_style_yaml(msg.stamp, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const PrecisionLanding_SendGoal_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace as2_msgs

namespace rosidl_generator_traits
{

[[deprecated("use as2_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const as2_msgs::action::PrecisionLanding_SendGoal_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  as2_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use as2_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const as2_msgs::action::PrecisionLanding_SendGoal_Response & msg)
{
  return as2_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<as2_msgs::action::PrecisionLanding_SendGoal_Response>()
{
  return "as2_msgs::action::PrecisionLanding_SendGoal_Response";
}

template<>
inline const char * name<as2_msgs::action::PrecisionLanding_SendGoal_Response>()
{
  return "as2_msgs/action/PrecisionLanding_SendGoal_Response";
}

template<>
struct has_fixed_size<as2_msgs::action::PrecisionLanding_SendGoal_Response>
  : std::integral_constant<bool, has_fixed_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct has_bounded_size<as2_msgs::action::PrecisionLanding_SendGoal_Response>
  : std::integral_constant<bool, has_bounded_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct is_message<as2_msgs::action::PrecisionLanding_SendGoal_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<as2_msgs::action::PrecisionLanding_SendGoal>()
{
  return "as2_msgs::action::PrecisionLanding_SendGoal";
}

template<>
inline const char * name<as2_msgs::action::PrecisionLanding_SendGoal>()
{
  return "as2_msgs/action/PrecisionLanding_SendGoal";
}

template<>
struct has_fixed_size<as2_msgs::action::PrecisionLanding_SendGoal>
  : std::integral_constant<
    bool,
    has_fixed_size<as2_msgs::action::PrecisionLanding_SendGoal_Request>::value &&
    has_fixed_size<as2_msgs::action::PrecisionLanding_SendGoal_Response>::value
  >
{
};

template<>
struct has_bounded_size<as2_msgs::action::PrecisionLanding_SendGoal>
  : std::integral_constant<
    bool,
    has_bounded_size<as2_msgs::action::PrecisionLanding_SendGoal_Request>::value &&
    has_bounded_size<as2_msgs::action::PrecisionLanding_SendGoal_Response>::value
  >
{
};

template<>
struct is_service<as2_msgs::action::PrecisionLanding_SendGoal>
  : std::true_type
{
};

template<>
struct is_service_request<as2_msgs::action::PrecisionLanding_SendGoal_Request>
  : std::true_type
{
};

template<>
struct is_service_response<as2_msgs::action::PrecisionLanding_SendGoal_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"

namespace as2_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const PrecisionLanding_GetResult_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const PrecisionLanding_GetResult_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const PrecisionLanding_GetResult_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace as2_msgs

namespace rosidl_generator_traits
{

[[deprecated("use as2_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const as2_msgs::action::PrecisionLanding_GetResult_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  as2_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use as2_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const as2_msgs::action::PrecisionLanding_GetResult_Request & msg)
{
  return as2_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<as2_msgs::action::PrecisionLanding_GetResult_Request>()
{
  return "as2_msgs::action::PrecisionLanding_GetResult_Request";
}

template<>
inline const char * name<as2_msgs::action::PrecisionLanding_GetResult_Request>()
{
  return "as2_msgs/action/PrecisionLanding_GetResult_Request";
}

template<>
struct has_fixed_size<as2_msgs::action::PrecisionLanding_GetResult_Request>
  : std::integral_constant<bool, has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<as2_msgs::action::PrecisionLanding_GetResult_Request>
  : std::integral_constant<bool, has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<as2_msgs::action::PrecisionLanding_GetResult_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'result'
// already included above
// #include "as2_msgs/action/detail/precision_landing__traits.hpp"

namespace as2_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const PrecisionLanding_GetResult_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: status
  {
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << ", ";
  }

  // member: result
  {
    out << "result: ";
    to_flow_style_yaml(msg.result, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const PrecisionLanding_GetResult_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: status
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << "\n";
  }

  // member: result
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "result:\n";
    to_block_style_yaml(msg.result, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const PrecisionLanding_GetResult_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace as2_msgs

namespace rosidl_generator_traits
{

[[deprecated("use as2_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const as2_msgs::action::PrecisionLanding_GetResult_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  as2_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use as2_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const as2_msgs::action::PrecisionLanding_GetResult_Response & msg)
{
  return as2_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<as2_msgs::action::PrecisionLanding_GetResult_Response>()
{
  return "as2_msgs::action::PrecisionLanding_GetResult_Response";
}

template<>
inline const char * name<as2_msgs::action::PrecisionLanding_GetResult_Response>()
{
  return "as2_msgs/action/PrecisionLanding_GetResult_Response";
}

template<>
struct has_fixed_size<as2_msgs::action::PrecisionLanding_GetResult_Response>
  : std::integral_constant<bool, has_fixed_size<as2_msgs::action::PrecisionLanding_Result>::value> {};

template<>
struct has_bounded_size<as2_msgs::action::PrecisionLanding_GetResult_Response>
  : std::integral_constant<bool, has_bounded_size<as2_msgs::action::PrecisionLanding_Result>::value> {};

template<>
struct is_message<as2_msgs::action::PrecisionLanding_GetResult_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<as2_msgs::action::PrecisionLanding_GetResult>()
{
  return "as2_msgs::action::PrecisionLanding_GetResult";
}

template<>
inline const char * name<as2_msgs::action::PrecisionLanding_GetResult>()
{
  return "as2_msgs/action/PrecisionLanding_GetResult";
}

template<>
struct has_fixed_size<as2_msgs::action::PrecisionLanding_GetResult>
  : std::integral_constant<
    bool,
    has_fixed_size<as2_msgs::action::PrecisionLanding_GetResult_Request>::value &&
    has_fixed_size<as2_msgs::action::PrecisionLanding_GetResult_Response>::value
  >
{
};

template<>
struct has_bounded_size<as2_msgs::action::PrecisionLanding_GetResult>
  : std::integral_constant<
    bool,
    has_bounded_size<as2_msgs::action::PrecisionLanding_GetResult_Request>::value &&
    has_bounded_size<as2_msgs::action::PrecisionLanding_GetResult_Response>::value
  >
{
};

template<>
struct is_service<as2_msgs::action::PrecisionLanding_GetResult>
  : std::true_type
{
};

template<>
struct is_service_request<as2_msgs::action::PrecisionLanding_GetResult_Request>
  : std::true_type
{
};

template<>
struct is_service_response<as2_msgs::action::PrecisionLanding_GetResult_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"
// Member 'feedback'
// already included above
// #include "as2_msgs/action/detail/precision_landing__traits.hpp"

namespace as2_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const PrecisionLanding_FeedbackMessage & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
    out << ", ";
  }

  // member: feedback
  {
    out << "feedback: ";
    to_flow_style_yaml(msg.feedback, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const PrecisionLanding_FeedbackMessage & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }

  // member: feedback
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "feedback:\n";
    to_block_style_yaml(msg.feedback, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const PrecisionLanding_FeedbackMessage & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace as2_msgs

namespace rosidl_generator_traits
{

[[deprecated("use as2_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const as2_msgs::action::PrecisionLanding_FeedbackMessage & msg,
  std::ostream & out, size_t indentation = 0)
{
  as2_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use as2_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const as2_msgs::action::PrecisionLanding_FeedbackMessage & msg)
{
  return as2_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<as2_msgs::action::PrecisionLanding_FeedbackMessage>()
{
  return "as2_msgs::action::PrecisionLanding_FeedbackMessage";
}

template<>
inline const char * name<as2_msgs::action::PrecisionLanding_FeedbackMessage>()
{
  return "as2_msgs/action/PrecisionLanding_FeedbackMessage";
}

template<>
struct has_fixed_size<as2_msgs::action::PrecisionLanding_FeedbackMessage>
  : std::integral_constant<bool, has_fixed_size<as2_msgs::action::PrecisionLanding_Feedback>::value && has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<as2_msgs::action::PrecisionLanding_FeedbackMessage>
  : std::integral_constant<bool, has_bounded_size<as2_msgs::action::PrecisionLanding_Feedback>::value && has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<as2_msgs::action::PrecisionLanding_FeedbackMessage>
  : std::true_type {};

}  // namespace rosidl_generator_traits


namespace rosidl_generator_traits
{

template<>
struct is_action<as2_msgs::action::PrecisionLanding>
  : std::true_type
{
};

template<>
struct is_action_goal<as2_msgs::action::PrecisionLanding_Goal>
  : std::true_type
{
};

template<>
struct is_action_result<as2_msgs::action::PrecisionLanding_Result>
  : std::true_type
{
};

template<>
struct is_action_feedback<as2_msgs::action::PrecisionLanding_Feedback>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits


#endif  // AS2_MSGS__ACTION__DETAIL__PRECISION_LANDING__TRAITS_HPP_
