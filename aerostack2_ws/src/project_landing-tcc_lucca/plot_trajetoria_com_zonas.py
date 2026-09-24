#!/usr/bin/env python3"""plot_trajetoria_com_zonas.py Reaproveita o CSV salvo por missao_hipotrocoide_pi.py e sobrepõe:  - as áreas de pouso candidatas (ground_truth_zones), como retângulos    tracejados, no estilo da figura de referência ("possible landing spots")  - a posição onde o drone realmente pousou (marcador 'x'), se informada  - qual zona corresponde ao pouso (a mais próxima do ponto de pouso) Como rodar:    python3 plot_trajetoria_com_zonas.py \        --csv trajetoria_hipotrocoide.csv \        --pouso -3.05 -2.20 Se você ainda não tiver a posição de pouso, rode sem --pouso: o gráficosai só com a trajetória e as zonas candidatas."""

import argparse
import csv
import json
import math
import os

import matplotlib
matplotlib.use('Agg')  # sem display -- só salva arquivo
import matplotlib.pyplot as plt
from matplotlib.patches import Rectangle

# ---------------- Zonas candidatas (ground truth) ----------------
# GROUND_TRUTH_ZONES = [
#     {'center': (-4.28,  4.28), 'size': (1.76, 1.76)},
#     {'center': (-3.11, -2.17), 'size': (1.76, 1.76)},
#     {'center': ( 3.34,  1.35), 'size': (1.76, 1.76)},
# ]
GROUND_TRUTH_ZONES = [
    {'center': (-5.5, -5.5), 'size': (2.5, 2.5), 'nome': 'Zona A (aceita)'},
    {'center': (6.0, 0.0),   'size': (1.0, 1.0), 'nome': 'Zona B (aceita)'},
    {'center': (0.0, -6.0),  'size': (1.0, 1.0), 'nome': 'Zona C (rampa, deve ser rejeitada)'},
]


def carregar_csv(csv_path):
    t, x, y, xd, yd = [], [], [], [], []
    with open(csv_path, newline='') as f:
        reader = csv.DictReader(f)
        for row in reader:
            t.append(float(row['t']))
            x.append(float(row['x_real']))
            y.append(float(row['y_real']))
            xd.append(float(row['x_desejado']))
            yd.append(float(row['y_desejado']))
    return t, x, y, xd, yd


def carregar_pouso_info(json_path):
    """Le o pouso_info.json salvo pela missao. Retorna (pouso_real, candidato) ou (None, None)."""
    if not os.path.exists(json_path):
        return None, None
    with open(json_path) as f:
        info = json.load(f)
    pouso_real = None
    if info.get('pouso_real'):
        pouso_real = (info['pouso_real']['x'], info['pouso_real']['y'])
    candidato = None
    if info.get('candidato'):
        candidato = (info['candidato']['x'], info['candidato']['y'])
    return pouso_real, candidato


def zona_mais_proxima(pouso, zonas):
    """Retorna o índice da zona cujo centro está mais perto do ponto de pouso."""
    px, py = pouso
    melhor_i, melhor_d = None, math.inf
    for i, z in enumerate(zonas):
        cx, cy = z['center']
        d = math.hypot(px - cx, py - cy)
        if d < melhor_d:
            melhor_i, melhor_d = i, d
    return melhor_i, melhor_d


def plotar(csv_path, png_path, pouso=None, zonas=GROUND_TRUTH_ZONES):
    t, x, y, xd, yd = carregar_csv(csv_path)

    fig, ax = plt.subplots(figsize=(7, 7))

    # Trajetória (mesmo estilo do script original)
    ax.plot(xd, yd, '--', label='Trajetoria desejada (hipotrocoide)', alpha=0.7)
    ax.plot(x, y, '-', label='Trajetoria real do drone')
    ax.scatter([x[0]], [y[0]], color='green', label='Inicio', zorder=5)
    ax.scatter([x[-1]], [y[-1]], color='red', label='Fim da varredura', zorder=5)

    # Zonas candidatas -- retângulos tracejados, estilo "possible landing spots"
    zona_pouso_idx = None
    if pouso is not None:
        zona_pouso_idx, dist = zona_mais_proxima(pouso, zonas)
        print(f'[plot] Pouso mais próximo da zona {zona_pouso_idx} '
              f'(centro {zonas[zona_pouso_idx]["center"]}), distância {dist:.2f} m')

    for i, z in enumerate(zonas):
        cx, cy = z['center']
        w, h = z['size']
        rect = Rectangle((cx - w / 2, cy - h / 2), w, h,
                          linestyle='--', edgecolor='red', facecolor='none',
                          linewidth=1.5,
                          label='Areas candidatas' if i == 0 else None)
        ax.add_patch(rect)
        # marca a zona onde o drone pousou com um preenchimento leve
        if i == zona_pouso_idx:
            rect_fill = Rectangle((cx - w / 2, cy - h / 2), w, h,
                                   facecolor='red', alpha=0.15, edgecolor='none')
            ax.add_patch(rect_fill)

    # Ponto de pouso real -- marcador 'x', estilo "spot detected"
    if pouso is not None:
        ax.scatter([pouso[0]], [pouso[1]], color='black', marker='x', s=120,
                   linewidths=2.5, label='Pouso detectado', zorder=6)

    ax.set_xlabel('x (m)')
    ax.set_ylabel('y (m)')
    ax.set_title('Trajetoria real vs desejada + areas de pouso candidatas')
    ax.axis('equal')
    ax.grid(True)
    ax.legend(loc='upper right', fontsize=9)

    fig.savefig(png_path, dpi=150)
    print(f'[plot] Grafico salvo em {png_path}')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--csv', default='trajetoria_hipotrocoide.csv',
                         help='CSV salvo por missao_hipotrocoide_pi.py')
    parser.add_argument('--out', default='trajetoria_com_zonas.png',
                         help='Caminho do PNG de saida')
    parser.add_argument('--pouso', nargs=2, type=float, default=None,
                         metavar=('X', 'Y'),
                         help='Sobrescreve manualmente a posicao de pouso '
                              '(por padrao, le automaticamente de pouso_info.json)')
    parser.add_argument('--pouso-json', default=None,
                         help='Caminho do pouso_info.json (por padrao, mesma pasta do --csv)')
    args = parser.parse_args()

    if args.pouso:
        pouso = tuple(args.pouso)
    else:
        json_path = args.pouso_json or os.path.join(
            os.path.dirname(os.path.abspath(args.csv)) or '.', 'pouso_info.json')
        pouso, candidato = carregar_pouso_info(json_path)
        if pouso is None:
            print(f'[plot] Aviso: nao achei {json_path} nem --pouso foi passado. '
                  f'Grafico sai sem marcador de pouso.')
        elif candidato and (round(pouso[0], 2), round(pouso[1], 2)) != (round(candidato[0], 2), round(candidato[1], 2)):
            print(f'[plot] Nota: pouso real ({pouso[0]:.2f}, {pouso[1]:.2f}) ficou um pouco '
                  f'diferente do candidato escolhido ({candidato[0]:.2f}, {candidato[1]:.2f}).')

    plotar(args.csv, args.out, pouso=pouso)


if __name__ == '__main__':
    main()