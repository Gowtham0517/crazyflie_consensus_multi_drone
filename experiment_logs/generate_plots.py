#!/usr/bin/env python3
"""
Generate publication-quality plots for all Crazyflie swarm fault experiments.
Produces 6 figures per experiment in a mirrored plots/ directory structure.

Usage:
    python3 generate_plots.py
"""

import os
import sys
import numpy as np
import pandas as pd
import matplotlib
matplotlib.use('Agg')  # Non-interactive backend
import matplotlib.pyplot as plt
from matplotlib.lines import Line2D
import matplotlib.gridspec as gridspec
from pathlib import Path

# ── Global Style ──────────────────────────────────────────────────────────
plt.rcParams.update({
    'font.family': 'serif',
    'font.size': 15,
    'axes.titlesize': 17,
    'axes.labelsize': 16,
    'xtick.labelsize': 14,
    'ytick.labelsize': 14,
    'legend.fontsize': 13,
    'figure.dpi': 600,
    'savefig.dpi': 600,
    'savefig.bbox': 'tight',
    'savefig.pad_inches': 0.1,
    'axes.grid': True,
    'grid.alpha': 0.3,
    'lines.linewidth': 2.5,
    'axes.spines.top': False,
    'axes.spines.right': False,
})

# Drone color palette (colorblind-friendly)
COLORS = {
    'cf1': '#E24A33',   # red-orange (faulted drone)
    'cf2': '#348ABD',   # blue
    'cf3': '#988ED5',   # purple
    'target': '#777777', # gray for targets
}
DRONE_LABELS = {'cf1': 'CF1', 'cf2': 'CF2', 'cf3': 'CF3'}

# Experiment registry (mirrors README.md)
BASE = Path(__file__).parent
EXPERIMENTS = {
    "B1_baseline":      "00_baseline/B1_no_fault/run_1",
    "A1_abrupt_0.7":    "01_abrupt_faults/A1_health_0.7/run_1",
    "A2_abrupt_0.5_noFT": "01_abrupt_faults/A2_health_0.5/without_FT/run_1",
    "A2_abrupt_0.5_FT": "01_abrupt_faults/A2_health_0.5/with_FT/run_1",
    "A3_abrupt_0.3_noFT": "01_abrupt_faults/A3_health_0.3/without_FT/run_1",
    "A3_abrupt_0.3_FT": "01_abrupt_faults/A3_health_0.3/with_FT/run_1",
    "I1_slow_noFT":     "02_incipient_faults/I1_slow/without_FT/run_1",
    "I1_slow_FT":       "02_incipient_faults/I1_slow/with_FT/run_1",
    "I2_medium_noFT":   "02_incipient_faults/I2_medium/without_FT/run_1",
    "I2_medium_FT":     "02_incipient_faults/I2_medium/with_FT/run_1",
    "I3_fast_noFT":     "02_incipient_faults/I3_fast/without_FT/run_1",
    "I3_fast_FT":       "02_incipient_faults/I3_fast/with_FT/run_1",
    "T1_10s":           "03_intermittent_faults/T1_10s_cycle/run_1",
    "T2_3s":            "03_intermittent_faults/T2_3s_cycle/run_1",
    "T3_asymmetric":    "03_intermittent_faults/T3_asymmetric/run_1",
}

# Pretty titles for each experiment
TITLES = {
    "B1_baseline":        "B1 — Baseline (No Fault)",
    "A1_abrupt_0.7":      "A1 — Abrupt Fault (Health = 0.7)",
    "A2_abrupt_0.5_noFT": "A2 — Abrupt Fault (Health = 0.5, No FT)",
    "A2_abrupt_0.5_FT":   "A2 — Abrupt Fault (Health = 0.5, With FT)",
    "A3_abrupt_0.3_noFT": "A3 — Abrupt Fault (Health = 0.3, No FT)",
    "A3_abrupt_0.3_FT":   "A3 — Abrupt Fault (Health = 0.3, With FT)",
    "I1_slow_noFT":       "I1 — Incipient Slow (Rate = 0.02, No FT)",
    "I1_slow_FT":         "I1 — Incipient Slow (Rate = 0.02, With FT)",
    "I2_medium_noFT":     "I2 — Incipient Medium (Rate = 0.05, No FT)",
    "I2_medium_FT":       "I2 — Incipient Medium (Rate = 0.05, With FT)",
    "I3_fast_noFT":       "I3 — Incipient Fast (Rate = 0.10, No FT)",
    "I3_fast_FT":         "I3 — Incipient Fast (Rate = 0.10, With FT)",
    "T1_10s":             "T1 — Intermittent (10s ON / 10s OFF)",
    "T2_3s":              "T2 — Intermittent (3s ON / 3s OFF)",
    "T3_asymmetric":      "T3 — Intermittent Asymmetric (5s ON / 15s OFF)",
}


