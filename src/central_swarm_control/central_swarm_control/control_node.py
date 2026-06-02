import rclpy
from rclpy.node import Node
from geometry_msgs.msg import PoseArray
from crazyflie_interfaces.msg import Position
from rclpy.qos import qos_profile_sensor_data 
import numpy as np
import time

class SwarmTrajectoryController(Node):
    def __init__(self):
        super().__init__('swarm_trajectory_controller')
        self.drones = ['cf1'] # ONLY CF1
        
        # Arrays shrunk to size 1
        self.current_positions = np.zeros((1, 3))
        self.target_positions = np.zeros((1, 3))
        self.takeoff_positions = np.zeros((1, 3))
        
        self.has_initial_pose = False
        self.has_targets = False
        
        self.state_sub = self.create_subscription(PoseArray, '/swarm/state', self.state_callback, 10)
        self.target_sub = self.create_subscription(PoseArray, '/swarm/targets', self.target_callback, 10)
        self.cmd_pubs = [self.create_publisher(Position, f'/{d}/cmd_position', qos_profile_sensor_data) for d in self.drones]
        
        self.timer = self.create_timer(0.02, self.control_loop)
        self.start_time = None
        self.state = 'WAITING'
        self.get_logger().info("⏳ SINGLE DRONE Controller: Waiting for state and target...")

    def state_callback(self, msg):
        for i, pose in enumerate(msg.poses):
            if i < 1: # Only read 1 pose
                self.current_positions[i] = [pose.position.x, pose.position.y, pose.position.z]
        
        if not self.has_initial_pose:
            self.takeoff_positions = np.copy(self.current_positions)
            self.has_initial_pose = True
            self.get_logger().info("✅ CF1 Floor position locked!")
            self._check_ready()

    def target_callback(self, msg):
        for i, pose in enumerate(msg.poses):
            if i < 1: # Only read 1 target
                self.target_positions[i] = [pose.position.x, pose.position.y, pose.position.z]
        
        if not self.has_targets:
            self.has_targets = True
            self._check_ready()

    def _check_ready(self):
        if self.state == 'WAITING' and self.has_initial_pose and self.has_targets:
            self.start_time = self.get_clock().now().nanoseconds / 1e9
            self.state = 'TAKEOFF'
            self.get_logger().info("🚀 Executing dynamic spiral takeoff...")

    def control_loop(self):
        if self.state == 'WAITING':
            return
            
        current_time = self.get_clock().now().nanoseconds / 1e9
        t = current_time - self.start_time
        
        if self.state == 'TAKEOFF':
            if t > 5.0:
                self.state = 'TRACK'
            else:
                progress = t / 5.0 
                for i, pub in enumerate(self.cmd_pubs):
                    cmd = Position()
                    cmd.header.stamp = self.get_clock().now().to_msg()
                    cmd.header.frame_id = 'world'
                    cmd.x = float(self.takeoff_positions[i][0] + (self.target_positions[i][0] - self.takeoff_positions[i][0]) * progress)
                    cmd.y = float(self.takeoff_positions[i][1] + (self.target_positions[i][1] - self.takeoff_positions[i][1]) * progress)
                    cmd.z = float(self.target_positions[i][2] * progress) 
                    cmd.yaw = 0.0
                    pub.publish(cmd)

        elif self.state == 'TRACK':
            for i, pub in enumerate(self.cmd_pubs):
                cmd = Position()
                cmd.header.stamp = self.get_clock().now().to_msg()
                cmd.header.frame_id = 'world'
                cmd.x = float(self.target_positions[i][0])
                cmd.y = float(self.target_positions[i][1])
                cmd.z = float(self.target_positions[i][2])
                cmd.yaw = 0.0
                pub.publish(cmd)

    def emergency_land(self):
        self.get_logger().warn("🚨 Dropping CF1...")
        for _ in range(5):
            for i, pub in enumerate(self.cmd_pubs):
                cmd = Position()
                cmd.header.stamp = self.get_clock().now().to_msg()
                cmd.x = float(self.current_positions[i][0]) 
                cmd.y = float(self.current_positions[i][1])
                cmd.z = 0.0 
                cmd.yaw = 0.0
                pub.publish(cmd)
            time.sleep(0.1)

def main(args=None):
    rclpy.init(args=args)
    node = SwarmTrajectoryController()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        node.emergency_land()
    finally:
        node.destroy_node()
        rclpy.shutdown()

if __name__ == '__main__':
    main()
