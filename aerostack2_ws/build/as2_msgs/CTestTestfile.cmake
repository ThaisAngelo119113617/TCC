# CMake generated Testfile for 
# Source directory: /home/developer/aerostack2_ws/src/aerostack2-tcc_lucca/as2_msgs
# Build directory: /home/developer/aerostack2_ws/build/as2_msgs
# 
# This file includes the relevant testing commands required for 
# testing this directory and lists subdirectories to be tested as well.
add_test(lint_cmake "/usr/bin/python3" "-u" "/opt/ros/humble/share/ament_cmake_test/cmake/run_test.py" "/home/developer/aerostack2_ws/build/as2_msgs/test_results/as2_msgs/lint_cmake.xunit.xml" "--package-name" "as2_msgs" "--output-file" "/home/developer/aerostack2_ws/build/as2_msgs/ament_lint_cmake/lint_cmake.txt" "--command" "/opt/ros/humble/bin/ament_lint_cmake" "--xunit-file" "/home/developer/aerostack2_ws/build/as2_msgs/test_results/as2_msgs/lint_cmake.xunit.xml")
set_tests_properties(lint_cmake PROPERTIES  LABELS "lint_cmake;linter" TIMEOUT "60" WORKING_DIRECTORY "/home/developer/aerostack2_ws/src/aerostack2-tcc_lucca/as2_msgs" _BACKTRACE_TRIPLES "/opt/ros/humble/share/ament_cmake_test/cmake/ament_add_test.cmake;125;add_test;/opt/ros/humble/share/ament_cmake_lint_cmake/cmake/ament_lint_cmake.cmake;47;ament_add_test;/home/developer/aerostack2_ws/src/aerostack2-tcc_lucca/as2_msgs/CMakeLists.txt;45;ament_lint_cmake;/home/developer/aerostack2_ws/src/aerostack2-tcc_lucca/as2_msgs/CMakeLists.txt;0;")
add_test(xmllint "/usr/bin/python3" "-u" "/opt/ros/humble/share/ament_cmake_test/cmake/run_test.py" "/home/developer/aerostack2_ws/build/as2_msgs/test_results/as2_msgs/xmllint.xunit.xml" "--package-name" "as2_msgs" "--output-file" "/home/developer/aerostack2_ws/build/as2_msgs/ament_xmllint/xmllint.txt" "--command" "/opt/ros/humble/bin/ament_xmllint" "--xunit-file" "/home/developer/aerostack2_ws/build/as2_msgs/test_results/as2_msgs/xmllint.xunit.xml")
set_tests_properties(xmllint PROPERTIES  LABELS "xmllint;linter" TIMEOUT "60" WORKING_DIRECTORY "/home/developer/aerostack2_ws/src/aerostack2-tcc_lucca/as2_msgs" _BACKTRACE_TRIPLES "/opt/ros/humble/share/ament_cmake_test/cmake/ament_add_test.cmake;125;add_test;/opt/ros/humble/share/ament_cmake_xmllint/cmake/ament_xmllint.cmake;50;ament_add_test;/home/developer/aerostack2_ws/src/aerostack2-tcc_lucca/as2_msgs/CMakeLists.txt;47;ament_xmllint;/home/developer/aerostack2_ws/src/aerostack2-tcc_lucca/as2_msgs/CMakeLists.txt;0;")
subdirs("as2_msgs__py")
subdirs("as2_msgs__rs")
