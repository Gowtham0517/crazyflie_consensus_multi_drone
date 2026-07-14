#!/usr/bin/env python3
import rclpy
from rclpy.node import Node
from geometry_msgs.msg import PoseArray, PoseStamped
from crazyflie_interfaces.msg import LogDataGeneric
from std_msgs.msg import Float64MultiArray

class SwarmNavigationNode(Node):
    def __init__(self):
        super().__init__('swarm_telemetry_node')
        self.drones = ['cf1', 'cf2', 'cf3']
        self.current_poses = {d: None for d in self.drones}
        
        # Full telemetry storage per drone
        # kinematics: [x, y, z, vx, vy, vz]
        # attitude_gyro: [roll, pitch, yaw, gyro_x, gyro_y, gyro_z]
        # actuators: [m1, m2, m3, m4]
        self.kinematics = {d: [0.0]*6 for d in self.drones}
        self.attitude_gyro = {d: [0.0]*6 for d in self.drones}
        self.actuators = {d: [0.0]*4 for d in self.drones}
        
        # Publishers
        self.state_pub = self.create_publisher(PoseArray, '/swarm/state', 10)
        self.telemetry_pub = self.create_publisher(Float64MultiArray, '/swarm/telemetry', 10)
        
        # Subscribe to active telemetry channels matching individual namespaces
        for d in self.drones:
            self.create_subscription(
                PoseStamped, 
                f'/{d}/pose', 
                lambda msg, dn=d: self.pose_callback(dn, msg), 
                10
            )
            # Subscribe to crazyswarm2 custom firmware log topics (LogDataGeneric)
            # These are configured in crazyflies.yaml under firmware_logging.custom_topics
            self.create_subscription(
                LogDataGeneric,
                f'/{d}/kinematics',
                lambda msg, dn=d: self.kinematics_callback(dn, msg),
                10
            )
            self.create_subscription(
                LogDataGeneric,
                f'/{d}/attitude_gyro',
                lambda msg, dn=d: self.attitude_gyro_callback(dn, msg),
                10
            )
            self.create_subscription(
                LogDataGeneric,
                f'/{d}/actuators',
                lambda msg, dn=d: self.actuators_callback(dn, msg),
                10
            )
            
        self.timer = self.create_timer(0.02, self.publish_swarm_state)
        self.get_logger().info("👀 3-Agent Telemetry Aggregator Active.")

    def pose_callback(self, drone_id, msg):
        self.current_poses[drone_id] = msg.pose

    def kinematics_callback(self, drone_id, msg):
        # Values order from YAML: [stateEstimate.x, y, z, vx, vy, vz]
        if len(msg.values) >= 6:
            self.kinematics[drone_id] = list(msg.values[:6])

    def attitude_gyro_callback(self, drone_id, msg):
        # Values order from YAML: [stabilizer.roll, pitch, yaw, gyro.x, y, z]
        if len(msg.values) >= 6:
            self.attitude_gyro[drone_id] = list(msg.values[:6])

    def actuators_callback(self, drone_id, msg):
        # Values order from YAML: [motor.m1, m2, m3, m4]
        if len(msg.values) >= 4:
            self.actuators[drone_id] = list(msg.values[:4])

    def publish_swarm_state(self):
        if any(self.current_poses[d] is None for d in self.drones):
            return
            
        # Publish PoseArray for state (position + attitude in orientation fields)
        msg = PoseArray()
        msg.header.stamp = self.get_clock().now().to_msg()
        msg.header.frame_id = 'world'
        
        for d in self.drones:
            msg.poses.append(self.current_poses[d])
            
        self.state_pub.publish(msg)
        
        # Publish full telemetry as Float64MultiArray
        # Layout per drone: [x, y, z, vx, vy, vz, roll, pitch, yaw, gx, gy, gz, m1, m2, m3, m4]
        # Total: 3 drones * 16 values = 48 floats
        telem_msg = Float64MultiArray()
        for d in self.drones:
            kin = self.kinematics[d]
            att = self.attitude_gyro[d]
            mot = self.actuators[d]
            telem_msg.data.extend([
                kin[0], kin[1], kin[2],      # x, y, z
                kin[3], kin[4], kin[5],      # vx, vy, vz
                att[0], att[1], att[2],      # roll, pitch, yaw
                att[3], att[4], att[5],      # gyro_x, gyro_y, gyro_z
                mot[0], mot[1], mot[2], mot[3]  # m1, m2, m3, m4
            ])
        self.telemetry_pub.publish(telem_msg)

def main(args=None):
    rclpy.init(args=args)
    node = SwarmNavigationNode()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        try:
            node.destroy_node()
            rclpy.shutdown()
        except Exception:
            pass

if __name__ == '__main__':
    main()