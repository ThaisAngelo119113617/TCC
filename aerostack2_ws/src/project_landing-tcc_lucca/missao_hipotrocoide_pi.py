#!/usr/bin/env python3
"""
missao_hipotrocoide_pi.py

Missao completa: decola, reseta a memoria de percepcao, executa uma
trajetoria de varredura em hipotrocoide (centrada na posicao de decolagem)
usando um controlador PI com feedforward (Cap. 11 do PDF de modelagem/
controle), salva um log/grafico da trajetoria real vs desejada, e so
entao pousa na melhor area detectada.

Como rodar:
    Terminal 1: simulacao (launch_sim_simples.bash ou equivalente)
    Terminal 2: python3 candidate_generation.py (fica rodando o tempo todo)
    Terminal 3: python3 missao_hipotrocoide_pi.py
"""

from mission_base import SimpleMission
from fractions import Fraction
import math
import time
import csv
import matplotlib
matplotlib.use('Agg')  # sem display -- so salva arquivo, roda dentro do container
import matplotlib.pyplot as plt
import json

DRONE_NAMESPACE = 'x500_px4'
ALTURA_DECOLAGEM = 5.0
TEMPO_BUSCA = 30.0  # segundos esperando candidato estavel apos a varredura

# ---------------- Hipotrocoide ----------------
R, r, d = 5.0, 2.0, 3.0      # raio fixo, raio rolante, offset do ponto tracador (m)
OMEGA = 0.07                 # velocidade angular (rad/s) -- reduzido p/ ficar mais devagar
K = (R - r) / r

_frac = Fraction(K).limit_denominator(20)
T_VARREDURA = 2 * math.pi * _frac.denominator / OMEGA  # tempo p/ fechar a curva

# ---------------- Controlador PI + feedforward (Cap. 11) ----------------
L0, KP, KI, TAU_P = 1.0, 0.3, 0.08, 1.0  # comeca so com P, sem I -- ajustar depois
H = 0.05  # periodo do loop de controle (20 Hz)
CENTRO_X, CENTRO_Y = 0.0, 0.0


def referencia_hipotrocoide(t, x0=0.0, y0=0.0):
    """Retorna ((x_d, y_d), (vx_d, vy_d), (ax_d, ay_d)) no instante t,
    com a curva centrada em (x0, y0)."""
    x_d = (R - r) * math.cos(OMEGA * t) + d * math.cos(K * OMEGA * t) + x0
    y_d = (R - r) * math.sin(OMEGA * t) - d * math.sin(K * OMEGA * t) + y0

    vx_d = -(R - r) * OMEGA * math.sin(OMEGA * t) - d * K * OMEGA * math.sin(K * OMEGA * t)
    vy_d =  (R - r) * OMEGA * math.cos(OMEGA * t) - d * K * OMEGA * math.cos(K * OMEGA * t)

    ax_d = -(R - r) * OMEGA**2 * math.cos(OMEGA * t) - d * K**2 * OMEGA**2 * math.cos(K * OMEGA * t)
    ay_d = -(R - r) * OMEGA**2 * math.sin(OMEGA * t) + d * K**2 * OMEGA**2 * math.sin(K * OMEGA * t)

    return (x_d, y_d), (vx_d, vy_d), (ax_d, ay_d)


