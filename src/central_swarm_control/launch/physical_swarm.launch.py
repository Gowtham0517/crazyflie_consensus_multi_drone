import os
import yaml
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch_ros.actions import Node

def generate_launch_description():
    package_dir = get_package_share_directory('central_swarm_control')
    config_file = os.path.join(package_dir, 'config', 'crazyflies.yaml')

    # Parse the YAML in Python to bypass strict ROS 2 parameter formatting rules
    with open(config_file, 'r') as ymlfile:
        crazyflies_dict = yaml.safe_load(ymlfile)

    crazyflie_backend = Node(
        package='crazyflie',
        executable='crazyflie_server',
        name='crazyflie_server',
        output='screen',
        parameters=[crazyflies_dict]
    )

# UPDATED: Now points to the new 'control_node'
#    central_controller = Node(
#        package='central_swarm_control',
#        executable='control_node',
#       name='swarm_controller_node',
#        output='screen'
#    )

    return LaunchDescription([
        crazyflie_backend,
        #central_controller
    ])
