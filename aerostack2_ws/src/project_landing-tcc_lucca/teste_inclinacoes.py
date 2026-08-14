#!/usr/bin/env python3
"""
teste_inclinacoes.py

Testa o pipeline de deteccao de candidatos (Estagio 1) contra planos
inclinados sinteticos, LENDO DIRETO DO GAZEBO via gz-transport,
sem ROS2/ponte no meio (elimina os problemas de bridge/QoS/tipo).

Uso:
    1. Gera e abre o mundo:  ./gerar_mundo_inclinado.bash 10 && gz sim teste_plano_inclinado.sdf
    2. Aperta play no Gazebo
    3. Roda este script:     python3 teste_inclinacoes.py
    4. Ctrl+C para parar
"""

import struct
import sys
import time

import numpy as np
import open3d as o3d

# Tenta achar a versao certa do pacote de mensagens gz automaticamente
PointCloudPacked = None
for v in (10, 11, 9, 12, 8):
    try:
        module = __import__(f'gz.msgs{v}.pointcloud_packed_pb2', fromlist=['PointCloudPacked'])
        PointCloudPacked = module.PointCloudPacked
        print(f'[teste_inclinacoes] Usando gz.msgs{v}')
        break
    except ModuleNotFoundError:
        continue

if PointCloudPacked is None:
    print('ERRO: nao achei gz.msgsN.pointcloud_packed_pb2 em nenhuma versao testada.')
    print('Roda: python3 -c "import gz; print(gz.__path__)" e me manda o resultado.')
    sys.exit(1)

from gz.transport13 import Node  # noqa: E402


# --- Parametros do pipeline (mesmos do candidate_generation.py) ---
VOXEL_SIZE = 0.15
NORMAL_SEARCH_RADIUS = 0.5
MAX_INCLINATION_DEG = 15.0
MAX_ROUGHNESS = 0.10
GRID_CELL_SIZE = 1.0
GRID_MAX_HEIGHT_RANGE = 0.30

# --- Transformacao fixa sensor->mundo (sabemos pela pose do SDF de teste:
#     z=2m, pitch=90 graus -- nao precisa de TF, e um rig estatico conhecido) ---
SENSOR_Z = 2.0
PITCH_RAD = np.pi / 2  # 90 graus
COS_P, SIN_P = np.cos(PITCH_RAD), np.sin(PITCH_RAD)
R_SENSOR_TO_WORLD = np.array([
    [COS_P, 0, SIN_P],
    [0,     1, 0],
    [-SIN_P, 0, COS_P],
])
T_SENSOR_TO_WORLD = np.array([0.0, 0.0, SENSOR_Z])


def decode_pointcloud(msg) -> np.ndarray:
    """Decodifica x,y,z de uma mensagem PointCloudPacked, usando os
    offsets declarados nos proprios campos da mensagem (generico,
    nao assume ordem fixa)."""
    field_offset = {f.name: f.offset for f in msg.field}
    if not all(k in field_offset for k in ('x', 'y', 'z')):
        return np.empty((0, 3))

    n_points = msg.width * msg.height
    point_step = msg.point_step
    data = msg.data

    ox, oy, oz = field_offset['x'], field_offset['y'], field_offset['z']
    points = np.empty((n_points, 3), dtype=np.float32)
    for i in range(n_points):
        base = i * point_step
        x, y, z = struct.unpack_from('<fff', data, base)  # assume offsets 0,4,8 contiguos
        points[i] = (x, y, z)

    valid = np.isfinite(points).all(axis=1)
    return points[valid]


def grid_coarse_filter(points: np.ndarray) -> np.ndarray:
    cell = GRID_CELL_SIZE
    cell_idx = np.floor(points[:, :2] / cell).astype(np.int64)
    keys = cell_idx[:, 0] * 1_000_000 + cell_idx[:, 1]
    unique_keys, inverse, counts = np.unique(keys, return_inverse=True, return_counts=True)

    z = points[:, 2]
    z_min = np.full(unique_keys.shape[0], np.inf)
    z_max = np.full(unique_keys.shape[0], -np.inf)
    np.minimum.at(z_min, inverse, z)
    np.maximum.at(z_max, inverse, z)

    height_range = z_max - z_min
    cell_ok = height_range <= GRID_MAX_HEIGHT_RANGE
    return cell_ok[inverse]


