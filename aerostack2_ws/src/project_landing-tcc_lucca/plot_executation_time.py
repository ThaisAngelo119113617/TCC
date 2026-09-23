#!/usr/bin/env python3
"""
plot_execution_time.py

Gera um grafico 2x2 comparando o tempo de downsample vs PCA para
diferentes valores de n_search_points, no mesmo estilo da Figura 7
de Loureiro et al. (2021): eixo X = tempo de simulacao (s), eixo Y
esquerdo = tempo de downsample (ms, azul), eixo Y direito = tempo
de PCA (ms, laranja).

Como rodar:
    python3 plot_execution_time.py \
        --csv20 logs_percepcao/timing_20.csv \
        --csv30 logs_percepcao/timing_30.csv \
        --csv50 logs_percepcao/timing_50.csv \
        --csv100 logs_percepcao/timing_100.csv \
        --output execution_time_comparison.png
"""

import argparse
import csv
import matplotlib.pyplot as plt


def ler_csv(path):
    tempos_sim, downsample, pca = [], [], []
    with open(path, newline='') as f:
        reader = csv.DictReader(f)
        for row in reader:
            tempos_sim.append(float(row['sim_time_s']))
            downsample.append(float(row['downsample_time_ms']))
            pca.append(float(row['pca_time_ms']))
    return tempos_sim, downsample, pca


def plot_subplot(ax, tempos_sim, downsample, pca, titulo):
    ax2 = ax.twinx()

    ax.plot(tempos_sim, downsample, color='tab:blue', label='Downsample Time')
    ax2.plot(tempos_sim, pca, color='tab:orange', label='PCA Time')

    ax.set_xlabel('Simulation Time (s)')
    ax.set_ylabel('Downsample Time (ms)', color='tab:blue')
    ax2.set_ylabel('PCA Time (ms)', color='tab:orange')
    ax.tick_params(axis='y', labelcolor='tab:blue')
    ax2.tick_params(axis='y', labelcolor='tab:orange')

    lines1, labels1 = ax.get_legend_handles_labels()
    lines2, labels2 = ax2.get_legend_handles_labels()
    ax.legend(lines1 + lines2, labels1 + labels2, loc='upper right', fontsize=8)

    ax.set_title(titulo, fontsize=10)
    ax.grid(True, alpha=0.3)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--csv20', required=True)
    parser.add_argument('--csv30', required=True)
    parser.add_argument('--csv50', required=True)
    parser.add_argument('--csv100', required=True)
    parser.add_argument('--r_max', type=float, default=0.8,
                         help='valor de r_max usado no experimento, para o titulo (metros)')
    parser.add_argument('--output', default='execution_time_comparison.png')
    args = parser.parse_args()

    configs = [
        (args.csv20, 20, '(a)'),
        (args.csv30, 30, '(b)'),
        (args.csv50, 50, '(c)'),
        (args.csv100, 100, '(d)'),
    ]

    fig, axes = plt.subplots(2, 2, figsize=(11, 8))
    axes_flat = axes.flatten()

    for ax, (csv_path, n_points, letra) in zip(axes_flat, configs):
        tempos_sim, downsample, pca = ler_csv(csv_path)
        titulo = f'{letra} Time for {n_points} search points and maximum {args.r_max} m for radius.'
        plot_subplot(ax, tempos_sim, downsample, pca, titulo)

    fig.suptitle('Comparison between downsampling and PCA execution time', fontsize=12)
    fig.tight_layout(rect=[0, 0, 1, 0.96])
    fig.savefig(args.output, dpi=200)
    print(f'Grafico salvo em: {args.output}')


if __name__ == '__main__':
    main()