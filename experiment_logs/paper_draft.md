# Resilient Swarm Control of Micro-UAVs Under Dynamic Motor Faults using Bidirectional Ring Phase-based Guidance

**Authors:** [Your Name / Co-authors]

## Abstract
[To be written last - will summarize the motivation, methodology, and key results of our 15 flight trials demonstrating swarm resilience under abrupt, incipient, and intermittent faults.]

## 1. Introduction
The deployment of micro-Unmanned Aerial Vehicle (UAV) swarms has seen rapid growth across diverse applications, ranging from environmental monitoring and search-and-rescue operations to coordinated payload delivery and entertainment displays. As the scale and complexity of these missions increase, the requirement for robust coordination and resilient formation flight becomes paramount. Unlike single-agent systems, multi-agent swarms operate under a decentralized paradigm where the failure of an individual node can propagate through the communication topology, potentially jeopardizing the safety and mission success of the entire fleet.

Among the various subsystems of a micro-UAV, the propulsion system—specifically the motors and propellers—is highly susceptible to wear, physical damage, and unexpected degradation. Motor faults can manifest in several forms: abrupt failures due to collisions or structural breakage, incipient (gradual) failures caused by bearing wear or overheating, and intermittent faults arising from loose electrical connections. When a single UAV within a coordinated swarm experiences such degradation, its inability to accurately track reference trajectories induces localized instability. In tightly coupled formation control strategies, this localized error can quickly destabilize healthy neighboring agents, leading to catastrophic swarm-wide collisions.

While significant research has been dedicated to fault-tolerant control (FTC) for individual quadrotors, ensuring the resilience of an entire swarm against single-node propulsion failures remains a significant challenge. Existing consensus protocols often assume idealized, nominal operating conditions and struggle to maintain formation integrity when an agent's physical capabilities degrade below operational thresholds. 

To address this gap, this paper presents a comprehensive experimental evaluation of a robust bidirectional ring, closed-loop phase-based guidance controller implemented on a swarm of Crazyflie 2.1 nano-quadrotors. We migrate a legacy centralized swarm algorithm into a modular ROS2 (Robot Operating System 2) architecture, enabling decentralized Guidance, Navigation, and Control (GNC). Through an extensive suite of 15 flight experiments, we systematically inject three distinct classes of motor faults (abrupt, incipient, and intermittent) into a single agent (CF1) during a coordinated circular flight task. We evaluate the swarm's consensus stability and analyze the boundary conditions under which the healthy drones (CF2 and CF3) can safely maintain their orbit despite the severe degradation or outright crash of the faulty node.

The primary contributions of this work are:
1. The formulation and real-time implementation of a ROS2-based bidirectional ring phase-guidance controller for micro-UAV swarms.
2. A rigorous experimental methodology categorizing and injecting abrupt, incipient, and intermittent motor faults during dynamic flight.
3. Quantitative analysis of swarm resilience, demonstrating the controller's ability to maintain formation integrity among healthy agents when a neighboring node experiences severe mechanical degradation or complete failure.

## 2. Related Work
[Drafting in progress...]

## 3. System Architecture and Control Design

To achieve reliable and modular swarm operations, we developed a structured Guidance, Navigation, and Control (GNC) architecture using ROS2 (Robot Operating System 2). This architecture decouples the complex centralized logic of legacy scripts into dedicated functional nodes, improving the scalability and real-time responsiveness of the swarm.

### 3.1. Modular ROS2 GNC Framework
The swarm software stack consists of three primary custom ROS2 packages that interact with the core Crazyswarm2 communication driver:

1. **Navigation Node (`swarm_navigation`)**: Responsible for telemetry aggregation. This node continuously subscribes to the localized pose and odometry data of all agents in the swarm, filters the measurements, and broadcasts a unified swarm state estimate.
2. **Guidance Node (`swarm_guidance`)**: Calculates the desired trajectory for each agent based on our consensus protocol. It utilizes a bidirectional ring phase-based guidance algorithm to maintain geometric formation even under dynamic conditions.
3. **Control Node (`central_swarm_control`)**: Acts as the executive state machine for the flight mission. It manages phase transitions (Takeoff, Track, Land), handles the continuous data logging to CSV for post-flight analysis, and most importantly, executes the fault injection framework.

