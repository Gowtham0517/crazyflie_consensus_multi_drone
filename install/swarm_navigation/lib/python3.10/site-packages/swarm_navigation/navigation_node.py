#!/usr/bin/env python3
import rclpy
from rclpy.node import Node
from geometry_msgs.msg import PoseArray, PoseStamped

class SwarmNavigationNode(Node):
    def __init__(self):
        super().__init__('swarm_telemetry_node')
        
        # Track only cf1
        self.drones = ['cf1']
        self.current_poses = {d: None for d in self.drones}
        
        # 1. Output Publisher: Sends bundled state straight to the Controller
        self.state_pub = self.create_publisher(PoseArray, '/swarm/state', 10)
        
        # 2. Input Subscriber: Listens only to cf1's pose from the server
        self.create_subscription(PoseStamped, '/cf1/pose', lambda msg: self.pose_callback('cf1', msg), 10)
        
        # 3. Aggregation Timer: 50Hz (0.02s) loop to bundle and push telemetry
        self.timer = self.create_timer(0.02, self.publish_swarm_state)
        self.get_logger().info("👀 SINGLE DRONE Telemetry: Tracking cf1 only...")

    def pose_callback(self, drone_id, msg):
        # Store incoming position data cleanly
        self.current_poses[drone_id] = msg.pose

    def publish_swarm_state(self):
        # Safety: Only build the array if cf1 has sent valid telemetry data
        if self.current_poses['cf1'] is None:
            return
            
        msg = PoseArray()
        msg.header.stamp = self.get_clock().now().to_msg()
        msg.header.frame_id = 'world'
        
        # Append only cf1's pose to the array
        msg.poses.append(self.current_poses['cf1'])
        
        # Ship it out to the controller
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
