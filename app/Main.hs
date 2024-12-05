module Main where

import Hue.Either (mapLeft)
import Hue.Lowering (Bytecode, lower)
import Hue.Parser (ParsingError, parse)
import Hue.Typechecker (TypecheckingError, typecheck)
import System.Environment (getArgs)

data Error
  = ParsingError ParsingError
  | TypecheckingError TypecheckingError
  deriving (Show)

main :: IO ()
main = do
  args <- getArgs
  case args of
    [file] -> do
      code <- readFile file
      case pipeline code of
        Left err ->
          print err
        Right result ->
          print result
    _ -> do
      putStrLn "no file given"

pipeline :: String -> Either Error Bytecode
pipeline code = do
  ast <- mapLeft ParsingError $ parse code
  typedAst <- mapLeft TypecheckingError $ typecheck ast
  Right (lower typedAst)
