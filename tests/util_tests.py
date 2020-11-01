
from microphoned import util
import sounddevice as sd
import numpy
import os


def test_get_resource_filename():
    filename = util.get_resource_filename('data/wake_word_stop_lite.tflite')
    assert filename is not None
    assert os.path.exists(filename)


def test_get_device_names():
    names = util.get_device_names()
    assert names
    assert len(names) > 0


def test_get_device_name_by_index():
    names = util.get_device_names()
    for i, name in enumerate(names):
        assert name == util.get_device_name_by_index(i)
    assert util.get_device_name_by_index(-1) is None
    assert util.get_device_name_by_index(len(names)) is None


def test_open_device_input_stream():
    device = util.get_device_names()[0]
    assert device

    times_called: int = 0

    def test_callback(indata: numpy.ndarray, frames: int, time: object, status: sd.CallbackFlags):
        print(str(indata))
        print(str(frames))
        print(str(time))
        print(str(status))
        nonlocal times_called
        times_called = times_called + 1

    with util.open_device_input_stream(
            device=device,
            callback=test_callback):
        while times_called < 10:
            pass

    assert times_called >= 10
