# Pulse-Code Modulation
#
# ffmpeg -i dame.ogg -f s16le -acodec pcm_s16le dame.pcm
#
# 播放
# ffplay -f s16le -ar 44100  dame.pcm

import numpy as np
import matplotlib.pyplot as plt

# 读取裸 PCM 文件
# dtype=np.int16 表示每个样本 16 位有符号整数
data = np.fromfile("dame.pcm", dtype=np.int16)

# 如果文件很大，可以只取前 5000 个样本画图
samples_to_plot = data[:5000]

# 生成横坐标（时间）
samplerate = 44100  # 你的 PCM 文件采样率
time = np.arange(len(samples_to_plot)) / samplerate

# 画波形
plt.figure(figsize=(10, 4))
plt.plot(time, samples_to_plot)
plt.xlabel("Time [s]")
plt.ylabel("Amplitude")
plt.title("PCM Waveform")
plt.show()
