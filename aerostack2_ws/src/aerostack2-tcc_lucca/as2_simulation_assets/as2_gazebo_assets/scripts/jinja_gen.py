#!/usr/bin/env python3
"""
Render assets file (SDF) from jinja template.

:raises argparse.ArgumentTypeError: Not used
:raises FileNotFoundError: Template not found
:raises OverwriteForbidden: Overwrite done in forbidden location/file.
:return: Rendered template SDF
"""

# Copyright 2022 Universidad Politécnica de Madrid
#
# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions are met:
#
#    * Redistributions of source code must retain the above copyright
#      notice, this list of conditions and the following disclaimer.
#
#    * Redistributions in binary form must reproduce the above copyright
#      notice, this list of conditions and the following disclaimer in the
#      documentation and/or other materials provided with the distribution.
#
#    * Neither the name of the the copyright holder nor the names of its
#      contributors may be used to endorse or promote products derived from
#      this software without specific prior written permission.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
# AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
# IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
# ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
# LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
# CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
# SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
# INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
# CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
# ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
# POSSIBILITY OF SUCH DAMAGE.

from __future__ import annotations

import argparse
import os
import shutil

import jinja2


__authors__ = 'Pedro Arias Pérez'
__copyright__ = 'Copyright (c) 2022 Universidad Politécnica de Madrid'
__license__ = 'BSD-3-Clause'


class OverwriteForbidden(Exception):
    """Overwrite not allowed."""


def get_namespace() -> str:
    """Get namespace."""
    return os.getenv('AEROSTACK2_SIMULATION_DRONE_ID', default='drone_sim')


def get_file_contents(filepath: str) -> bytes:
    """Get file content."""
    with open(filepath, 'rb') as file:
        return file.read()


def str2bool(value: str) -> bool:
    """Cast string to bool."""
    if value.lower() in ('yes', 'true', 't', 'y', '1'):
        return True
    if value.lower() in ('no', 'false', 'f', 'n', '0'):
        return False
    raise argparse.ArgumentTypeError('Boolean value expected.')


def get_sensors(sensors_array: list[str]) -> dict[str, str]:
    """Get sensors from payload."""
    sensors = []
    while sensors_array and sensors_array[0]:
        name = sensors_array.pop(0)
        model = sensors_array.pop(0)
        pose, sensor_attached, sensor_attached_type, gimbal_name, \
            drone_model_name, gimbaled, sensors_array = sensors_array[:6], \
            sensors_array[6], sensors_array[7], sensors_array[8], sensors_array[9], \
            sensors_array[10], sensors_array[11:]

        sensors.append({'name': name, 'model': model,
                        'pose': f'{pose[0]} {pose[1]} {pose[2]} {pose[3]} {pose[4]} {pose[5]}',
                        'sensor_attached': sensor_attached,
                        'sensor_attached_type': sensor_attached_type,
                        'gimbal_name': gimbal_name,
                        'drone_model_name': drone_model_name,
                        'gimbaled': str2bool(gimbaled)})
        print(sensors)
    return sensors


def get_embedded_models(embedded_array: list[str]) -> list[dict[str, str]]:
    """
    Get models to embed directly into the world SDF (as <include> blocks).

    Each model is described by 8 consecutive tokens:
    path name x y z roll pitch yaw

    This is used to embed already-generated drone SDF files (with all their
    payload sensors nested inside) directly into the world file at boot time,
    instead of spawning them later via the `ros_gz_sim create` service.

    Rationale: spawning models containing multi-row (2D/3D) raycast sensors
    (gpu_lidar or CPU ray with horizontal AND vertical scan) via the `create`
    service AFTER the world has already loaded triggers a known upstream race
    condition in gz-sim's rendering scene setup (SensorsPrivate::RenderThread
    vs the GUI's RenderUtil), causing a segfault in SceneManager::CreateLight/
    CreateMaterial/CreateScene. Models that are part of the world SDF from the
    start do not trigger this bug. See:
      - https://github.com/gazebosim/gz-sensors/issues/370
      - https://github.com/gazebosim/gz-sim/issues/2851
    """
    models = []
    while embedded_array and embedded_array[0]:
        path = embedded_array.pop(0)
        name = embedded_array.pop(0)
        pose, embedded_array = embedded_array[:6], embedded_array[6:]
        models.append({
            'path': path,
            'name': name,
            'pose': ' '.join(pose),
        })
    return models


