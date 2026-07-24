// BSD-3-Clause
/**
 * @file precision_landing_plugin_pid.cpp
 *
 * Plugin: full PID for XY + proportional yaw matching + constant descent inside radius.
 *
 * XY position error is controlled with independent P+I+D loops that handle
 * steady-state disturbances (wind, sensor bias) via the integral term and
 * damp oscillations via the derivative term.
 *
 * Yaw is matched to the landing pad orientation extracted from TF using a
 * proportional controller (yaw_speed = kp * yaw_error).
 *
 * Descent is enabled only when the XY error is below landing_radius,
 * ensuring the drone is centred before committing to the approach.
 *
 * When yaw_detached is true the run loop goes through two sequential alignment
 * stages before descent:
 *   ALIGNING_XY  – PID-correct XY until dist_xy < yaw_detached_xy_threshold
 *   ALIGNING_YAW – correct yaw until |yaw_err| < yaw_detached_yaw_threshold
 * After both stages pass, normal PID+yaw+descent runs (PID state is reset on entry).
 */

#include <algorithm>
#include <cmath>
#include <tuple>

#include <pluginlib/class_list_macros.hpp>
#include <tf2_geometry_msgs/tf2_geometry_msgs.hpp>

#include "as2_behavior/behavior_server.hpp"
#include "as2_core/utils/frame_utils.hpp"
#include "as2_motion_reference_handlers/speed_motion.hpp"
#include "precision_landing_behavior/precision_landing_base.hpp"

namespace precision_landing_plugin_pid
{

static double normalizeAngle(double a)
{
  return std::atan2(std::sin(a), std::cos(a));
}

class Plugin : public precision_landing_base::PrecisionLandingBase
{
public:
  void ownInit() override
  {
    RCLCPP_INFO(node_ptr_->get_logger(), "[pid] Init start");

    speed_motion_handler_ =
        std::make_shared<as2::motionReferenceHandlers::SpeedMotion>(node_ptr_);

    node_ptr_->declare_parameter<std::string>("marker_frame_id", "landing_pad");
    node_ptr_->get_parameter("marker_frame_id", marker_frame_id_);

    // Plugin-specific
    node_ptr_->declare_parameter<double>("pid_xy_kp", 1.0);
    node_ptr_->get_parameter("pid_xy_kp", xy_kp_);
    node_ptr_->declare_parameter<double>("pid_xy_ki", 0.05);
    node_ptr_->get_parameter("pid_xy_ki", xy_ki_);
    node_ptr_->declare_parameter<double>("pid_xy_kd", 0.15);
    node_ptr_->get_parameter("pid_xy_kd", xy_kd_);
    node_ptr_->declare_parameter<double>("pid_xy_integral_max", 0.3);
    node_ptr_->get_parameter("pid_xy_integral_max", xy_integral_max_);
    node_ptr_->declare_parameter<double>("pid_yaw_kp", 1.0);
    node_ptr_->get_parameter("pid_yaw_kp", yaw_kp_);

    // Shared parameters
    node_ptr_->declare_parameter<double>("z_descent", 0.3);
    node_ptr_->get_parameter("z_descent", z_descent_);
    node_ptr_->declare_parameter<double>("z_distance_threshold", 0.1);
    node_ptr_->get_parameter("z_distance_threshold", z_distance_threshold_);
    node_ptr_->declare_parameter<double>("xy_speed_max", 1.0);
    node_ptr_->get_parameter("xy_speed_max", xy_speed_max_);
    node_ptr_->declare_parameter<double>("landing_radius", 0.3);
    node_ptr_->get_parameter("landing_radius", landing_radius_);
    node_ptr_->declare_parameter<double>("yaw_speed_max", 0.5);
    node_ptr_->get_parameter("yaw_speed_max", yaw_speed_max_);

    // Yaw-detached alignment
    node_ptr_->declare_parameter<bool>("yaw_detached", false);
    node_ptr_->get_parameter("yaw_detached", yaw_detached_);
    node_ptr_->declare_parameter<double>("yaw_detached_xy_threshold", 0.3);
    node_ptr_->get_parameter("yaw_detached_xy_threshold", yaw_detached_xy_threshold_);
    node_ptr_->declare_parameter<double>("yaw_detached_yaw_threshold", 0.1);
    node_ptr_->get_parameter("yaw_detached_yaw_threshold", yaw_detached_yaw_threshold_);

    RCLCPP_INFO(
        node_ptr_->get_logger(),
        "[pid] xy_kp=%.2f xy_ki=%.2f xy_kd=%.2f xy_vmax=%.2f xy_imax=%.2f"
        " | z_descent=%.2f z_th=%.2f radius=%.2f"
        " | yaw_kp=%.2f yaw_vmax=%.2f | yaw_detached=%s | marker=%s",
        xy_kp_, xy_ki_, xy_kd_, xy_speed_max_, xy_integral_max_,
        z_descent_, z_distance_threshold_, landing_radius_,
        yaw_kp_, yaw_speed_max_,
        yaw_detached_ ? "true" : "false", marker_frame_id_.c_str());

    resetStatus();
  }

