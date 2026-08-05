#!/bin/bash
# Limpa TUDO relacionado a uma sessão anterior de simulação antes de relançar.
# Rode isso sempre antes de ./launch_sim_simples.bash (ou ./launch_sim.bash)
# para evitar processos fantasma do Gazebo gerando drones duplicados.
#
# Uso (dentro do container):
#   bash limpar_tudo.bash

echo "[limpar_tudo] Matando sessao tmux..."
tmux kill-server 2>/dev/null

echo "[limpar_tudo] Matando processos do Gazebo (gz)..."
pkill -9 -f "gz sim" 2>/dev/null
pkill -9 -f "gz-sim" 2>/dev/null
pkill -9 -f ruby.*gz 2>/dev/null

echo "[limpar_tudo] Matando processos do ros_gz_bridge..."
pkill -9 -f ros_gz_bridge 2>/dev/null
pkill -9 -f parameter_bridge 2>/dev/null

echo "[limpar_tudo] Matando MicroXRCEAgent (se estiver rodando)..."
pkill -9 -f MicroXRCEAgent 2>/dev/null

echo "[limpar_tudo] Matando PX4 SITL (se estiver rodando)..."
pkill -9 -f "bin/px4" 2>/dev/null

echo "[limpar_tudo] Aguardando processos encerrarem..."
sleep 2

echo "[limpar_tudo] Verificando se sobrou algo..."
RESTANTE=$(ps aux | grep -iE "gz sim|gz-sim|ros_gz_bridge|MicroXRCEAgent|bin/px4" | grep -v grep)

if [ -z "$RESTANTE" ]; then
    echo "[limpar_tudo] Tudo limpo! Pode relançar a simulação."
else
    echo "[limpar_tudo] ATENÇÃO: ainda restaram processos:"
    echo "$RESTANTE"
    echo "[limpar_tudo] Pode ser necessário matar manualmente com: kill -9 <PID>"
fi
