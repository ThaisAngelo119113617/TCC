#!/usr/bin/env python3
"""
mission_base.py

Classe base reutilizável para missões do TCC.

Encapsula a sequência básica de conexão/arm/offboard/decolagem/pouso
que toda missão precisa, como um context manager. Missões específicas
do TCC (detecção de zona de pouso, etc.) devem importar SimpleMission
e usar 'with', em vez de duplicar essa lógica.

Uso básico:
    from mission_base import SimpleMission

    with SimpleMission('x500_px4') as mission:
        mission.takeoff(2.0)
        mission.hover(3.0)
        # ... lógica específica da missão aqui ...
        mission.land()
"""

from time import sleep

import rclpy
from as2_python_api.drone_interface import DroneInterface


class SimpleMission:
    """
    Wrapper de missão básica sobre DroneInterface.

    Garante, via context manager, que o drone sempre desarma e
    desconecta corretamente, mesmo se a missão lançar uma exceção
    no meio do caminho.
    """

    def __init__(self, drone_namespace: str, use_sim_time: bool = True,
                 verbose: bool = True):
        self.drone_namespace = drone_namespace
        self.use_sim_time = use_sim_time
        self.verbose = verbose
        self.drone: DroneInterface = None

    def __enter__(self) -> 'SimpleMission':
        self._log(f'Inicializando ROS2 e conectando ao drone "{self.drone_namespace}"...')
        rclpy.init()
        self.drone = DroneInterface(
            drone_id=self.drone_namespace,
            use_sim_time=self.use_sim_time,
            verbose=self.verbose,
        )
        return self

    def __exit__(self, exc_type, exc_value, traceback) -> bool:
        if exc_type is not None:
            self._log(f'Missão interrompida por exceção: {exc_value}')

        self._log('Garantindo desarme antes de encerrar...')
        try:
            self.drone.disarm()
        except Exception as e:
            self._log(f'Aviso: desarme retornou erro (provavelmente já '
                      f'estava desarmado): {e}')

        self._log('Encerrando conexão com o drone...')
        self.drone.shutdown()
        rclpy.shutdown()
        self._log('Missão finalizada.')

        # Não suprime a exceção original (se houver) -- deixa propagar
        # depois da limpeza, pra você ver o traceback de verdade.
        return False

    def arm(self) -> None:
        """Arma os motores."""
        self._log('Armando motores...')
        self.drone.arm()
        sleep(1.0)

    def offboard(self) -> None:
        """Ativa modo offboard (controle via software)."""
        self._log('Ativando modo offboard...')
        self.drone.offboard()
        sleep(1.0)

    def takeoff(self, height: float, speed: float = 0.5) -> None:
        """
        Sequência completa de decolagem: arma, ativa offboard, decola.

        Chama arm() e offboard() automaticamente, então não precisa
        chamar eles antes -- só usar takeoff() direto.
        """
        self.arm()
        self.offboard()
        self._log(f'Decolando até {height} m...')
        self.drone.takeoff(height=height, speed=speed)
        self._log('Decolagem concluída!')

    def hover(self, seconds: float) -> None:
        """Paira (fica parado no ar) pelo tempo especificado."""
        self._log(f'Pairando por {seconds} s...')
        sleep(seconds)

    def land(self, speed: float = 0.3) -> None:
        """Pousa o drone."""
        self._log('Pousando...')
        self.drone.land(speed=speed)
        self._log('Pouso concluído!')
        sleep(2.0)  # aguarda confirmação de pouso

    def _log(self, message: str) -> None:
        if self.verbose:
            print(f'[{self.__class__.__name__}] {message}')