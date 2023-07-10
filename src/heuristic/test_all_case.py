# ローカルでテストケースを並列実行するスクリプト
# FIXME: joblibに変更する

import pandas as pd
import subprocess
import os

in_folder = "./testcases/a/in/"
out_folder = "./testcases/a/out/"
os.makedirs(in_folder, exist_ok=True)
os.makedirs(out_folder, exist_ok=True)
proc_list = []
max_process = 4 # 並列実行する数、環境によって変える
num_cases = 100

# 並列実行
df = pd.DataFrame()
for i in range(num_cases):
    print(f"executing case {i:04d}...")
    # 実行するバイナリの名前に変更する
    proc = subprocess.Popen(f"cargo run --release --bin hogehoge-a < {in_folder}{i:04d}.txt > {out_folder}{i:04d}.txt",
                            shell=True,
                            stderr=subprocess.PIPE,
                            text=True
                            )
    proc_list.append([proc, f"{i:04d}"])
    if (i+1) % max_process == 0 or (i+1) == num_cases:
        # 並列実行数の最大に達したか、全てのケースを実行できたら
        for subproc in proc_list:
            # Rustのプログラムの実行結果の標準エラーをcontent変数に保持する
            content = subproc[0].stderr.read()
            # スコアの標準エラー内での表示位置に注意しつつ，スコアの値を得る
            score = content.split("\n")[-3].split(" ")[-1]
            tmp = pd.Series(data=[subproc[1], score],
                            index=["case", "score"]
                            )
            df = df.append(tmp, ignore_index=True)
            # proc_list内のprocessが終わるまで待つ
            subproc[0].wait()
        # proc_listを初期化
        proc_list = []

df.to_csv("./result.csv", index=False)
print("done!")