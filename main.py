import pandas as pd
import time
from mylib.lib import get_max
import psutil

# Record the start time
start_time = time.time()

if __name__ == "__main__":
    df = pd.read_csv("bmi.csv", sep=";")
    print(get_max(df))
    end_time = time.time()

# Calculate the elapsed time
elapsed_time = end_time - start_time
cpu_percent = psutil.cpu_percent()
memory_info = psutil.virtual_memory()

print(f"Elapsed time: {elapsed_time:.4f} seconds")
print(f"CPU Usage: {cpu_percent}%")
print(f"Memory Usage: {memory_info.percent}%")