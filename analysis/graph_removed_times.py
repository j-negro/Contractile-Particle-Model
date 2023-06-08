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

            if (N, d) not in data.keys():
                data[(N, d)] = {
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

            data[(N, d)]["times"].append(times)
            data[(N, d)]["particles"] = counts

    return data


def plot(data, sufix):
    fig1 = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.ylabel("Tiempo (s)")
    plt.xlabel("Partículas")

    for key in data.keys():
        plt.plot(
            data[key]["particles"],
            data[key]["times"][0],
            label=f"N = {key[0]}, d = {key[1]}",
        )
    plt.legend()

    fig1.savefig(RESULTS_PATH + f"removed_times_{sufix}.png")

    fig2 = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.ylabel("Partículas")
    plt.xlabel("Tiempo (s)")

    mean_times = {}

    for key in data.keys():
        mean_times[key] = np.mean(data[key]["times"], axis=0)
        std_times = np.std(data[key]["times"], axis=0)
        plt.errorbar(
            mean_times[key],
            data[key]["particles"],
            xerr=std_times,
            fmt="x",
            ecolor="r",
            label=f"N = {key[0]}, d = {key[1]}",
        )

    plt.legend()

    fig2.savefig(RESULTS_PATH + f"avergage_times_{sufix}.png")

    fig3 = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.ylabel("Caudal")
    plt.xlabel("Tiempo (s)")

    Q = {}
    for key in data.keys():
        Q[key] = []
        for time in range(int(mean_times[key][-1]) - DELTA_TIME):
            start_idx = -1
            end_idx = -1
            for j in range(len(mean_times[key])):
                if start_idx == -1 and mean_times[key][j] > time:
                    start_idx = j

                if mean_times[key][j] > time + DELTA_TIME:
                    end_idx = j - 1
                    break

            delta_particles = (
                data[key]["particles"][end_idx] - data[key]["particles"][start_idx]
            )
            Q[key].append(delta_particles / DELTA_TIME)

        times = [
            i + DELTA_TIME / 2 for i in range(int(mean_times[key][-1]) - DELTA_TIME)
        ]

        plt.scatter(
            times,
            Q[key],
            label=f"N = {key[0]}, d = {key[1]}",
        )

    plt.legend()

    fig3.savefig(RESULTS_PATH + f"caudal_{sufix}.png")

    fig4 = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.ylabel("Caudal")
    plt.xlabel("N")

    x = []
    y = []
    for key in data.keys():
        q = np.array(Q[key][12:64])
        y.append(np.mean(q))
        x.append(key[0])
        plt.errorbar(
            key[0],
            np.mean(q),
            yerr=np.std(q),
            fmt="bx",
            ecolor="r",
        )

    m, b = np.polyfit(x, y, 1)

    plt.plot(x, m * np.array(x) + b, label=f"{m:.3f}x + {b:.3f}")

    plt.legend()

    fig4.savefig(RESULTS_PATH + f"caudales_{sufix}.png")


if __name__ == "__main__":
    os.makedirs(RESULTS_PATH, exist_ok=True)
    data = read_position_data()

    only_first = {(200, 1.2): data[200, 1.2]}

    plot(only_first, "only_one")

    plot(data, "all")

    plt.show()
