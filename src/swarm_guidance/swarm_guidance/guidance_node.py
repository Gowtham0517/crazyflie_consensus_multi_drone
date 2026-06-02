import rclpy
from rclpy.node import Node
from geometry_msgs.msg import PoseArray, Pose
import math

class SwarmGuidanceNode(Node):
    def __init__(self):
        super().__init__('swarm_guidance_node')
        self.target_pub = self.create_publisher(PoseArray, '/swarm/targets', 10)
        self.timer = self.create_timer(0.02, self.generate_trajectory)
        self.start_time = self.get_clock().now().nanoseconds / 1e9
        
        self.radius = 0.5
        self.omega = 0.4
        self.z_height = 0.8
        self.get_logger().info("🎯 SINGLE DRONE Guidance: Generating moving target...")

    def generate_trajectory(self):
        current_time = self.get_clock().now().nanoseconds / 1e9
        t = current_time - self.start_time
        
        msg = PoseArray()
        msg.header.stamp = self.get_clock().now().to_msg()
        msg.header.frame_id = 'world'
        
        # --- Only Drone 1 ---
        p1 = Pose()
        p1.position.x = self.radius * math.cos(self.omega * t)
        p1.position.y = self.radius * math.sin(self.omega * t)
        p1.position.z = self.z_height
        msg.poses.append(p1)
        
        self.target_pub.publish(msg)

def main(args=None):
    rclpy.init(args=args)
    node = SwarmGuidanceNode()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()

if __name__ == '__main__':
    main()
