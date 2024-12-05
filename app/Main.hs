module Main where

import Hue.Parser (ParsingError, parse)
import Hue.Typechecker (TypecheckingError, TypedNode, typecheck)
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

pipeline :: String -> Either Error TypedNode
pipeline code = do
  ast <- mapLeft ParsingError $ parse code
  typedAst <- mapLeft TypecheckingError $ typecheck ast
  Right typedAst

mapLeft :: (a -> c) -> Either a b -> Either c b
mapLeft f = mapBoth f id

mapBoth :: (a -> c) -> (b -> d) -> Either a b -> Either c d
mapBoth f _ (Left x) = Left (f x)
mapBoth _ f (Right x) = Right (f x)
