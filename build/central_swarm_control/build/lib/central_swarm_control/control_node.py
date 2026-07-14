#!/usr/bin/env python3
import rclpy
from rclpy.node import Node
from rcl_interfaces.srv import SetParameters
from rcl_interfaces.msg import Parameter, ParameterType, ParameterValue
from geometry_msgs.msg import PoseArray
from crazyflie_interfaces.msg import Position, Hover
from std_msgs.msg import Float64MultiArray
from rclpy.qos import qos_profile_sensor_data
import numpy as np
import os
import time
import csv
import threading
from datetime import datetime

class SwarmConsensusController(Node):
    def __init__(self):
        super().__init__('swarm_trajectory_controller')

        # =============================================
        # EXPERIMENT PARAMETERS (set via launch file)
        # =============================================
        self.declare_parameter('experiment_id', 'B1_no_fault')
        self.declare_parameter('run_number', 1)
        self.declare_parameter('fault_type', 'none')        # none | abrupt | incipient | intermittent
        self.declare_parameter('fault_motor', 1)             # which motor on cf1 (1-4)
        self.declare_parameter('fault_magnitude', 0.7)       # target health for abrupt / intermittent
        self.declare_parameter('fault_rate', 0.05)           # health decrement per step (incipient)
        self.declare_parameter('fault_step_interval', 5.0)   # seconds between degradation steps (incipient)
        self.declare_parameter('fault_min_health', 0.3)      # minimum health floor (incipient)
        self.declare_parameter('fault_on_time', 10.0)        # fault active duration (intermittent)
        self.declare_parameter('fault_off_time', 10.0)       # fault inactive duration (intermittent)
        self.declare_parameter('fault_start_delay', 10.0)    # seconds into TRACK before fault begins
        self.declare_parameter('track_duration', 60.0)       # total orbit tracking time

        # Read parameters
        self.experiment_id = self.get_parameter('experiment_id').value
        self.run_number = self.get_parameter('run_number').value
        self.fault_type = self.get_parameter('fault_type').value
        self.fault_motor = self.get_parameter('fault_motor').value
        self.fault_magnitude = self.get_parameter('fault_magnitude').value
        self.fault_rate = self.get_parameter('fault_rate').value
        self.fault_step_interval = self.get_parameter('fault_step_interval').value
        self.fault_min_health = self.get_parameter('fault_min_health').value
        self.fault_on_time = self.get_parameter('fault_on_time').value
        self.fault_off_time = self.get_parameter('fault_off_time').value
        self.fault_start_delay = self.get_parameter('fault_start_delay').value
        self.track_duration = self.get_parameter('track_duration').value

        # =============================================
        # DRONE & FLIGHT CONFIG
        # =============================================
        self.drones = ['cf1', 'cf2', 'cf3']
        self.num_drones = len(self.drones)
        self.target_z = 0.5
        self.loop_rate_hz = 50
        self.ts = 1.0 / self.loop_rate_hz

        # State arrays
        self.current_positions = np.zeros((self.num_drones, 3))
        self.target_positions = np.zeros((self.num_drones, 3))
        self.takeoff_positions = np.zeros((self.num_drones, 3))
        self.land_positions = np.zeros((self.num_drones, 3))
        self.guidance_vx = np.zeros(self.num_drones)
        self.guidance_vy = np.zeros(self.num_drones)
        self.safe_hover_flag = np.zeros(self.num_drones)
        self.velocities = np.zeros((self.num_drones, 3))
        self.attitudes = np.zeros((self.num_drones, 3))
        self.gyros = np.zeros((self.num_drones, 3))
        self.motors = np.zeros((self.num_drones, 4))

        self.has_initial_pose = False
        self.has_targets = False
        self.fault_triggered = False
        self.current_motor_health = 1.0  # tracked for CSV logging

        # Circle slot positions (from standalone script phases)
        self.circle_start = np.array([
            [0.6, 0.0],       # cf1: phase=0
            [-0.3, 0.5196],   # cf2: phase=120°
            [-0.3, -0.5196]   # cf3: phase=240°
        ])

        # State machine timing
        self.takeoff_ramp_dur = 1.2
        self.takeoff_hover_dur = 3.0
        self.transit_dur = 4.0
        self.settle_dur = 2.0
        self.post_track_dur = 1.0
        self.land_dur = 1.2

        self.t_takeoff_end = self.takeoff_ramp_dur + self.takeoff_hover_dur
        self.t_transit_end = self.t_takeoff_end + self.transit_dur
        self.t_settle_end = self.t_transit_end + self.settle_dur
        self.t_track_end = self.t_settle_end + self.track_duration
        self.t_post_track_end = self.t_track_end + self.post_track_dur
        self.t_land_end = self.t_post_track_end + self.land_dur

        # =============================================
        # CSV LOGGING — Organized by experiment/run
        # =============================================
        log_base = os.path.join(
            os.path.expanduser('~'), 'crazyflie_ws', 'experiment_logs',
            self.experiment_id, f'run_{self.run_number}'
        )
        os.makedirs(log_base, exist_ok=True)

        timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
        self.csv_filename = os.path.join(log_base, f'flight_data_{timestamp}.csv')
        self.csv_file = open(self.csv_filename, mode='w', newline='')
        self.csv_writer = csv.writer(self.csv_file)

        headers = ['timestamp_sec', 'flight_state', 'fault_type', 'fault_motor_health']
        for d in self.drones:
            headers.extend([
                f'{d}_X', f'{d}_Y', f'{d}_Z',
                f'{d}_VelX', f'{d}_VelY', f'{d}_VelZ',
                f'{d}_Roll', f'{d}_Pitch', f'{d}_Yaw',
                f'{d}_GyroX', f'{d}_GyroY', f'{d}_GyroZ',
                f'{d}_Motor1', f'{d}_Motor2', f'{d}_Motor3', f'{d}_Motor4',
                f'{d}_targ_x', f'{d}_targ_y', f'{d}_targ_z'
            ])
        self.csv_writer.writerow(headers)

        # Write experiment metadata header as comment rows
        meta_file = os.path.join(log_base, f'experiment_meta_{timestamp}.txt')
        with open(meta_file, 'w') as mf:
            mf.write(f"experiment_id: {self.experiment_id}\n")
            mf.write(f"run_number: {self.run_number}\n")
            mf.write(f"fault_type: {self.fault_type}\n")
            mf.write(f"fault_motor: m{self.fault_motor}\n")
            mf.write(f"fault_magnitude: {self.fault_magnitude}\n")
            mf.write(f"fault_rate: {self.fault_rate}\n")
            mf.write(f"fault_step_interval: {self.fault_step_interval}\n")
            mf.write(f"fault_min_health: {self.fault_min_health}\n")
            mf.write(f"fault_on_time: {self.fault_on_time}\n")
            mf.write(f"fault_off_time: {self.fault_off_time}\n")
            mf.write(f"fault_start_delay: {self.fault_start_delay}\n")
            mf.write(f"track_duration: {self.track_duration}\n")
            mf.write(f"timestamp: {timestamp}\n")

        # =============================================
        # ROS2 INTERFACES
        # =============================================
        self.param_client = self.create_client(SetParameters, '/cf1/set_parameters')

        self.state_sub = self.create_subscription(PoseArray, '/swarm/state', self.state_callback, 10)
        self.target_sub = self.create_subscription(PoseArray, '/swarm/targets', self.target_callback, 10)
        self.telemetry_sub = self.create_subscription(Float64MultiArray, '/swarm/telemetry', self.telemetry_callback, 10)

        self.cmd_pos_pubs = [
            self.create_publisher(Position, f'/{d}/cmd_position', qos_profile_sensor_data) for d in self.drones
        ]
        self.cmd_hover_pubs = [
            self.create_publisher(Hover, f'/{d}/cmd_hover', qos_profile_sensor_data) for d in self.drones
        ]

        self.state = 'WAITING'
        self.start_time = None
        self.track_start_time = None
        self.land_start_time = None

        self.timer = self.create_timer(self.ts, self.control_loop)

        self.get_logger().info("=" * 60)
        self.get_logger().info(f"📋 EXPERIMENT: {self.experiment_id} | Run #{self.run_number}")
        self.get_logger().info(f"🔧 Fault: {self.fault_type} on cf1/m{self.fault_motor}")
        if self.fault_type == 'abrupt':
            self.get_logger().info(f"   Drop to health={self.fault_magnitude} at t={self.fault_start_delay}s")
        elif self.fault_type == 'incipient':
            self.get_logger().info(f"   Rate=-{self.fault_rate}/step, interval={self.fault_step_interval}s, floor={self.fault_min_health}")
        elif self.fault_type == 'intermittent':
            self.get_logger().info(f"   Health={self.fault_magnitude}, ON={self.fault_on_time}s / OFF={self.fault_off_time}s")
        self.get_logger().info(f"💾 Log: {self.csv_filename}")
        self.get_logger().info("=" * 60)

    # =============================================
    # CALLBACKS
    # =============================================
    def state_callback(self, msg):
        for i, pose in enumerate(msg.poses):
            if i < self.num_drones:
                self.current_positions[i] = [pose.position.x, pose.position.y, pose.position.z]
        if not self.has_initial_pose and len(msg.poses) >= self.num_drones:
            self.takeoff_positions = np.copy(self.current_positions)
            self.has_initial_pose = True
            self.get_logger().info("✅ Swarm configuration references initialized.")
            self._check_ready()

    def target_callback(self, msg):
        for i, pose in enumerate(msg.poses):
            if i < self.num_drones:
                self.target_positions[i] = [pose.position.x, pose.position.y, pose.position.z]
                self.guidance_vx[i] = pose.orientation.y
                self.guidance_vy[i] = pose.orientation.z
                self.safe_hover_flag[i] = pose.orientation.w
        if not self.has_targets and len(msg.poses) >= self.num_drones:
            self.has_targets = True
            self._check_ready()

    def telemetry_callback(self, msg):
        if len(msg.data) >= self.num_drones * 16:
            for i in range(self.num_drones):
                off = i * 16
                self.velocities[i] = msg.data[off+3:off+6]
                self.attitudes[i] = msg.data[off+6:off+9]
                self.gyros[i] = msg.data[off+9:off+12]
                self.motors[i] = msg.data[off+12:off+16]

    def _check_ready(self):
        if self.state == 'WAITING' and self.has_initial_pose and self.has_targets:
            # ── PRE-FLIGHT POSITION SAFETY CHECK ──
            # Verify each drone is physically near its expected circle_start slot.
            # Prevents crashes caused by swapped or misplaced drones.
            max_start_error = 0.30  # metres — abort if any drone is further
            placement_ok = True
            for i in range(self.num_drones):
                dx = self.takeoff_positions[i][0] - self.circle_start[i][0]
                dy = self.takeoff_positions[i][1] - self.circle_start[i][1]
                dist = float(np.sqrt(dx*dx + dy*dy))
                self.get_logger().info(
                    f"📍 {self.drones[i]}: actual=({self.takeoff_positions[i][0]:.3f}, "
                    f"{self.takeoff_positions[i][1]:.3f})  expected=({self.circle_start[i][0]:.3f}, "
                    f"{self.circle_start[i][1]:.3f})  error={dist:.3f}m"
                )
                if dist > max_start_error:
                    placement_ok = False
                    self.get_logger().error(
                        f"🚨 {self.drones[i]} is {dist:.2f}m from its expected slot! "
                        f"Max allowed: {max_start_error}m. Check physical placement."
                    )

            if not placement_ok:
                self.get_logger().error("=" * 60)
                self.get_logger().error("❌ FLIGHT ABORTED — Drone placement mismatch detected!")
                self.get_logger().error("   Re-position drones to match circle_start slots,")
                self.get_logger().error("   or update circle_start / crazyflies.yaml to match")
                self.get_logger().error("   the actual physical layout.")
                self.get_logger().error("=" * 60)
                self.state = 'DONE'
                return

            self.start_time = self.get_clock().now().nanoseconds / 1e9
            self.state = 'TAKEOFF'
            self.get_logger().info("🚀 Flight sequence initiated: Executing vertical takeoff...")

    # =============================================
    # MAIN CONTROL LOOP
    # =============================================
    def control_loop(self):
        if self.state == 'WAITING' or self.state == 'DONE':
            return

        current_time = self.get_clock().now().nanoseconds / 1e9
        t = current_time - self.start_time

        if self.state == 'TAKEOFF':
            if t < self.takeoff_ramp_dur:
                progress = t / self.takeoff_ramp_dur
                for i, pub in enumerate(self.cmd_pos_pubs):
                    cmd = Position()
                    cmd.x = float(self.takeoff_positions[i][0])
                    cmd.y = float(self.takeoff_positions[i][1])
                    cmd.z = float(self.target_z * progress)
                    pub.publish(cmd)
            elif t < self.t_takeoff_end:
                for i, pub in enumerate(self.cmd_pos_pubs):
                    cmd = Position()
                    cmd.x = float(self.takeoff_positions[i][0])
                    cmd.y = float(self.takeoff_positions[i][1])
                    cmd.z = float(self.target_z)
                    pub.publish(cmd)
            else:
                self.state = 'TRANSIT'
                self.get_logger().info("➔ Sidestepping to target radius slots...")

        elif self.state == 'TRANSIT':
            if t < self.t_transit_end:
                progress = (t - self.t_takeoff_end) / self.transit_dur
                for i, pub in enumerate(self.cmd_pos_pubs):
                    cmd = Position()
                    cmd.x = float(self.takeoff_positions[i][0] + (self.circle_start[i][0] - self.takeoff_positions[i][0]) * progress)
                    cmd.y = float(self.takeoff_positions[i][1] + (self.circle_start[i][1] - self.takeoff_positions[i][1]) * progress)
                    cmd.z = float(self.target_z)
                    pub.publish(cmd)
            else:
                self.state = 'SETTLE'
                self.get_logger().info("➔ Settling at circle start positions...")

        elif self.state == 'SETTLE':
            if t < self.t_settle_end:
                for i, pub in enumerate(self.cmd_pos_pubs):
                    cmd = Position()
                    cmd.x = float(self.circle_start[i][0])
                    cmd.y = float(self.circle_start[i][1])
                    cmd.z = float(self.target_z)
                    pub.publish(cmd)
            else:
                self.state = 'TRACK'
                self.track_start_time = current_time
                self.get_logger().info("🚀 Ring Consensus Tracking Unlocked.")

        elif self.state == 'TRACK':
            t_track = current_time - self.track_start_time
            if t_track >= self.track_duration:
                self.state = 'POST_TRACK'
                self.get_logger().info("➔ Post-track hover hold...")
            else:
                # Send velocity commands from guidance
                for i in range(self.num_drones):
                    cmd = Hover()
                    if self.safe_hover_flag[i] > 0.5:
                        cmd.vx, cmd.vy = 0.0, 0.0
                    else:
                        cmd.vx = float(self.guidance_vx[i])
                        cmd.vy = float(self.guidance_vy[i])
                    cmd.yaw_rate = 0.0
                    cmd.z_distance = float(self.target_z)
                    self.cmd_hover_pubs[i].publish(cmd)

                # Trigger fault injection
                if self.fault_type != 'none' and t_track >= self.fault_start_delay and not self.fault_triggered:
                    self.fault_triggered = True
                    threading.Thread(target=self._run_fault_engine, daemon=True).start()

        elif self.state == 'POST_TRACK':
            t_post = t - (self.t_settle_end + self.track_duration)
            if t_post >= self.post_track_dur:
                self.state = 'LAND'
                self.land_start_time = current_time
                self.land_positions = np.copy(self.current_positions)
                self.get_logger().info("🛬 Executing auto-land sequence...")
            else:
                for i in range(self.num_drones):
                    cmd = Hover()
                    cmd.vx, cmd.vy, cmd.yaw_rate = 0.0, 0.0, 0.0
                    cmd.z_distance = float(self.target_z)
                    self.cmd_hover_pubs[i].publish(cmd)

        elif self.state == 'LAND':
            t_land = current_time - self.land_start_time
            if t_land >= self.land_dur:
                self.state = 'DONE'
                self.get_logger().info("🛑 Swarm landed safely.")
                self.close_log()
                raise KeyboardInterrupt
            else:
                progress = t_land / self.land_dur
                for i, pub in enumerate(self.cmd_pos_pubs):
                    cmd = Position()
                    cmd.x = float(self.land_positions[i][0])
                    cmd.y = float(self.land_positions[i][1])
                    cmd.z = float(max(self.target_z * (1.0 - progress), 0.0))
                    pub.publish(cmd)

        # Log row
        row = [round(t, 3), self.state, self.fault_type, round(self.current_motor_health, 3)]
        for i in range(self.num_drones):
            row.extend([
                float(self.current_positions[i][0]), float(self.current_positions[i][1]), float(self.current_positions[i][2]),
                float(self.velocities[i][0]), float(self.velocities[i][1]), float(self.velocities[i][2]),
                float(self.attitudes[i][0]), float(self.attitudes[i][1]), float(self.attitudes[i][2]),
                float(self.gyros[i][0]), float(self.gyros[i][1]), float(self.gyros[i][2]),
                float(self.motors[i][0]), float(self.motors[i][1]), float(self.motors[i][2]), float(self.motors[i][3]),
                float(self.target_positions[i][0]), float(self.target_positions[i][1]), float(self.target_positions[i][2])
            ])
        self.csv_writer.writerow(row)

    # =============================================
    # CONFIGURABLE FAULT ENGINE
    # =============================================
    def _send_motor_health(self, motor_num, health_val):
        """Send a single motor health parameter to cf1 firmware."""
        if not self.param_client.wait_for_service(timeout_sec=1.0):
            self.get_logger().error("Param service unavailable for cf1")
            return
        req = SetParameters.Request()
        param = Parameter()
        param.name = f'powerDist.m{motor_num}Health'
        param.value.type = ParameterType.PARAMETER_DOUBLE
        param.value.double_value = float(health_val)
        req.parameters.append(param)
        self.param_client.call_async(req)

    def _restore_all_motors(self):
        """Restore all motors to full health."""
        for m in range(1, 5):
            self._send_motor_health(m, 1.0)
        self.current_motor_health = 1.0
        self.get_logger().info("🔄 [FAULT] All motors restored to health=1.0")

    def _run_fault_engine(self):
        """Dispatches to the appropriate fault injection routine based on fault_type."""
        self.get_logger().warn(f"⚠️ [FAULT ENGINE] Starting {self.fault_type} fault on cf1/m{self.fault_motor}")

        if self.fault_type == 'abrupt':
            self._fault_abrupt()
        elif self.fault_type == 'incipient':
            self._fault_incipient()
        elif self.fault_type == 'intermittent':
            self._fault_intermittent()

    def _fault_abrupt(self):
        """Instant health drop to fault_magnitude, held until end of TRACK."""
        self.get_logger().info(f"⚡ [ABRUPT] m{self.fault_motor}: 1.0 → {self.fault_magnitude}")
        self._send_motor_health(self.fault_motor, self.fault_magnitude)
        self.current_motor_health = self.fault_magnitude

        # Hold until TRACK ends
        while rclpy.ok() and self.state == 'TRACK':
            time.sleep(0.5)

        self._restore_all_motors()

    def _fault_incipient(self):
        """Gradual degradation: health decreases by fault_rate every fault_step_interval seconds."""
        health = 1.0
        while health > self.fault_min_health and rclpy.ok() and self.state == 'TRACK':
            health -= self.fault_rate
            health = max(health, self.fault_min_health)
            self.get_logger().info(f"📉 [INCIPIENT] m{self.fault_motor}: health → {health:.3f}")
            self._send_motor_health(self.fault_motor, health)
            self.current_motor_health = health
            time.sleep(self.fault_step_interval)

        self.get_logger().info(f"📉 [INCIPIENT] Reached floor health={health:.3f}, holding...")
        # Hold at floor until TRACK ends
        while rclpy.ok() and self.state == 'TRACK':
            time.sleep(0.5)

        self._restore_all_motors()

    def _fault_intermittent(self):
        """Cyclic fault: ON for fault_on_time, OFF for fault_off_time, repeat."""
        cycle = 0
        while rclpy.ok() and self.state == 'TRACK':
            cycle += 1
            # FAULT ON
            self.get_logger().info(f"🔴 [INTERMITTENT] Cycle {cycle}: FAULT ON (health={self.fault_magnitude}) for {self.fault_on_time}s")
            self._send_motor_health(self.fault_motor, self.fault_magnitude)
            self.current_motor_health = self.fault_magnitude

            t_start = time.time()
            while (time.time() - t_start) < self.fault_on_time and rclpy.ok() and self.state == 'TRACK':
                time.sleep(0.1)

            if self.state != 'TRACK':
                break

            # FAULT OFF
            self.get_logger().info(f"🟢 [INTERMITTENT] Cycle {cycle}: FAULT OFF (health=1.0) for {self.fault_off_time}s")
            self._send_motor_health(self.fault_motor, 1.0)
            self.current_motor_health = 1.0

            t_start = time.time()
            while (time.time() - t_start) < self.fault_off_time and rclpy.ok() and self.state == 'TRACK':
                time.sleep(0.1)

        self._restore_all_motors()

    # =============================================
    # SAFETY & CLEANUP
    # =============================================
    def emergency_land(self):
        self.get_logger().warn("🚨 Safety Kill! Restoring motors and dropping swarm...")
        self._restore_all_motors()
        for _ in range(5):
            for i, pub in enumerate(self.cmd_pos_pubs):
                cmd = Position()
                cmd.x = float(self.current_positions[i][0])
                cmd.y = float(self.current_positions[i][1])
                cmd.z = 0.0
                pub.publish(cmd)
            time.sleep(0.1)

    def close_log(self):
        if hasattr(self, 'csv_file') and not self.csv_file.closed:
            self.csv_file.close()
            self.get_logger().info(f"💾 Data saved: {self.csv_filename}")


def main(args=None):
    rclpy.init(args=args)
    node = SwarmConsensusController()
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