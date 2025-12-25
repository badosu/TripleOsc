# Triple Osc

A simple triple oscillator CLAP synthesizer.

## Parameters

- Gain: -36dB - 0dB
- Attack: 0ms - 2.000ms
- Release: 0ms - 2.000ms
- For oscillators 1, 2, 3:
  - Gain: -36dB - 0dB
  - Detune: -50 cents - 50 cents
  - Wave: Sin, Saw, Triangle, Pulse

## TODO

- Make modulation work for all parameters similar to current master gain (currently only new notes get updated parameters).
- Add Coarse detuning
- Add Decay and Release
- Add Pan for each oscillator
- Add some form of waveform modulation, e.g. pulse width
- Add fine detuning for each channel and oscillator
- Add note retrigger parameter
- Add initial phase randomization parameter for each oscillator
- Add phase offset for each oscillator
- Add phase offset between channels for each oscillator

### Consider

- Add Mix (waveforms are added)
- Add Osc n,m Sync (when the primary oscillator resets the phase, so does the secondary)
- Add Osc n,m Phase modulation
- Add Osc n,m Amplitude modulation

## Reference

- Minimoog: https://images.tcdn.com.br/img/img_prod/1156377/minimoog_model_d_renovado_2022_2003_2_d41c03d836271aa556989effda9ab35a.jpg
- LMMS 3xOsc: https://docs.lmms.io/user-manual/instruments/triple-oscillator
