# Crazyflie Consensus Multi-Drone

A ROS 2 system for implementing distributed consensus-based formation control on multiple Crazyflie 2.1 nano-quadrotors using Lighthouse localization and dynamic network topology reconfiguration.

## Overview

This repository implements fault-tolerant consensus formation control for quadrotor swarms, enabling decentralized coordination where each drone makes decisions based on local neighbor information. The system is designed for experimental validation on real hardware (Crazyflie 2.1+) and simulation environments.

**Key Features:**
- Distributed consensus algorithms with dynamic topology adaptation
- Fault detection and isolation mechanisms at the firmware level
- Real-time ROS 2 integration (Humble/Jazzy compatible)
- Lighthouse positioning system support
- Experimental validation on 3-drone formations

## Hardware Setup

### Drones
- **Platform:** Bitcraze Crazyflie 2.1+
- **Flight Controller:** Crazyflie with FreeRTOS firmware
- **Localization:** Lighthouse deck expansion (10+ base stations recommended)
- **Communication:** Crazyradio 2.4 GHz

### Supporting Equipment
- **Crazyradio PA** or Crazyradio 2.0 for wireless command/telemetry
- **Host Computer:** Ubuntu 22.04+ (ROS 2 Humble/Jazzy)
- **Lighthouse Geometry:** 5m × 5m arena (configurable)

## Software Architecture

```
├── src/
│   ├── consensus_node/        # Consensus algorithm implementation
│   │   ├── consensus_control.py
│   │   ├── topology_manager.py    # Dynamic topology reconfiguration
│   │   └── fault_detector.py      # Fault isolation logic
│   ├── crazyflie_driver/      # Crazyflie interface layer
│   │   ├── cflib_wrapper.py
│   │   └── lighthouse_localization.py
│   ├── msg_definitions/       # Custom ROS 2 message types
│   │   ├── DroneState.msg
│   │   ├── ConsensusData.msg
│   │   └── TopologyUpdate.msg
│   └── launch/
│       ├── single_drone.launch.py
│       ├── multi_drone.launch.py
│       └── simulation.launch.py
├── config/
│   ├── drone_ids.yaml         # Drone configuration
│   ├── consensus_params.yaml  # Algorithm parameters
│   └── lighthouse_geometry.yaml
├── test/
│   ├── test_consensus_math.py
│   ├── test_fault_scenarios.py
│   └── test_hardware_integration.py
└── sim/
    ├── crazyflie_swarm_sim.py # Simulation environment
    └── sim_config.yaml
```

## Dependencies

### Core
- **ROS 2:** Humble or Jazzy (`ros-humble-*` or `ros-jazzy-*` packages)
- **Python 3.10+**
- **cflib:** `pip install cflib`
- **NumPy:** `pip install numpy`
- **SciPy:** `pip install scipy`

### Optional (Simulation)
- **Gazebo:** Fortress or Garden
- **PyBullet:** `pip install pybullet` (CrazySim backend)

### Build Dependencies
```bash
sudo apt-get install -y \
  python3-rosdep \
  python3-colcon-common-extensions \
  build-essential
```

## Installation

### 1. Set Up ROS 2 Workspace
```bash
mkdir -p ~/crazyswarm_ws/src
cd ~/crazyswarm_ws
```

### 2. Clone Repository
```bash
cd src
git clone https://github.com/Gowtham0517/crazyflie_consensus_multi_drone.git
cd crazyflie_consensus_multi_drone
```

### 3. Install Dependencies
```bash
# Python dependencies
pip install -r requirements.txt

# ROS 2 dependencies
rosdep install --from-paths src --ignore-src -r -y
```

### 4. Build
```bash
cd ~/crazyswarm_ws
colcon build --symlink-install
source install/setup.bash
```

## Quick Start

### Single Drone Test
```bash
# Terminal 1: Launch Crazyflie driver
ros2 launch crazyflie_driver single_drone.launch.py drone_id:=cf1

# Terminal 2: Test position hold
python3 src/crazyflie_consensus_multi_drone/test/test_basic_flight.py
```

### Multi-Drone Consensus Formation

#### Hardware (Real Drones)
```bash
# Terminal 1: Launch multi-drone system
ros2 launch crazyflie_consensus_multi_drone multi_drone.launch.py \
  drone_ids:="[cf1,cf2,cf3]"

# Terminal 2: Start consensus formation
python3 src/crazyflie_consensus_multi_drone/scripts/start_formation.py \
  --drones cf1 cf2 cf3 \
  --formation circle \
  --radius 1.0
```

#### Simulation
```bash
# Launch simulation environment with 3 drones
ros2 launch crazyflie_consensus_multi_drone simulation.launch.py \
  num_drones:=3 \
  backend:=mujoco
```

## Configuration

