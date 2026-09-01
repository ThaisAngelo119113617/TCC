from mission_base import SimpleMission

with SimpleMission('x500_px4') as mission:
    mission.takeoff(5.0)
    mission.reset_perception()
    waypoints = [(-6,-6),(-6,0),(-6,6),(-2,6),(-2,0),(-2,-6),
                 (2,-6),(2,0),(2,6),(6,6),(6,0),(6,-6)]
    for x, y in waypoints:
        mission.go_to(x, y, 5.0, speed=0.6)
    mission.land()
