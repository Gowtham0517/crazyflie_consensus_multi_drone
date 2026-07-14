from pathlib import Path
from shutil import which

import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation

# ==========================================
# 1. CONFIGURATION & DATA LOADING
# ==========================================
# Choose which CSV file to visualize
CSV_FILE = "flight_data_20260626_0059361.csv"
OUTPUT_VIDEO = "flight_analysis_sync.mp4"

# Resolve the file path relative to this script, and search if needed.
base_dir = Path(__file__).resolve().parent
csv_path = base_dir / CSV_FILE
if not csv_path.exists():
    matches = list(base_dir.rglob(CSV_FILE))
    if len(matches) == 1:
        csv_path = matches[0]
        print(f"Found CSV file at: {csv_path}")
    elif len(matches) > 1:
        raise FileNotFoundError(
            f"Multiple CSV files named '{CSV_FILE}' were found:\n" + "\n".join(str(m) for m in matches)
        )
    else:
        raise FileNotFoundError(
            f"Could not find '{CSV_FILE}' in {base_dir}\n"
            "Please set CSV_FILE to the correct relative path inside the workspace."
        )

# Load data
df = pd.read_csv(csv_path)

# Drop any repeated header rows that may have been embedded in the CSV.
header_values = list(df.columns.astype(str))
repeat_header_mask = df.apply(lambda row: row.astype(str).tolist() == header_values, axis=1)
if repeat_header_mask.any():
    df = df.loc[~repeat_header_mask].reset_index(drop=True)

# Convert expected numeric fields to floats, allowing CSVs with string-encoded numbers.
numeric_columns = [
    'timestamp_sec', 'fault_motor_health',
    'cf1_X', 'cf1_Y', 'cf2_X', 'cf2_Y', 'cf3_X', 'cf3_Y'
]
for col in numeric_columns:
    if col not in df.columns:
        raise KeyError(f"Expected numeric column '{col}' was not found in the CSV.")
    df[col] = pd.to_numeric(df[col], errors='coerce')

if df[numeric_columns].isna().any().any():
    bad_cols = df[numeric_columns].columns[df[numeric_columns].isna().any()].tolist()
    raise ValueError(
        f"Non-numeric or missing values found in numeric columns: {bad_cols}. "
        "Please clean the CSV or verify column formatting."
    )

# Downsampling factor: The data is sampled at 50Hz (every 0.02s). 
# To speed up rendering and match standard video frame rates, we step through rows.
# Step = 2 means 25 frames per second in the video scale.
STEP = 2 
df_sampled = df.iloc[::STEP].reset_index(drop=True)

# Extract time and health arrays
timestamps = df_sampled['timestamp_sec'].values
motor_health = df_sampled['fault_motor_health'].values

# Extract drone trajectories
cf1_x, cf1_y = df_sampled['cf1_X'].values, df_sampled['cf1_Y'].values
cf2_x, cf2_y = df_sampled['cf2_X'].values, df_sampled['cf2_Y'].values
cf3_x, cf3_y = df_sampled['cf3_X'].values, df_sampled['cf3_Y'].values

# ==========================================
# 2. PLOT SETUP
# ==========================================
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 6))
fig.suptitle(f"Synchronized Swarm Telemetry & Fault Analysis\nSource: {CSV_FILE}", fontsize=14, fontweight='bold')

# --- Panel 1: X-Y Trajectory ---
ax1.set_title("Live X-Y Swarm Trajectory", fontsize=12)
ax1.set_xlabel("X Position (m)")
ax1.set_ylabel("Y Position (m)")
ax1.grid(True, linestyle='--', alpha=0.6)

# Set dynamic or fixed axis limits based on data boundaries
all_x = np.concatenate([cf1_x, cf2_x, cf3_x])
all_y = np.concatenate([cf1_y, cf2_y, cf3_y])
ax1.set_xlim(all_x.min() - 0.2, all_x.max() + 0.2)
ax1.set_ylim(all_y.min() - 0.2, all_y.max() + 0.2)

# Trajectory line objects (historical trail)
trail_cf1, = ax1.plot([], [], 'r-', alpha=0.4, label='cf1 path')
trail_cf2, = ax1.plot([], [], 'g-', alpha=0.4, label='cf2 path')
trail_cf3, = ax1.plot([], [], 'b-', alpha=0.4, label='cf3 path')

# Current position marker objects
pos_cf1, = ax1.plot([], [], 'ro', markersize=8, label='cf1 (Active)')
pos_cf2, = ax1.plot([], [], 'go', markersize=8, label='cf2 (Active)')
pos_cf3, = ax1.plot([], [], 'bo', markersize=8, label='cf3 (Active)')
ax1.legend(loc='upper right')