### `config/drone_ids.yaml`
```yaml
drones:
  cf1:
    uri: "radio://0/100/2M/E7E7E7E7E7"
    type: "cf21"
    initial_position: [0.5, 0.0, 0.5]
  cf2:
    uri: "radio://0/101/2M/E7E7E7E7E8"
    type: "cf21"
    initial_position: [-0.25, 0.43, 0.5]
  cf3:
    uri: "radio://0/102/2M/E7E7E7E7E9"
    type: "cf21"
    initial_position: [-0.25, -0.43, 0.5]
```

### `config/consensus_params.yaml`
```yaml
consensus:
  algorithm: "avaerage_consensus"  # or: "max_consensus", "min_consensus"
  gain: 0.1
  update_frequency: 10  # Hz
  convergence_threshold: 0.01
  
topology:
  type: "ring"  # or: "line", "complete_graph", "dynamic"
  update_interval: 5.0  # seconds
  
fault_detection:
  enabled: true
  method: "residual_monitoring"
  threshold: 0.5  # m
  recovery_action: "topology_reconfiguration"
```

## Fault Scenarios

The system supports detection and recovery from multiple fault types:

1. **Abrupt Faults:** Sudden motor or sensor failures
   - Detection: Residual monitoring with configured threshold
   - Recovery: Immediate topology update to exclude faulty drone

2. **Incipient Faults:** Gradual performance degradation
   - Detection: Rate-of-change monitoring
   - Recovery: Gradual weight reduction in consensus

3. **Intermittent Faults:** Transient communication or localization loss
   - Detection: Message timeout and state divergence
   - Recovery: Neighbor re-discovery protocol

Run fault scenarios:
```bash
python3 test/test_fault_scenarios.py --fault_type abrupt --severity 0.8
```

## Performance Metrics

The system logs and analyzes:
- **Convergence time:** Time to reach consensus state
- **Formation stability:** RMS position deviation from desired formation
- **Communication overhead:** Message count and bandwidth
- **Fault detection latency:** Time from fault occurrence to detection
- **Topology changes:** Number and timing of topology reconfigurations

View logged data:
```bash
python3 scripts/analyze_flight.py /path/to/rosbag2_folder
```

## Publications & References

This work implements algorithms from:

1. Consensus Formation Control using Dynamic Topology Reconfiguration for Quadrotor Swarms
2. Experimental validation on Crazyflie 2.1 platforms with Lighthouse localization
3. Rajahmundry Urban Driving Cycle dataset integration (companion work)

See `REFERENCES.md` for full citations.

## Troubleshooting

### Issue: Drones not found
**Solution:** Verify Crazyradio connection and drone URIs
```bash
python3 -c "import cflib; cflib.crtp.scan_interfaces()"
```

### Issue: Position hold unstable after ~5 minutes
**Cause:** IMU thermal drift with default Crazyflie firmware
**Solution:** Apply thermal compensation firmware patch
```bash
# See docs/thermal_drift_mitigation.md
```

### Issue: Lighthouse position jitter
**Solution:** Calibrate Lighthouse geometry and validate base station sync
```bash
ros2 run crazyflie_driver calibrate_lighthouse
```

## Development

### Running Tests
```bash
# Unit tests
colcon test --packages-select crazyflie_consensus_multi_drone

# Integration tests (requires hardware)
python3 test/test_hardware_integration.py --dry_run  # Simulation mode
```

### Code Style
```bash
# Format Python code
black src/
pylint src/

# Format C++ (if applicable)
clang-format -i src/**/*.cpp
```

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Commit changes (`git commit -m 'Add feature'`)
4. Push to branch (`git push origin feature/your-feature`)
5. Open a Pull Request

See `CONTRIBUTING.md` for detailed guidelines.

## License

This project is licensed under the MIT License - see LICENSE file for details.

## Citation

If you use this code in your research, please cite:

```bibtex
@software{gowtham2024consensus,
  author = {Gowtham, Balagani Venkata and Akumalla, Ravi Kiran and Jain, Tushar},
  title = {Crazyflie Consensus Multi-Drone: Fault-Tolerant Formation Control},
  year = {2024},
  url = {https://github.com/Gowtham0517/crazyflie_consensus_multi_drone}
}
```

## Authors

- **Balagani Venkata Gowtham** – Lead Developer (IIT Mandi Internship)
- **Ravi Kiran Akumalla** – Co-author

## Support

For issues, questions, or discussions:
- **GitHub Issues:** https://github.com/Gowtham0517/crazyflie_consensus_multi_drone/issues
- **Email:** venkatagowtham517@gmail.com
- **ROS Discourse:** Tag with `crazyflie` and `consensus`

## Acknowledgments

- Bitcraze for Crazyflie platform and ecosystem
- IIT Mandi for research infrastructure and mentorship
- RGUKT Nuzvid for academic support
- ROS 2 community for framework and tools
