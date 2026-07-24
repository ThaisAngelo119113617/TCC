# CMake generated Testfile for 
# Source directory: /home/developer/aerostack2_ws/build/as2_behaviors_trajectory_generation/_deps/dynamic_trajectory_generator-src/subpackages/mav_trajectory_generation
# Build directory: /home/developer/aerostack2_ws/build/as2_behaviors_trajectory_generation/_deps/dynamic_trajectory_generator-build/subpackages/mav_trajectory_generation
# 
# This file includes the relevant testing commands required for 
# testing this directory and lists subdirectories to be tested as well.
add_test(TRAJ_TEST "test_polynomial")
set_tests_properties(TRAJ_TEST PROPERTIES  _BACKTRACE_TRIPLES "/home/developer/aerostack2_ws/build/as2_behaviors_trajectory_generation/_deps/dynamic_trajectory_generator-src/subpackages/mav_trajectory_generation/CMakeLists.txt;82;add_test;/home/developer/aerostack2_ws/build/as2_behaviors_trajectory_generation/_deps/dynamic_trajectory_generator-src/subpackages/mav_trajectory_generation/CMakeLists.txt;0;")
subdirs("../../../glog-build")
subdirs("../../../nlopt-build")
