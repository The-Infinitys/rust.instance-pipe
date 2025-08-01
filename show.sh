#!/bin/bash

# 引数の確認
if [ $# -ne 2 ]; then
    echo "使い方: $0 ディレクトリ 拡張子"
    exit 1
fi

directory=$1
extension=$2

# ディレクトリが存在するか確認
if [ ! -d "$directory" ]; then
    echo "エラー: ディレクトリ $directory が存在しません"
    exit 1
fi

# ファイルを検索して表示
find "$directory" -type f -name "*.$extension" | while read -r file; do
    echo "$file"
    echo ""
    echo "\`\`\`"
    cat $file
    echo "\`\`\`"
    echo ""
done