# --- Panel 2: Fault Injection Graph ---
ax2.set_title("Fault Injection Status (Motor Health)", fontsize=12)
ax2.set_xlabel("Time (seconds)")
ax2.set_ylabel("Motor Health Factor")
ax2.set_xlim(timestamps.min(), timestamps.max())
ax2.set_ylim(motor_health.min() - 0.1, motor_health.max() + 0.1)
ax2.grid(True, linestyle='--', alpha=0.6)

# Plot full baseline health curve in the background
ax2.plot(timestamps, motor_health, 'k--', alpha=0.3, label='Full Schedule')
# Dynamic line tracking the progression
fault_line, = ax2.plot([], [], 'm-', linewidth=2, label='Current Execution')
# Current time vertical tracking bar
time_bar = ax2.axvline(x=0, color='r', linestyle=':', alpha=0.8)
ax2.legend(loc='lower left')

# Text element for live statistics overlay
stats_text = ax1.text(0.02, 0.02, '', transform=ax1.transAxes, 
                      bbox=dict(facecolor='white', alpha=0.8, boxstyle='round,pad=0.5'))

# ==========================================
# 3. ANIMATION CORE LOGIC
# ==========================================
def init():
    """Initializes empty plot components."""
    trail_cf1.set_data([], [])
    trail_cf2.set_data([], [])
    trail_cf3.set_data([], [])
    pos_cf1.set_data([], [])
    pos_cf2.set_data([], [])
    pos_cf3.set_data([], [])
    fault_line.set_data([], [])
    stats_text.set_text('')
    return trail_cf1, trail_cf2, trail_cf3, pos_cf1, pos_cf2, pos_cf3, fault_line, stats_text

def update(frame):
    """Updates handles for each frame step."""
    current_time = timestamps[frame]
    
    # Update Trajectory trails (0 up to current frame)
    trail_cf1.set_data(cf1_x[:frame+1], cf1_y[:frame+1])
    trail_cf2.set_data(cf2_x[:frame+1], cf2_y[:frame+1])
    trail_cf3.set_data(cf3_x[:frame+1], cf3_y[:frame+1])
    
    # Update Current Positions (using lists to unpack safely)
    pos_cf1.set_data([cf1_x[frame]], [cf1_y[frame]])
    pos_cf2.set_data([cf2_x[frame]], [cf2_y[frame]])
    pos_cf3.set_data([cf3_x[frame]], [cf3_y[frame]])
    
    # Update Fault Chart
    fault_line.set_data(timestamps[:frame+1], motor_health[:frame+1])
    time_bar.set_xdata([current_time])
    
    # Highlight status updates if health degrades
    current_health = motor_health[frame]
    state_str = df_sampled['flight_state'].iloc[frame]
    
    if current_health < 1.0:
        pos_cf1.set_color('darkred')
        pos_cf1.set_marker('X')  # Change marker style when fault triggers
        status = f"FAULT ACTIVE ({df_sampled['fault_type'].iloc[frame]})"
    else:
        pos_cf1.set_color('red')
        pos_cf1.set_marker('o')
        status = "NOMINAL"
        
    stats_text.set_text(
        f"Time: {current_time:.2f}s\n"
        f"State: {state_str}\n"
        f"Health Status: {status}\n"
        f"Health Value: {current_health:.2f}"
    )
    
    return trail_cf1, trail_cf2, trail_cf3, pos_cf1, pos_cf2, pos_cf3, fault_line, stats_text

# ==========================================
# 4. RENDER AND SAVE
# ==========================================
total_frames = len(df_sampled)
print(f"Starting compilation of {total_frames} video frames...")

ani = animation.FuncAnimation(
    fig, update, frames=total_frames, init_func=init, blit=True, interval=40
)

# Initialize standard MP4 video writer using ffmpeg, fallback to GIF when missing.
if which("ffmpeg"):
    writer = animation.FFMpegWriter(fps=25, metadata=dict(artist='Drone Swarm Lab'), bitrate=1800)
    ani.save(OUTPUT_VIDEO, writer=writer)
    print(f"Video visualization successfully generated and saved to: {OUTPUT_VIDEO}")
else:
    gif_output = OUTPUT_VIDEO.replace('.mp4', '.gif')
    print("ffmpeg not found; falling back to GIF output.")
    ani.save(gif_output, writer='imagemagick', fps=25)
    print(f"Animation saved as GIF instead: {gif_output}")

plt.close()