  bool own_activate(as2_msgs::action::PrecisionLanding::Goal &) override
  {
    RCLCPP_INFO(node_ptr_->get_logger(), "[pid] Precision Landing accepted");
    resetStatus();
    return true;
  }

  bool own_deactivate(const std::shared_ptr<std::string> &) override
  {
    RCLCPP_INFO(node_ptr_->get_logger(), "[pid] Precision Landing canceled, hover");
    sendHover();
    return true;
  }

  bool own_pause(const std::shared_ptr<std::string> &) override
  {
    RCLCPP_INFO(node_ptr_->get_logger(), "[pid] Precision Landing paused");
    sendHover();
    return true;
  }

  bool own_resume(const std::shared_ptr<std::string> &) override
  {
    RCLCPP_INFO(node_ptr_->get_logger(), "[pid] Precision Landing resumed");
    resetStatus();
    return true;
  }

  void own_execution_end(const as2_behavior::ExecutionStatus & state) override
  {
    RCLCPP_INFO(node_ptr_->get_logger(), "[pid] end - state: %d", (int)state);
    if (state != as2_behavior::ExecutionStatus::SUCCESS)
      sendHover();
  }

  as2_behavior::ExecutionStatus own_run() override
  {
    geometry_msgs::msg::TransformStamped tf_aruco;
    if (!tryGetArucoTF(tf_aruco)) {
      if (arucoTimeoutExceeded()) {
        RCLCPP_WARN(node_ptr_->get_logger(), "[pid] ArUco TF timeout -> failure");
        result_.precision_landing_success = false;
        return as2_behavior::ExecutionStatus::FAILURE;
      }
      RCLCPP_INFO(node_ptr_->get_logger(), "[pid] No ArUco TF yet, hovering...");
      resetPIDState();
      sendHover();
      return as2_behavior::ExecutionStatus::RUNNING;
    }

    // dt for I and D terms; clamp to avoid spikes after pauses.
    const rclcpp::Time now = node_ptr_->now();
    const double dt        = last_run_valid_ ? (now - last_run_time_).seconds() : 0.0;
    last_run_time_         = now;
    last_run_valid_        = true;
    const double dt_c      = std::min(dt, 0.1);

    const auto [dx, dy, dz] = computeRelativeError(tf_aruco);
    const double dist_xy    = std::hypot(dx, dy);
    const double abs_dz     = std::fabs(dz);

    if (abs_dz < z_distance_threshold_) {
      RCLCPP_INFO(node_ptr_->get_logger(),
                  "[pid] z threshold reached (|dz|=%.3f). Success.", abs_dz);
      result_.precision_landing_success = true;
      sendHover();
      return as2_behavior::ExecutionStatus::SUCCESS;
    }

    const double pad_yaw   = static_cast<double>(
        as2::frame::getYawFromQuaternion(tf_aruco.transform.rotation));
    const double drone_yaw = static_cast<double>(getActualYaw());
    const double yaw_err   = normalizeAngle(pad_yaw - drone_yaw);

    double vx = 0.0, vy = 0.0, vz = 0.0, yaw_speed = 0.0;

    if (yaw_detached_ && state_ != AlignState::DESCENDING) {
      runAlignmentFSM(dx, dy, dist_xy, yaw_err, dt_c, vx, vy, vz, yaw_speed);
    } else {
      computePIDVelocity(dx, dy, dt_c, vx, vy);
      vz        = (dist_xy < landing_radius_) ? -std::fabs(z_descent_) : 0.0;
      yaw_speed = std::clamp(yaw_kp_ * yaw_err, -yaw_speed_max_, yaw_speed_max_);
    }

    RCLCPP_INFO(node_ptr_->get_logger(),
                "[pid] vx=%.3f vy=%.3f vz=%.3f yaw_sp=%.3f | "
                "dist_xy=%.3f dz=%.3f yaw_err=%.3f dt=%.3f",
                vx, vy, vz, yaw_speed, dist_xy, dz, yaw_err, dt_c);

    if (!speed_motion_handler_->sendSpeedCommandWithYawSpeed(
            "earth", vx, vy, vz, yaw_speed)) {
      RCLCPP_ERROR(node_ptr_->get_logger(), "[pid] Error sending speed command");
      result_.precision_landing_success = false;
      return as2_behavior::ExecutionStatus::FAILURE;
    }

    feedback_.distance_xy = dist_xy;
    feedback_.distance_z  = dz;
    return as2_behavior::ExecutionStatus::RUNNING;
  }

private:
  enum class AlignState { ALIGNING_XY, ALIGNING_YAW, DESCENDING };

  std::shared_ptr<as2::motionReferenceHandlers::SpeedMotion> speed_motion_handler_{nullptr};

  std::string marker_frame_id_{"landing_pad"};

  // Plugin-specific
  double xy_kp_{1.0};
  double xy_ki_{0.05};
  double xy_kd_{0.15};
  double xy_integral_max_{0.3};
  double yaw_kp_{1.0};

  // Shared
  double z_descent_{0.3};
  double z_distance_threshold_{0.1};
  double xy_speed_max_{1.0};
  double landing_radius_{0.3};
  double yaw_speed_max_{0.5};