def get_origin(origin_array: list[str]) -> tuple[dict[str, float], bool]:
    """Get GPS origin."""
    origin = {}

    use_origin = False
    if len(origin_array) == 3:
        origin['latitude'] = float(origin_array.pop(0))
        origin['longitude'] = float(origin_array.pop(0))
        origin['altitude'] = float(origin_array.pop(0))
        use_origin = True

    return origin, use_origin


def main():
    """Entrypoint."""
    parser = argparse.ArgumentParser()
    parser.add_argument(
        'filename', help='file that the sdf file should be generated from')
    parser.add_argument('env_dir')
    parser.add_argument('--output-file', help='sdf output file')
    parser.add_argument('--stdout', action='store_true',
                        default=False, help='dump to stdout instead of file')
    parser.add_argument('--namespace', default=get_namespace(),
                        help='Drone ROS namespace')
    parser.add_argument('--origin', default='',
                        help='Set world origin values: lat, lon, alt')
    parser.add_argument('--sensors', default='', help='Drone model sensors')
    parser.add_argument('--embedded-models', default='',
                         help='Models (e.g. drones) to embed directly into the world SDF, '
                              'as file://<uri> <include> blocks, instead of spawning them '
                              'later via the ros_gz_sim create service.')
    parser.add_argument('--no-odom', action='store_false',
                        dest='odom', help='Disable odometry plugin on model')
    parser.add_argument('--battery', dest='bat_capacity', default=0.0,
                        help='Enable battery plugin on model with given capacity')
    parser.add_argument('--enable_velocity_control', action='store_true',
                        help='Enable velocity control')
    parser.add_argument('--enable_acro_control', action='store_true',
                        help='Enable ACRO control')
    args = parser.parse_args()
    env = jinja2.Environment(loader=jinja2.FileSystemLoader(args.env_dir))
    template = env.get_template(os.path.relpath(args.filename, args.env_dir))

    sensors = get_sensors(str(args.sensors).split(sep=' '))

    origin, use_origin = get_origin(str(args.origin).split(sep=' '))

    embedded_models = get_embedded_models(str(args.embedded_models).split(sep=' '))

    dict_ = {'namespace': args.namespace, 'sensors': sensors, 'odom_plugin': args.odom,
             'battery_plugin': bool(float(args.bat_capacity)),
             'velocity_controller': args.enable_velocity_control,
             'acro_controller': args.enable_acro_control,
             'capacity': float(args.bat_capacity), 'origin': origin, 'use_origin': use_origin,
             'embedded_models': embedded_models}
    result = template.render(dict_)

    if args.stdout:
        print(f'{result=}')
    else:
        if args.output_file:
            filename_out = args.output_file
        else:
            if not args.filename.endswith('.sdf.jinja'):
                raise FileNotFoundError('ERROR: Output file can only be determined automatically' +
                                        ' for input files with the .sdf.jinja extension')
            filename_out = args.filename.replace('.sdf.jinja', '.sdf')
            assert filename_out != args.filename, 'Not allowed to overwrite template'

        # Overwrite protection mechanism: after generation, the file will be copied
        # to a 'last_generated' file. In the next run, we can check whether the target
        # file is still unmodified.
        filename_out_last_generated = filename_out + '.last_generated'

        if os.path.exists(filename_out) and os.path.exists(filename_out_last_generated):
            # Check whether the target file is still unmodified.
            if (get_file_contents(filename_out).strip() !=
                    get_file_contents(filename_out_last_generated).strip()):
                raise OverwriteForbidden('ERROR: generation would overwrite changes to ' +
                                         f'`{filename_out}`. Changes should only be ' +
                                         f'made to the template file `{args.filename}`. ' +
                                         f'Remove `{os.path.basename(filename_out)}` ' +
                                         '(after extracting your changes) to disable ' +
                                         'this overwrite protection.')

        with open(filename_out, 'w', encoding='utf-8') as f_out:
            print(f'{args.filename} -> {filename_out}')
            f_out.write(result)

        # Copy the contents to a 'last_generated' file for overwrite protection check next time.
        shutil.copy(filename_out, filename_out_last_generated)


if __name__ == '__main__':
    main()
