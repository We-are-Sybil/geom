import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import csv
import numpy as np
from matplotlib.animation import FFMpegWriter
import matplotlib.animation as animation

def create_3d_plot_upper_view(ax, points, title, colora='b', colorb='r', marker='o'):
    x, y, z = zip(*points)
    ax.scatter(x, y, z, c=colora, marker=marker)
    ax.plot(x, y, z, c=colorb, linestyle='-')
    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    ax.set_zlabel('Z')
    ax.set_title(title)
    ax.view_init(elev=90, azim=-90)

csv_file_path = './outputg.csv'  # Update this to the actual path of your CSV file

all_points = []
original_points = []
batch_ids = set()

with open(csv_file_path, 'r') as csv_file:
    csv_reader = csv.DictReader(csv_file)
    for row in csv_reader:
        batch_ids.add(row['BatchID'])
        point = (float(row['Processed_Longitude']), float(row['Processed_Latitude']), float(row['Processed_Elevation']))
        all_points.append((row['BatchID'], point))
        if row['Is_Original'].lower() == 'true':
            original_points.append((row['BatchID'], point))

print("Available BatchIDs:")
for bid in batch_ids:
    print(bid)

batch_id = input("Enter the BatchID you want to plot: ")

batch_all_points = [p for bid, p in all_points if bid == batch_id]
batch_original_points = [p for bid, p in original_points if bid == batch_id]

fig = plt.figure(figsize=(20, 8))

ax1 = fig.add_subplot(121, projection='3d')
ax2 = fig.add_subplot(122, projection='3d', sharex=ax1, sharey=ax1, sharez=ax1)

create_3d_plot_upper_view(ax1, batch_original_points, f'Original Points', marker='^')
create_3d_plot_upper_view(ax2, batch_all_points, f'All Points', marker='o')

# Set up video writer
writer = FFMpegWriter(fps=30)
video_file = 'visualization_recording.mp4'

def update(frame):
    return []

with writer.saving(fig, video_file, dpi=100):
    anim = animation.FuncAnimation(fig, update, frames=np.arange(0, 1000), interval=33)
    
    def on_move(event):
        if event.inaxes == ax1:
            ax2.view_init(elev=ax1.elev, azim=ax1.azim)
        elif event.inaxes == ax2:
            ax1.view_init(elev=ax2.elev, azim=ax2.azim)
        writer.grab_frame()

    def on_zoom(event):
        if event.inaxes == ax1:
            ax2.set_xlim3d(ax1.get_xlim3d())
            ax2.set_ylim3d(ax1.get_ylim3d())
            ax2.set_zlim3d(ax1.get_zlim3d())
        elif event.inaxes == ax2:
            ax1.set_xlim3d(ax2.get_xlim3d())
            ax1.set_ylim3d(ax2.get_ylim3d())
            ax1.set_zlim3d(ax2.get_zlim3d())
        writer.grab_frame()

    fig.canvas.mpl_connect('motion_notify_event', on_move)
    fig.canvas.mpl_connect('scroll_event', on_zoom)

    plt.tight_layout()
    plt.show()

print(f"Video saved as {video_file}")