  bool   yaw_detached_{false};
  double yaw_detached_xy_threshold_{0.3};
  double yaw_detached_yaw_threshold_{0.1};

  AlignState   state_{AlignState::ALIGNING_XY};

  // PID state
  double       x_integral_{0.0};
  double       y_integral_{0.0};
  double       x_prev_err_{0.0};
  double       y_prev_err_{0.0};
  rclcpp::Time last_run_time_;
  bool         last_run_valid_{false};
  rclcpp::Time last_aruco_time_;

  void resetStatus()
  {
    state_           = AlignState::ALIGNING_XY;
    last_aruco_time_ = node_ptr_->now();
    resetPIDState();
    RCLCPP_INFO(node_ptr_->get_logger(), "[pid] Status reset");
  }

  void resetPIDState()
  {
    x_integral_    = 0.0;
    y_integral_    = 0.0;
    x_prev_err_    = 0.0;
    y_prev_err_    = 0.0;
    last_run_valid_ = false;
  }

  void computePIDVelocity(double dx, double dy, double dt_c, double & vx, double & vy)
  {
    if (dt_c > 1e-6) {
      x_integral_ = std::clamp(
          x_integral_ + dx * dt_c, -xy_integral_max_, xy_integral_max_);
      y_integral_ = std::clamp(
          y_integral_ + dy * dt_c, -xy_integral_max_, xy_integral_max_);
    }
    const double dx_d = (dt_c > 1e-6) ? (dx - x_prev_err_) / dt_c : 0.0;
    const double dy_d = (dt_c > 1e-6) ? (dy - y_prev_err_) / dt_c : 0.0;
    x_prev_err_       = dx;
    y_prev_err_       = dy;

    vx = xy_kp_ * dx + xy_ki_ * x_integral_ + xy_kd_ * dx_d;
    vy = xy_kp_ * dy + xy_ki_ * y_integral_ + xy_kd_ * dy_d;

    const double vxy = std::hypot(vx, vy);
    if (vxy > xy_speed_max_) {
      const double s = xy_speed_max_ / (vxy + 1e-9);
      vx *= s;
      vy *= s;
    }
  }

  void runAlignmentFSM(double dx, double dy, double dist_xy, double yaw_err, double dt_c,
                       double & vx, double & vy, double & vz, double & yaw_speed)
  {
    switch (state_) {
      case AlignState::ALIGNING_XY:
        RCLCPP_INFO_THROTTLE(node_ptr_->get_logger(), *node_ptr_->get_clock(), 1000,
                             "[pid] ALIGNING_XY  dist_xy=%.3f (th=%.3f)",
                             dist_xy, yaw_detached_xy_threshold_);
        computePIDVelocity(dx, dy, dt_c, vx, vy);
        vz = 0.0;
        yaw_speed = 0.0;
        if (dist_xy < yaw_detached_xy_threshold_) {
          RCLCPP_INFO(node_ptr_->get_logger(), "[pid] XY aligned -> ALIGNING_YAW");
          resetPIDState();
          state_ = AlignState::ALIGNING_YAW;
        }
        break;

      case AlignState::ALIGNING_YAW:
        RCLCPP_INFO_THROTTLE(node_ptr_->get_logger(), *node_ptr_->get_clock(), 1000,
                             "[pid] ALIGNING_YAW  yaw_err=%.3f (th=%.3f)",
                             yaw_err, yaw_detached_yaw_threshold_);
        vx = 0.0;
        vy = 0.0;
        vz = 0.0;
        yaw_speed = std::clamp(yaw_kp_ * yaw_err, -yaw_speed_max_, yaw_speed_max_);
        if (std::fabs(yaw_err) < yaw_detached_yaw_threshold_) {
          RCLCPP_INFO(node_ptr_->get_logger(), "[pid] YAW aligned -> DESCENDING");
          resetPIDState();
          state_ = AlignState::DESCENDING;
        }
        break;

      default:
        break;
    }
  }

  bool tryGetArucoTF(geometry_msgs::msg::TransformStamped & tf_out)
  {
    try {
      tf_out           = tf_handler_->getTransform("earth", marker_frame_id_);
      last_aruco_time_ = node_ptr_->now();
      return true;
    } catch (const tf2::TransformException & ex) {
      RCLCPP_DEBUG(node_ptr_->get_logger(), "[pid] TF unavailable: %s", ex.what());
      return false;
    }
  }

  bool arucoTimeoutExceeded() const
  {
    return (node_ptr_->now() - last_aruco_time_).seconds() > params_.aruco_timeout_threshold;
  }

  std::tuple<double, double, double> computeRelativeError(
      const geometry_msgs::msg::TransformStamped & tf_aruco)
  {
    const auto & p = actual_pose_.pose.position;
    const auto & t = tf_aruco.transform.translation;
    return {t.x - p.x, t.y - p.y, t.z - p.z};
  }
};

}  // namespace precision_landing_plugin_pid

PLUGINLIB_EXPORT_CLASS(precision_landing_plugin_pid::Plugin,
                       precision_landing_base::PrecisionLandingBase)