def pca_normal_and_roughness(points: np.ndarray):
    pcd = o3d.geometry.PointCloud()
    pcd.points = o3d.utility.Vector3dVector(points)
    kdtree = o3d.geometry.KDTreeFlann(pcd)

    n = points.shape[0]
    normals = np.full((n, 3), np.nan)
    roughness = np.full(n, np.nan)
    min_neighbors = 4

    for i in range(n):
        _, idx, _ = kdtree.search_radius_vector_3d(points[i], NORMAL_SEARCH_RADIUS)
        if len(idx) < min_neighbors:
            continue
        neighborhood = points[np.asarray(idx)]
        centroid = neighborhood.mean(axis=0)
        centered = neighborhood - centroid
        cov = (centered.T @ centered) / centered.shape[0]
        eigvals, eigvecs = np.linalg.eigh(cov)
        normal = eigvecs[:, 0]
        if normal[2] < 0:
            normal = -normal
        normals[i] = normal
        roughness[i] = np.sqrt(max(eigvals[0], 0.0))

    return normals, roughness


def process_cloud(msg):
    points_sensor = decode_pointcloud(msg)
    if points_sensor.shape[0] == 0:
        print('[teste_inclinacoes] nuvem vazia, ignorando')
        return

    points_world = points_sensor @ R_SENSOR_TO_WORLD.T + T_SENSOR_TO_WORLD

    pcd = o3d.geometry.PointCloud()
    pcd.points = o3d.utility.Vector3dVector(points_world)
    pcd_down = pcd.voxel_down_sample(voxel_size=VOXEL_SIZE)
    points_down = np.asarray(pcd_down.points)
    if points_down.shape[0] < 3:
        print(f'[teste_inclinacoes] so {points_down.shape[0]} pontos apos voxel, pulando')
        return

    coarse_mask = grid_coarse_filter(points_down)
    points_coarse = points_down[coarse_mask]
    if points_coarse.shape[0] < 3:
        print('[teste_inclinacoes] nada sobrou do filtro coarse')
        return

    normals, roughness = pca_normal_and_roughness(points_coarse)
    valid = ~np.isnan(roughness)
    if not valid.any():
        print('[teste_inclinacoes] PCA nao gerou nenhuma normal valida')
        return

    vertical = np.array([0.0, 0.0, 1.0])
    cos_angle = np.clip(np.abs(normals[valid] @ vertical), -1.0, 1.0)
    angle_deg = np.degrees(np.arccos(cos_angle))
    rough_valid = roughness[valid]

    safe_mask = (angle_deg <= MAX_INCLINATION_DEG) & (rough_valid <= MAX_ROUGHNESS)
    n_candidatos = safe_mask.sum()

    print(
        f'pontos brutos: {points_sensor.shape[0]} | apos voxel: {points_down.shape[0]} | '
        f'apos coarse: {points_coarse.shape[0]} || '
        f'angulo min/mediana/max: {angle_deg.min():.1f}/{np.median(angle_deg):.1f}/{angle_deg.max():.1f} graus | '
        f'rugosidade mediana: {np.median(rough_valid)*100:.1f}cm || '
        f'CANDIDATOS: {n_candidatos}/{valid.sum()}'
    )


def main():
    node = Node()
    topic = '/lidar/points'
    ok = node.subscribe(PointCloudPacked, topic, process_cloud)
    if not ok:
        print(f'ERRO: nao consegui inscrever em {topic}')
        sys.exit(1)

    print(f'[teste_inclinacoes] Inscrito em {topic} via gz-transport. Ctrl+C para parar.')
    try:
        while True:
            time.sleep(1.0)
    except KeyboardInterrupt:
        print('\n[teste_inclinacoes] Encerrando.')


if __name__ == '__main__':
    main()
    