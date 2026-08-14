# #!/usr/bin/env python3
# """
# Missão simples para testar a conexão básica Aerostack2 <-> PX4.

# O que este script faz, passo a passo:
#     1. Inicializa o ROS2 (rclpy)
#     2. Conecta ao drone via Aerostack2 (as2_python_api)
#     3. Arma os motores
#     4. Muda para modo offboard (controle via software, não manual)
#     5. Decola até 2 metros de altura
#     6. Paira (fica parado no ar) por alguns segundos
#     7. Pousa
#     8. Desarma
#     9. Encerra o ROS2

# NÃO usa árvore de comportamento, NÃO usa detecção de ArUco.
# Serve só para confirmar que a camada Aerostack2 <-> PX4 está
# respondendo a comandos básicos de voo.

# Como rodar (dentro do container, com a simulação já aberta em outra janela):
#     cd ~/aerostack2_ws/src/aerostack2_ws/src/project_landing-tcc_lucca
#     python3 missao_simples.py
# """

# from time import sleep

# import rclpy
# from as2_python_api.drone_interface import DroneInterface

# # Precisa bater com o namespace usado no world.yaml / launch_sim.bash
# DRONE_NAMESPACE = 'x500_px4'
# ALTURA_DECOLAGEM = 2.0  # metros
# TEMPO_PAIRADO = 8.0     # segundos parado no ar antes de pousar


# def main():
#     print('[missao_simples] Inicializando ROS2...')
#     rclpy.init()

#     print(f'[missao_simples] Conectando ao drone "{DRONE_NAMESPACE}"...')
#     drone = DroneInterface(
#         drone_id=DRONE_NAMESPACE,
#         use_sim_time=True,
#         verbose=True
#     )

#     try:
#         print('[missao_simples] Armando motores...')
#         drone.arm()
#         sleep(1.0)

#         print('[missao_simples] Ativando modo offboard...')
#         drone.offboard()
#         sleep(1.0)

#         print(f'[missao_simples] Decolando até {ALTURA_DECOLAGEM} m...')
#         drone.takeoff(height=ALTURA_DECOLAGEM, speed=0.5)
#         print('[missao_simples] Decolagem concluída!')

#         print(f'[missao_simples] Pairando por {TEMPO_PAIRADO} s...')
#         sleep(TEMPO_PAIRADO)

#         print('[missao_simples] Pousando...')
#         drone.land(speed=0.3)
#         print('[missao_simples] Pouso concluído!')

#         print('[missao_simples] Aguardando confirmação de pouso...')
#         sleep(2.0)

#         print('[missao_simples] Desarmando (se ainda não estiver desarmado)...')
#         try:
#             drone.disarm()
#         except Exception as e:
#             print(f'[missao_simples] Aviso: desarme retornou erro (provavelmente '
#                   f'já estava desarmado automaticamente após o pouso): {e}')

#     finally:
#         print('[missao_simples] Encerrando conexão com o drone...')
#         drone.shutdown()
#         rclpy.shutdown()
#         print('[missao_simples] Missão finalizada.')


# if __name__ == '__main__':
#     main()


#!/usr/bin/env python3
"""
Missão simples para testar a conexão básica Aerostack2 <-> PX4.

Decola, paira, pousa. NÃO usa árvore de comportamento, NÃO usa
detecção de ArUco ou LiDAR -- serve só para confirmar que a camada
Aerostack2 <-> PX4 está respondendo a comandos básicos de voo.

Como rodar (dentro do container, com a simulação já aberta):
    cd ~/aerostack2_ws/src/aerostack2_ws/src/project_landing-tcc_lucca
    python3 missao_simples.py
"""

from mission_base import SimpleMission

DRONE_NAMESPACE = 'x500_px4'
ALTURA_DECOLAGEM = 10.0  # metros
TEMPO_PAIRADO = 15.0     # segundos parado no ar antes de pousar


def main():
    with SimpleMission(DRONE_NAMESPACE) as mission:
        mission.takeoff(ALTURA_DECOLAGEM)
        mission.hover(TEMPO_PAIRADO)
        mission.land()


if __name__ == '__main__':
    main()