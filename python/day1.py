import numpy as np
from typing import *


def readData(filename:str) -> Tuple[np.ndarray]:

    col1, col2 = [], []
    with open(filename, 'r') as f:
        for line in f:
            val1, val2 = line.split()
            col1.append(val1)
            col2.append(val2)
    return (np.array(col1, dtype=np.int32), np.array(col2, dtype=np.int32))

def part1(col1:np.ndarray, col2:np.ndarray) -> None:
    """given 2 list of numbers compute the delta sum of the 2 increasingly sorted lists

    :param col1: unsorted data col1
    :type col1: np.ndarray
    :param col2: unsorted data col2
    :type col2: np.ndarray
    """
    col1 = np.sort(col1)
    col2 = np.sort(col2)
    diff = np.abs(col1 - col2)
    diffSum = diff.sum()
    print(f'Part 1: {diffSum}') 

def part2(col1:np.ndarray, col2:np.ndarray) -> None:
    """given 2 list of numbers, compute the similarity score which is
    col1 numbers * # times it appears in col2 then add all of these up

    :param col1: unsorted data col1
    :type col1: np.ndarray
    :param col2: unsorted data col2
    :type col2: np.ndarray
    """
    col1Unique, col1UniqueCount = np.unique(col1, return_counts=True)
    col2Unique, col2UniqueCount = np.unique(col2, return_counts=True)
    lookup_col2Count = dict(zip(col2Unique,col2UniqueCount))
    total = 0
    for col1Val, col1ValCount in zip(col1Unique, col1UniqueCount):
        total += (col1Val * lookup_col2Count.get(col1Val,0)) * col1ValCount
    print(f'Part 2: {total}')


if __name__ == '__main__':
    filename = 'day1.txt'
    col1, col2 = readData(filename)
    part1(col1, col2)
    part2(col1, col2)