### 3.2. Bidirectional Ring Phase-Based Guidance
At the core of the swarm's coordination is the phase-based guidance law. The drones are logically structured in a bidirectional ring topology, meaning each drone $i$ communicates its phase state with its immediate spatial neighbors $i-1$ and $i+1$. 

During the "Track" phase, the swarm attempts to maintain a synchronized circular orbit. The guidance node calculates the reference position for each agent by updating its phase angle based on the phase errors relative to its neighbors. If one agent falls behind or deviates due to mechanical degradation, the bidirectional coupling ensures that neighboring agents adjust their trajectories to prevent collisions and attempt to restore the global formation.

### 3.3. Fault Injection Framework
To rigorously test the resilience of the guidance controller, a deterministic fault injection mechanism was integrated. Crucially, the actual motor degradation is executed on-board the UAVs through a custom-modified Crazyflie firmware. The ROS2 Control Node acts as the high-level commander, sending fault parameters over the radio link to the target drone (CF1).

Upon receiving the command, the modified firmware directly scales the PWM (Pulse Width Modulation) signals sent to the motor controllers. The injected faults are quantified by a "Motor Health" parameter ($H \in [0, 1]$), where $H=1.0$ represents nominal operation and lower values represent reduced thrust capability. By handling the PWM scaling at the firmware level, we ensure the fault simulation is dynamically realistic and bypasses the latency of off-board control loops.

The framework supports triggering faults based on predefined temporal profiles:
*   **Abrupt Faults:** Instantaneous drops in motor health simulating mechanical breakage (e.g., $H$ drops instantly to 0.7, 0.5, or 0.3).
*   **Incipient Faults:** Linear degradation rates simulating gradual wear, thermal throttling, or battery droop.
*   **Intermittent Faults:** Cyclic toggling of health states (e.g., 10s ON / 10s OFF) simulating loose wiring or temporary electrical shorts.

## 4. Experimental Setup and Methodology

To evaluate the bidirectional ring phase-based guidance controller under realistic conditions, we conducted a systematic campaign of 15 flight experiments using a swarm of three Crazyflie 2.1 nano-quadrotors. 

### 4.1. Hardware and Software Infrastructure
The physical experiments were conducted in an indoor flight arena equipped with a Vicon motion capture system, which provided high-accuracy pose estimates at 100 Hz. The central GNC architecture was executed on a ground station computer running Ubuntu 22.04 and ROS2 Humble. Commands and telemetry were bridged to the drones via the Crazyradio PA using the Crazyswarm2 driver. As detailed in Section 3.3, CF1 was flashed with our custom fault-injection firmware, while CF2 and CF3 ran standard firmware.

### 4.2. Flight Task
For all experiments, the swarm was tasked with a coordinated circular tracking mission:
1. **Takeoff:** The drones ascend to a predefined hover altitude.
2. **Track (Nominal):** The swarm initiates a circular orbit (radius = 0.8m) maintaining a 120° phase separation using the bidirectional ring consensus law.
3. **Fault Injection:** Once steady-state tracking is achieved, a deterministic fault profile is triggered on CF1.
4. **Recovery / Land:** The system logs the swarm's response before commanding a safe landing.

### 4.3. Test Matrix
Our methodology encompasses 10 distinct fault scenarios categorized by the temporal profile of the degradation:
*   **Baseline (1 trial):** Nominal flight with no faults injected.
*   **Abrupt Faults (5 trials):** Health of CF1 dropped instantaneously to 0.7, 0.5, and 0.3. Trials at $H=0.5$ and $H=0.3$ were conducted both with and without the fault-tolerant (FT) consensus logic engaged to provide an ablation study.
*   **Incipient Faults (6 trials):** Health linearly degraded at Slow, Medium, and Fast rates. Each rate was tested with and without FT logic.
*   **Intermittent Faults (3 trials):** Health oscillated between nominal and degraded states using 10-second symmetric, 3-second symmetric, and asymmetric (5s ON / 15s OFF) duty cycles.

## 5. Results and Discussion
[Drafting in progress...]

## 6. Conclusion and Future Work
[Drafting in progress...]