def find_csv(run_dir):
    """Find the flight_data CSV in a run directory."""
    csvs = list(Path(run_dir).glob("flight_data_*.csv"))
    if not csvs:
        raise FileNotFoundError(f"No CSV found in {run_dir}")
    return csvs[0]


def load_data(csv_path):
    """Load and preprocess flight data."""
    df = pd.read_csv(csv_path)
    # Strip whitespace from column names (Windows line endings)
    df.columns = df.columns.str.strip()
    return df


def get_track_mask(df):
    """Return boolean mask for TRACK phase only."""
    return df['flight_state'].str.strip() == 'TRACK'


def compute_tracking_error(df, drone):
    """Compute Euclidean distance between position and target."""
    dx = df[f'{drone}_X'] - df[f'{drone}_targ_x']
    dy = df[f'{drone}_Y'] - df[f'{drone}_targ_y']
    dz = df[f'{drone}_Z'] - df[f'{drone}_targ_z']
    return np.sqrt(dx**2 + dy**2 + dz**2)


def detect_crash(df, drone, z_threshold=0.1):
    """Detect if a drone crashed (Z drops below threshold during TRACK)."""
    track = get_track_mask(df)
    if track.sum() == 0:
        return False
    z_track = df.loc[track, f'{drone}_Z']
    return z_track.iloc[-1] < z_threshold if len(z_track) > 0 else False


# ── Plot Functions ────────────────────────────────────────────────────────

def plot_xyz_position(df, title, save_path):
    """X, Y, Z position vs time for all drones."""
    fig, axes = plt.subplots(3, 1, figsize=(10, 8), sharex=True)

    t = df['timestamp_sec'] - df['timestamp_sec'].iloc[0]

    axis_labels = ['X', 'Y', 'Z']
    for i, axis in enumerate(axis_labels):
        for drone in ['cf1', 'cf2', 'cf3']:
            axes[i].plot(t, df[f'{drone}_{axis}'], color=COLORS[drone],
                         label=DRONE_LABELS[drone], linewidth=2.5, alpha=1.0)
        axes[i].set_ylabel(f'{axis} (m)')
        if axis == 'Z':
            max_z = max([df[f'{drone}_Z'].max() for drone in ['cf1', 'cf2', 'cf3']])
            z_limit = max(0.6, max_z + 0.1)
            if "t2_3s" in str(save_path).lower():
                z_limit = max(1.4, z_limit)
            elif "t3_asymmetric" in str(save_path).lower():
                z_limit = max(0.9, z_limit)
            axes[i].set_ylim(0, z_limit)
        else:
            max_val = max([df[f'{drone}_{axis}'].abs().max() for drone in ['cf1', 'cf2', 'cf3']])
            limit = max(1.0, max_val + 0.1)
            axes[i].set_ylim(-limit, limit)
        axes[i].set_xlim(0, 75)

    axes[0].legend(bbox_to_anchor=(1.02, 1), loc='upper left', borderaxespad=0)
    axes[-1].set_xlabel('Time (s)')
    fig.tight_layout()
    fig.savefig(save_path)
    plt.close(fig)


