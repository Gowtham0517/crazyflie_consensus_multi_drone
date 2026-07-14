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

        # Bidirectional ring topology: each drone couples with BOTH neighbors
        # cf1(0) ↔ cf2(1) ↔ cf3(2) ↔ cf1(0)
        self.ring_neighbors = {
            0: [1, 2],   # cf1 looks at cf2 and cf3
            1: [0, 2],   # cf2 looks at cf1 and cf3
            2: [0, 1],   # cf3 looks at cf1 and cf2
        }
        # Desired phase separation to each neighbor (120° = 2π/3)
        self.desired_separations = {
            0: {1: 2.0944, 2: -2.0944},   # cf1→cf2 = +120°, cf1→cf3 = -120°
            1: {0: -2.0944, 2: 2.0944},   # cf2→cf1 = -120°, cf2→cf3 = +120°
            2: {0: 2.0944, 1: -2.0944},   # cf3→cf1 = +120°, cf3→cf2 = -120°
        }

        self.center_x = 0.0
        self.center_y = 0.0
        self.radius = 0.6
        self.omega = 0.25       # nominal angular velocity (rad/s)
        self.kp = 1.2           # position tracking gain
        self.k_consensus = 0.08 # consensus coupling gain
        self.z_height = 0.5
        self.desired_spacing = 2.0944  # 120° in radians

        # Reference phases (internal, seeded at startup)
        self.phases = np.array([0.0, 2.0944, 4.1888])

        self.measured_positions = np.zeros((self.num_drones, 3))
        self.airborne = False
        self.start_time_sync = None
        self.dt = 0.02

        self.state_sub = self.create_subscription(
            PoseArray, '/swarm/state', self.state_callback, 10)
        self.timer = self.create_timer(self.dt, self.generate_trajectory)
        self.get_logger().info(
            f"🎯 Ring Consensus Online. Center=({self.center_x},{self.center_y}), "
            f"R={self.radius}, w={self.omega}, Bidirectional Ring")

    def state_callback(self, msg):
        for i in range(min(len(msg.poses), self.num_drones)):
            self.measured_positions[i] = [
                msg.poses[i].position.x,
                msg.poses[i].position.y,
                msg.poses[i].position.z
            ]
        if not self.airborne and len(msg.poses) >= self.num_drones:
            if self.start_time_sync is None:
                self.start_time_sync = self.get_clock().now().nanoseconds / 1e9
                self.get_logger().info("⏱️ Syncing clocks...")
            if (self.get_clock().now().nanoseconds / 1e9 - self.start_time_sync) > 10.0:
                self.airborne = True
                self.get_logger().info("🚀 Ring Consensus phase propagation active.")

    def _wrap_angle(self, angle):
        """Wrap angle to [-π, π]."""
        return math.atan2(math.sin(angle), math.cos(angle))

    def _get_measured_phase(self, i):
        """Derive the angular phase of drone i from its measured XY position
        relative to the orbit center. Falls back to the internal phase if
        the drone is too close to the center (degenerate atan2)."""
        dx = self.measured_positions[i][0] - self.center_x
        dy = self.measured_positions[i][1] - self.center_y
        r_meas = math.sqrt(dx * dx + dy * dy)
        if r_meas > 0.15:  # only trust measurement if meaningfully far from center
            return math.atan2(dy, dx)
        return self.phases[i]  # fallback to internal phase

    def generate_trajectory(self):
        msg = PoseArray()
        msg.header.stamp = self.get_clock().now().to_msg()
        msg.header.frame_id = 'world'

        # Before airborne: publish static circle start targets
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

        # ── MEASURED-PHASE CONSENSUS (closed-loop) ──
        # 1. Derive each drone's actual phase from its measured position
        measured_phases = np.array([self._get_measured_phase(i) for i in range(self.num_drones)])

        # 2. Determine which drones have valid (trustworthy) position data.
        #    A crashed drone's Kalman filter produces wildly divergent estimates;
        #    excluding it from consensus prevents corruption of healthy neighbors.
        drone_valid = []
        for i in range(self.num_drones):
            xm = float(self.measured_positions[i][0])
            ym = float(self.measured_positions[i][1])
            zm = float(self.measured_positions[i][2])
            valid = abs(xm) <= 2.0 and abs(ym) <= 2.0 and abs(zm) <= 1.5
            drone_valid.append(valid)

        for i in range(self.num_drones):
            # 3. Compute consensus correction from BOTH ring neighbors,
            #    but SKIP any neighbor whose telemetry is invalid (crashed)
            consensus_sum = 0.0
            for nb in self.ring_neighbors[i]:
                if not drone_valid[nb]:
                    continue  # ignore crashed neighbor's garbage phase
                desired_sep = self.desired_separations[i][nb]
                # Phase error: how far is the neighbor from where it should be?
                phase_err = self._wrap_angle(measured_phases[nb] - measured_phases[i] - desired_sep)
                consensus_sum += phase_err

            # 4. Update internal phase: nominal rotation + consensus coupling
            self.phases[i] += (self.omega * self.dt) + (self.k_consensus * consensus_sum * self.dt)
            self.phases[i] = self._wrap_angle(self.phases[i])

            # 5. Compute target position on the circle
            tx = self.center_x + self.radius * math.cos(self.phases[i])
            ty = self.center_y + self.radius * math.sin(self.phases[i])

            # 6. Position tracking: velocity command = feedforward + proportional correction
            xm = float(self.measured_positions[i][0])
            ym = float(self.measured_positions[i][1])
            zm = float(self.measured_positions[i][2])

            sh = 0.0
            if not drone_valid[i]:
                # This drone is out of bounds (crashed) — zero commands
                sh, vx, vy = 1.0, 0.0, 0.0
            else:
                vx = -self.radius * self.omega * math.sin(self.phases[i]) + self.kp * (tx - xm)
                vy =  self.radius * self.omega * math.cos(self.phases[i]) + self.kp * (ty - ym)

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
        try:
            node.destroy_node()
            rclpy.shutdown()
        except Exception:
            pass

if __name__ == '__main__':
    main()