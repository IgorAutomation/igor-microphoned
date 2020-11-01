from typing import Optional
import argparse
import util


def daemon_main():
    default_model_file = util.get_resource_filename('wake_word_stop_lite.tflite')

    parser = argparse.ArgumentParser(description='Igor Microphone Daemon (igormd)')
    parser.add_argument('-v', '--verbose', action='store_true',
                        help='Enables some debug logging')
    parser.add_argument('-l', '--list-devices', action='store_true',
                        help='Lists all known input device names')
    parser.add_argument('-d', '--device-name', type=str, required=False, default=None,
                        help='Sets the device name to record from (instead of device index)')
    parser.add_argument('-i', '--device-index', type=int, required=False, default=None,
                        help='The device index to use (instead of device name)')
    parser.add_argument('-w', '--wakeword-model', type=str, required=False, default=default_model_file,
                        help='The TensorFlowLite model to use for wakeword detection')
    parser.add_argument('-r', '--sample-rate', type=int, required=False, default=48_000,
                        help='Sample rate in hertz (ie: 48000)')
    parser.add_argument('-s', '--resample-rate', type=int, required=False, default=8_000,
                        help='Resample rate in hertz (ie: 8000)')
    parser.add_argument('-t', '--record-duration', type=float, required=False, default=0.5,
                        help='Record duration in seconds')
    parser.add_argument('-c', '--wakeword-confidence-threshold', type=float, required=False, default=0.5,
                        help='Confidence threshold for wakeword')

    args = parser.parse_args()

    # list devices
    if args.list_devices:
        print("")
        print("Device names:")
        for i, device_name in enumerate(util.get_device_names()):
            print(f"\t {i}: {device_name}")
        if args.verbose:
            for i, device in enumerate(util.get_devices()):
                print(f"\t {i}: {str(device)}")

    # get chosen device
    device_name: Optional[str] = None
    if args.device_name:
        device_name = args.device_name
    elif args.device_index is not None:
        device_name = util.get_device_name_by_index(args.device_index)
        if not device_name:
            print("")
            print(f"Device index f{args.device_index} was not found")
            exit(187)
    if device_name:
        if device_name not in util.get_device_names():
            print("")
            print(f"Device f{device_name} was not found")
            exit(187)
        print("")
        print(f"Chose device: {device_name}")

    def heard_wakeword(confidence: float):
        print("")
        print("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!")
        print(f"heard word with confidence of {confidence}")
        print("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!")
        print("")

    wakeword_model = util.load_wakeword_model(args.wakeword_model)

    mfcc_handler = util.create_mccc_handler(
        interpreter=wakeword_model,
        wakeword_callback=heard_wakeword,
        confidence_threshold=args.wakeword_confidence_threshold,
        debug_enabled=args.verbose)

    stream_handler = util.create_audio_stream_listener(
        mfcc_handler=mfcc_handler,
        sample_rate=args.sample_rate,
        resample_rate=args.resample_rate,
        rec_duration=args.record_duration,
        debug_enabled=args.verbose)

    with util.open_device_input_stream(
            device=device_name,
            callback=stream_handler,
            channels=1,
            sample_rate=args.sample_rate,
            block_size=int(args.sample_rate * args.record_duration)):
        while True:
            pass


if __name__ == "__main__":
    daemon_main()
