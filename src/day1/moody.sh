#!/usr/bin/env zsh

awk '{ print $1 }' <input | sort -n >leftside
awk '{ print $2 }' <input | sort -n >rightside
awk '{ if(a[FNR] == 0){ a[FNR] = $1 } else{ a[FNR] -= $1}} END { for(i = 1; i <= NR; i++){ if(a[i] < 0){ a[i] *= -1} total += a[i]} print(total)}' leftside rightside
awk 'FILENAME == "rightside" { a[$1]+=1 } FILENAME == "leftside" {total += $1*a[$1]} END{ print total}' rightside leftside