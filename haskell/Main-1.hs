module Main where
import Control.Applicative
import Data.Traversable (sequenceA)
import Data.List (tails)

transpose' :: [[a]] -> [[a]]
transpose' = getZipList . sequenceA . map ZipList
windows :: Int -> [a] -> [[a]]
windows m = transpose' . take m . tails

main :: IO ()
pairs xs = zip xs (tail xs)

increasing el = (snd el) > (fst el)

main = do
    inputs <- lines <$> readFile "input.txt"
    let numbers = map read inputs :: [Int]
    let new = filter increasing (pairs numbers)
    print (length new)

    let triples = filter increasing (pairs ( map sum (windows 3 numbers)))
    print (length triples)
