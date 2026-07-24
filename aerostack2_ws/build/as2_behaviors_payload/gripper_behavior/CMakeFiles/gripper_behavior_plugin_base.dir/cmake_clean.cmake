file(REMOVE_RECURSE
  "libgripper_behavior_plugin_base.pdb"
  "libgripper_behavior_plugin_base.so"
)

# Per-language clean rules from dependency scanning.
foreach(lang )
  include(CMakeFiles/gripper_behavior_plugin_base.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