def plot_xy_trajectory(df, title, save_path):
    """Top-down XY trajectory with start/end markers."""
    fig, ax = plt.subplots(figsize=(8, 6.5))

    for drone in ['cf1', 'cf2', 'cf3']:
        ax.plot(df[f'{drone}_X'], df[f'{drone}_Y'],
                color=COLORS[drone], label=DRONE_LABELS[drone], linewidth=2.5, alpha=1.0)
        if len(df) > 0:
            ax.scatter(df[f'{drone}_X'].iloc[0], df[f'{drone}_Y'].iloc[0],
                       color=COLORS[drone], s=80, marker='o', zorder=5, edgecolors='black', linewidths=0.5)
            ax.scatter(df[f'{drone}_X'].iloc[-1], df[f'{drone}_Y'].iloc[-1],
                       color=COLORS[drone], s=80, marker='X', zorder=5, edgecolors='black', linewidths=0.5)

    ax.set_xlabel('X (m)')
    ax.set_ylabel('Y (m)')
    
    max_val_x = max([df[f'{drone}_X'].abs().max() for drone in ['cf1', 'cf2', 'cf3']])
    max_val_y = max([df[f'{drone}_Y'].abs().max() for drone in ['cf1', 'cf2', 'cf3']])
    limit = max(1.0, max_val_x + 0.1, max_val_y + 0.1)
    
    ax.set_xlim(-limit, limit)
    ax.set_ylim(-limit, limit)
    ax.set_aspect('equal')
    ax.legend(bbox_to_anchor=(1.02, 1), loc='upper left', borderaxespad=0)

    fig.savefig(save_path)
    plt.close(fig)


def plot_tracking_error(df, title, save_path):
    """Tracking error over time for each drone."""
    fig, axes = plt.subplots(3, 1, figsize=(10, 8), sharex=True)

    t = df['timestamp_sec'] - df['timestamp_sec'].iloc[0]

    for i, drone in enumerate(['cf1', 'cf2', 'cf3']):
        err = compute_tracking_error(df, drone)

        # Clip extreme values for better visualization
        err_clipped = err.clip(upper=err.quantile(0.99) * 1.5) if len(err) > 10 else err

        axes[i].plot(t, err_clipped, color=COLORS[drone], linewidth=2.5, alpha=1.0)
        axes[i].fill_between(t, 0, err_clipped, color=COLORS[drone], alpha=0.2)
        axes[i].set_ylabel(f'{DRONE_LABELS[drone]} Error (m)')
        axes[i].set_ylim(bottom=0)
        axes[i].set_xlim(0, 75)

        # Add RMS annotation (computed on TRACK phase only for metric correctness)
        track = get_track_mask(df)
        err_track = compute_tracking_error(df[track], drone) if track.sum() > 0 else err
        rms = np.sqrt(np.mean(err_track**2))
        axes[i].annotate(f'RMS = {rms:.3f} m', xy=(0.98, 0.85),
                         xycoords='axes fraction', ha='right', fontsize=11,
                         bbox=dict(boxstyle='round,pad=0.3', facecolor='white', alpha=0.8))

    axes[-1].set_xlabel('Time (s)')
    fig.tight_layout()
    fig.savefig(save_path)
    plt.close(fig)


def plot_motor_health(df, title, save_path):
    """Motor health timeline."""
    fig, ax1 = plt.subplots(figsize=(10, 3.5))

    t = df['timestamp_sec']
    if len(t) > 0:
        t = t - t.iloc[0]

    # ── Motor health ──
    ax1.plot(t, df['fault_motor_health'], color='#E24A33', linewidth=3.0)
    ax1.fill_between(t, df['fault_motor_health'], 1.0, color='#E24A33', alpha=0.2)
    ax1.set_ylabel('Motor Health')
    ax1.set_ylim(-0.05, 1.1)

    # Add flight state bands
    states = df['flight_state'].str.strip()
    state_colors = {'TAKEOFF': '#FDE725', 'TRACK': '#21918C', 'LAND': '#440154'}
    for state, color in state_colors.items():
        mask = states == state
        if mask.any():
            ax1.fill_between(t, -0.05, -0.02, where=mask, color=color, alpha=0.7)
    # State legend
    state_handles = [plt.Rectangle((0,0),1,1, color=c, alpha=0.7) for c in state_colors.values()]
    ax1.legend(state_handles, state_colors.keys(), bbox_to_anchor=(1.02, 1), loc='upper left', borderaxespad=0)

    ax1.set_xlabel('Time (s)')
    ax1.set_xlim(0, 75)

    fig.tight_layout()
    fig.savefig(save_path)
    plt.close(fig)


