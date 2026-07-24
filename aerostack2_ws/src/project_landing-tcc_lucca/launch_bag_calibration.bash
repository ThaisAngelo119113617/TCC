#!/bin/bash

usage() {
    echo "usage: $0 [options]"
    echo "  options:"
    echo "      -m: multi agent. Default not set"
    echo "      -n: select drones namespace to launch, values are comma separated. Use \"\" for empty namespace"
    echo "      -s: if set, the simulation will not be launched. Default launch simulation"
    echo "      -g: launch using gnome-terminal instead of tmux. Default not set"
    echo "      -y: launch YOLO node"
}

# Variáveis padrão
swarm="false"
drones_namespace_comma=""
namespace_provided="false"
launch_simulation="true"
use_gnome="false"
mission=""
sim="false"

TEMP=$(getopt -o "mn:sgyt:e:123" -n "$0" -- "$@")
if [ $? != 0 ]; then
    echo "Erro ao analisar os argumentos." >&2
    usage
    exit 1
fi

eval set -- "$TEMP"

while true; do
    case "$1" in
        -m)
            swarm="true"
            shift
            ;;
        -n)
            namespace_provided="true"
            drones_namespace_comma="$2"
            shift 2
            ;;
        -s)
            launch_simulation="false"
            shift
            ;;
        -g)
            use_gnome="true"
            shift
            ;;
        --)
            shift
            break
            ;;
        *)
            echo "Erro interno!"
            exit 1
            ;;
    esac
done

# Missão padrão
if [ -z "$mission" ]; then
    mission="1"
fi

# Arquivo de mundo
if [[ "$swarm" == "true" ]]; then
    simulation_config="config/world_swarm.yaml"
else
    simulation_config="config/world.yaml"
fi

# Namespaces
# If -n was NOT provided, load from world file.
# If -n "" was provided, keep empty namespace intentionally.
if [[ "$namespace_provided" == "false" ]]; then
    drones_namespace_comma=$(python3 utils/get_drones.py -p "$simulation_config" --sep ',')
fi

# Convert comma-separated list into array.
# Special case: intentional empty namespace.
if [[ "$namespace_provided" == "true" && -z "$drones_namespace_comma" ]]; then
    drone_namespaces=("")
else
    IFS=',' read -r -a drone_namespaces <<< "$drones_namespace_comma"
fi

# tmux vs gnome-terminal
tmuxinator_mode="start"
tmuxinator_end="wait"

first_namespace="${drone_namespaces[0]}"
first_session_name="${first_namespace:-default}"

tmp_file="/tmp/as2_project_launch_${first_session_name}.txt"

if [[ "$use_gnome" == "true" ]]; then
    tmuxinator_mode="debug"
fi

# Lançamento por namespace
for namespace in "${drone_namespaces[@]}"; do
    base_launch="false"

    if [[ "$namespace" == "${drone_namespaces[0]}" && "$launch_simulation" == "true" ]]; then
        base_launch="true"
    fi

    # tmuxinator cannot use an empty session name
    session_name="${namespace:-default}"

    if [[ "$use_gnome" == "true" ]]; then
        tmuxinator "$tmuxinator_mode" \
            -n "$session_name" \
            -p tmuxinator/hardware_calibration.yaml \
            "drone_namespace=$namespace" \
            "simulation_config_file=$simulation_config" \
            "base_launch=$base_launch" \
            "mission=$mission" \
            "sim=$sim" \
            > "$tmp_file"

        python3 utils/tmuxinator_to_genome.py -p "$tmp_file"
        wait
    else
        tmuxinator "$tmuxinator_mode" \
            -n "$session_name" \
            -p tmuxinator/hardware_calibration.yaml \
            "drone_namespace=$namespace" \
            "simulation_config_file=$simulation_config" \
            "base_launch=$base_launch" \
            "mission=$mission" \
            "sim=$sim" \
            wait
    fi

    sleep 0.1
done

# Pós-processamento
if [[ "$use_gnome" == "false" ]]; then
    tmux attach-session -t "$first_session_name"
elif [[ -f "$tmp_file" ]]; then
    rm "$tmp_file"
fi