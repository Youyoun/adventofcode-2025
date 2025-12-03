from times import cpuTime
from os import paramStr
import strutils

proc run(s: var string): string =
    var res = 0
    for l in s.splitLines:
        var max: uint8 = 0
        var maxIdx: uint8 = 0
        for i in (0.uint8)..<(l.len-1).uint8:
            var d = l[i]
            var maybeMax = (cast[uint8](d) - cast[uint8]('0'))
            if maybeMax > max:
                max = maybeMax
                maxIdx = i
        var max2: uint8 = 0
        for i in (maxIdx+1)..<l.len.uint8:
            var d = l[i]
            var maybeMax = (cast[uint8](d) - cast[uint8]('0'))
            if maybeMax > max2:
                max2 = maybeMax
        res += max.int * 10 + max2.int
    return $res

var input: string = paramStr(1)

let t0 = cpuTime()
let output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
