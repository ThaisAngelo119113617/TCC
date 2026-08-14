#!/bin/bash
# Uso: ./gerar_mundo_inclinado.bash <angulo_em_graus>
ANGLE_DEG=${1:-0}
ANGLE_RAD=$(python3 -c "import math; print(math.radians($ANGLE_DEG))")

cat > teste_plano_inclinado.sdf << EOF
<?xml version="1.0" ?>
<sdf version="1.9">
  <world name="teste_inclinado">
    <physics name="1ms" type="ignored">
      <max_step_size>0.001</max_step_size>
      <real_time_factor>1.0</real_time_factor>
    </physics>
    <plugin filename="gz-sim-physics-system" name="gz::sim::systems::Physics"/>
    <plugin filename="gz-sim-scene-broadcaster-system" name="gz::sim::systems::SceneBroadcaster"/>
    <plugin filename="gz-sim-sensors-system" name="gz::sim::systems::Sensors">
      <render_engine>ogre2</render_engine>
    </plugin>

    <light type="directional" name="sun">
      <pose>0 0 10 0 0 0</pose>
      <direction>-0.5 0.1 -0.9</direction>
    </light>

    <model name="ground_plane">
      <pose>0 0 0 0 ${ANGLE_RAD} 0</pose>
      <static>true</static>
      <link name="link">
        <collision name="collision">
          <geometry><plane><normal>0 0 1</normal></plane></geometry>
        </collision>
        <visual name="visual">
          <geometry><plane><normal>0 0 1</normal><size>50 50</size></plane></geometry>
        </visual>
      </link>
    </model>

    <model name="livox_avia_test">
      <pose>0 0 2 0 1.5707 0</pose>
      <static>true</static>
      <link name="link">
        <visual name="visual">
          <geometry><box><size>0.09 0.06 0.06</size></box></geometry>
        </visual>
        <sensor name="gpu_ray" type="gpu_lidar">
          <pose>0 0 0 0 0 0</pose>
          <topic>lidar</topic>
          <update_rate>10</update_rate>
          <always_on>true</always_on>
          <visualize>false</visualize>
          <lidar>
            <scan>
              <horizontal><samples>360</samples><resolution>1</resolution><min_angle>-0.61436</min_angle><max_angle>0.61436</max_angle></horizontal>
              <vertical><samples>200</samples><resolution>1</resolution><min_angle>-0.6737</min_angle><max_angle>0.6737</max_angle></vertical>
            </scan>
            <range><min>0.1</min><max>450.0</max><resolution>0.01</resolution></range>
            <noise><type>gaussian</type><mean>0.0</mean><stddev>0.02</stddev></noise>
          </lidar>
        </sensor>
      </link>
    </model>
  </world>
</sdf>
EOF

echo "Mundo gerado: teste_plano_inclinado.sdf (angulo = ${ANGLE_DEG} graus)"