# Crazyflie Swarm Consensus — Experiment Logs

Flight telemetry data from a 3-drone Crazyflie swarm performing circular formation tracking under various motor fault injection scenarios. The swarm uses a **bidirectional ring, closed-loop phase-based consensus controller** for coordinated trajectory tracking.

**Date:** 2026-06-26  
**Drones:** `cf1`, `cf2`, `cf3`  
**Formation:** Circular orbit (radius ≈ 0.6 m, altitude 0.5 m)  
**Track Duration:** 60 s per experiment  
**Fault Target Motor:** `m1` (on `cf1`)  
**Fault Injection Delay:** 10 s after tracking begins  

---

## Directory Structure

```
experiment_logs/
├── 00_baseline/          ← No-fault reference
├── 01_abrupt_faults/     ← Sudden motor health reduction
├── 02_incipient_faults/  ← Gradual motor degradation
├── 03_intermittent_faults/ ← Cyclic on/off fault pattern
└── README.md             ← This file
```

Experiments with both fault-tolerant (FT) and non-fault-tolerant variants are split into `with_FT/` and `without_FT/` subdirectories.

---

## Experiment Matrix

### 00 — Baseline

| ID | Fault Type | Health | FT | Folder Path | Timestamp |
|----|-----------|--------|-----|-------------|-----------|
| B1 | none | 1.0 (no fault) | — | `00_baseline/B1_no_fault/run_1/` | 20260626_151652 |

### 01 — Abrupt Faults

Motor health is **instantly reduced** to the specified level at `t = 10 s`.

| ID | Health After Fault | FT | Folder Path | Timestamp |
|----|-------------------|-----|-------------|-----------|
| A1 | 0.7 (mild) | — | `01_abrupt_faults/A1_health_0.7/run_1/` | 20260626_004811 |
| A2 | 0.5 (moderate) | ✗ | `01_abrupt_faults/A2_health_0.5/without_FT/run_1/` | 20260626_153501 |
| A2 | 0.5 (moderate) | ✓ | `01_abrupt_faults/A2_health_0.5/with_FT/run_1/` | 20260626_154240 |
| A3 | 0.3 (severe) | ✗ | `01_abrupt_faults/A3_health_0.3/without_FT/run_1/` | 20260626_005936 |
| A3 | 0.3 (severe) | ✓ | `01_abrupt_faults/A3_health_0.3/with_FT/run_1/` | 20260626_172221 |

### 02 — Incipient Faults

Motor health **gradually degrades** from initial magnitude (0.7) toward `min_health` (0.5) at a configurable `fault_rate` per `step_interval`.

| ID | Degradation Rate | FT | Folder Path | Timestamp |
|----|-----------------|-----|-------------|-----------|
| I1 | 0.02 (slow) | ✗ | `02_incipient_faults/I1_slow/without_FT/run_1/` | 20260626_010444 |
| I1 | 0.02 (slow) | ✓ | `02_incipient_faults/I1_slow/with_FT/run_1/` | 20260626_154708 |
| I2 | 0.05 (medium) | ✗ | `02_incipient_faults/I2_medium/without_FT/run_1/` | 20260626_010941 |
| I2 | 0.05 (medium) | ✓ | `02_incipient_faults/I2_medium/with_FT/run_1/` | 20260626_161105 |
| I3 | 0.10 (fast) | ✗ | `02_incipient_faults/I3_fast/without_FT/run_1/` | 20260626_011540 |
| I3 | 0.10 (fast) | ✓ | `02_incipient_faults/I3_fast/with_FT/run_1/` | 20260626_172435 |

### 03 — Intermittent Faults

Motor health oscillates between **faulty** and **healthy** states in a cyclic pattern.

| ID | On/Off Cycle | FT | Folder Path | Timestamp |
|----|-------------|-----|-------------|-----------|
| T1 | 10 s / 10 s (symmetric) | — | `03_intermittent_faults/T1_10s_cycle/run_1/` | 20260626_020750 |
| T2 | 3 s / 3 s (fast cycle) | — | `03_intermittent_faults/T2_3s_cycle/run_1/` | 20260626_021918 |
| T3 | 5 s / 15 s (asymmetric) | — | `03_intermittent_faults/T3_asymmetric/run_1/` | 20260626_022300 |

---

## CSV Column Dictionary

Each `flight_data_*.csv` contains **60 columns** of telemetry sampled at ~50 Hz. Below is the column mapping per drone (`cfX` = `cf1`, `cf2`, or `cf3`):

### Global Columns (3)

| Column | Description | Unit |
|--------|-------------|------|
| `timestamp_sec` | Elapsed time since experiment start | seconds |
| `flight_state` | Current state machine phase | `TAKEOFF`, `TRACK`, `LAND` |
| `fault_type` | Active fault type label | `none`, `abrupt`, `incipient`, `intermittent` |
| `fault_motor_health` | Current health factor of faulted motor (1.0 = healthy) | dimensionless |

### Per-Drone Columns (19 × 3 drones = 57)

For each drone `cfX`:

| Column | Description | Unit |
|--------|-------------|------|
| `cfX_X`, `cfX_Y`, `cfX_Z` | Position from Kalman estimator | meters |
| `cfX_VelX`, `cfX_VelY`, `cfX_VelZ` | Velocity estimate | m/s |
| `cfX_Roll`, `cfX_Pitch`, `cfX_Yaw` | Euler attitude angles | degrees |
| `cfX_GyroX`, `cfX_GyroY`, `cfX_GyroZ` | Angular rate from gyroscope | deg/s |
| `cfX_Motor1` – `cfX_Motor4` | Motor PWM command values | PWM ticks (0–65535) |
| `cfX_targ_x`, `cfX_targ_y`, `cfX_targ_z` | Consensus target position | meters |

