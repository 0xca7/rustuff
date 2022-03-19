import serial
import matplotlib.pyplot as plt
import matplotlib.animation as animation
from matplotlib import style

fig = plt.figure()
ax1 = fig.add_subplot(1,1,1)

ser = serial.Serial('/dev/ttyUSB0', 9600)

xs = []
ys = []

t = 0

def animate(i):
    data = ser.readline()
    raw_temp = data[1] << 8 | data[0]
    temp = (raw_temp * 0.02) - 273.1

    xs.append(float(i))
    ys.append(temp)

    ax1.clear()
    ax1.grid()
    ax1.plot(xs, ys, "r")

ani = animation.FuncAnimation(fig, animate, interval=10)
plt.show()
