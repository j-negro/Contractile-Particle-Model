import os

import matplotlib.pyplot as plt
import numpy as np

RESULTS_PATH = "./analysis/figs/"

DIR = "./analysis/data/"


def read_position_data():
    data: dict[int, dict[float, list[list[float]]]] = {}

    for file in os.listdir(DIR):
        if file.endswith(".txt"):
            # remove extension
            filename, _ = os.path.splitext(file)

            filename = filename.split("_")

            d = float(filename[0])
            N = int(filename[1])

            if N not in data.keys():
                data[N] = []

            times = []
            counts = []

            # Read the file and get the times
            with open(DIR + file, "r") as f:
                for line in f:
                    splits = line.split(" ")
                    times.append(float(splits[0]))
                    counts.append(int(splits[1]))

            data[N].append(
                {
                    "times": times,
                    "counts": counts,
                }
            )

    print(data[200])
    return data


def plot():
    data = read_position_data()

    fig = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.ylabel("Tiempo (s)")
    plt.xlabel("Particulas")

    plt.plot(data[200][0]["counts"], data[200][0]["times"])

    fig.savefig(RESULTS_PATH + "removed_times.png")

    plt.show()


if __name__ == "__main__":
    os.makedirs(RESULTS_PATH, exist_ok=True)
    plot()