> **Note:** Motor PWM values are `0.0` during `TAKEOFF` and `LAND` phases (firmware handles those autonomously). Non-zero PWM values appear during the `TRACK` phase.

---

## Metadata Format

Each `experiment_meta_*.txt` file contains key-value pairs describing the experiment configuration:

| Field | Description |
|-------|-------------|
| `experiment_id` | Unique experiment identifier (e.g., `A1_abrupt_health_0.7`) |
| `run_number` | Run index for repeated experiments |
| `fault_type` | `none`, `abrupt`, `incipient`, or `intermittent` |
| `fault_motor` | Target motor (`m1`) |
| `fault_magnitude` | Initial health value applied at fault onset |
| `fault_rate` | Degradation rate per step (for incipient faults) |
| `fault_step_interval` | Time between degradation steps (seconds) |
| `fault_min_health` | Floor health value for degradation |
| `fault_on_time` | Duration of fault-active window (intermittent) |
| `fault_off_time` | Duration of fault-inactive window (intermittent) |
| `fault_start_delay` | Delay before fault injection begins (seconds) |
| `track_duration` | Total tracking phase duration (seconds) |
| `timestamp` | Experiment start time (`YYYYMMDD_HHMMSS`) |

---

## Quick-Access Paths for Analysis Scripts

```python
import os

BASE = "experiment_logs"

EXPERIMENTS = {
    # Baseline
    "B1_baseline":      "00_baseline/B1_no_fault/run_1",
    
    # Abrupt faults
    "A1_abrupt_0.7":    "01_abrupt_faults/A1_health_0.7/run_1",
    "A2_abrupt_0.5":    "01_abrupt_faults/A2_health_0.5/without_FT/run_1",
    "A2_abrupt_0.5_FT": "01_abrupt_faults/A2_health_0.5/with_FT/run_1",
    "A3_abrupt_0.3":    "01_abrupt_faults/A3_health_0.3/without_FT/run_1",
    "A3_abrupt_0.3_FT": "01_abrupt_faults/A3_health_0.3/with_FT/run_1",
    
    # Incipient faults
    "I1_slow":          "02_incipient_faults/I1_slow/without_FT/run_1",
    "I1_slow_FT":       "02_incipient_faults/I1_slow/with_FT/run_1",
    "I2_medium":        "02_incipient_faults/I2_medium/without_FT/run_1",
    "I2_medium_FT":     "02_incipient_faults/I2_medium/with_FT/run_1",
    "I3_fast":          "02_incipient_faults/I3_fast/without_FT/run_1",
    "I3_fast_FT":       "02_incipient_faults/I3_fast/with_FT/run_1",
    
    # Intermittent faults
    "T1_10s":           "03_intermittent_faults/T1_10s_cycle/run_1",
    "T2_3s":            "03_intermittent_faults/T2_3s_cycle/run_1",
    "T3_asymmetric":    "03_intermittent_faults/T3_asymmetric/run_1",
}

def get_csv(key):
    """Get the flight data CSV path for an experiment."""
    d = os.path.join(BASE, EXPERIMENTS[key])
    csvs = [f for f in os.listdir(d) if f.endswith('.csv')]
    return os.path.join(d, csvs[0])

def get_meta(key):
    """Get the metadata TXT path for an experiment."""
    d = os.path.join(BASE, EXPERIMENTS[key])
    txts = [f for f in os.listdir(d) if f.endswith('.txt')]
    return os.path.join(d, txts[0])
```

---

## File Inventory

| # | Experiment | CSV Size | Data File |
|---|-----------|----------|-----------|
| 1 | B1 baseline | 3.3 MB | `flight_data_20260626_151652.csv` |
| 2 | A1 abrupt 0.7 | 3.4 MB | `flight_data_20260626_004811.csv` |
| 3 | A2 abrupt 0.5 (no FT) | 3.3 MB | `flight_data_20260626_153501.csv` |
| 4 | A2 abrupt 0.5 (FT) | 3.0 MB | `flight_data_20260626_154240.csv` |
| 5 | A3 abrupt 0.3 (no FT) | 3.4 MB | `flight_data_20260626_005936.csv` |
| 6 | A3 abrupt 0.3 (FT) | 3.0 MB | `flight_data_20260626_172221.csv` |
| 7 | I1 slow (no FT) | 3.4 MB | `flight_data_20260626_010444.csv` |
| 8 | I1 slow (FT) | 3.3 MB | `flight_data_20260626_154708.csv` |
| 9 | I2 medium (no FT) | 3.4 MB | `flight_data_20260626_010941.csv` |
| 10 | I2 medium (FT) | 3.3 MB | `flight_data_20260626_161105.csv` |
| 11 | I3 fast (no FT) | 3.4 MB | `flight_data_20260626_011540.csv` |
| 12 | I3 fast (FT) | 3.1 MB | `flight_data_20260626_172435.csv` |
| 13 | T1 10s cycle | 3.4 MB | `flight_data_20260626_020750.csv` |
| 14 | T2 3s cycle | 3.4 MB | `flight_data_20260626_021918.csv` |
| 15 | T3 asymmetric | 3.4 MB | `flight_data_20260626_022300.csv` |

**Total: 15 experiments, ~49.4 MB of flight telemetry**
