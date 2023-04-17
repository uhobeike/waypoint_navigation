import os

from ament_index_python.packages import get_package_share_directory

from launch import LaunchDescription
from launch.actions import GroupAction
from launch_ros.actions import Node


def generate_launch_description():
    waypoint_navigation_dir = get_package_share_directory('waypoint_navigation')
    waypoint_file = os.path.join(waypoint_navigation_dir, 'config', 'waypoint.yaml')

    launch_node = GroupAction([
        Node(
            name='waypoint_navigation',
            package='waypoint_navigation',
            executable='waypoint_navigation',
            arguments=[waypoint_file],
            output='screen')
    ])

    ld = LaunchDescription()
    ld.add_action(launch_node)

    return ld
