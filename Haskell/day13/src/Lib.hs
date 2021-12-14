module Lib
    ( someFunc,
      foldPaper
    ) where

import Data.Foldable (maximumBy)
import Data.Function (on)

someFunc :: IO ()
someFunc = putStrLn "someFunc"

data Direction = X | Y deriving (Eq, Enum)


foldPaper :: [(Int, Int)] -> Direction -> Int -> [(Int, Int)]
foldPaper paper direction value = map (\(x, y) -> if direction == Y && x > value
                                                then (x - (x - value) * 2, y)
                                              else if direction == X && y > value
                                                  then (x, y - (y - value) * 2)
                                              else (x, y) ) paper 

-- Return the max sizes of [(x, y)]
dimensions :: [(Int, Int)] -> (Int, Int)
dimensions paper = 
  where (max_x, _) = maximumBy (\(x1, _) (x2, _) -> compare x1 x2) paper
        (_, y = maximumBy (\(_, y1) (_, y2) -> compare y1 y2) paper