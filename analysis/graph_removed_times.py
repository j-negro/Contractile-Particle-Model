import os

import matplotlib.pyplot as plt
import numpy as np

RESULTS_PATH = "./analysis/figs/"

DIR = "./analysis/data/"

DELTA_TIME = 5


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
                data[N] = {
                    "times": [],
                    "particles": [],
                }

            times = []
            counts = []

            # Read the file and get the times
            with open(DIR + file, "r") as f:
                for line in f:
                    splits = line.split(" ")
                    counts.append(int(splits[0]))
                    times.append(float(splits[1]))

            data[N]["times"].append(times)
            data[N]["particles"] = counts

    return data


def plot():
    data = read_position_data()

    fig1 = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.ylabel("Tiempo (s)")
    plt.xlabel("Particulas")

    plt.plot(data[200]["particles"], data[200]["times"][0])

    fig1.savefig(RESULTS_PATH + "removed_times.png")

    fig2 = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.ylabel("Particulas")
    plt.xlabel("Tiempo (s)")

    mean_times = np.mean(data[200]["times"], axis=0)
    std_times = np.std(data[200]["times"], axis=0)

    plt.errorbar(
        mean_times,
        data[200]["particles"],
        xerr=std_times,
        fmt="bx",
        ecolor="r",
        capsize=5,
    )

    fig2.savefig(RESULTS_PATH + "avergage_times.png")

    fig3 = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.ylabel("Caudal")
    plt.xlabel("Tiempo (s)")

    Q = []
    for time in range(int(mean_times[-1]) - DELTA_TIME):
        start_idx = -1
        end_idx = -1
        for j in range(len(mean_times)):
            if start_idx == -1 and mean_times[j] > time:
                start_idx = j

            if mean_times[j] > time + DELTA_TIME:
                end_idx = j - 1
                break

        delta_particles = (
            data[200]["particles"][end_idx] - data[200]["particles"][start_idx]
        )
        Q.append(delta_particles / DELTA_TIME)

    times = [i + DELTA_TIME / 2 for i in range(int(mean_times[-1]) - DELTA_TIME)]

    plt.scatter(times, Q)

    fig3.savefig(RESULTS_PATH + "caudal.png")

    plt.show()


if __name__ == "__main__":
    os.makedirs(RESULTS_PATH, exist_ok=True)
    plot()
