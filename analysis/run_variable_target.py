import os
import subprocess

RUNS = 10


RESULTS_PATH = "./analysis/data/"

DATA = [(1.2, 200), (1.8, 260), (2.4, 320), (3.0, 380)]

os.makedirs(RESULTS_PATH, exist_ok=True)
for d, N in DATA:
    print(f"Starting runs d {d} and N {N}")
    for run_idx in range(RUNS):
        subprocess.run(
            [
                "./target/release/contractile-particle-model",
                "-p",
                str(N),
                "-t",
                str(d),
                "--data-output-path",
                RESULTS_PATH + f"{d}_{N}_{run_idx}.txt",
            ],
            capture_output=True,
            text=True,
        )
