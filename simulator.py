import numpy as np
from tqdm import tqdm
import pandas as pd
import time
import multiprocessing
import subprocess
import json
import datetime

def run(i):
    output_str = subprocess.run(f'powershell cat in/{i:04}.txt | .\\target\\debug\\ahc034.exe > out/{i:04}.txt', shell=True, capture_output=True, text=True).stderr
    # print('output_str:', output_str)
    result = json.loads(output_str.split('\n')[0])
    return result

def main(i):
    start = time.time()
    # print(i, 'start')
    r = run(i)
    t = round(time.time()-start, 4)
    base = r['base']
    cost = r['cost']
    data = [i, base, cost, t]
    print('\r', 'end', i, end='')
    # print(i, 'end')
    return data


if __name__ == '__main__':
    start = time.time()
    print("start: ", datetime.datetime.fromtimestamp(start))
    trial = 150
    result = []
    '''
    for i in tqdm(range(trial)):
        data = main(i)
        result.append(data)
    '''
    processes = multiprocessing.cpu_count()
    with multiprocessing.Pool(processes=processes) as pool:
        data = [pool.apply_async(main, (i,)) for i in range(trial)]
        result = [d.get() for d in data]
    print()
    df = pd.DataFrame(result, columns=['i', 'base', 'cost', 'time'])
    df['score'] = round(1_000_000_000 * df['base'] / df['cost'])
    score = df['score'].sum()
    print(f'score:', format(int(score), ','), 'score mean:', format(int(score/trial), ','))
    df.to_csv('result.csv', index=False)
    print(f'end elapsed time: {time.time()-start:.2f}s')
