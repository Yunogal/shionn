#include <iostream>
#include <windows.h>

// Pulse-Code Modulation
// 播放裸 PCM 数据
// 参数说明:
//   pcmData: PCM 数据缓冲区指针
//   dataSize: PCM 数据大小（字节）
//   sampleRate: 采样率（Hz）
//   bitsPerSample: 每样本位数（8 或 16）
//   channels: 声道数（1=单声道, 2=立体声）
bool PlayPCM(const BYTE *pcmData, DWORD dataSize, DWORD sampleRate,
             WORD bitsPerSample, WORD channels) {
  if (!pcmData || dataSize == 0)
    return false;

  // 设置音频格式
  WAVEFORMATEX wfx = {0};
  wfx.wFormatTag = WAVE_FORMAT_PCM;
  wfx.nChannels = channels;
  wfx.nSamplesPerSec = sampleRate;
  wfx.wBitsPerSample = bitsPerSample;
  wfx.nBlockAlign = (bitsPerSample / 8) * channels;
  wfx.nAvgBytesPerSec = sampleRate * wfx.nBlockAlign;
  wfx.cbSize = 0;

  HWAVEOUT hWaveOut = nullptr;
  if (waveOutOpen(&hWaveOut, WAVE_MAPPER, &wfx, 0, 0, CALLBACK_NULL) !=
      MMSYSERR_NOERROR) {
    std::cerr << "Failed to open wave out device." << std::endl;
    return false;
  }

  WAVEHDR whdr = {0};
  whdr.lpData = (LPSTR)pcmData;
  whdr.dwBufferLength = dataSize;
  whdr.dwFlags = 0;

  if (waveOutPrepareHeader(hWaveOut, &whdr, sizeof(WAVEHDR)) !=
      MMSYSERR_NOERROR) {
    std::cerr << "Failed to prepare header." << std::endl;
    waveOutClose(hWaveOut);
    return false;
  }

  if (waveOutWrite(hWaveOut, &whdr, sizeof(WAVEHDR)) != MMSYSERR_NOERROR) {
    std::cerr << "Failed to write PCM data." << std::endl;
    waveOutUnprepareHeader(hWaveOut, &whdr, sizeof(WAVEHDR));
    waveOutClose(hWaveOut);
    return false;
  }

  // 等待播放完成
  while (!(whdr.dwFlags & WHDR_DONE)) {
    Sleep(10);
  }

  waveOutUnprepareHeader(hWaveOut, &whdr, sizeof(WAVEHDR));
  waveOutClose(hWaveOut);

  return true;
}

// 示例使用
int main(int argc, char **argv) {
  // 这里假设 pcmBuffer 已经存放裸 PCM 数据，大小为 pcmSize
  BYTE *pcmBuffer = nullptr;
  DWORD pcmSize = 0;

  // 读取 PCM 文件示例
  FILE *f = fopen(argv[1], "rb");
  if (!f) {
    std::cerr << "Cannot open PCM file." << std::endl;
    return -1;
  }
  fseek(f, 0, SEEK_END);
  pcmSize = ftell(f);
  fseek(f, 0, SEEK_SET);

  pcmBuffer = new BYTE[pcmSize];
  fread(pcmBuffer, 1, pcmSize, f);
  fclose(f);

  // 播放 PCM：44100Hz, 16位, (1)单声道(Mono) (2)立体声(Stereo)
  PlayPCM(pcmBuffer, pcmSize, 44100, 16, 1);

  delete[] pcmBuffer;
  return 0;
}

// g++ examples/dame-playback.cpp -lwinmm & a examples/dame.pcm
