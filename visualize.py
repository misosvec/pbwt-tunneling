import matplotlib.pyplot as plt
import os

haplotypes_count = []
haplotypes_size = []
wg_size = []
wg_from_encoded_size = []
haplotypes_bzip2_size = []
encoded_haplotypes_bzip2_size = []

variable_sites = 10000

for i in [5000, 50000, 100000, 200000, 500000, 1000000]:
    filename = "haplotypes_" + str(i) + ".txt"
    haplotypes = open(filename, "r")
    haplotype_count = len(haplotypes.readlines())
    haplotypes_bytes_size = haplotype_count * variable_sites / 8
    haplotypes_size.append(os.path.getsize(filename) / 1024 ** 2)
    haplotypes_count.append(haplotype_count)
    haplotypes.close()

    filename = "wg_" + str(i) + ".txt"
    wg = open(filename, "r")
    wg_size.append(os.path.getsize(filename) / 1024 ** 2)
    wg.close()

    filename = "wg_from_encoded_" + str(i) + ".txt"
    wg_from_encoded = open(filename, "r")
    wg_from_encoded_size.append(os.path.getsize(filename) / 1024 ** 2)
    wg_from_encoded.close()

    filename = "haplotypes_" + str(i) + ".txt.bz2"
    haplotypes_bzip2_size.append(os.path.getsize(filename) / 1024 ** 2)

    filename = "haplotypes_encoded_" + str(i) + ".txt.bz2"
    encoded_haplotypes_bzip2_size.append(os.path.getsize(filename) / 1024 ** 2)

fig, ax = plt.subplots()
ax.scatter(haplotypes_count, haplotypes_size, label="haplotypes")
ax.scatter(haplotypes_count, wg_size, label="wheeler graph from haplotypes")
ax.scatter(haplotypes_count, wg_from_encoded_size, label="wheeler graph from encoded haplotypes")
ax.scatter(haplotypes_count, encoded_haplotypes_bzip2_size, label="b2zipped encoded haplotypes")
ax.scatter(haplotypes_count, haplotypes_bzip2_size, label="b2zipped haplotypes")

ax.set_xticks(haplotypes_count)
ax.set_xlabel("number of haplotypes (" + str(variable_sites) + " variable sites)")
ax.set_ylabel("size (MB)")

ax.legend()
plt.xticks(rotation=37, fontsize=8)
plt.savefig("haplotypes_size_visualization.png")