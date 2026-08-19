#!/usr/bin/env python3
"""
candidate_generation_node.py

Estagio 1 do pipeline de percepcao do TCC: geracao de candidatos
geometricos de zona de pouso segura, usando apenas o LiDAR.

Pipeline completo planejado (Escolha_Tecnicas_Pipeline_TCC.md):
    1. Voxel Grid (downsampling)                              [OK - este incremento]
    2. Grid-map 2D de elevacao (filtro coarse)                 [proximo incremento]
    3. PCA em vizinhanca esferica, raio expansivel (Octree/KDTree) [OK - este incremento, raio fixo por ora]
    4. Validacao geometrica: inclinacao                        [OK - este incremento]
                            + rugosidade (RMS ao plano)         [proximo incremento]
                            + tamanho (transformada de distancia) [proximo incremento]
    5. Score ponderado (spotgrade)                              [proximo incremento]

Este e o PRIMEIRO INCREMENTO: pipeline minimo (voxel + PCA + inclinacao),
publicando o resultado filtrado em /perception/landing_candidates/points
para visualizacao/validacao no RViz antes de adicionar o resto.

Como rodar (com a simulacao ja de pe):
    ros2 run <pacote> candidate_generation_node.py
    # ou direto:
    python3 candidate_generation_node.py
"""

import numpy as np
import open3d as o3d
import rclpy
from rclpy.node import Node
from rclpy.qos import qos_profile_sensor_data
from sensor_msgs.msg import PointCloud2, PointField
from sensor_msgs_py import point_cloud2 as pc2
from std_msgs.msg import Header
from tf2_ros import Buffer, TransformListener, LookupException, ExtrapolationException
# from tf2_sensor_msgs.tf2_sensor_msgs import do_transform_cloud
from geometry_msgs.msg import PointStamped

