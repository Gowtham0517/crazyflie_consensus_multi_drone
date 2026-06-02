import rclpy
from rclpy.node import Node
from geometry_msgs.msg import PoseArray, PoseStamped

class SwarmNavigationNode(Node):
    def __init__(self):
        super().__init__('swarm_telemetry_node')
        self.drones = ['cf1', 'cf2', 'cf3']
        self.current_poses = {d: None for d in self.drones}
        
        self.state_pub = self.create_publisher(PoseArray, '/swarm/state', 10)
        
        # Create subscribers for all 3 drones dynamically
        for d in self.drones:
            self.create_subscription(PoseStamped, f'/{d}/pose', lambda msg, dn=d: self.pose_callback(dn, msg), 10)
            
        self.timer = self.create_timer(0.02, self.publish_swarm_state)
        self.get_logger().info("👀 SWARM Telemetry: Tracking cf1, cf2, and cf3...")

    def pose_callback(self, drone_id, msg):
        self.current_poses[drone_id] = msg.pose

    def publish_swarm_state(self):
        # Ensure telemetry has arrived for all drones before sending array
        if any(self.current_poses[d] is None for d in self.drones):
            return
            
        msg = PoseArray()
        msg.header.stamp = self.get_clock().now().to_msg()
        msg.header.frame_id = 'world'
        
        for d in self.drones:
            msg.poses.append(self.current_poses[d])
            
        self.state_pub.publish(msg)

def main(args=None):
    rclpy.init(args=args)
    node = SwarmNavigationNode()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()

if __name__ == '__main__':
    main()
