import json
from pathlib import Path
from os import system
from tempfile import NamedTemporaryFile

from wasmtime import Config, Engine, Linker, Module, Store, WasiConfig, ExitTrap, Func

BUILD_DETAIL_NAME = ".howtobuild.json"


def run_solution(
    wasm_path: Path, input_data: str, print_log: bool = True
) -> tuple[str, str]:
    sol1: str = ""
    sol2: str = ""

    def print_out(data: bytes):
        nonlocal sol1, sol2
        line = data.decode(errors="replace")
        if line.startswith("SOLUTION PART 1: "):
            sol1 = line.split(" ")[-1]
        if line.startswith("SOLUTION PART 2: "):
            sol2 = line.split(" ")[-1]
        if print_log:
            print(line)

    engine_cfg = Config()
    # I don't care about fuel, but this is how to do it:
    # engine_cfg.consume_fuel = True
    # engine_cfg.cache = True

    linker = Linker(Engine(engine_cfg))
    linker.define_wasi()

    module = Module.from_file(linker.engine, wasm_path)
    config = WasiConfig()
    with NamedTemporaryFile(delete_on_close=False) as fw:
        fw.write(input_data.encode())
        fw.close()

        config.stderr_custom = print_out
        config.stdout_custom = print_out
        config.stdin_file = fw.name

        store = Store(linker.engine)
        # TODO what does this do? The docstring is TODO lol
        store.set_wasi(config)
        instance = linker.instantiate(store, module)

        # _start is the default wasi main function
        start = instance.exports(store)["_start"]
        assert isinstance(start, Func)
        mem = instance.exports(store)["memory"]
        try:
            start(store)
        except ExitTrap as et:
            # looks like return cod 0 is still handled like an exception
            if et.code != 0:
                raise et
        return (sol1, sol2)


def main():
    for folder in Path(".").glob("day*"):
        print(f"Found {folder}")
        day = folder.name.split("_")[0]
        wasm_path = build_solution(folder)
        with open(f"inputs/{day}.txt") as fr:
            s1, s2 = run_solution(wasm_path, fr.read(), print_log=False)
            print(f"solutions from {folder} {s1, s2}")


def build_solution(path: Path) -> Path:
    specs_path = path / BUILD_DETAIL_NAME
    if not specs_path.exists():
        raise FileNotFoundError(f"no file {BUILD_DETAIL_NAME} found at {path}")
    with open(specs_path) as sr:
        specs = json.load(sr)
        if specs["language"] == "zig":
            main_file = specs.get("main", "main.zig")
            retcode = system(
                f"cd {path.absolute()} && zig build-exe {main_file} -target wasm32-wasi -fno-entry -rdynamic -femit-bin={main_file}.wasm"
            )
            if retcode != 0:
                raise OSError(f"Build failed with code {retcode}")
            return path / f"{main_file}.wasm"
        elif specs["language"] == "golang":
            main_file = specs.get("main", "main.go")
            retcode = system(
                f"cd {path.absolute()} && GOOS=wasip1 GOARCH=wasm go build -o {main_file}.wasm {main_file}"
            )
            if retcode != 0:
                raise OSError(f"Build failed with code {retcode}")
            return path / f"{main_file}.wasm"
        else:
            raise ValueError(f"Unknown language in {specs}")


if __name__ == "__main__":
    main()
