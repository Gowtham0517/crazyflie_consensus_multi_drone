import os
import yaml
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument, Shutdown
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node

def generate_launch_description():
    package_dir = get_package_share_directory('central_swarm_control')
    config_file = os.path.join(package_dir, 'config', 'crazyflies.yaml')

    with open(config_file, 'r') as ymlfile:
        crazyflies_dict = yaml.safe_load(ymlfile)

    # =============================================
    # EXPERIMENT LAUNCH ARGUMENTS
    # =============================================
    experiment_args = [
        DeclareLaunchArgument('experiment_id',  default_value='B1_no_fault'),
        DeclareLaunchArgument('run_number',      default_value='1'),
        DeclareLaunchArgument('fault_type',      default_value='none'),
        DeclareLaunchArgument('fault_motor',     default_value='1'),
        DeclareLaunchArgument('fault_magnitude', default_value='0.7'),
        DeclareLaunchArgument('fault_rate',      default_value='0.05'),
        DeclareLaunchArgument('fault_step_interval', default_value='5.0'),
        DeclareLaunchArgument('fault_min_health', default_value='0.3'),
        DeclareLaunchArgument('fault_on_time',   default_value='10.0'),
        DeclareLaunchArgument('fault_off_time',  default_value='10.0'),
        DeclareLaunchArgument('fault_start_delay', default_value='10.0'),
        DeclareLaunchArgument('track_duration',  default_value='60.0'),
    ]

    crazyflie_backend = Node(
        package='crazyflie',
        executable='crazyflie_server',
        name='crazyflie_server',
        output='screen',
        parameters=[crazyflies_dict]
    )

    navigation_node = Node(
        package='swarm_navigation',
        executable='navigation_node',
        name='swarm_telemetry_node',
        output='screen'
    )

    guidance_node = Node(
        package='swarm_guidance',
        executable='guidance_node',
        name='swarm_guidance_node',
        output='screen'
    )

    central_controller = Node(
        package='central_swarm_control',
        executable='control_node',
        name='swarm_controller_node',
        output='screen',
        on_exit=Shutdown(reason='Control node finished — shutting down all nodes.'),
        parameters=[{
            'experiment_id':      LaunchConfiguration('experiment_id'),
            'run_number':         LaunchConfiguration('run_number'),
            'fault_type':         LaunchConfiguration('fault_type'),
            'fault_motor':        LaunchConfiguration('fault_motor'),
            'fault_magnitude':    LaunchConfiguration('fault_magnitude'),
            'fault_rate':         LaunchConfiguration('fault_rate'),
            'fault_step_interval': LaunchConfiguration('fault_step_interval'),
            'fault_min_health':   LaunchConfiguration('fault_min_health'),
            'fault_on_time':      LaunchConfiguration('fault_on_time'),
            'fault_off_time':     LaunchConfiguration('fault_off_time'),
            'fault_start_delay':  LaunchConfiguration('fault_start_delay'),
            'track_duration':     LaunchConfiguration('track_duration'),
        }]
    )

    return LaunchDescription(
        experiment_args + [
            crazyflie_backend,
            navigation_node,
            guidance_node,
            central_controller
        ]
    )
