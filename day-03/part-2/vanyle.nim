from times import cpuTime
from os import paramStr

import strutils

proc run(s: var string): string =
    var res = 0
    for l in s.splitLines:
        var maxValues: array[12, int8]
        var lastIdx: uint8 = 0

        for c in (0.int8)..<12.int8:
            var currMaxIdx: uint8 = 0
            var maxDigit: int8 = -1
            for i in lastIdx..<(l.len-(11-c)).uint8:
                var d = l[i]
                var maybeMax = (cast[uint8](d) - cast[uint8]('0')).int8
                if maybeMax > maxDigit:
                    maxDigit = maybeMax
                    currMaxIdx = i
            maxValues[c] = maxDigit
            lastIdx = currMaxIdx + 1

        var n = 0
        var acc = 1
        for i in countdown(11, 0):
            n += maxValues[i] * acc
            acc *= 10
        res += n
    return $res


var input: string = paramStr(1)

let t0 = cpuTime()
let output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
