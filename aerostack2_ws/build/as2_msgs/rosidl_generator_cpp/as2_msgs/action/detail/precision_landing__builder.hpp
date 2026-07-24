// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from as2_msgs:action/PrecisionLanding.idl
// generated code does not contain a copyright notice

#ifndef AS2_MSGS__ACTION__DETAIL__PRECISION_LANDING__BUILDER_HPP_
#define AS2_MSGS__ACTION__DETAIL__PRECISION_LANDING__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "as2_msgs/action/detail/precision_landing__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace as2_msgs
{

namespace action
{

namespace builder
{

class Init_PrecisionLanding_Goal_marker_frame_id
{
public:
  Init_PrecisionLanding_Goal_marker_frame_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::as2_msgs::action::PrecisionLanding_Goal marker_frame_id(::as2_msgs::action::PrecisionLanding_Goal::_marker_frame_id_type arg)
  {
    msg_.marker_frame_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::as2_msgs::action::PrecisionLanding_Goal msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::as2_msgs::action::PrecisionLanding_Goal>()
{
  return as2_msgs::action::builder::Init_PrecisionLanding_Goal_marker_frame_id();
}

}  // namespace as2_msgs


namespace as2_msgs
{

namespace action
{

namespace builder
{

class Init_PrecisionLanding_Result_precision_landing_success
{
public:
  Init_PrecisionLanding_Result_precision_landing_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::as2_msgs::action::PrecisionLanding_Result precision_landing_success(::as2_msgs::action::PrecisionLanding_Result::_precision_landing_success_type arg)
  {
    msg_.precision_landing_success = std::move(arg);
    return std::move(msg_);
  }

private:
  ::as2_msgs::action::PrecisionLanding_Result msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::as2_msgs::action::PrecisionLanding_Result>()
{
  return as2_msgs::action::builder::Init_PrecisionLanding_Result_precision_landing_success();
}

}  // namespace as2_msgs


namespace as2_msgs
{

namespace action
{

namespace builder
{

class Init_PrecisionLanding_Feedback_distance_z
{
public:
  explicit Init_PrecisionLanding_Feedback_distance_z(::as2_msgs::action::PrecisionLanding_Feedback & msg)
  : msg_(msg)
  {}
  ::as2_msgs::action::PrecisionLanding_Feedback distance_z(::as2_msgs::action::PrecisionLanding_Feedback::_distance_z_type arg)
  {
    msg_.distance_z = std::move(arg);
    return std::move(msg_);
  }

private:
  ::as2_msgs::action::PrecisionLanding_Feedback msg_;
};

class Init_PrecisionLanding_Feedback_distance_xy
{
public:
  explicit Init_PrecisionLanding_Feedback_distance_xy(::as2_msgs::action::PrecisionLanding_Feedback & msg)
  : msg_(msg)
  {}
  Init_PrecisionLanding_Feedback_distance_z distance_xy(::as2_msgs::action::PrecisionLanding_Feedback::_distance_xy_type arg)
  {
    msg_.distance_xy = std::move(arg);
    return Init_PrecisionLanding_Feedback_distance_z(msg_);
  }

private:
  ::as2_msgs::action::PrecisionLanding_Feedback msg_;
};

class Init_PrecisionLanding_Feedback_precision_landing_height
{
public:
  explicit Init_PrecisionLanding_Feedback_precision_landing_height(::as2_msgs::action::PrecisionLanding_Feedback & msg)
  : msg_(msg)
  {}
  Init_PrecisionLanding_Feedback_distance_xy precision_landing_height(::as2_msgs::action::PrecisionLanding_Feedback::_precision_landing_height_type arg)
  {
    msg_.precision_landing_height = std::move(arg);
    return Init_PrecisionLanding_Feedback_distance_xy(msg_);
  }

private:
  ::as2_msgs::action::PrecisionLanding_Feedback msg_;
};

class Init_PrecisionLanding_Feedback_precision_landing_speed
{
public:
  Init_PrecisionLanding_Feedback_precision_landing_speed()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_PrecisionLanding_Feedback_precision_landing_height precision_landing_speed(::as2_msgs::action::PrecisionLanding_Feedback::_precision_landing_speed_type arg)
  {
    msg_.precision_landing_speed = std::move(arg);
    return Init_PrecisionLanding_Feedback_precision_landing_height(msg_);
  }

private:
  ::as2_msgs::action::PrecisionLanding_Feedback msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::as2_msgs::action::PrecisionLanding_Feedback>()
{
  return as2_msgs::action::builder::Init_PrecisionLanding_Feedback_precision_landing_speed();
}

}  // namespace as2_msgs


namespace as2_msgs
{

namespace action
{

namespace builder
{

class Init_PrecisionLanding_SendGoal_Request_goal
{
public:
  explicit Init_PrecisionLanding_SendGoal_Request_goal(::as2_msgs::action::PrecisionLanding_SendGoal_Request & msg)
  : msg_(msg)
  {}
  ::as2_msgs::action::PrecisionLanding_SendGoal_Request goal(::as2_msgs::action::PrecisionLanding_SendGoal_Request::_goal_type arg)
  {
    msg_.goal = std::move(arg);
    return std::move(msg_);
  }

private:
  ::as2_msgs::action::PrecisionLanding_SendGoal_Request msg_;
};

class Init_PrecisionLanding_SendGoal_Request_goal_id
{
public:
  Init_PrecisionLanding_SendGoal_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_PrecisionLanding_SendGoal_Request_goal goal_id(::as2_msgs::action::PrecisionLanding_SendGoal_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_PrecisionLanding_SendGoal_Request_goal(msg_);
  }

private:
  ::as2_msgs::action::PrecisionLanding_SendGoal_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::as2_msgs::action::PrecisionLanding_SendGoal_Request>()
{
  return as2_msgs::action::builder::Init_PrecisionLanding_SendGoal_Request_goal_id();
}

}  // namespace as2_msgs


namespace as2_msgs
{

namespace action
{

namespace builder
{

class Init_PrecisionLanding_SendGoal_Response_stamp
{
public:
  explicit Init_PrecisionLanding_SendGoal_Response_stamp(::as2_msgs::action::PrecisionLanding_SendGoal_Response & msg)
  : msg_(msg)
  {}
  ::as2_msgs::action::PrecisionLanding_SendGoal_Response stamp(::as2_msgs::action::PrecisionLanding_SendGoal_Response::_stamp_type arg)
  {
    msg_.stamp = std::move(arg);
    return std::move(msg_);
  }

private:
  ::as2_msgs::action::PrecisionLanding_SendGoal_Response msg_;
};

class Init_PrecisionLanding_SendGoal_Response_accepted
{
public:
  Init_PrecisionLanding_SendGoal_Response_accepted()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_PrecisionLanding_SendGoal_Response_stamp accepted(::as2_msgs::action::PrecisionLanding_SendGoal_Response::_accepted_type arg)
  {
    msg_.accepted = std::move(arg);
    return Init_PrecisionLanding_SendGoal_Response_stamp(msg_);
  }

private:
  ::as2_msgs::action::PrecisionLanding_SendGoal_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::as2_msgs::action::PrecisionLanding_SendGoal_Response>()
{
  return as2_msgs::action::builder::Init_PrecisionLanding_SendGoal_Response_accepted();
}

}  // namespace as2_msgs


namespace as2_msgs
{

namespace action
{

namespace builder
{

class Init_PrecisionLanding_GetResult_Request_goal_id
{
public:
  Init_PrecisionLanding_GetResult_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::as2_msgs::action::PrecisionLanding_GetResult_Request goal_id(::as2_msgs::action::PrecisionLanding_GetResult_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::as2_msgs::action::PrecisionLanding_GetResult_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::as2_msgs::action::PrecisionLanding_GetResult_Request>()
{
  return as2_msgs::action::builder::Init_PrecisionLanding_GetResult_Request_goal_id();
}

}  // namespace as2_msgs


namespace as2_msgs
{

namespace action
{

namespace builder
{

class Init_PrecisionLanding_GetResult_Response_result
{
public:
  explicit Init_PrecisionLanding_GetResult_Response_result(::as2_msgs::action::PrecisionLanding_GetResult_Response & msg)
  : msg_(msg)
  {}
  ::as2_msgs::action::PrecisionLanding_GetResult_Response result(::as2_msgs::action::PrecisionLanding_GetResult_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return std::move(msg_);
  }

private:
  ::as2_msgs::action::PrecisionLanding_GetResult_Response msg_;
};

class Init_PrecisionLanding_GetResult_Response_status
{
public:
  Init_PrecisionLanding_GetResult_Response_status()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_PrecisionLanding_GetResult_Response_result status(::as2_msgs::action::PrecisionLanding_GetResult_Response::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_PrecisionLanding_GetResult_Response_result(msg_);
  }

private:
  ::as2_msgs::action::PrecisionLanding_GetResult_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::as2_msgs::action::PrecisionLanding_GetResult_Response>()
{
  return as2_msgs::action::builder::Init_PrecisionLanding_GetResult_Response_status();
}

}  // namespace as2_msgs


namespace as2_msgs
{

namespace action
{

namespace builder
{

class Init_PrecisionLanding_FeedbackMessage_feedback
{
public:
  explicit Init_PrecisionLanding_FeedbackMessage_feedback(::as2_msgs::action::PrecisionLanding_FeedbackMessage & msg)
  : msg_(msg)
  {}
  ::as2_msgs::action::PrecisionLanding_FeedbackMessage feedback(::as2_msgs::action::PrecisionLanding_FeedbackMessage::_feedback_type arg)
  {
    msg_.feedback = std::move(arg);
    return std::move(msg_);
  }

private:
  ::as2_msgs::action::PrecisionLanding_FeedbackMessage msg_;
};

class Init_PrecisionLanding_FeedbackMessage_goal_id
{
public:
  Init_PrecisionLanding_FeedbackMessage_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_PrecisionLanding_FeedbackMessage_feedback goal_id(::as2_msgs::action::PrecisionLanding_FeedbackMessage::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_PrecisionLanding_FeedbackMessage_feedback(msg_);
  }

private:
  ::as2_msgs::action::PrecisionLanding_FeedbackMessage msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::as2_msgs::action::PrecisionLanding_FeedbackMessage>()
{
  return as2_msgs::action::builder::Init_PrecisionLanding_FeedbackMessage_goal_id();
}

}  // namespace as2_msgs

#endif  // AS2_MSGS__ACTION__DETAIL__PRECISION_LANDING__BUILDER_HPP_
