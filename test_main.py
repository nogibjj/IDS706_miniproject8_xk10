import pandas as pd

dataset = "bmi.csv"

def test_max_bmi():
    data = pd.read_csv(dataset, sep=",")  # Updated for bmi.csv and assuming the delimiter is a comma
    assert data['Bmi'].max() == 66.30134998