def plot_motor_pwm(df, title, save_path):
    """Motor PWM commands for the faulted drone (cf1) over time."""
    fig, ax = plt.subplots(figsize=(10, 4.5))

    t = df['timestamp_sec'] - df['timestamp_sec'].iloc[0]

    motor_colors = ['#E24A33', '#348ABD', '#988ED5', '#8EBA42']
    motor_labels = ['Motor 1', 'Motor 2', 'Motor 3', 'Motor 4']

    for i in range(1, 5):
        col = f'cf1_Motor{i}'
        if col in df.columns:
            pwm = df[col]
            pwm_clipped = pwm.clip(upper=65535)
            ax.plot(t, pwm_clipped, color=motor_colors[i-1],
                    label=motor_labels[i-1], linewidth=1.5, alpha=1.0)

    ax.set_xlabel('Time (s)')
    ax.set_ylabel('PWM Command')
    ax.ticklabel_format(style='plain', axis='y')
    ax.set_xlim(0, 75)
    ax.legend(bbox_to_anchor=(1.02, 1), loc='upper left', borderaxespad=0)

    fig.tight_layout()
    fig.savefig(save_path)
    plt.close(fig)


def plot_attitude(df, title, save_path):
    """Roll, Pitch, and Yaw angles for all drones — shows instability from faults."""
    fig, axes = plt.subplots(3, 1, figsize=(10, 8.5), sharex=True, sharey=True)

    t = df['timestamp_sec'] - df['timestamp_sec'].iloc[0]

    for drone in ['cf1', 'cf2', 'cf3']:
        roll = df[f'{drone}_Roll'].clip(lower=-180, upper=180)
        pitch = df[f'{drone}_Pitch'].clip(lower=-180, upper=180)
        yaw = df[f'{drone}_Yaw'].clip(lower=-180, upper=180)

        axes[0].plot(t, roll, color=COLORS[drone],
                     label=DRONE_LABELS[drone], linewidth=1.5, alpha=1.0)
        axes[1].plot(t, pitch, color=COLORS[drone],
                     label=DRONE_LABELS[drone], linewidth=1.5, alpha=1.0)
        axes[2].plot(t, yaw, color=COLORS[drone],
                     label=DRONE_LABELS[drone], linewidth=1.5, alpha=1.0)

    axes[0].set_ylabel('Roll (°)')
    axes[0].legend(bbox_to_anchor=(1.02, 1), loc='upper left', borderaxespad=0)
    axes[0].axhline(y=0, color='gray', linestyle='-', linewidth=0.5, alpha=0.3)

    axes[1].set_ylabel('Pitch (°)')
    axes[1].axhline(y=0, color='gray', linestyle='-', linewidth=0.5, alpha=0.3)

    axes[2].set_ylabel('Yaw (°)')
    axes[2].set_xlabel('Time (s)')
    axes[2].axhline(y=0, color='gray', linestyle='-', linewidth=0.5, alpha=0.3)
    axes[0].set_xlim(0, 75)

    fig.tight_layout()
    fig.savefig(save_path)
    plt.close(fig)


# ── Main ──────────────────────────────────────────────────────────────────

def main():
    plots_dir = BASE / "plots"
    plots_dir.mkdir(exist_ok=True)

    total = len(EXPERIMENTS)
    generated = 0

    for idx, (key, rel_path) in enumerate(EXPERIMENTS.items(), 1):
        run_dir = BASE / rel_path
        title = TITLES[key]
        print(f"\n[{idx}/{total}] {key}")
        print(f"  Source: {rel_path}")

        try:
            csv_path = find_csv(run_dir)
        except FileNotFoundError as e:
            print(f"  ⚠ SKIPPED: {e}")
            continue

        df = load_data(csv_path)
        print(f"  Loaded {len(df)} rows, {len(df.columns)} columns")

        # Create output directory mirroring experiment path
        out_dir = plots_dir / rel_path.replace('/run_1', '')
        out_dir.mkdir(parents=True, exist_ok=True)

        # Generate all plots
        plots = [
            ("01_xyz_position.pdf",          plot_xyz_position),
            ("02_xy_trajectory.pdf",         plot_xy_trajectory),
            ("03_tracking_error.pdf",        plot_tracking_error),
            ("motor_health.pdf",             plot_motor_health),
            ("05_motor_pwm_cf1.pdf",         plot_motor_pwm),
            ("06_attitude.pdf",              plot_attitude),
        ]

        for fname, plot_fn in plots:
            save_path = out_dir / fname
            try:
                plot_fn(df, title, save_path)
                print(f"  ✓ {fname}")
            except Exception as e:
                print(f"  ✗ {fname}: {e}")

        generated += 1

    print(f"\n{'='*60}")
    print(f"Done! Generated plots for {generated}/{total} experiments.")
    print(f"Output: {plots_dir}/")


if __name__ == '__main__':
    main()
