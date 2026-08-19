#!/usr/bin/env python3
"""
Missao completa: decola, procura uma area de pouso valida usando o
Estagio 1 (candidate_generation.py, que precisa estar rodando em
paralelo), voa ate la, e pousa.

Como rodar:
    Terminal 1: simulacao (launch_sim_simples.bash)
    Terminal 2: python3 candidate_generation.py
    Terminal 3: python3 missao_pouso_autonomo.py
"""

from mission_base import SimpleMission

DRONE_NAMESPACE = 'x500_px4'
ALTURA_DECOLAGEM = 5.0
TEMPO_BUSCA = 30.0  # segundos esperando candidato


def main():
    with SimpleMission(DRONE_NAMESPACE) as mission:
        mission.takeoff(ALTURA_DECOLAGEM)

        candidato = mission.wait_for_landing_candidate(timeout=TEMPO_BUSCA)

        if candidato is None:
            print('[missao] Nenhuma area segura encontrada. Pousando na origem.')
            mission.go_home()
            mission.land()

        else:
            x, y, z_candidato = candidato
            print(f'[missao] Indo pousar em ({x:.2f}, {y:.2f})')
            mission.go_to(x, y, ALTURA_DECOLAGEM)  # mantem altitude, so muda x,y
            mission.land()

        # mission.land()


if __name__ == '__main__':
    main()