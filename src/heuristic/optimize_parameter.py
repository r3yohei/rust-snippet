# Rustのプログラムへコマンドライン引数を渡して実行し，最も良いスコアを出したハイパーパラメータを保存する
# 以下は設定例

import os
import json
import time
import numpy as np
import subprocess
import joblib
import statistics
import optuna

n_parallel = 12
n_files = 100

def calc_score_each(
        seed: int,
        TO: float,
        T1: float,
        MULTI_START: float,
        PROB_2OPT: int,
        PROB_3OPT: int,
        PROB_INSERT: int,
        PROB_MOVE_STATION: int,
        PROB_INSERT_STATION: int,
        MAX_DELTA: float, 
        MIN_DELTA: float, 
        KICK_THRESHOLD: int,
        PROB_DOUBLE_BRIDGE: float,
        CALC_TEMP_FREQUENCY: int,
        NUM_STATION: int
    ):
    in_file = f"testcases/a/in/{seed:04}.txt"
    out_file = f"testcases/a/out/{seed:04}.txt"
    # "optuna"という名前のバイナリを作ることを前提とする
    proc = subprocess.Popen(f"RUST_BACKTRACE=1 cargo run --release --bin optuna < {in_file} > {out_file} \
                            {TO} {T1} {MULTI_START} {PROB_2OPT} {PROB_3OPT} {PROB_INSERT} {PROB_MOVE_STATION} {PROB_INSERT_STATION} \
                            {MAX_DELTA} {MIN_DELTA} {KICK_THRESHOLD} {PROB_DOUBLE_BRIDGE} {CALC_TEMP_FREQUENCY} {NUM_STATION}",
                            shell=True,
                            stderr=subprocess.PIPE,
                            text=True
                            )
    # Rustのプログラムの実行結果の標準エラーをcontent変数に保持する
    content = proc.stderr.read()
    # スコアの標準エラー内での表示位置に注意しつつ，スコアの値を得る
    score = content.split("\n")[-3].split(" ")[-1]
    # int型にしないと動かなかった
    return int(score)

def calc_scores(
        TO: float,
        T1: float,
        MULTI_START: float,
        PROB_2OPT: int,
        PROB_3OPT: int,
        PROB_INSERT: int,
        PROB_MOVE_STATION: int,
        PROB_INSERT_STATION: int,
        MAX_DELTA: float,
        MIN_DELTA: float,
        KICK_THRESHOLD: int,
        PROB_DOUBLE_BRIDGE: float,
        CALC_TEMP_FREQUENCY: int,
        NUM_STATION: int
    ):
    # 並列実行
    scores = joblib.Parallel(n_jobs=n_parallel)(
        joblib.delayed(calc_score_each)(i, TO, T1, MULTI_START, PROB_2OPT, PROB_3OPT, PROB_INSERT, PROB_MOVE_STATION, PROB_INSERT_STATION, MAX_DELTA, MIN_DELTA, KICK_THRESHOLD, PROB_DOUBLE_BRIDGE, CALC_TEMP_FREQUENCY, NUM_STATION) for i in range(n_files)
    )
    return scores

def objective(trial: optuna.trial.Trial):
    start = time.time()
    TO = trial.suggest_float("TO", 10000.0, 10000000.0)
    T1 = trial.suggest_float("T1", 10.0, 10000.0)
    MULTI_START = trial.suggest_int("MULTI_START", 1, 10)
    lower = np.random.randint(30, 41)
    upper = np.random.randint(50, 71)
    PROB_2OPT = trial.suggest_int("PROB_2OPT", lower, upper)
    lower = upper
    upper = np.random.randint(lower, 85)
    PROB_3OPT = trial.suggest_int("PROB_3OPT", lower, upper)
    lower = upper
    upper = np.random.randint(lower, 90)
    PROB_INSERT = trial.suggest_int("PROB_INSERT", lower, upper)
    lower = upper
    upper = np.random.randint(lower, 95)
    PROB_MOVE_STATION = trial.suggest_int("PROB_MOVE_STATION", lower, upper)
    lower = upper
    upper = np.random.randint(lower, 99)
    PROB_INSERT_STATION = trial.suggest_int("PROB_INSERT_STATION", lower, upper)
    MAX_DELTA = trial.suggest_float("MAX_DELTA", 30.0, 500.0)
    MIN_DELTA = trial.suggest_float("MIN_DELTA", 1.0, 20.0)
    KICK_THRESHOLD = trial.suggest_int("KICK_THRESHOLD", 5000, 100000)
    PROB_DOUBLE_BRIDGE = trial.suggest_float("PROB_DOUBLE_BRIDGE", 0.1, 1.0)
    CALC_TEMP_FREQUENCY = trial.suggest_int("CALC_TEMP_FREQUENCY", 1, 5000)
    NUM_STATION = trial.suggest_int("NUM_STATION", 5, 8)

    scores = calc_scores(TO, T1, MULTI_START, PROB_2OPT, PROB_3OPT, PROB_INSERT, PROB_MOVE_STATION, PROB_INSERT_STATION, MAX_DELTA, MIN_DELTA, KICK_THRESHOLD, PROB_DOUBLE_BRIDGE, CALC_TEMP_FREQUENCY, NUM_STATION)
    print(f"elapsed: {time.time() - start}")
    return statistics.mean(scores)


if __name__ == "__main__":
    # スコアの最大化か最小化かに注意
    study = optuna.create_study(
        direction="maximize",
        storage="sqlite:///hogehoge.db",
    )
    study.optimize(objective, n_trials=5000)

    print(study.best_value)
    print(study.best_params)
    
    with open("./optuna_args.json", "w") as f:
        json.dump(study.best_params, f, indent=4)

    print("done!")