
from microphoned import sound
import sounddevice as sd
import numpy


def test_get_device_names():
    names = sound.get_device_names()
    assert names
    assert len(names) > 0


def test_open_device_input_stream():
    device = sound.get_device_names()[0]
    assert device

    times_called: int = 0

    def test_callback(indata: numpy.ndarray, frames: int, time: object, status: sd.CallbackFlags):
        print(str(indata))
        print(str(frames))
        print(str(time))
        print(str(status))
        nonlocal times_called
        times_called = times_called + 1

    with sound.open_device_input_stream(
            device=device,
            callback=test_callback):
        while times_called < 10:
            pass

    assert times_called >= 10
