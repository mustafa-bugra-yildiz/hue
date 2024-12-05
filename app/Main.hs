module Main where

import Hue (parse)
import System.Environment (getArgs)

main :: IO ()
main = do
  args <- getArgs
  case args of
    [file] -> do
      sourceCode <- readFile file
      putStr $ show $ parse sourceCode
    _ -> do
      putStrLn "no file given"
