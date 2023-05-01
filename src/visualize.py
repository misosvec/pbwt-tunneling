import matplotlib.pyplot as plt

original_haplotypes_data = []
wg_data = []
wg_from_encoded_data = []

for i in [5000, 25000, 50000, 100000, 200000, 500000, 1000000, 2000000, 5000000, 10000000]:
    haplotypes = open("haplotypes_" + str(i) + ".txt", "r")
    haplotype_count = len(haplotypes.readlines())
    haplotype_variable_sites = 100 #fixed length for now
    haplotypes_bytes_size = haplotype_count * haplotype_variable_sites / 8
    original_haplotypes_data.append((haplotype_count, haplotype_variable_sites, haplotypes_bytes_size))
    haplotypes.close()

    wg = open("wg_" + str(i) + ".txt", "r")
    lines = wg.readlines()
    wg_bytes_size = len(lines[0]) * 2 / 8
    wg_data.append(wg_bytes_size)
    wg.close()

    wg_from_encoded = open("wg_from_encoded_" + str(i) + ".txt", "r")
    lines = wg_from_encoded.readlines()
    wg_bytes_size = len(lines[0]) * 2 / 8
    wg_from_encoded_data.append(wg_bytes_size)
    wg_from_encoded.close()

# Plotting
fig, ax = plt.subplots()
haplotypes_counts = [i[0] for i in original_haplotypes_data]
variable_sites = 100
haplotypes_bytes_size = [i[2] for i in original_haplotypes_data]
ax.plot(haplotypes_counts, haplotypes_bytes_size, label="haplotypes")
ax.plot(haplotypes_counts, wg_data, label="wheeler graph")
ax.plot(haplotypes_counts, wg_from_encoded_data, label="wheeler graph from encoded haplotypes")
ax.set_xlabel("number of haplotypes (100 variable sites)")
ax.set_ylabel("size in bytes")
ax.set_title("comparing haplotypes in standard and wheeler graph format")
ax.legend()
plt.savefig("haplotypes_size_visualization.png")