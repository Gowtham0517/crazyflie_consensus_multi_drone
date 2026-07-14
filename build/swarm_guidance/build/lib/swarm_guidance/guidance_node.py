#!/usr/bin/env python3
import rclpy
from rclpy.node import Node
from geometry_msgs.msg import PoseArray, Pose
import numpy as np
import math

class RingConsensusGuidance(Node):
    def __init__(self):
        super().__init__('swarm_guidance_node')
        self.target_pub = self.create_publisher(PoseArray, '/swarm/targets', 10)
        self.drones = ['cf1', 'cf2', 'cf3']
        self.num_drones = len(self.drones)
        self.ring_topology = {0: 2, 1: 0, 2: 1}
        self.center_x = 0.0
        self.center_y = 0.0
        self.radius = 0.6
        self.omega = 0.25
        self.kp = 1.2
        self.k_consensus = 0.08
        self.z_height = 0.5
        self.desired_spacing = 2.0944
        self.phases = np.array([0.0, 2.0944, 4.1888])
        self.measured_positions = np.zeros((self.num_drones, 3))
        self.airborne = False
        self.start_time_sync = None
        self.dt = 0.02
        self.state_sub = self.create_subscription(PoseArray, '/swarm/state', self.state_callback, 10)
        self.timer = self.create_timer(self.dt, self.generate_trajectory)
        self.get_logger().info(f"🎯 Ring Consensus Online. Center=({self.center_x},{self.center_y}), R={self.radius}, w={self.omega}")

    def state_callback(self, msg):
        for i in range(min(len(msg.poses), self.num_drones)):
            self.measured_positions[i] = [msg.poses[i].position.x, msg.poses[i].position.y, msg.poses[i].position.z]
        if not self.airborne and len(msg.poses) >= self.num_drones:
            if self.start_time_sync is None:
                self.start_time_sync = self.get_clock().now().nanoseconds / 1e9
                self.get_logger().info("⏱️ Syncing clocks...")
            if (self.get_clock().now().nanoseconds / 1e9 - self.start_time_sync) > 10.0:
                self.airborne = True
                self.get_logger().info("🚀 Ring Consensus phase propagation active.")

    def generate_trajectory(self):
        msg = PoseArray()
        msg.header.stamp = self.get_clock().now().to_msg()
        msg.header.frame_id = 'world'
        if not self.airborne:
            for i in range(self.num_drones):
                p = Pose()
                p.position.x = self.center_x + self.radius * math.cos(self.phases[i])
                p.position.y = self.center_y + self.radius * math.sin(self.phases[i])
                p.position.z = self.z_height
                p.orientation.x = float(self.phases[i])
                p.orientation.y = 0.0
                p.orientation.z = 0.0
                p.orientation.w = 0.0
                msg.poses.append(p)
            self.target_pub.publish(msg)
            return
        for i in range(self.num_drones):
            nb = self.ring_topology[i]
            pe = ((self.phases[nb] - self.phases[i] - self.desired_spacing + math.pi) % (2*math.pi)) - math.pi
            self.phases[i] += (self.omega * self.dt) + (self.k_consensus * pe * self.dt)
            self.phases[i] = math.atan2(math.sin(self.phases[i]), math.cos(self.phases[i]))
            tx = self.center_x + self.radius * math.cos(self.phases[i])
            ty = self.center_y + self.radius * math.sin(self.phases[i])
            xm, ym, zm = float(self.measured_positions[i][0]), float(self.measured_positions[i][1]), float(self.measured_positions[i][2])
            sh = 0.0
            if abs(xm) > 2.0 or abs(ym) > 2.0 or abs(zm) > 1.5:
                sh, vx, vy = 1.0, 0.0, 0.0
            else:
                vx = -self.radius * self.omega * math.sin(self.phases[i]) + self.kp * (tx - xm)
                vy = self.radius * self.omega * math.cos(self.phases[i]) + self.kp * (ty - ym)
            p = Pose()
            p.position.x, p.position.y, p.position.z = tx, ty, self.z_height
            p.orientation.x = float(self.phases[i])
            p.orientation.y = float(vx)
            p.orientation.z = float(vy)
            p.orientation.w = float(sh)
            msg.poses.append(p)
        self.target_pub.publish(msg)

def main(args=None):
    rclpy.init(args=args)
    node = RingConsensusGuidance()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()

if __name__ == '__main__':
    main()