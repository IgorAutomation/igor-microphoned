
from typing import List, Callable, Optional
from tflite_runtime.interpreter import Interpreter
import pkg_resources
import sounddevice as sd
import numpy
import timeit
import scipy.signal
import python_speech_features


def load_wakeword_model(model_file: str) -> Interpreter:
    interpreter = Interpreter(model_file)
    interpreter.allocate_tensors()
    return interpreter


def get_resource_filename(file: str) -> str:
    return pkg_resources.resource_filename('resources', file)


def get_devices() -> List[dict]:
    return [d for d in sd.query_devices() if d['max_input_channels'] > 0]


def get_device_names() -> List[str]:
    return [d['name'] for d in get_devices() if d['max_input_channels'] > 0]


def get_device_name_by_index(index: int) -> Optional[str]:
    names = get_device_names()
    if index < 0 or index > (len(names) - 1):
        return None
    return names[index]


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


def down_sample(data, old_fs, new_fs):
    if new_fs > old_fs:
        print("Error: target sample rate higher than original")
        exit(187)
    dec_factor = old_fs / new_fs
    if not dec_factor.is_integer():
        print("Error: can only decimate by integer factor")
        exit(187)
    resampled_data = scipy.signal.decimate(data, int(dec_factor))
    return resampled_data, new_fs


def create_mccc_handler(
        interpreter: Interpreter,
        wakeword_callback: Callable[[float], None],
        confidence_threshold: float = 0.5,
        debug_enabled: bool = False) -> Callable[[numpy.ndarray], None]:

    input_details = interpreter.get_input_details()
    output_details = interpreter.get_output_details()

    def ret(mfccs: numpy.ndarray):
        nonlocal input_details
        nonlocal output_details
        start = timeit.default_timer()
        in_tensor = numpy.float32(mfccs.reshape(1, mfccs.shape[0], mfccs.shape[1], 1))
        interpreter.set_tensor(input_details[0]['index'], in_tensor)
        interpreter.invoke()
        output_data = interpreter.get_tensor(output_details[0]['index'])
        val = float(output_data[0][0])
        if debug_enabled:
            print(f"confidence = {val}, confidence_threshold = {confidence_threshold}")
        if debug_enabled:
            print(f"Time to detect wake word {timeit.default_timer() - start}")
        if val > confidence_threshold:
            if debug_enabled:
                print(f"Detected word with confidence of {val}")
            wakeword_callback(val)

    return ret


def create_audio_stream_listener(
        mfcc_handler: Callable[[numpy.ndarray], None],
        sample_rate: int = 48000,
        resample_rate: int = 8000,
        rec_duration: float = 0.5,
        num_mfcc: int = 16,
        debug_enabled: bool = False) \
        -> Callable[[numpy.ndarray, int, object, sd.CallbackFlags], None]:

    window = numpy.zeros(int(rec_duration * resample_rate) * 2)

    def ret(data: numpy.ndarray, frames: int, time: object, flags: sd.CallbackFlags) -> None:
        start = timeit.default_timer()

        if flags:
            print("")
            print(f"Error: {flags}")

        # Remove 2nd dimension from recording sample
        data = numpy.squeeze(data)

        # Resample
        data, new_fs = down_sample(data, sample_rate, resample_rate)

        # Save recording onto sliding window
        window[:len(window) // 2] = window[len(window) // 2:]
        window[len(window) // 2:] = data

        # Compute features
        mfccs = python_speech_features.base.mfcc(window,
                                                 samplerate=new_fs,
                                                 winlen=0.256,
                                                 winstep=0.050,
                                                 numcep=num_mfcc,
                                                 nfilt=26,
                                                 nfft=2048,
                                                 preemph=0.0,
                                                 ceplifter=0,
                                                 appendEnergy=False,
                                                 winfunc=numpy.hanning)
        mfccs = mfccs.transpose()

        if debug_enabled:
            print(f"Time to calculate mfccs {timeit.default_timer() - start}")

        mfcc_handler(mfccs)

    return ret