class CandidateGenerationNode(Node):
    """No ROS2 que gera candidatos geometricos de pouso a partir do LiDAR."""

    def __init__(self):
        super().__init__('candidate_generation_node')

        # --- Parametros (ajustaveis via ROS2 param, defaults do doc de escolha de tecnicas) ---
        self.declare_parameter('input_topic', '/x500_px4/sensor_measurements/livox_avia/points')
        self.declare_parameter('output_topic', '/perception/landing_candidates/points')
        self.declare_parameter('voxel_size', 0.15)            # metros
        self.declare_parameter('normal_search_radius', 0.5)   # metros (raio inicial da PCA)
        self.declare_parameter('max_inclination_deg', 15.0)   # limiar de consenso da literatura
        self.declare_parameter('world_frame', 'earth')
        self.world_frame = self.get_parameter('world_frame').value

        self.declare_parameter('max_roughness', 0.10)          # metros (RMS ao plano) -- Loureiro: 0.05-0.20m
        self.declare_parameter('grid_cell_size', 1.0)           # metros -- tamanho da celula do grid coarse
        self.declare_parameter('grid_max_height_range', 0.50)   # metros -- variacao de altura maxima aceita por celula

        # --- Novos parametros (Algorithm 1, Loureiro et al. 2021) ---
        self.declare_parameter('n_search_points', 30)   # quantos pontos aleatorios testar por ciclo
        self.declare_parameter('r_min', 0.3)             # raio inicial (metros)
        self.declare_parameter('r_max', 3.0)             # raio maximo (metros)
        self.declare_parameter('r_step', 0.2)            # incremento do raio a cada iteracao
        self.declare_parameter('n_min_points', 4)        # minimo de pontos p/ aceitar tentar PCA

        self.n_search_points = self.get_parameter('n_search_points').value
        self.r_min = self.get_parameter('r_min').value
        self.r_max = self.get_parameter('r_max').value
        self.r_step = self.get_parameter('r_step').value
        self.n_min_points = self.get_parameter('n_min_points').value

        self.max_roughness = self.get_parameter('max_roughness').value
        self.grid_cell_size = self.get_parameter('grid_cell_size').value
        self.grid_max_height_range = self.get_parameter('grid_max_height_range').value

        self.tf_buffer = Buffer()
        self.tf_listener = TransformListener(self.tf_buffer, self)

        input_topic = self.get_parameter('input_topic').value
        output_topic = self.get_parameter('output_topic').value
        self.voxel_size = self.get_parameter('voxel_size').value
        self.normal_search_radius = self.get_parameter('normal_search_radius').value
        self.max_inclination_deg = self.get_parameter('max_inclination_deg').value

        self.sub = self.create_subscription(
            PointCloud2, input_topic, self.cloud_callback, qos_profile_sensor_data)
        self.pub = self.create_publisher(PointCloud2, output_topic, 10)
        self.best_pub = self.create_publisher(PointStamped, '/perception/landing_candidates/best', 10)

        self.get_logger().info(
            f'Inscrito em {input_topic}, publicando candidatos em {output_topic}. '
            f'voxel_size={self.voxel_size}m, normal_search_radius={self.normal_search_radius}m, '
            f'max_inclination={self.max_inclination_deg} graus.')

    # def cloud_callback(self, msg: PointCloud2) -> None:
    #     try:
    #         transform = self.tf_buffer.lookup_transform(
    #             self.world_frame, msg.header.frame_id, msg.header.stamp)
    #     except (LookupException, ExtrapolationException) as e:
    #         self.get_logger().warn(f'TF nao disponivel ainda: {e}', throttle_duration_sec=2.0)
    #         return

    #     msg_world = do_transform_cloud(msg, transform)


    #     points = self._pointcloud2_to_numpy(msg)
    #     if points.shape[0] == 0:
    #         return

    #     pcd = o3d.geometry.PointCloud()
    #     pcd.points = o3d.utility.Vector3dVector(points)

    #     # 1. Voxel Grid downsampling
    #     pcd_down = pcd.voxel_down_sample(voxel_size=self.voxel_size)
    #     if len(pcd_down.points) < 3:
    #         return

    #     # 2. Estimacao de normais via PCA em vizinhanca esferica (KDTree, Open3D)
    #     pcd_down.estimate_normals(
    #         search_param=o3d.geometry.KDTreeSearchParamRadius(
    #             radius=self.normal_search_radius)
    #     )
    #     # Orienta as normais para "cima" (+Z), consistente com pouso
    #     pcd_down.orient_normals_to_align_with_direction(
    #         orientation_reference=np.array([0.0, 0.0, 1.0]))

    #     # 3. Validacao geometrica: inclinacao (angulo entre normal e vertical)
    #     normals = np.asarray(pcd_down.normals)
    #     vertical = np.array([0.0, 0.0, 1.0])
    #     cos_angle = np.clip(normals @ vertical, -1.0, 1.0)
    #     angle_deg = np.degrees(np.arccos(np.abs(cos_angle)))

    #     safe_mask = angle_deg <= self.max_inclination_deg
    #     candidate_points = np.asarray(pcd_down.points)[safe_mask]

    #     self.get_logger().info(
    #         f'{len(pcd_down.points)} pontos apos voxel -> '
    #         f'{candidate_points.shape[0]} candidatos (inclinacao <= '
    #         f'{self.max_inclination_deg} graus)',
    #         throttle_duration_sec=2.0)

    #     self.get_logger().info(
    #         f'angulos -- min: {angle_deg.min():.1f} graus, '
    #         f'media: {angle_deg.mean():.1f} graus, '
    #         f'max: {angle_deg.max():.1f} graus, '
    #         f'mediana: {np.median(angle_deg):.1f} graus',
    #         throttle_duration_sec=2.0)

    #     header_world = msg_world.header
    #     self._publish_candidates(candidate_points, msg.header)

    # def cloud_callback(self, msg: PointCloud2) -> None:
    #     try:
    #         transform = self.tf_buffer.lookup_transform(
    #             self.world_frame, msg.header.frame_id, msg.header.stamp)
    #     except (LookupException, ExtrapolationException) as e:
    #         self.get_logger().warn(f'TF nao disponivel ainda: {e}', throttle_duration_sec=2.0)
    #         return

    #     points_sensor = self._pointcloud2_to_numpy(msg)
    #     if points_sensor.shape[0] == 0:
    #         return

    #     points_world = self._apply_transform(points_sensor, transform)

    #     pcd = o3d.geometry.PointCloud()
    #     pcd.points = o3d.utility.Vector3dVector(points_world)

    #     # 1. Voxel Grid downsampling
    #     pcd_down = pcd.voxel_down_sample(voxel_size=self.voxel_size)
    #     if len(pcd_down.points) < 3:
    #         return

    #     # 2. Estimacao de normais via PCA em vizinhanca esferica (KDTree, Open3D)
    #     pcd_down.estimate_normals(
    #         search_param=o3d.geometry.KDTreeSearchParamRadius(
    #             radius=self.normal_search_radius)
    #     )
    #     pcd_down.orient_normals_to_align_with_direction(
    #         orientation_reference=np.array([0.0, 0.0, 1.0]))

    #     # 3. Validacao geometrica: inclinacao (agora em coordenadas do MUNDO)
    #     normals = np.asarray(pcd_down.normals)
    #     vertical = np.array([0.0, 0.0, 1.0])
    #     cos_angle = np.clip(normals @ vertical, -1.0, 1.0)
    #     angle_deg = np.degrees(np.arccos(np.abs(cos_angle)))

    #     self.get_logger().info(
    #         f'angulos -- min: {angle_deg.min():.1f} graus, '
    #         f'media: {angle_deg.mean():.1f} graus, '
    #         f'max: {angle_deg.max():.1f} graus, '
    #         f'mediana: {np.median(angle_deg):.1f} graus',
    #         throttle_duration_sec=2.0)

    #     safe_mask = angle_deg <= self.max_inclination_deg
    #     candidate_points = np.asarray(pcd_down.points)[safe_mask]

    #     self.get_logger().info(
    #         f'{len(pcd_down.points)} pontos apos voxel -> '
    #         f'{candidate_points.shape[0]} candidatos (inclinacao <= '
    #         f'{self.max_inclination_deg} graus)',
    #         throttle_duration_sec=2.0)

    #     header_world = Header()
    #     header_world.stamp = msg.header.stamp
    #     header_world.frame_id = self.world_frame
    #     self._publish_candidates(candidate_points, header_world)

    def cloud_callback(self, msg: PointCloud2) -> None:
        try:
            transform = self.tf_buffer.lookup_transform(
                self.world_frame, msg.header.frame_id, msg.header.stamp)
                
        except (LookupException, ExtrapolationException) as e:
            self.get_logger().warn(f'TF nao disponivel ainda: {e}', throttle_duration_sec=2.0)
            return

        points_sensor = self._pointcloud2_to_numpy(msg)
        if points_sensor.shape[0] == 0:
            return

        points_world = self._apply_transform(points_sensor, transform)

        pcd = o3d.geometry.PointCloud()
        pcd.points = o3d.utility.Vector3dVector(points_world)

        # 1. Voxel Grid downsampling
        pcd_down = pcd.voxel_down_sample(voxel_size=self.voxel_size)
        points_down = np.asarray(pcd_down.points)
        if points_down.shape[0] < 3:
            return

        # 2. Grid-map 2D coarse: descarta celulas com muita variacao de altura
        #    ANTES de gastar PCA nelas (filtro barato, primeiro estagio)
        coarse_mask = self._grid_coarse_filter(points_down)
        points_coarse = points_down[coarse_mask]

        self.get_logger().info(
            f'{points_down.shape[0]} pontos apos voxel -> '
            f'{points_coarse.shape[0]} sobrevivem ao filtro coarse (grid {self.grid_cell_size}m, '
            f'altura max {self.grid_max_height_range}m)',
            throttle_duration_sec=2.0)

        if points_coarse.shape[0] < 3:
            return

        # 3. Raio expansivel (Algorithm 1, Loureiro et al. 2021):
        #    amostra pontos aleatorios, cresce o raio enquanto plano for valido
        candidates = self._grow_candidates(points_coarse)

        if candidates:
            radii = [c['radius'] for c in candidates]
            angles = [c['inclination_deg'] for c in candidates]
            self.get_logger().info(
                f'{len(candidates)}/{self.n_search_points} candidatos validos | '
                f'raio min/media/max: {min(radii):.2f}/{np.mean(radii):.2f}/{max(radii):.2f}m | '
                f'inclinacao media: {np.mean(angles):.1f} graus',
                throttle_duration_sec=2.0)

              # Publica o primeiro candidato valido encontrado nesse frame
            first = candidates[0]
            pt = PointStamped()
            pt.header.stamp = msg.header.stamp
            pt.header.frame_id = self.world_frame
            pt.point.x = float(first['center'][0])
            pt.point.y = float(first['center'][1])
            pt.point.z = float(first['center'][2])
            self.best_pub.publish(pt)
            self.get_logger().info(
                f'PRIMEIRA AREA VALIDA: ({pt.point.x:.2f}, {pt.point.y:.2f}, {pt.point.z:.2f}) '
                f'raio={first["radius"]:.2f}m',
                throttle_duration_sec=2.0)   

        else:
            self.get_logger().info('0 candidatos validos', throttle_duration_sec=2.0)

        candidate_points = (
            np.array([c['center'] for c in candidates]) if candidates else np.empty((0, 3))
        )

        header_world = Header()
        header_world.stamp = msg.header.stamp
        header_world.frame_id = self.world_frame
        self._publish_candidates(candidate_points, header_world)

    def _grid_coarse_filter(self, points: np.ndarray) -> np.ndarray:
        """
        Filtro coarse: projeta os pontos num grid 2D (X,Y) e descarta os que
        caem em celulas com variacao de altura (Z) acima do limite -- indica
        obstaculo/parede/vegetacao alta, nao uma superficie plana candidata.
        """
        cell = self.grid_cell_size
        cell_idx = np.floor(points[:, :2] / cell).astype(np.int64)

        # Agrupa por celula (chave unica combinando indices X,Y)
        keys = cell_idx[:, 0] * 1_000_000 + cell_idx[:, 1]
        unique_keys, inverse, counts = np.unique(keys, return_inverse=True, return_counts=True)

        z = points[:, 2]
        z_min = np.full(unique_keys.shape[0], np.inf)
        z_max = np.full(unique_keys.shape[0], -np.inf)
        np.minimum.at(z_min, inverse, z)
        np.maximum.at(z_max, inverse, z)

        height_range = z_max - z_min
        cell_ok = height_range <= self.grid_max_height_range

        return cell_ok[inverse]

    # def _pca_normal_and_roughness(self, points: np.ndarray):
    #     """
    #     PCA explicita por ponto via KDTree: para cada ponto, busca vizinhos
    #     no raio configurado, calcula a matriz de covariancia, e extrai:
    #       - normal = autovetor do menor autovalor
    #       - rugosidade = sqrt(menor autovalor) = RMS da distancia ao plano

    #     Pontos sem vizinhos suficientes recebem NaN (descartados depois).
    #     """
    #     pcd = o3d.geometry.PointCloud()
    #     pcd.points = o3d.utility.Vector3dVector(points)
    #     kdtree = o3d.geometry.KDTreeFlann(pcd)

    #     n = points.shape[0]
    #     normals = np.full((n, 3), np.nan)
    #     roughness = np.full(n, np.nan)
    #     min_neighbors = 4

    #     for i in range(n):
    #         _, idx, _ = kdtree.search_radius_vector_3d(points[i], self.normal_search_radius)
    #         if len(idx) < min_neighbors:
    #             continue

    #         neighborhood = points[np.asarray(idx)]
    #         centroid = neighborhood.mean(axis=0)
    #         centered = neighborhood - centroid
    #         cov = (centered.T @ centered) / centered.shape[0]

    #         eigvals, eigvecs = np.linalg.eigh(cov)  # ordenado crescente
    #         normal = eigvecs[:, 0]
    #         if normal[2] < 0:
    #             normal = -normal  # orienta pra cima

    #         normals[i] = normal
    #         roughness[i] = np.sqrt(max(eigvals[0], 0.0))

    #     return normals, roughness

    def _grow_candidates(self, points: np.ndarray) -> list:
        """
        Algorithm 1 (Loureiro et al. 2021): para n_search_points pontos
        aleatorios, cresce o raio de busca enquanto a inclinacao continuar
        dentro do limite. Retorna uma lista de candidatos, cada um com
        centro, raio final (tamanho da area plana), normal e rugosidade.
        """
        pcd = o3d.geometry.PointCloud()
        pcd.points = o3d.utility.Vector3dVector(points)
        kdtree = o3d.geometry.KDTreeFlann(pcd)

        n = points.shape[0]
        if n == 0:
            return []

        sample_idx = np.random.choice(n, size=min(self.n_search_points, n), replace=False)
        candidates = []

        for i in sample_idx:
            center = points[i]
            best = None  # (radius, normal, roughness)
            r = self.r_min

            while r <= self.r_max:
                _, idx, _ = kdtree.search_radius_vector_3d(center, r)
                if len(idx) < self.n_min_points:
                    break  # sem pontos suficientes nem nesse raio -- para de crescer

                neighborhood = points[np.asarray(idx)]
                centroid = neighborhood.mean(axis=0)
                centered = neighborhood - centroid
                cov = (centered.T @ centered) / centered.shape[0]
                eigvals, eigvecs = np.linalg.eigh(cov)
                normal = eigvecs[:, 0]
                if normal[2] < 0:
                    normal = -normal
                roughness = float(np.sqrt(max(eigvals[0], 0.0)))

                cos_angle = np.clip(abs(normal @ np.array([0.0, 0.0, 1.0])), -1.0, 1.0)
                angle_deg = float(np.degrees(np.arccos(cos_angle)))

                if angle_deg <= self.max_inclination_deg:
                    # continua plano nesse raio -- guarda como melhor valido, tenta crescer mais
                    best = (r, normal, roughness, angle_deg)
                    r += self.r_step
                else:
                    break  # estourou o limite -- para, mantem o ultimo 'best' valido

            if best is not None:
                radius, normal, roughness, angle_deg = best
                if roughness <= self.max_roughness:
                    candidates.append({
                        'center': center,
                        'radius': radius,
                        'normal': normal,
                        'inclination_deg': angle_deg,
                        'roughness': roughness,
                    })

        return candidates

    def _apply_transform(self, points: np.ndarray, transform) -> np.ndarray:
        """Aplica manualmente a transformacao rigida (rotacao + translacao) do TF."""
        t = transform.transform.translation
        q = transform.transform.rotation

        # Quaternion (x,y,z,w) -> matriz de rotacao 3x3
        x, y, z, w = q.x, q.y, q.z, q.w
        R = np.array([
            [1 - 2*(y*y + z*z),     2*(x*y - z*w),         2*(x*z + y*w)],
            [    2*(x*y + z*w), 1 - 2*(x*x + z*z),         2*(y*z - x*w)],
            [    2*(x*z - y*w),     2*(y*z + x*w),     1 - 2*(x*x + y*y)],
        ])
        translation = np.array([t.x, t.y, t.z])

        return points @ R.T + translation

    def _pointcloud2_to_numpy(self, msg: PointCloud2) -> np.ndarray:
        # points = pc2.read_points_numpy(msg, field_names=('x', 'y', 'z'), skip_nans=True)
        # return np.asarray(points, dtype=np.float64)
        # read_points (sem _numpy) retorna array estruturado -- funciona mesmo
        # quando a nuvem tem campos com tipos diferentes (ex: intensity, ring),
        # que read_points_numpy rejeita.
        structured = pc2.read_points(msg, field_names=('x', 'y', 'z'), skip_nans=True)
        if structured.size == 0:
            return np.empty((0, 3), dtype=np.float64)
        points = np.column_stack([structured['x'], structured['y'], structured['z']])
        return points.astype(np.float64)

    def _publish_candidates(self, points: np.ndarray, header: Header) -> None:
        fields = [
            PointField(name='x', offset=0, datatype=PointField.FLOAT32, count=1),
            PointField(name='y', offset=4, datatype=PointField.FLOAT32, count=1),
            PointField(name='z', offset=8, datatype=PointField.FLOAT32, count=1),
        ]
        cloud_msg = pc2.create_cloud(header, fields, points.astype(np.float32))
        self.pub.publish(cloud_msg)


def main(args=None):
    rclpy.init(args=args)
    node = CandidateGenerationNode()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()