module Lib
    ( someFunc
    ) where

import System.IO

someFunc :: IO ()
someFunc = do
    contents <- readFile "input.txt"
    putStrLn (part1 contents)
--    putStrLn (part2 contents)

part1 :: String -> String
part1 contents = output
    where
        vs = map (read:: String -> Int).lines $ contents
        zs = zip vs (tail vs)
        diffs = map (\(a, b) -> if b - a > 0 then 1 else 0) zs
        output = show.sum $ diffs

        

