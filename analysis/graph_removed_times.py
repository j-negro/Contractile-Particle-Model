import os

import matplotlib.pyplot as plt
import numpy as np
from mpl_toolkits.axes_grid1.inset_locator import mark_inset, zoomed_inset_axes

RESULTS_PATH = "./analysis/figs/"

DIR = "./analysis/data/"

DELTA_TIME = 5


def read_position_data():
    data: dict[tuple[int, float], dict[str, list[list[float]]]] = {}

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
    plt.xlabel("Partículas que salieron del recinto")

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
    plt.ylabel("Partículas que salieron del recinto")
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

    plt.legend(
        loc="lower right",
    )

    # Create inset axes object with desired zoom level
    axin = zoomed_inset_axes(
        plt.gca(),
        4,
        loc=2,
        borderpad=2,
    )

    for key in data.keys():
        mean_times[key] = np.mean(data[key]["times"], axis=0)
        std_times = np.std(data[key]["times"], axis=0)
        axin.errorbar(
            mean_times[key],
            data[key]["particles"],
            xerr=std_times,
            fmt="x",
            ecolor="r",
            label=f"N = {key[0]}, d = {key[1]}",
        )

    axin.set_xlim(10, 15)
    axin.set_ylim(20, 60)
    plt.yticks(visible=True)
    plt.xticks(visible=True)

    # Show the lines zooming in
    mark_inset(
        plt.gca(),
        axin,
        loc1=1,
        loc2=3,
        fc="none",
        ec="0.5",
    )

    fig2.savefig(RESULTS_PATH + f"avergage_times_{sufix}.png")

    fig3 = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.ylabel("Caudal (particulas / s)")
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
    plt.ylabel("Caudal promedio (particulas / s)")
    plt.xlabel("Ancho de la puerta (m)")

    x = []
    y = []
    for key in data.keys():
        q = np.array(Q[key][12:64])
        y.append(np.mean(q))
        x.append(key[1])
        plt.errorbar(
            key[1],
            np.mean(q),
            yerr=np.std(q),
            fmt="bx",
            ecolor="r",
        )

    # Linear regression
    m = (np.dot(x, y) - 4 * np.mean(x) * np.mean(y)) / (
        np.dot(x, x) - 0.25 * np.square(np.sum(x))
    )
    b = np.mean(y) - m * np.mean(x)

    plt.plot(x, m * np.array(x) + b, label=f"{m:.3f}x + {b:.3f}")

    plt.legend()

    fig4.savefig(RESULTS_PATH + f"caudales_{sufix}.png")

    fig5 = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.ylabel("Error del ajuste (partículas / s)²")
    plt.xlabel("Pendiente (partículas / (m*s))")

    def calculate_error(x, y, m):
        mean_x = np.mean(x)
        mean_y = np.mean(y)

        error = 0
        for point in range(len(x)):
            error += np.square(y[point] - m * x[point])

        return error

    m_values = np.linspace(1.2, 2.5, 10000)
    errors = []
    for point_m in m_values:
        errors.append(calculate_error(x, y, point_m))

    plt.plot(m_values, errors)

    m = m_values[errors.index(min(errors))]
    error = min(errors)

    # Show a point in the plot with its coordinates
    plt.plot(m, error, "rx")
    plt.annotate(
        f"({m:.3f}, {error:.3f})",
        (m_values[errors.index(min(errors))], min(errors)),
        textcoords="offset points",
        xytext=(100, -10),
        ha="center",
    )

    fig5.savefig(RESULTS_PATH + f"error_{sufix}.png")

    fig6 = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.ylabel("Caudal promedio (particulas / s)")
    plt.xlabel("Ancho de la puerta (m)")

    for key in data.keys():
        q = np.array(Q[key][12:64])
        plt.errorbar(
            key[1],
            np.mean(q),
            yerr=np.std(q),
            fmt="bx",
            ecolor="r",
        )

    plt.plot(
        x,
        m * np.array(x),
        label=f"{m:.3f}x",
    )

    plt.legend()

    fig6.savefig(RESULTS_PATH + f"caudales_{sufix}.png")


if __name__ == "__main__":
    os.makedirs(RESULTS_PATH, exist_ok=True)
    data = read_position_data()

    only_first = {(200, 1.2): data[200, 1.2]}

    plot(only_first, "only_one")

    plot(data, "all")

    plt.show()
