import rclpy
from rclpy.node import Node
from geometry_msgs.msg import PoseArray
from crazyflie_interfaces.msg import Position
from rclpy.qos import qos_profile_sensor_data 
import numpy as np
import time
import csv
from datetime import datetime

class SwarmTrajectoryController(Node):
    def __init__(self):
        super().__init__('swarm_trajectory_controller')
        self.drones = ['cf1', 'cf2', 'cf3']
        
        self.current_positions = np.zeros((3, 3))
        self.target_positions = np.zeros((3, 3))
        self.takeoff_positions = np.zeros((3, 3))
        self.land_positions = np.zeros((3, 3))
        
        self.has_initial_pose = False
        self.has_targets = False
        
        # --- Multi-Drone CSV Initialization ---
        timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
        self.csv_filename = f"swarm_flight_log_{timestamp}.csv"
        self.csv_file = open(self.csv_filename, mode='w', newline='')
        self.csv_writer = csv.writer(self.csv_file)
        
        headers = ['timestamp_sec', 'flight_state']
        for d in self.drones:
            headers.extend([f'{d}_act_x', f'{d}_act_y', f'{d}_act_z', f'{d}_targ_x', f'{d}_targ_y', f'{d}_targ_z'])
        self.csv_writer.writerow(headers)
        
        self.state_sub = self.create_subscription(PoseArray, '/swarm/state', self.state_callback, 10)
        self.target_sub = self.create_subscription(PoseArray, '/swarm/targets', self.target_callback, 10)
        self.cmd_pubs = [self.create_publisher(Position, f'/{d}/cmd_position', qos_profile_sensor_data) for d in self.drones]
        
        self.timer = self.create_timer(0.02, self.control_loop)
        self.start_time = None
        self.track_start_time = None
        self.land_start_time = None
        self.state = 'WAITING'
        self.get_logger().info(f"💾 Logging Swarm to {self.csv_filename} | Waiting for telemetry...")

    def state_callback(self, msg):
        for i, pose in enumerate(msg.poses):
            if i < 3: 
                self.current_positions[i] = [pose.position.x, pose.position.y, pose.position.z]
        
        if not self.has_initial_pose:
            self.takeoff_positions = np.copy(self.current_positions)
            self.has_initial_pose = True
            self.get_logger().info("✅ Swarm floor frames locked!")
            self._check_ready()

    def target_callback(self, msg):
        for i, pose in enumerate(msg.poses):
            if i < 3: 
                self.target_positions[i] = [pose.position.x, pose.position.y, pose.position.z]
        if not self.has_targets:
            self.has_targets = True
            self._check_ready()

    def _check_ready(self):
        if self.state == 'WAITING' and self.has_initial_pose and self.has_targets:
            self.start_time = self.get_clock().now().nanoseconds / 1e9
            self.state = 'TAKEOFF'
            self.get_logger().info("🚀 Swarm Launching: 3D Spiral Takeoff active...")

    def control_loop(self):
        if self.state == 'WAITING':
            return
            
        current_time = self.get_clock().now().nanoseconds / 1e9
        t = current_time - self.start_time
        
        # 1. TAKEOFF PHASE (5 Seconds)
        if self.state == 'TAKEOFF':
            if t > 5.0:
                self.state = 'TRACK'
                self.track_start_time = current_time
                self.get_logger().info("🔄 Orbit intercept confirmed. Orbiting exactly 1 full circle...")
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

        # 2. TRACKING PHASE (Exactly 1 Circle = 12.57 seconds)
        elif self.state == 'TRACK':
            t_track = current_time - self.track_start_time
            if t_track >= 12.57:
                self.get_logger().info("🛬 Circle complete! Initiating automated structural vertical landing...")
                self.state = 'LAND'
                self.land_start_time = current_time
                self.land_positions = np.copy(self.current_positions)
            else:
                for i, pub in enumerate(self.cmd_pubs):
                    cmd = Position()
                    cmd.header.stamp = self.get_clock().now().to_msg()
                    cmd.header.frame_id = 'world'
                    cmd.x = float(self.target_positions[i][0])
                    cmd.y = float(self.target_positions[i][1])
                    cmd.z = float(self.target_positions[i][2])
                    cmd.yaw = 0.0
                    pub.publish(cmd)

        # 3. AUTOMATED LAND PHASE (4 Second Ramp Down)
        elif self.state == 'LAND':
            t_land = current_time - self.land_start_time
            if t_land > 4.0:
                self.get_logger().info("🛑 Swarm landed safely. Terminating flight script.")
                raise KeyboardInterrupt # Clean exit trick to trigger finally blocks
            else:
                progress = t_land / 4.0
                for i, pub in enumerate(self.cmd_pubs):
                    cmd = Position()
                    cmd.header.stamp = self.get_clock().now().to_msg()
                    cmd.header.frame_id = 'world'
                    cmd.x = float(self.land_positions[i][0]) # Freeze X
                    cmd.y = float(self.land_positions[i][1]) # Freeze Y
                    cmd.z = float(1.0 * (1.0 - progress))    # Ramp down height
                    cmd.yaw = 0.0
                    pub.publish(cmd)

        # --- Write 3-Drone Parallel CSV Line ---
        row = [round(t, 3), self.state]
        for i in range(3):
            row.extend([
                float(self.current_positions[i][0]), float(self.current_positions[i][1]), float(self.current_positions[i][2]),
                float(self.target_positions[i][0]), float(self.target_positions[i][1]), float(self.target_positions[i][2])
            ])
        self.csv_writer.writerow(row)

    def emergency_land(self):
        self.get_logger().warn("🚨 Manual Emergency Override! Cutting elevation...")
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

    def close_log(self):
        if hasattr(self, 'csv_file') and not self.csv_file.closed:
            self.csv_file.close()
            self.get_logger().info(f"💾 Multi-drone tracking data saved to: {self.csv_filename}")

def main(args=None):
    rclpy.init(args=args)
    node = SwarmTrajectoryController()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        node.emergency_land()
    finally:
        node.close_log()
        node.destroy_node()
        rclpy.shutdown()

if __name__ == '__main__':
    main()
