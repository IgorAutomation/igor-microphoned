
from typing import List, Callable
import sounddevice as sd
import numpy


def get_device_names() -> List[str]:
    return [d['name'] for d in sd.query_devices() if d['max_input_channels'] > 0]


def open_device_input_stream(
        device: str,
        callback: Callable[[numpy.ndarray, int, object, sd.CallbackFlags], None],
        channels: int = None,
        sample_rate: int = None,
        block_size: int = None) -> sd.InputStream:
    return sd.InputStream(
        device=device,
        channels=channels,
        samplerate=sample_rate,
        blocksize=block_size,
        callback=callback)
