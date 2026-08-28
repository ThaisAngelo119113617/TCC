#!/usr/bin/env python3
"""
Missao completa: decola, reseta a memoria de percepcao, executa uma
trajetoria de varredura sobre o cenario (garantindo que o pipeline
compare as areas candidatas ANTES de decidir), e so entao pousa na
melhor area detectada.

Como rodar:
    Terminal 1: simulacao (launch_sim_simples.bash ou equivalente)
    Terminal 2: python3 candidate_generation.py (fica rodando o tempo todo)
    Terminal 3: python3 missao_pouso_autonomo.py
"""

from mission_base import SimpleMission

DRONE_NAMESPACE = 'x500_px4'
ALTURA_DECOLAGEM = 5.0
TEMPO_BUSCA = 30.0  # segundos esperando candidato estavel apos a varredura
VELOCIDADE_VARREDURA = 0.8  # m/s -- ajustar conforme necessidade

# Trajetoria "cobra" cobrindo o quadrado 15x15m (x,y de -6 a +6, com margem
# de seguranca das bordas do heightmap). Passa perto das 3 zonas planas
# conhecidas: (-4.3,-4.3), (-3.1,+2.1), (+3.4,-1.3) aprox.
WAYPOINTS_VARREDURA = [
    (-6.0, -6.0, ALTURA_DECOLAGEM),
    (-6.0,  0.0, ALTURA_DECOLAGEM),
    (-6.0,  6.0, ALTURA_DECOLAGEM),
    (-2.0,  6.0, ALTURA_DECOLAGEM),
    (-2.0,  0.0, ALTURA_DECOLAGEM),
    (-2.0, -6.0, ALTURA_DECOLAGEM),
    ( 2.0, -6.0, ALTURA_DECOLAGEM),
    ( 2.0,  0.0, ALTURA_DECOLAGEM),
    ( 2.0,  6.0, ALTURA_DECOLAGEM),
    ( 6.0,  6.0, ALTURA_DECOLAGEM),
    ( 6.0,  0.0, ALTURA_DECOLAGEM),
    ( 6.0, -6.0, ALTURA_DECOLAGEM),
]


def main():
    with SimpleMission(DRONE_NAMESPACE) as mission:
        mission.takeoff(ALTURA_DECOLAGEM)

        # Zera a memoria de areas conhecidas -- so a partir daqui as
        # observacoes contam, evitando vies do que foi visto parado
        # em cima do ponto de decolagem.
        mission.reset_perception()

        print('[missao] Iniciando varredura do cenario...')
        for x, y, z in WAYPOINTS_VARREDURA:
            mission.go_to(x, y, z, speed=VELOCIDADE_VARREDURA)
        print('[missao] Varredura concluida.')

        candidato = mission.wait_for_landing_candidate(timeout=TEMPO_BUSCA)

        if candidato is None:
            print('[missao] Nenhuma area segura encontrada. Pousando na origem.')
            mission.go_home()
            mission.land()
        else:
            x, y, z_candidato = candidato
            print(f'[missao] Indo pousar em ({x:.2f}, {y:.2f})')
            mission.go_to(x, y, ALTURA_DECOLAGEM)
            mission.land()


if __name__ == '__main__':
    main()