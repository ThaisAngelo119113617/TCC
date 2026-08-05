#!/bin/bash
# Versao simplificada de launch_sim.bash
# Sobe a simulacao SEM PX4 real: usa as2_platform_gazebo em vez de
# PX4-Autopilot + MicroXRCEAgent + as2_platform_pixhawk.
# Mesmo modelo de drone (x500_px4) e mesmo mundo (config/world.yaml).
#
# Uso:
#   ./launch_sim_simples.bash

simulation_config="config/world.yaml"

drones_namespace_comma=$(python3 utils/get_drones.py -p "${simulation_config}" --sep ',')
IFS=',' read -r -a drone_namespaces <<< "$drones_namespace_comma"

namespace="${drone_namespaces[0]}"

tmuxinator start -n "${namespace}" -p tmuxinator/simulation_simples.yaml \
  drone_namespace="${namespace}" \
  simulation_config_file="${simulation_config}" \
  base_launch=true \
  wait

tmux attach-session -t "${namespace}"