def salvar_log_e_grafico(log_t, log_x, log_y, log_xd, log_yd,
                          csv_path='trajetoria_hipotrocoide.csv',
                          png_path='trajetoria_hipotrocoide.png'):
    with open(csv_path, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow(['t', 'x_real', 'y_real', 'x_desejado', 'y_desejado'])
        writer.writerows(zip(log_t, log_x, log_y, log_xd, log_yd))
    print(f'[missao] Log salvo em {csv_path}')

    plt.figure(figsize=(7, 7))
    plt.plot(log_xd, log_yd, '--', label='Trajetoria desejada (hipotrocoide)', alpha=0.7)
    plt.plot(log_x, log_y, '-', label='Trajetoria real do drone')
    plt.scatter([log_x[0]], [log_y[0]], color='green', label='Inicio', zorder=5)
    plt.scatter([log_x[-1]], [log_y[-1]], color='red', label='Fim', zorder=5)
    plt.xlabel('x (m)')
    plt.ylabel('y (m)')
    plt.title('Trajetoria real vs desejada')
    plt.axis('equal')
    plt.legend()
    plt.grid(True)
    plt.savefig(png_path, dpi=150)
    print(f'[missao] Grafico salvo em {png_path}')


def salvar_pouso_info(candidato, pose_final, json_path='pouso_info.json'):
    info = {
        'candidato': {'x': candidato[0], 'y': candidato[1], 'z': candidato[2]} if candidato else None,
        'pouso_real': {'x': pose_final[0], 'y': pose_final[1]},
    }
    with open(json_path, 'w') as f:
        json.dump(info, f, indent=2)
    print(f'[missao] Info de pouso salva em {json_path}')



def main():
    with SimpleMission(DRONE_NAMESPACE) as mission:
        mission.takeoff(ALTURA_DECOLAGEM)
        mission._iniciar_pose_cache()
        mission.drone.load_module('motion_reference_handler')

        print(f'[DEBUG] Home position capturada: {mission.home_position}')

        mission.reset_perception()

        # Ponto onde a hipotrocoide comeca (t=0), ja centrada na area de busca
        x_inicio = CENTRO_X + (R - r) + d
        y_inicio = CENTRO_Y
        print(f'[missao] Indo para o ponto inicial da hipotrocoide ({x_inicio:.2f}, {y_inicio:.2f})...')
        mission.go_to(x_inicio, y_inicio, ALTURA_DECOLAGEM)

        print(f'[missao] Iniciando varredura por hipotrocoide (duracao ~{T_VARREDURA:.1f}s)...')
        int_sigma_x = 0.0
        int_sigma_y = 0.0
        t0 = time.time()

        log_t, log_x, log_y, log_xd, log_yd = [], [], [], [], []

        while (time.time() - t0) < T_VARREDURA:
            t = time.time() - t0
            (x_d, y_d), (vx_d, vy_d), (ax_d, ay_d) = referencia_hipotrocoide(t, x0=CENTRO_X, y0=CENTRO_Y)

            # pos = mission._get_pose_now()
            pos = mission._get_pose_cached()
            vel = mission.drone.speed

            ex, ey = pos[0] - x_d, pos[1] - y_d
            dex, dey = vel[0] - vx_d, vel[1] - vy_d

            sigma_x = dex + L0 * ex
            sigma_y = dey + L0 * ey
            int_sigma_x += H * sigma_x
            int_sigma_y += H * sigma_y

            uff_x = TAU_P * ax_d + vx_d
            uff_y = TAU_P * ay_d + vy_d

            vx_cmd = uff_x + L0 * (TAU_P * L0 - 1) * ex - KP * sigma_x - KI * int_sigma_x
            vy_cmd = uff_y + L0 * (TAU_P * L0 - 1) * ey - KP * sigma_y - KI * int_sigma_y

            mission.send_speed(vx_cmd, vy_cmd, 0.0)

            log_t.append(t)
            log_x.append(pos[0])
            log_y.append(pos[1])
            log_xd.append(x_d)
            log_yd.append(y_d)

            time.sleep(H)

        mission.send_speed(0.0, 0.0, 0.0)
        mission.drone.motion_ref_handler.hover()
        time.sleep(1.5)  # deixa o modo de controle assentar antes do go_to/land
        print('[missao] Varredura concluida.')

        salvar_log_e_grafico(log_t, log_x, log_y, log_xd, log_yd)

        candidato = mission.wait_for_landing_candidate(timeout=TEMPO_BUSCA)

        if candidato is None:
            print('[missao] Nenhuma area segura encontrada. Pousando na origem.')
            mission.go_home()
            mission.land()

            pose_final = mission._get_pose_now()
            salvar_pouso_info(None, pose_final)
        else:
            x, y, z_candidato = candidato
            print(f'[missao] Indo verificar area em ({x:.2f}, {y:.2f})')
            mission.go_to(x, y, ALTURA_DECOLAGEM)

            pose_apos_goto = mission._get_pose_now()
            print(f'[DEBUG] Alvo: ({x:.2f}, {y:.2f}) | Pose apos go_to: {pose_apos_goto}')

            print('[missao] Pairando 5s sobre a area candidata antes de pousar...')
            for i in range(5):
                time.sleep(1.0)
                pose_hover = mission._get_pose_now()
                print(f'[DEBUG] Pose durante hover (s={i+1}): {pose_hover}')

            print('[missao] Pouso confirmado.')
            mission.land()

            pose_apos_land = mission._get_pose_now()
            print(f'[DEBUG] Pose apos land: {pose_apos_land}')
            salvar_pouso_info(candidato, pose_apos_land)


if __name__ == '__main__':
    main()